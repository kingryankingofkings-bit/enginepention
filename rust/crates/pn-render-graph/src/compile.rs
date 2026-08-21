// Pention Engine - pn-render-graph/compile.rs
// Requirement: PN-RND-005
// Decision:    ADR-0005, ADR-0006
//
// Compilation: edges, cycle detection, culling, ordering, lifetimes, barriers.
//
// The order of these steps is not arbitrary. Culling must precede ordering,
// because a culled pass must not contribute an edge; ordering must precede
// lifetime analysis, because a lifetime is a span in execution order; and
// barrier derivation must come last, because it consumes the final order. Doing
// them in any other order produces answers that look plausible and are wrong.
//
// ## What declaration order means
//
// Declaration order versions resources. A read sees the contents written by the
// most recently declared writer before it; if there is none, it sees whatever
// the resource held on entry to the graph. That is defined for a persistent
// resource, which something outside filled, and undefined for a transient one,
// which is why the latter is rejected.
//
// This is a deliberate semantic, not an accident of implementation. The
// alternative - binding a read to a writer declared after it - would mean
// declaring a pass in a different place silently changed which frame's contents
// it sampled. Under this rule, moving a declaration is either a no-op or a
// visible error.
//
// A consequence worth naming: every dependency edge runs from a lower
// declaration index to a higher one, so the edge set cannot contain a cycle.
// The cycle check below is therefore an invariant guard rather than a feature,
// and it is kept so that adding explicit resource versioning later fails loudly
// instead of quietly emitting a wrong order.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use pn_rhi::{
    Barrier, DerivationError, Pass, PassId, ResourceId, ResourceKind, Timeline, derive_barriers,
};

use crate::graph::{Graph, PassIndex, Residency};

/// A resource's span in execution order, inclusive at both ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lifetime {
    pub resource: ResourceId,
    /// Position in execution order of the first pass that touches it.
    pub first: usize,
    /// Position in execution order of the last pass that touches it.
    pub last: usize,
    /// Carried here because aliasing is only ever legal between transients. A
    /// persistent resource outlives the graph, so a disjoint span inside the
    /// graph says nothing about whether its memory is free.
    pub residency: Residency,
}

impl Lifetime {
    /// Whether two lifetimes overlap, and therefore cannot share memory.
    ///
    /// This is the question PN-RND-006 will ask of every pair; getting it wrong
    /// by one produces aliasing that is correct in every test that does not
    /// happen to put two resources end to end.
    pub fn overlaps(&self, other: &Lifetime) -> bool {
        self.first <= other.last && other.first <= self.last
    }
}

/// A failure that makes the graph uncompilable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    Cycle { passes: Vec<String> },
    /// A pass reads a resource nothing in the graph writes, and which does not
    /// come from outside. Its contents are whatever was in memory.
    ReadOfUnwrittenResource { pass: String, resource: String },
    /// An intent named a resource this graph never declared.
    UnknownResource { pass: String, id: ResourceId },
    /// A pass declared a texture intent on a buffer, or the reverse. The two
    /// carry different barrier kinds, so this is not a naming slip that could be
    /// tolerated - it changes what is emitted.
    ResourceKindMismatch { pass: String, resource: String, declared: ResourceKind, used: ResourceKind },
    /// Barrier derivation rejected the ordered passes.
    Derivation(DerivationError),
}

impl From<DerivationError> for CompileError {
    fn from(error: DerivationError) -> Self {
        CompileError::Derivation(error)
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Cycle { passes } => {
                write!(f, "pass dependencies form a cycle: {}", passes.join(" -> "))
            }
            CompileError::ReadOfUnwrittenResource { pass, resource } => write!(
                f,
                "pass '{pass}' reads transient resource '{resource}', which no pass writes; \
                 its contents are undefined"
            ),
            CompileError::UnknownResource { pass, id } => {
                write!(f, "pass '{pass}' names resource {}, which was never declared", id.0)
            }
            CompileError::ResourceKindMismatch { pass, resource, declared, used } => write!(
                f,
                "pass '{pass}' uses '{resource}' as a {used:?} but it was declared a \
                 {declared:?}; the two produce different barrier kinds"
            ),
            CompileError::Derivation(error) => write!(f, "{error}"),
        }
    }
}

/// Something worth reporting that does not stop compilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diagnostic {
    /// The pass was removed: nothing reads what it writes, it writes nothing
    /// that outlives the graph, and it is not pinned.
    PassCulled { pass: String },
    /// A transient resource is written and never read. The write is real work
    /// producing something nobody wants.
    WrittenNeverRead { resource: String, writer: String },
    /// A write is entirely overwritten by a later write with no reader in
    /// between. The earlier pass still runs - it may have other outputs - but
    /// this particular output is discarded.
    OverwrittenBeforeRead { resource: String, discarded: String, by: String },
    /// Forwarded from barrier derivation.
    Barrier(pn_rhi::derive::Diagnostic),
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Diagnostic::PassCulled { pass } => {
                write!(f, "pass '{pass}' culled: nothing consumes its output")
            }
            Diagnostic::WrittenNeverRead { resource, writer } => write!(
                f,
                "'{writer}' writes '{resource}', which nothing reads and which does not \
                 outlive the graph"
            ),
            Diagnostic::OverwrittenBeforeRead { resource, discarded, by } => write!(
                f,
                "'{discarded}' writes '{resource}' and '{by}' overwrites it with no reader \
                 in between; that output is discarded"
            ),
            Diagnostic::Barrier(inner) => write!(f, "{inner}"),
        }
    }
}

/// A compiled graph, ready to record.
#[derive(Debug, Clone)]
pub struct Compiled {
    /// Surviving passes in execution order, as declaration indices.
    pub order: Vec<PassIndex>,
    /// Passes removed by culling, in declaration order.
    pub culled: Vec<PassIndex>,
    /// Lifetimes of every resource that survives, keyed by resource.
    pub lifetimes: BTreeMap<ResourceId, Lifetime>,
    /// Barriers, indexed by position in `order`.
    pub timeline: Timeline,
    pub diagnostics: Vec<Diagnostic>,
}

impl Compiled {
    /// The barriers to run before the pass at `position` in execution order.
    pub fn barriers_before(&self, position: usize) -> &[Barrier] {
        self.timeline.before(PassId(position as u32))
    }

    pub fn barrier_count(&self) -> usize {
        self.timeline.barrier_count()
    }

    /// Transient lifetimes that could share memory with `resource`.
    ///
    /// The foundation PN-RND-006 needs. Reported now, unused now: an aliasing
    /// allocator that gets this wrong is worse than none.
    pub fn disjoint_from(&self, resource: ResourceId) -> Vec<ResourceId> {
        let Some(subject) = self.lifetimes.get(&resource) else { return Vec::new() };
        if subject.residency != Residency::Transient {
            // A persistent resource's memory is not the graph's to reuse, however
            // its span inside the graph looks.
            return Vec::new();
        }
        self.lifetimes
            .values()
            .filter(|other| {
                other.residency == Residency::Transient
                    && other.resource != resource
                    && !other.overlaps(subject)
            })
            .map(|other| other.resource)
            .collect()
    }
}

/// Per-resource bookkeeping gathered in one pass over the declarations.
#[derive(Default)]
struct Usage {
    writers: Vec<usize>,
    readers: Vec<usize>,
}

/// Compiles the graph.
pub fn compile(graph: &Graph) -> Result<Compiled, CompileError> {
    let usage = gather_usage(graph)?;
    let live = live_passes(graph, &usage);
    let order = topological_order(graph, &usage, &live)?;

    let mut diagnostics = Vec::new();
    for (index, pass) in graph.passes.iter().enumerate() {
        if !live.contains(&index) {
            diagnostics.push(Diagnostic::PassCulled { pass: pass.name.clone() });
        }
    }
    report_wasted_writes(graph, &usage, &live, &order, &mut diagnostics);

    // Only surviving passes reach the barrier derivation, and in execution
    // order. Handing it declaration order would derive a correct timeline for a
    // frame that is not the one being recorded.
    let ordered: Vec<Pass> = order
        .iter()
        .map(|index| {
            let declaration = &graph.passes[index.0 as usize];
            Pass::new(
                declaration.name.clone(),
                declaration.queue,
                declaration.intents.clone(),
            )
        })
        .collect();

    let timeline = derive_barriers(&ordered)?;
    diagnostics.extend(timeline.diagnostics.iter().cloned().map(Diagnostic::Barrier));

    let lifetimes = lifetimes(graph, &order);

    let culled: Vec<PassIndex> = (0..graph.passes.len())
        .filter(|index| !live.contains(index))
        .map(|index| PassIndex(index as u32))
        .collect();

    Ok(Compiled { order, culled, lifetimes, timeline, diagnostics })
}

/// Collects who writes and who reads each resource, and rejects a read of
/// something nothing produces.
fn gather_usage(graph: &Graph) -> Result<BTreeMap<ResourceId, Usage>, CompileError> {
    let mut usage: BTreeMap<ResourceId, Usage> = BTreeMap::new();

    for (index, pass) in graph.passes.iter().enumerate() {
        for intent in &pass.intents {
            let Some(declaration) = graph.resource(intent.resource) else {
                return Err(CompileError::UnknownResource {
                    pass: pass.name.clone(),
                    id: intent.resource,
                });
            };
            if declaration.kind != intent.kind {
                return Err(CompileError::ResourceKindMismatch {
                    pass: pass.name.clone(),
                    resource: declaration.name.clone(),
                    declared: declaration.kind,
                    used: intent.kind,
                });
            }
            let entry = usage.entry(intent.resource).or_default();
            if intent.writes() {
                entry.writers.push(index);
            } else if !intent.access.is_empty() {
                entry.readers.push(index);
            }
        }
    }

    // A transient resource read before anything writes it holds whatever was in
    // memory. A persistent one is fine: something outside the graph filled it.
    for (id, entry) in &usage {
        let declaration = graph.resource(*id).expect("checked above");
        if declaration.residency == Residency::Persistent {
            continue;
        }
        for &reader in &entry.readers {
            let has_earlier_writer = entry.writers.iter().any(|&writer| writer < reader);
            if !has_earlier_writer {
                return Err(CompileError::ReadOfUnwrittenResource {
                    pass: graph.passes[reader].name.clone(),
                    resource: declaration.name.clone(),
                });
            }
        }
    }

    Ok(usage)
}

/// The passes that survive culling.
///
/// Reverse reachability from the roots: a pass is live if it is pinned, if it
/// writes something that outlives the graph, or if a live pass reads something
/// it writes. Iterated to a fixed point, because liveness propagates backwards
/// through arbitrarily long chains.
fn live_passes(graph: &Graph, usage: &BTreeMap<ResourceId, Usage>) -> BTreeSet<usize> {
    let mut live = BTreeSet::new();

    for (index, pass) in graph.passes.iter().enumerate() {
        if pass.pinned {
            live.insert(index);
            continue;
        }
        let writes_persistent = pass.intents.iter().any(|intent| {
            intent.writes()
                && graph
                    .resource(intent.resource)
                    .is_some_and(|decl| decl.residency == Residency::Persistent)
        });
        if writes_persistent {
            live.insert(index);
        }
    }

    loop {
        let mut grew = false;
        let live_snapshot: Vec<usize> = live.iter().copied().collect();
        for consumer in live_snapshot {
            for intent in &graph.passes[consumer].intents {
                if intent.writes() || intent.access.is_empty() {
                    continue;
                }
                let Some(entry) = usage.get(&intent.resource) else { continue };
                for &writer in &entry.writers {
                    if writer < consumer && live.insert(writer) {
                        grew = true;
                    }
                }
            }
        }
        if !grew {
            break;
        }
    }

    live
}

/// Orders the live passes.
///
/// Kahn's algorithm with the ready set kept sorted by declaration index. The
/// tie-break is not cosmetic: without it the order depends on iteration order
/// and the derived barrier timeline changes between runs, which would make every
/// timeline assertion in the test suite flaky rather than wrong.
///
/// Under declaration-order versioning every edge runs forward, so the result is
/// always the live passes in declaration index order and this is, today,
/// equivalent to sorting them. That equivalence is asserted by a test, so that
/// introducing explicit resource versioning - which would make the two differ -
/// breaks a test that names the assumption instead of silently changing the
/// order every barrier is derived against.
///
/// It is written as a sort over the dependency edges rather than over the
/// indices because the edge set is what a future scheduler needs (PN-RND-007
/// assigns queues from exactly these dependencies), and because building it here
/// is what makes the acyclicity invariant checkable at all.
fn topological_order(
    graph: &Graph,
    usage: &BTreeMap<ResourceId, Usage>,
    live: &BTreeSet<usize>,
) -> Result<Vec<PassIndex>, CompileError> {
    let mut successors: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut indegree: BTreeMap<usize, usize> = live.iter().map(|&index| (index, 0)).collect();

    let add_edge = |from: usize, to: usize, successors: &mut BTreeMap<usize, BTreeSet<usize>>,
                        indegree: &mut BTreeMap<usize, usize>| {
        if from == to || !live.contains(&from) || !live.contains(&to) {
            return;
        }
        if successors.entry(from).or_default().insert(to) {
            *indegree.entry(to).or_insert(0) += 1;
        }
    };

    for entry in usage.values() {
        // Read after write, and write after write.
        for &writer in &entry.writers {
            for &reader in &entry.readers {
                if writer < reader {
                    add_edge(writer, reader, &mut successors, &mut indegree);
                }
            }
            for &later in &entry.writers {
                if writer < later {
                    add_edge(writer, later, &mut successors, &mut indegree);
                }
            }
        }
        // Write after read: a later writer must follow every earlier reader.
        for &reader in &entry.readers {
            for &writer in &entry.writers {
                if reader < writer {
                    add_edge(reader, writer, &mut successors, &mut indegree);
                }
            }
        }
    }

    let mut ready: BTreeSet<usize> =
        indegree.iter().filter(|(_, &degree)| degree == 0).map(|(&index, _)| index).collect();
    let mut order = Vec::with_capacity(live.len());

    while let Some(&next) = ready.iter().next() {
        ready.remove(&next);
        order.push(PassIndex(next as u32));
        if let Some(children) = successors.get(&next) {
            for &child in children {
                let degree = indegree.get_mut(&child).expect("live");
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(child);
                }
            }
        }
    }

    if order.len() != live.len() {
        let stuck: Vec<String> = live
            .iter()
            .filter(|index| !order.contains(&PassIndex(**index as u32)))
            .map(|&index| graph.passes[index].name.clone())
            .collect();
        return Err(CompileError::Cycle { passes: stuck });
    }

    Ok(order)
}

/// First and last touch of each resource, in execution order.
fn lifetimes(graph: &Graph, order: &[PassIndex]) -> BTreeMap<ResourceId, Lifetime> {
    let mut lifetimes: BTreeMap<ResourceId, Lifetime> = BTreeMap::new();

    for (position, index) in order.iter().enumerate() {
        for intent in &graph.passes[index.0 as usize].intents {
            let residency = graph
                .resource(intent.resource)
                .map(|declaration| declaration.residency)
                .unwrap_or(Residency::Persistent);
            lifetimes
                .entry(intent.resource)
                .and_modify(|lifetime| lifetime.last = position)
                .or_insert(Lifetime {
                    resource: intent.resource,
                    first: position,
                    last: position,
                    residency,
                });
        }
    }

    lifetimes
}

/// Reports work whose output nobody wants.
fn report_wasted_writes(
    graph: &Graph,
    usage: &BTreeMap<ResourceId, Usage>,
    live: &BTreeSet<usize>,
    order: &[PassIndex],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let position: BTreeMap<usize, usize> =
        order.iter().enumerate().map(|(slot, index)| (index.0 as usize, slot)).collect();

    for (id, entry) in usage {
        let declaration = graph.resource(*id).expect("gathered");
        if declaration.residency == Residency::Persistent {
            continue;
        }

        let live_readers: Vec<usize> =
            entry.readers.iter().copied().filter(|index| live.contains(index)).collect();
        let live_writers: Vec<usize> =
            entry.writers.iter().copied().filter(|index| live.contains(index)).collect();

        if live_readers.is_empty() {
            for &writer in &live_writers {
                diagnostics.push(Diagnostic::WrittenNeverRead {
                    resource: declaration.name.clone(),
                    writer: graph.passes[writer].name.clone(),
                });
            }
            continue;
        }

        // Two writes with no reader between them in execution order: the first
        // output is discarded. The pass still runs, because it may produce
        // something else that is wanted.
        let mut ordered_writers: Vec<usize> = live_writers.clone();
        ordered_writers.sort_by_key(|index| position.get(index).copied().unwrap_or(usize::MAX));
        for window in ordered_writers.windows(2) {
            let (earlier, later) = (window[0], window[1]);
            let (Some(&earlier_slot), Some(&later_slot)) =
                (position.get(&earlier), position.get(&later))
            else {
                continue;
            };
            let read_between = live_readers.iter().any(|reader| {
                position
                    .get(reader)
                    .is_some_and(|&slot| slot > earlier_slot && slot < later_slot)
            });
            if !read_between {
                diagnostics.push(Diagnostic::OverwrittenBeforeRead {
                    resource: declaration.name.clone(),
                    discarded: graph.passes[earlier].name.clone(),
                    by: graph.passes[later].name.clone(),
                });
            }
        }
    }
}
