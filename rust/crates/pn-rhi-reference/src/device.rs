// Pention Engine - pn-rhi-reference/device.rs
// Requirement: PN-RND-003, PN-OPS-007
// Decision:    ADR-0005, ADR-0006
//
// The validating replay.
//
// State is built up from the barriers as they execute, and pass intents are then
// checked against it. Nothing here consults the rules the derivation used - see
// the crate comment for why that direction matters.

use std::collections::BTreeMap;
use std::fmt;

use pn_rhi::{
    Access, Arrangement, Barrier, Pass, PassId, ResourceId, ResourceKind, Scope, Stages, Timeline,
};

use crate::trace::Trace;

/// A correctness failure found during replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hazard {
    /// A pass used a resource in an arrangement the barriers did not leave it
    /// in. Either a barrier is missing, or one names the wrong destination.
    WrongArrangement {
        position: usize,
        pass: String,
        resource: ResourceId,
        expected: Arrangement,
        actual: Arrangement,
    },
    /// A barrier claimed to move a resource out of an arrangement it was not in.
    /// The derivation's idea of the resource's state has diverged from what the
    /// preceding barriers actually did.
    TransitionFromWrongArrangement {
        position: usize,
        resource: ResourceId,
        declared: Arrangement,
        actual: Arrangement,
    },
    /// A read of a resource whose last write has not been made visible to this
    /// reader's stages and access. The classic missing read-after-write barrier,
    /// and the one that survives testing on a machine where the two happen not
    /// to overlap.
    WriteNotVisible {
        position: usize,
        pass: String,
        resource: ResourceId,
        required: Scope,
        visible: Scope,
    },
    /// A write that is not ordered after the accesses preceding it - a
    /// write-after-write or write-after-read with nothing separating them.
    WriteNotOrdered {
        position: usize,
        pass: String,
        resource: ResourceId,
        required: Scope,
        permitted: Scope,
    },
    /// A read of a resource nothing has written and which was not declared as
    /// arriving with contents.
    ReadOfUndefinedContents { position: usize, pass: String, resource: ResourceId },
    /// A barrier named a resource this replay was never told about.
    UnknownResource { position: usize, resource: ResourceId },
    /// A texture barrier for a buffer, or the reverse.
    WrongBarrierKind { position: usize, resource: ResourceId, declared: ResourceKind },
}

impl fmt::Display for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hazard::WrongArrangement { position, pass, resource, expected, actual } => write!(
                f,
                "at {position} pass '{pass}' needs resource {} as {expected}, but the barriers \
                 left it as {actual}",
                resource.0
            ),
            Hazard::TransitionFromWrongArrangement { position, resource, declared, actual } => {
                write!(
                    f,
                    "at {position} a barrier moves resource {} out of {declared}, but it is \
                     actually {actual}",
                    resource.0
                )
            }
            Hazard::WriteNotVisible { position, pass, resource, required, visible } => write!(
                f,
                "at {position} pass '{pass}' reads resource {} with stages {:#x}/access {:#x}, \
                 but the last write is only visible to stages {:#x}/access {:#x}",
                resource.0, required.stages.0, required.access.0, visible.stages.0,
                visible.access.0
            ),
            Hazard::WriteNotOrdered { position, pass, resource, required, permitted } => write!(
                f,
                "at {position} pass '{pass}' writes resource {} with stages {:#x}/access {:#x} \
                 with nothing ordering it after the preceding accesses (permitted stages {:#x}/\
                 access {:#x})",
                resource.0, required.stages.0, required.access.0, permitted.stages.0,
                permitted.access.0
            ),
            Hazard::ReadOfUndefinedContents { position, pass, resource } => write!(
                f,
                "at {position} pass '{pass}' reads resource {}, which nothing has written",
                resource.0
            ),
            Hazard::UnknownResource { position, resource } => {
                write!(f, "at {position} a barrier names unregistered resource {}", resource.0)
            }
            Hazard::WrongBarrierKind { position, resource, declared } => write!(
                f,
                "at {position} a {declared:?} barrier is applied to resource {}, which is not \
                 one",
                resource.0
            ),
        }
    }
}

/// The outcome of a replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub trace: Trace,
    pub hazards: Vec<Hazard>,
}

impl Report {
    pub fn is_clean(&self) -> bool {
        self.hazards.is_empty()
    }

    /// The hazards rendered one per line, for a test failure message that says
    /// what went wrong rather than only that something did.
    pub fn describe(&self) -> String {
        self.hazards.iter().map(|hazard| format!("  {hazard}\n")).collect()
    }
}

/// What the replay knows about one resource, built entirely from barriers.
#[derive(Debug, Clone)]
struct Tracked {
    kind: ResourceKind,
    arrangement: Arrangement,
    /// True once something has put contents there, or the caller declared that
    /// it arrives with them.
    has_contents: bool,
    /// The most recent write, if any.
    last_write: Option<Scope>,
    /// Scopes a barrier has made that write visible to.
    write_visible_to: Scope,
    /// Reads since that write.
    reads_since_write: Scope,
    /// Scopes a barrier has ordered the outstanding accesses before.
    ordered_before: Scope,
}

impl Tracked {
    fn new(kind: ResourceKind, has_contents: bool) -> Self {
        Self {
            kind,
            arrangement: Arrangement::Undefined,
            has_contents,
            last_write: None,
            write_visible_to: Scope::NONE,
            reads_since_write: Scope::NONE,
            ordered_before: Scope::NONE,
        }
    }

    /// Everything a subsequent write has to be ordered after.
    fn outstanding(&self) -> Scope {
        let mut scope = self.reads_since_write;
        if let Some(write) = self.last_write {
            scope = Scope::new(
                scope.stages.union(write.stages),
                scope.access.union(write.access),
            );
        }
        scope
    }
}

/// True when `wider` includes everything in `narrower`.
fn covers(wider: Scope, narrower: Scope) -> bool {
    wider.stages.contains(narrower.stages) && wider.access.contains(narrower.access)
}

/// A backend with no hardware behind it.
#[derive(Debug, Clone, Default)]
pub struct ReferenceDevice {
    resources: BTreeMap<ResourceId, Tracked>,
}

impl ReferenceDevice {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a resource created by the graph, with no contents yet.
    pub fn create(&mut self, resource: ResourceId, kind: ResourceKind) {
        self.resources.insert(resource, Tracked::new(kind, false));
    }

    /// Registers a resource that arrives with contents from outside - a
    /// swapchain image, a history buffer, a streamed texture.
    ///
    /// Taking the arrangement it arrives in rather than assuming one: a
    /// swapchain image the presentation engine just released is not in the same
    /// arrangement as a texture a previous frame left as a render target, and
    /// assuming either would make the first barrier of every frame unverifiable.
    pub fn import(&mut self, resource: ResourceId, kind: ResourceKind, arrangement: Arrangement) {
        let mut tracked = Tracked::new(kind, true);
        tracked.arrangement = arrangement;
        self.resources.insert(resource, tracked);
    }

    /// Replays an ordered pass list against a derived timeline.
    ///
    /// Every hazard is collected rather than returned at the first one: a
    /// timeline with three missing barriers should report three, not send
    /// someone round the loop three times.
    pub fn replay(&mut self, passes: &[Pass], timeline: &Timeline) -> Report {
        let mut trace = Trace::default();
        let mut hazards = Vec::new();

        for (position, pass) in passes.iter().enumerate() {
            for barrier in timeline.before(PassId(position as u32)) {
                trace.record_barrier(position, barrier);
                self.apply(position, barrier, &mut hazards);
            }

            trace.record_pass(position, &pass.name);
            for intent in &pass.intents {
                self.check(position, pass, intent, &mut hazards);
            }
            // Effects are recorded only after every intent in the pass has been
            // checked. Folding them in as we go would let one intent's write
            // satisfy another intent's read within the same pass, which is
            // exactly the undefined intra-pass ordering the RHI rejects.
            for intent in &pass.intents {
                self.record(intent);
            }
        }

        Report { trace, hazards }
    }

    fn apply(&mut self, position: usize, barrier: &Barrier, hazards: &mut Vec<Hazard>) {
        let (resource, before, after, arrangement) = match barrier {
            Barrier::Texture { resource, before, after, from, to } => {
                (*resource, *before, *after, Some((*from, *to)))
            }
            Barrier::Buffer { resource, before, after } => (*resource, *before, *after, None),
            Barrier::Global { before, after } => {
                // Applies to every resource in the queue, so it orders all of
                // them at once.
                for tracked in self.resources.values_mut() {
                    Self::order(tracked, *before, *after);
                }
                return;
            }
        };

        let Some(tracked) = self.resources.get_mut(&resource) else {
            hazards.push(Hazard::UnknownResource { position, resource });
            return;
        };

        match (arrangement, tracked.kind) {
            (Some(_), ResourceKind::Buffer) => {
                hazards.push(Hazard::WrongBarrierKind {
                    position,
                    resource,
                    declared: ResourceKind::Texture,
                });
                return;
            }
            (None, ResourceKind::Texture) => {
                hazards.push(Hazard::WrongBarrierKind {
                    position,
                    resource,
                    declared: ResourceKind::Buffer,
                });
                return;
            }
            _ => {}
        }

        if let Some((from, to)) = arrangement {
            // Undefined is always a legal source: the hardware may discard
            // rather than convert, and a barrier that says so is asserting less,
            // not more.
            if from != tracked.arrangement && from != Arrangement::Undefined {
                hazards.push(Hazard::TransitionFromWrongArrangement {
                    position,
                    resource,
                    declared: from,
                    actual: tracked.arrangement,
                });
            }
            tracked.arrangement = to;
        }

        Self::order(tracked, before, after);
    }

    /// Folds one barrier's source and destination scopes into a resource.
    fn order(tracked: &mut Tracked, before: Scope, after: Scope) {
        if let Some(write) = tracked.last_write {
            if covers(before, write) {
                tracked.write_visible_to = Scope::new(
                    tracked.write_visible_to.stages.union(after.stages),
                    tracked.write_visible_to.access.union(after.access),
                );
            }
        }
        let outstanding = tracked.outstanding();
        if !outstanding.stages.is_empty() && covers(before, outstanding) {
            tracked.ordered_before = Scope::new(
                tracked.ordered_before.stages.union(after.stages),
                tracked.ordered_before.access.union(after.access),
            );
        }
    }

    fn check(
        &mut self,
        position: usize,
        pass: &Pass,
        intent: &pn_rhi::Intent,
        hazards: &mut Vec<Hazard>,
    ) {
        let Some(tracked) = self.resources.get(&intent.resource) else {
            hazards.push(Hazard::UnknownResource { position, resource: intent.resource });
            return;
        };

        if tracked.kind == ResourceKind::Texture && tracked.arrangement != intent.arrangement {
            hazards.push(Hazard::WrongArrangement {
                position,
                pass: pass.name.clone(),
                resource: intent.resource,
                expected: intent.arrangement,
                actual: tracked.arrangement,
            });
        }

        let required = Scope::new(intent.stages, intent.access);

        if intent.writes() {
            let outstanding = tracked.outstanding();
            if !outstanding.stages.is_empty() && !covers(tracked.ordered_before, required) {
                hazards.push(Hazard::WriteNotOrdered {
                    position,
                    pass: pass.name.clone(),
                    resource: intent.resource,
                    required,
                    permitted: tracked.ordered_before,
                });
            }
            return;
        }

        if intent.access.is_empty() {
            return; // an arrangement change and nothing more
        }

        if !tracked.has_contents {
            hazards.push(Hazard::ReadOfUndefinedContents {
                position,
                pass: pass.name.clone(),
                resource: intent.resource,
            });
            return;
        }

        if tracked.last_write.is_some() && !covers(tracked.write_visible_to, required) {
            hazards.push(Hazard::WriteNotVisible {
                position,
                pass: pass.name.clone(),
                resource: intent.resource,
                required,
                visible: tracked.write_visible_to,
            });
        }
    }

    fn record(&mut self, intent: &pn_rhi::Intent) {
        let Some(tracked) = self.resources.get_mut(&intent.resource) else { return };

        if intent.writes() {
            tracked.has_contents = true;
            tracked.last_write = Some(Scope::new(intent.stages, intent.access));
            tracked.write_visible_to = Scope::NONE;
            tracked.reads_since_write = Scope::NONE;
            tracked.ordered_before = Scope::NONE;
        } else if !intent.access.is_empty() {
            tracked.reads_since_write = Scope::new(
                tracked.reads_since_write.stages.union(intent.stages),
                tracked.reads_since_write.access.union(intent.access),
            );
        }
    }

    /// The arrangement a resource ended in. For asserting that a frame leaves
    /// the swapchain image presentable, which nothing else checks.
    pub fn arrangement_of(&self, resource: ResourceId) -> Option<Arrangement> {
        self.resources.get(&resource).map(|tracked| tracked.arrangement)
    }
}

/// Convenience for the common shape: everything created empty, nothing imported.
pub fn replay_all_transient(
    passes: &[Pass],
    timeline: &Timeline,
    resources: &[(ResourceId, ResourceKind)],
) -> Report {
    let mut device = ReferenceDevice::new();
    for (resource, kind) in resources {
        device.create(*resource, *kind);
    }
    device.replay(passes, timeline)
}

/// Re-exported so callers need not depend on the exact scope type to build one.
pub fn scope(stages: Stages, access: Access) -> Scope {
    Scope::new(stages, access)
}
