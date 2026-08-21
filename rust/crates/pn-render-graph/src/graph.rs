// Pention Engine - pn-render-graph/graph.rs
// Requirement: PN-RND-005
// Decision:    ADR-0006
//
// Building the graph. Nothing is analysed here - this is the declaration
// surface, and it is deliberately dull. Every question worth getting wrong is
// answered in compile.rs, where it can be answered once against the whole graph
// rather than incrementally as passes arrive.

use pn_rhi::{Access, Arrangement, Intent, QueueKind, ResourceId, ResourceKind, Stages};

/// Whether a resource outlives the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Residency {
    /// Created and destroyed within one graph execution. Its memory may be
    /// reused by any resource whose lifetime does not overlap.
    Transient,
    /// Comes from outside and survives afterwards - a swapchain image, a
    /// persistent history buffer. A pass that writes one is never culled, even
    /// if nothing inside the graph reads it, because something outside does.
    Persistent,
}

/// A handle to a declared resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceHandle(pub u32);

impl ResourceHandle {
    pub fn id(self) -> ResourceId {
        ResourceId(self.0)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ResourceDecl {
    pub name: String,
    pub kind: ResourceKind,
    pub residency: Residency,
}

#[derive(Debug, Clone)]
pub(crate) struct PassDecl {
    pub name: String,
    pub queue: QueueKind,
    pub intents: Vec<Intent>,
    /// Kept even when nothing reads its output.
    pub pinned: bool,
}

/// A graph under construction.
#[derive(Debug, Default, Clone)]
pub struct Graph {
    pub(crate) resources: Vec<ResourceDecl>,
    pub(crate) passes: Vec<PassDecl>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares a texture that lives only inside this graph.
    pub fn transient_texture(&mut self, name: impl Into<String>) -> ResourceHandle {
        self.declare(name, ResourceKind::Texture, Residency::Transient)
    }

    /// Declares a texture that comes from outside and survives afterwards.
    pub fn persistent_texture(&mut self, name: impl Into<String>) -> ResourceHandle {
        self.declare(name, ResourceKind::Texture, Residency::Persistent)
    }

    pub fn transient_buffer(&mut self, name: impl Into<String>) -> ResourceHandle {
        self.declare(name, ResourceKind::Buffer, Residency::Transient)
    }

    pub fn persistent_buffer(&mut self, name: impl Into<String>) -> ResourceHandle {
        self.declare(name, ResourceKind::Buffer, Residency::Persistent)
    }

    fn declare(
        &mut self,
        name: impl Into<String>,
        kind: ResourceKind,
        residency: Residency,
    ) -> ResourceHandle {
        let handle = ResourceHandle(self.resources.len() as u32);
        self.resources.push(ResourceDecl { name: name.into(), kind, residency });
        handle
    }

    /// Begins a pass. The returned builder must be finished with
    /// [`PassBuilder::build`]; a builder that is dropped adds nothing, which is
    /// the behaviour a `#[must_use]` makes hard to reach by accident.
    #[must_use = "a pass is only added when the builder is built"]
    pub fn pass(&mut self, name: impl Into<String>, queue: QueueKind) -> PassBuilder<'_> {
        PassBuilder { graph: self, name: name.into(), queue, intents: Vec::new(), pinned: false }
    }

    pub(crate) fn resource(&self, id: ResourceId) -> Option<&ResourceDecl> {
        self.resources.get(id.0 as usize)
    }

    pub fn resource_name(&self, handle: ResourceHandle) -> &str {
        &self.resources[handle.0 as usize].name
    }

    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }

    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }
}

/// Accumulates one pass's declarations.
pub struct PassBuilder<'graph> {
    graph: &'graph mut Graph,
    name: String,
    queue: QueueKind,
    intents: Vec<Intent>,
    pinned: bool,
}

impl PassBuilder<'_> {
    /// Marks the pass as never culled.
    ///
    /// For a pass whose effect is outside the graph's model - a present, a
    /// readback, a query the CPU consumes. Without it, a pass whose only output
    /// leaves through a side channel looks like dead work.
    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    /// Declares a texture intent.
    pub fn texture(
        mut self,
        handle: ResourceHandle,
        stages: Stages,
        access: Access,
        arrangement: Arrangement,
    ) -> Self {
        self.intents.push(Intent::texture(handle.id(), stages, access, arrangement));
        self
    }

    /// Declares a buffer intent.
    pub fn buffer(mut self, handle: ResourceHandle, stages: Stages, access: Access) -> Self {
        self.intents.push(Intent::buffer(handle.id(), stages, access));
        self
    }

    /// Adds the pass to the graph.
    pub fn build(self) -> PassIndex {
        let index = PassIndex(self.graph.passes.len() as u32);
        self.graph.passes.push(PassDecl {
            name: self.name,
            queue: self.queue,
            intents: self.intents,
            pinned: self.pinned,
        });
        index
    }
}

/// A pass's position in declaration order.
///
/// Deliberately not its position in execution order: the two differ once
/// culling and topological sorting have run, and conflating them is how a
/// caller ends up indexing the barrier timeline with a declaration index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassIndex(pub u32);
