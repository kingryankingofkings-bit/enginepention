// Pention Engine - pn-rhi-reference
// Requirement: PN-RND-003, PN-OPS-007
// Decision:    ADR-0004, ADR-0005, ADR-0006
//
// The reference backend from ADR-0005: an RHI implementation with no hardware
// behind it, whose job is to say whether a barrier timeline is actually correct.
//
// ## Why this is not circular
//
// The obvious way to write this is to re-run the derivation and compare. That
// would prove nothing: a bug in the derivation would be reproduced identically
// here and the two would agree.
//
// So this works from the opposite direction. The derivation reads pass intents
// and produces barriers. This reads the *barriers* and asks what state they
// leave each resource in, then checks the intents against that state. The two
// only agree when the barriers really do put resources where the passes need
// them - which is the property worth checking.
//
// A missing barrier, a barrier with the wrong source arrangement, or one whose
// destination scope does not cover the reader it was emitted for all show up
// here as a hazard rather than as silence.
//
// ## What this is not
//
// It is a correctness oracle, not a renderer. It draws nothing, and makes no
// performance claim of any kind. ADR-0005 also describes a simple rasterizer for
// golden-image tests; that is a separate piece of work and does not exist yet.

pub mod device;
pub mod trace;

pub use device::{Hazard, ReferenceDevice, Report};
pub use trace::{Event, Trace};
