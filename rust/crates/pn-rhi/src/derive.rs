// Pention Engine - pn-rhi/derive.rs
// Requirement: PN-RND-003, PN-RND-005
// Decision:    ADR-0005, ADR-0006
//
// Barrier derivation. Passes declare intents; this computes the barriers.
//
// ADR-0006 is explicit that passes never emit barriers themselves, because
// hand-written barriers stop being maintainable at roughly ten passes and fail
// in vendor-specific ways. The engineering value is here, and it is independent
// of any graphics API - which is why it can be tested on a machine with no GPU.
//
// The input is an ordered list of passes. Ordering is the render graph's job
// (PN-RND-005); this consumes the order a topological sort produced and does not
// reorder anything itself.

use std::collections::BTreeMap;
use std::fmt;

use crate::barrier::{
    Access, Arrangement, Barrier, Intent, PassId, QueueKind, ResourceId, ResourceKind, Scope,
    Stages,
};

/// One unit of work with its declared resource intents.
#[derive(Debug, Clone)]
pub struct Pass {
    pub name: String,
    pub queue: QueueKind,
    pub intents: Vec<Intent>,
}

impl Pass {
    pub fn new(name: impl Into<String>, queue: QueueKind, intents: Vec<Intent>) -> Self {
        Self { name: name.into(), queue, intents }
    }
}

/// A hazard or a malformed declaration. Derivation fails rather than emitting a
/// barrier set that would be wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivationError {
    /// A buffer intent named an arrangement. Buffers have none, and silently
    /// ignoring it would let the author believe a transition happened.
    BufferHasNoArrangement { pass: PassId, resource: ResourceId, named: Arrangement },
    /// The declared arrangement cannot serve the declared access.
    ArrangementCannotServeAccess {
        pass: PassId,
        resource: ResourceId,
        arrangement: Arrangement,
        access: Access,
    },
    /// The pass's queue cannot perform this arrangement change.
    QueueCannotArrange {
        pass: PassId,
        resource: ResourceId,
        queue: QueueKind,
        arrangement: Arrangement,
    },
    /// One pass declared the same resource twice with conflicting arrangements.
    /// No barrier can satisfy both, because a barrier separates passes and there
    /// is nothing to separate here.
    ConflictingArrangementsInOnePass {
        pass: PassId,
        resource: ResourceId,
        first: Arrangement,
        second: Arrangement,
    },
    /// One pass both writes and reads the same resource through different
    /// declarations. Within a pass there is no ordering to impose, so the result
    /// depends on execution order the API does not define.
    ReadAndWriteInOnePass { pass: PassId, resource: ResourceId },
    /// An intent declared no stage at all. A barrier against no stage
    /// synchronises nothing, so this is a declaration that cannot mean anything.
    NoStagesDeclared { pass: PassId, resource: ResourceId },
    /// An intent that touches memory named no access, or named an access with
    /// no stage that could perform it.
    AccessWithoutStage { pass: PassId, resource: ResourceId, access: Access },
}

impl fmt::Display for DerivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DerivationError::BufferHasNoArrangement { pass, resource, named } => write!(
                f,
                "pass {} declares buffer {} with arrangement {named}; buffers have no arrangement",
                pass.0, resource.0
            ),
            DerivationError::ArrangementCannotServeAccess {
                pass,
                resource,
                arrangement,
                access,
            } => write!(
                f,
                "pass {} declares resource {} as {arrangement} but accesses it as {:#x}; \
                 that arrangement cannot serve that access",
                pass.0, resource.0, access.0
            ),
            DerivationError::QueueCannotArrange { pass, resource, queue, arrangement } => write!(
                f,
                "pass {} runs on a {queue:?} queue and cannot put resource {} into {arrangement}",
                pass.0, resource.0
            ),
            DerivationError::ConflictingArrangementsInOnePass {
                pass,
                resource,
                first,
                second,
            } => write!(
                f,
                "pass {} declares resource {} as both {first} and {second}; \
                 a barrier separates passes, so there is nothing here to separate",
                pass.0, resource.0
            ),
            DerivationError::ReadAndWriteInOnePass { pass, resource } => write!(
                f,
                "pass {} both reads and writes resource {} in separate declarations; \
                 the order between them is undefined",
                pass.0, resource.0
            ),
            DerivationError::NoStagesDeclared { pass, resource } => write!(
                f,
                "pass {} declares resource {} with no pipeline stage",
                pass.0, resource.0
            ),
            DerivationError::AccessWithoutStage { pass, resource, access } => write!(
                f,
                "pass {} accesses resource {} as {:#x} with no stage that performs it",
                pass.0, resource.0, access.0
            ),
        }
    }
}

/// A warning that does not stop derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diagnostic {
    /// A pass declared every stage rather than the ones it uses. The resulting
    /// barrier synchronises against work the pass has nothing to do with.
    BlanketStages { pass: PassId, resource: ResourceId },
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Diagnostic::BlanketStages { pass, resource } => write!(
                f,
                "pass {} declares every stage for resource {}; \
                 the derived barrier will synchronise against unrelated work",
                pass.0, resource.0
            ),
        }
    }
}

/// The barriers to run before one pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassBarriers {
    pub pass: PassId,
    pub barriers: Vec<Barrier>,
}

/// The derived result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timeline {
    /// One entry per pass, in the order given, including passes needing none.
    pub entries: Vec<PassBarriers>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Timeline {
    /// Every barrier, flattened, in execution order.
    pub fn barriers(&self) -> Vec<Barrier> {
        self.entries.iter().flat_map(|entry| entry.barriers.iter().copied()).collect()
    }

    pub fn barrier_count(&self) -> usize {
        self.entries.iter().map(|entry| entry.barriers.len()).sum()
    }

    /// The barriers before a given pass.
    pub fn before(&self, pass: PassId) -> &[Barrier] {
        self.entries
            .iter()
            .find(|entry| entry.pass == pass)
            .map(|entry| entry.barriers.as_slice())
            .unwrap_or(&[])
    }
}

/// What has happened to one resource so far.
#[derive(Debug, Clone)]
struct ResourceState {
    kind: ResourceKind,
    arrangement: Arrangement,
    /// The most recent write, if any. A read that follows it needs a barrier.
    last_write: Option<Scope>,
    /// Reads since that write, accumulated. This set does two jobs.
    ///
    /// A write that follows must wait for *all* of them, not only the most
    /// recent - that is why it accumulates rather than overwrites.
    ///
    /// It is also the set the last write has already been made visible to. A
    /// reader whose stages and access are already inside it needs no barrier;
    /// a reader in a stage nobody has synchronised for still does, even though
    /// the arrangement has not changed. Treating "read after read" as always
    /// free is the mistake here - the write was made visible to the first
    /// reader's stage, not to every stage.
    reads_since_write: Scope,
}

impl ResourceState {
    fn new(kind: ResourceKind) -> Self {
        Self {
            kind,
            arrangement: Arrangement::Undefined,
            last_write: None,
            reads_since_write: Scope::NONE,
        }
    }
}

/// Derives the barrier timeline for an ordered list of passes.
///
/// Every hazard that cannot be fixed by a barrier is an error rather than a
/// silently over-conservative barrier, because an over-conservative barrier
/// hides the declaration that was wrong.
pub fn derive_barriers(passes: &[Pass]) -> Result<Timeline, DerivationError> {
    let mut states: BTreeMap<ResourceId, ResourceState> = BTreeMap::new();
    let mut entries = Vec::with_capacity(passes.len());
    let mut diagnostics = Vec::new();

    for (index, pass) in passes.iter().enumerate() {
        let pass_id = PassId(index as u32);
        validate_pass(pass_id, pass, &mut diagnostics)?;

        let mut barriers = Vec::new();
        for intent in &pass.intents {
            let state = states
                .entry(intent.resource)
                .or_insert_with(|| ResourceState::new(intent.kind));

            if let Some(barrier) = barrier_for(intent, state) {
                if let Barrier::Texture { to, .. } = barrier {
                    if !pass.queue.can_arrange(to) {
                        return Err(DerivationError::QueueCannotArrange {
                            pass: pass_id,
                            resource: intent.resource,
                            queue: pass.queue,
                            arrangement: to,
                        });
                    }
                }
                barriers.push(barrier);
            }

            advance(intent, state);
        }

        entries.push(PassBarriers { pass: pass_id, barriers });
    }

    Ok(Timeline { entries, diagnostics })
}

/// Checks one pass's declarations in isolation, before any history is consulted.
fn validate_pass(
    pass_id: PassId,
    pass: &Pass,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), DerivationError> {
    // Per-resource within this pass, so that two declarations of the same
    // resource can be compared against each other.
    let mut seen: BTreeMap<ResourceId, (Arrangement, bool, bool)> = BTreeMap::new();

    for intent in &pass.intents {
        if intent.kind == ResourceKind::Buffer && intent.arrangement != Arrangement::Undefined {
            return Err(DerivationError::BufferHasNoArrangement {
                pass: pass_id,
                resource: intent.resource,
                named: intent.arrangement,
            });
        }
        if intent.stages.is_empty() {
            return Err(DerivationError::NoStagesDeclared {
                pass: pass_id,
                resource: intent.resource,
            });
        }
        if !intent.access.is_empty() && !stages_can_perform(intent.stages, intent.access) {
            return Err(DerivationError::AccessWithoutStage {
                pass: pass_id,
                resource: intent.resource,
                access: intent.access,
            });
        }
        if !intent.arrangement_supports_access() {
            return Err(DerivationError::ArrangementCannotServeAccess {
                pass: pass_id,
                resource: intent.resource,
                arrangement: intent.arrangement,
                access: intent.access,
            });
        }
        if intent.is_blanket() {
            diagnostics.push(Diagnostic::BlanketStages {
                pass: pass_id,
                resource: intent.resource,
            });
        }

        let writes = intent.writes();
        let reads = !intent.access.is_empty() && !writes;
        match seen.get_mut(&intent.resource) {
            None => {
                seen.insert(intent.resource, (intent.arrangement, writes, reads));
            }
            Some((arrangement, seen_write, seen_read)) => {
                if *arrangement != intent.arrangement {
                    return Err(DerivationError::ConflictingArrangementsInOnePass {
                        pass: pass_id,
                        resource: intent.resource,
                        first: *arrangement,
                        second: intent.arrangement,
                    });
                }
                if (writes && *seen_read) || (reads && *seen_write) {
                    return Err(DerivationError::ReadAndWriteInOnePass {
                        pass: pass_id,
                        resource: intent.resource,
                    });
                }
                *seen_write |= writes;
                *seen_read |= reads;
            }
        }
    }
    Ok(())
}

/// Whether the declared stages include one that could perform the access.
///
/// Catches the common transposition where a pass declares a compute stage and a
/// colour-write access, which no hardware performs and which would otherwise
/// derive a barrier that reads plausibly and synchronises the wrong thing.
fn stages_can_perform(stages: Stages, access: Access) -> bool {
    let required = |access_bit: Access, allowed: Stages| -> bool {
        !access.intersects(access_bit) || stages.0 & allowed.0 != 0
    };

    let shader_stages = Stages::VERTEX_WORK
        | Stages::FRAGMENT_WORK
        | Stages::COMPUTE_WORK
        | Stages::RAY_TRACING;

    required(Access::INDIRECT_ARGUMENTS, Stages::INDIRECT_FETCH)
        && required(Access::INDEX_DATA, Stages::GEOMETRY_FETCH)
        && required(Access::VERTEX_DATA, Stages::GEOMETRY_FETCH)
        && required(Access::UNIFORM_DATA, shader_stages)
        && required(Access::SAMPLED_READ, shader_stages)
        && required(Access::STORAGE_READ, shader_stages)
        && required(Access::STORAGE_WRITE, shader_stages)
        && required(Access::COLOR_READ, Stages::COLOR_OUTPUT)
        && required(Access::COLOR_WRITE, Stages::COLOR_OUTPUT)
        && required(Access::DEPTH_READ, Stages::DEPTH_TEST | Stages::FRAGMENT_WORK)
        && required(Access::DEPTH_WRITE, Stages::DEPTH_TEST)
        && required(Access::TRANSFER_READ, Stages::TRANSFER | Stages::RESOLVE)
        && required(Access::TRANSFER_WRITE, Stages::TRANSFER | Stages::RESOLVE)
        && required(Access::PRESENT_READ, Stages::PRESENT)
}

/// The barrier this intent needs given what has happened so far, if any.
fn barrier_for(intent: &Intent, state: &ResourceState) -> Option<Barrier> {
    let is_texture = state.kind == ResourceKind::Texture;
    let arrangement_changes = is_texture && intent.arrangement != state.arrangement;

    let after = Scope::new(intent.stages, intent.access);

    // Read after write, and write after write: wait for the writer.
    // Write after read: wait for every reader since the last write.
    let before = if intent.writes() {
        let mut stages = state.reads_since_write.stages;
        let mut access = state.reads_since_write.access;
        if let Some(write) = state.last_write {
            stages = stages.union(write.stages);
            access = access.union(write.access);
        }
        Scope::new(stages, access)
    } else if let Some(write) = state.last_write {
        write
    } else {
        Scope::NONE
    };

    let mut needs_execution_barrier = !before.stages.is_empty();

    // A read whose scope the last write has already been made visible to needs
    // nothing further. This is what keeps a chain of identical readers at one
    // barrier instead of one each.
    if !intent.writes()
        && needs_execution_barrier
        && state.reads_since_write.stages.contains(intent.stages)
        && state.reads_since_write.access.contains(intent.access)
    {
        needs_execution_barrier = false;
    }

    if !arrangement_changes && !needs_execution_barrier {
        return None;
    }

    if is_texture {
        Some(Barrier::Texture {
            resource: intent.resource,
            before,
            after,
            from: state.arrangement,
            to: intent.arrangement,
        })
    } else {
        Some(Barrier::Buffer { resource: intent.resource, before, after })
    }
}

/// Folds an intent into the resource's state.
fn advance(intent: &Intent, state: &mut ResourceState) {
    if state.kind == ResourceKind::Texture {
        state.arrangement = intent.arrangement;
    }
    if intent.writes() {
        state.last_write = Some(Scope::new(intent.stages, intent.access));
        state.reads_since_write = Scope::NONE;
    } else if !intent.access.is_empty() {
        state.reads_since_write = Scope::new(
            state.reads_since_write.stages.union(intent.stages),
            state.reads_since_write.access.union(intent.access),
        );
    }
}
