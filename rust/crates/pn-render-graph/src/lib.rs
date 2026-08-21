// Pention Engine - pn-render-graph
// Requirement: PN-RND-005
// Decision:    ADR-0004, ADR-0005, ADR-0006, ADR-0009
//
// The render graph. Passes declare what they read and write; the graph works out
// the order, discards what nobody consumes, computes how long each transient
// resource has to live, and derives the barriers.
//
// Everything here is a question about a directed acyclic graph, not about
// silicon (ADR-0005), which is why it is testable on a machine with no GPU.
//
// It sits above the RHI in the dependency graph and knows nothing about any
// graphics API - not even which one is active.

pub mod compile;
pub mod graph;
pub mod visualize;

pub use compile::{CompileError, Compiled, Diagnostic, Lifetime};
pub use graph::{Graph, PassBuilder, Residency, ResourceHandle};
pub use visualize::to_dot;
