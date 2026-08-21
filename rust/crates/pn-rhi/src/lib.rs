// Pention Engine - pn-rhi
// Requirement: PN-RND-003, PN-RND-005
// Decision:    ADR-0004, ADR-0005, ADR-0006, ADR-0009
//
// The backend-independent half of the RHI: the vocabulary passes use to declare
// what they do with a resource, and the derivation that turns those declarations
// into a barrier timeline.
//
// Nothing here talks to a graphics API. That is the point - ADR-0005 observes
// that the render graph's correctness is a question about a directed acyclic
// graph, not about silicon, and this crate is the part of the renderer that can
// be verified on a machine with no GPU.
//
// Nothing here is a renaming of Direct3D 12's or Vulkan's enumerations either.
// ADR-0006 rejects that explicitly: a renamed vendor enum inherits that vendor's
// quirks and is the rebranding the dependency boundary prohibits. The vocabulary
// below is named for what the graph reasons about.

pub mod barrier;
pub mod derive;

pub use barrier::{
    Access, Arrangement, Barrier, Intent, PassId, QueueKind, ResourceId, ResourceKind, Scope,
    Stages,
};
pub use derive::{DerivationError, Pass, Timeline, derive_barriers};
