// Pention Engine - pn-rhi/barrier.rs
// Requirement: PN-RND-003
// Decision:    ADR-0006
//
// The three axes a barrier controls, kept independent because both target APIs
// keep them independent. Merging them would mean reconstructing, at lowering
// time, information the abstraction had already discarded.
//
//   Stages      which pipeline work must finish before, and waits after
//   Access      which memory accesses must be made visible
//   Arrangement how a texture's memory is arranged; buffers have none
//
// Buffers genuinely have no arrangement. That is not a simplification: a buffer
// intent that names one is rejected rather than ignored, because ignoring it
// would let a pass author believe a transition happened.

use std::fmt;

/// Identifies a resource within one derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(pub u32);

/// Identifies a pass within one derivation, by its position in the order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassId(pub u32);

/// Whether a resource has an arrangement at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Texture,
    Buffer,
}

/// The queue a pass runs on.
///
/// This matters to barriers because not every queue can perform every
/// arrangement change: a copy queue cannot put a texture into a colour-target
/// arrangement, because it has no colour-output hardware to arrange it for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueueKind {
    Graphics,
    Compute,
    Transfer,
}

impl QueueKind {
    /// Whether this queue can move a texture into `arrangement`.
    ///
    /// ADR-0006 requires the stricter of the two APIs' rules wherever they
    /// differ, so that code validated on one backend is valid on the other.
    pub fn can_arrange(self, arrangement: Arrangement) -> bool {
        match arrangement {
            Arrangement::Undefined => true,
            Arrangement::TransferSource | Arrangement::TransferDestination => true,
            Arrangement::ShaderRead | Arrangement::Storage => {
                matches!(self, QueueKind::Graphics | QueueKind::Compute)
            }
            Arrangement::ColorTarget
            | Arrangement::DepthTarget
            | Arrangement::DepthReadOnly
            | Arrangement::Presentable => matches!(self, QueueKind::Graphics),
        }
    }
}

/// How a texture's memory is arranged for a particular kind of use.
///
/// A closed set: unlike a driver-returned enumeration, every value here is
/// authored in this repository, so exhaustive matching is sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arrangement {
    /// Contents are not preserved. The only legal starting point for a texture
    /// whose contents nobody has written yet, and the cheapest source for any
    /// transition, because the hardware may discard rather than convert.
    Undefined,
    ColorTarget,
    DepthTarget,
    /// Depth readable by shaders while still bound for testing. Distinct from
    /// `DepthTarget` because a read-only arrangement permits simultaneous
    /// sampling, and merging the two would forfeit that.
    DepthReadOnly,
    ShaderRead,
    Storage,
    TransferSource,
    TransferDestination,
    Presentable,
}

impl fmt::Display for Arrangement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Arrangement::Undefined => "Undefined",
            Arrangement::ColorTarget => "ColorTarget",
            Arrangement::DepthTarget => "DepthTarget",
            Arrangement::DepthReadOnly => "DepthReadOnly",
            Arrangement::ShaderRead => "ShaderRead",
            Arrangement::Storage => "Storage",
            Arrangement::TransferSource => "TransferSource",
            Arrangement::TransferDestination => "TransferDestination",
            Arrangement::Presentable => "Presentable",
        };
        formatter.write_str(name)
    }
}

/// Pipeline stages, as a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Stages(pub u32);

impl Stages {
    pub const NONE: Self = Self(0);
    pub const INDIRECT_FETCH: Self = Self(1 << 0);
    pub const GEOMETRY_FETCH: Self = Self(1 << 1);
    pub const VERTEX_WORK: Self = Self(1 << 2);
    pub const FRAGMENT_WORK: Self = Self(1 << 3);
    pub const DEPTH_TEST: Self = Self(1 << 4);
    pub const COLOR_OUTPUT: Self = Self(1 << 5);
    pub const COMPUTE_WORK: Self = Self(1 << 6);
    pub const TRANSFER: Self = Self(1 << 7);
    pub const RESOLVE: Self = Self(1 << 8);
    pub const RAY_TRACING: Self = Self(1 << 9);
    pub const PRESENT: Self = Self(1 << 10);

    /// Every stage a graphics pass can occupy.
    pub const ALL_GRAPHICS: Self = Self(
        Stages::INDIRECT_FETCH.0
            | Stages::GEOMETRY_FETCH.0
            | Stages::VERTEX_WORK.0
            | Stages::FRAGMENT_WORK.0
            | Stages::DEPTH_TEST.0
            | Stages::COLOR_OUTPUT.0,
    );

    /// Every stage. Legal, but see [`Intent::is_blanket`]: declaring it is a
    /// diagnostic rather than a convenience.
    pub const ALL: Self = Self(0x7FF);

    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl std::ops::BitOr for Stages {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Memory accesses, as a set.
///
/// Deliberately narrow. S-001 records that a blanket before-state implies every
/// write access and causes cache flushes nobody asked for, so the vocabulary
/// offers no "everything" value that a pass author would reach for by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Access(pub u32);

impl Access {
    pub const NONE: Self = Self(0);
    pub const INDIRECT_ARGUMENTS: Self = Self(1 << 0);
    pub const INDEX_DATA: Self = Self(1 << 1);
    pub const VERTEX_DATA: Self = Self(1 << 2);
    pub const UNIFORM_DATA: Self = Self(1 << 3);
    pub const SAMPLED_READ: Self = Self(1 << 4);
    pub const STORAGE_READ: Self = Self(1 << 5);
    pub const STORAGE_WRITE: Self = Self(1 << 6);
    pub const COLOR_READ: Self = Self(1 << 7);
    pub const COLOR_WRITE: Self = Self(1 << 8);
    pub const DEPTH_READ: Self = Self(1 << 9);
    pub const DEPTH_WRITE: Self = Self(1 << 10);
    pub const TRANSFER_READ: Self = Self(1 << 11);
    pub const TRANSFER_WRITE: Self = Self(1 << 12);
    pub const PRESENT_READ: Self = Self(1 << 13);

    /// The accesses that modify memory. Everything else only observes it.
    pub const WRITES: Self = Self(
        Access::STORAGE_WRITE.0 | Access::COLOR_WRITE.0 | Access::DEPTH_WRITE.0
            | Access::TRANSFER_WRITE.0,
    );

    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether this set modifies memory.
    #[inline]
    pub const fn writes(self) -> bool {
        self.intersects(Access::WRITES)
    }

    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl std::ops::BitOr for Access {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// What one pass does with one resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Intent {
    pub resource: ResourceId,
    pub kind: ResourceKind,
    pub stages: Stages,
    pub access: Access,
    /// Required arrangement. Must be [`Arrangement::Undefined`] for a buffer.
    pub arrangement: Arrangement,
}

impl Intent {
    /// A texture intent.
    pub fn texture(
        resource: ResourceId,
        stages: Stages,
        access: Access,
        arrangement: Arrangement,
    ) -> Self {
        Self { resource, kind: ResourceKind::Texture, stages, access, arrangement }
    }

    /// A buffer intent. Buffers have no arrangement, so none is taken.
    pub fn buffer(resource: ResourceId, stages: Stages, access: Access) -> Self {
        Self {
            resource,
            kind: ResourceKind::Buffer,
            stages,
            access,
            arrangement: Arrangement::Undefined,
        }
    }

    #[inline]
    pub fn writes(&self) -> bool {
        self.access.writes()
    }

    /// Whether this intent declares every stage rather than the ones it uses.
    ///
    /// Not rejected outright - a debug or capture pass may genuinely touch
    /// everything - but reported, because the usual cause is a pass author
    /// reaching for the widest value to make a hazard go away, and the barrier
    /// that results synchronises against work it has nothing to do with.
    #[inline]
    pub fn is_blanket(&self) -> bool {
        self.stages == Stages::ALL
    }

    /// Whether the declared arrangement can serve the declared access.
    ///
    /// This is the check that catches a pass asking to write colour into a
    /// texture arranged for sampling: legal to express, guaranteed to be wrong.
    pub fn arrangement_supports_access(&self) -> bool {
        if self.kind == ResourceKind::Buffer {
            return self.arrangement == Arrangement::Undefined;
        }
        match self.arrangement {
            // Nothing may be read from or written to an undefined arrangement;
            // it exists to be transitioned out of.
            Arrangement::Undefined => self.access.is_empty(),
            Arrangement::ColorTarget => {
                !self.access.intersects(Access(!(Access::COLOR_READ.0 | Access::COLOR_WRITE.0)))
            }
            Arrangement::DepthTarget => {
                !self.access.intersects(Access(!(Access::DEPTH_READ.0 | Access::DEPTH_WRITE.0)))
            }
            // Read-only depth may be sampled while still bound for testing,
            // which is the whole reason it is a separate arrangement.
            Arrangement::DepthReadOnly => !self
                .access
                .intersects(Access(!(Access::DEPTH_READ.0 | Access::SAMPLED_READ.0))),
            Arrangement::ShaderRead => {
                !self.access.intersects(Access(!(Access::SAMPLED_READ.0 | Access::STORAGE_READ.0)))
            }
            Arrangement::Storage => !self
                .access
                .intersects(Access(!(Access::STORAGE_READ.0 | Access::STORAGE_WRITE.0))),
            Arrangement::TransferSource => self.access.contains(Access::TRANSFER_READ)
                && !self.access.intersects(Access(!Access::TRANSFER_READ.0)),
            Arrangement::TransferDestination => self.access.contains(Access::TRANSFER_WRITE)
                && !self.access.intersects(Access(!Access::TRANSFER_WRITE.0)),
            Arrangement::Presentable => {
                !self.access.intersects(Access(!Access::PRESENT_READ.0))
            }
        }
    }
}

/// One side of a barrier: what to wait on, or what to unblock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scope {
    pub stages: Stages,
    pub access: Access,
}

impl Scope {
    pub const NONE: Self = Self { stages: Stages::NONE, access: Access::NONE };

    pub fn new(stages: Stages, access: Access) -> Self {
        Self { stages, access }
    }
}

/// A derived barrier.
///
/// Three kinds, because the two target APIs agree on three: a texture barrier
/// carries all three axes, a buffer barrier carries two, and a global barrier
/// carries two and applies across resources. A global barrier explicitly cannot
/// express an arrangement change, so it has no field for one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Barrier {
    Texture {
        resource: ResourceId,
        before: Scope,
        after: Scope,
        from: Arrangement,
        to: Arrangement,
    },
    Buffer {
        resource: ResourceId,
        before: Scope,
        after: Scope,
    },
    Global {
        before: Scope,
        after: Scope,
    },
}

impl Barrier {
    /// The resource this barrier applies to, if it applies to just one.
    pub fn resource(&self) -> Option<ResourceId> {
        match self {
            Barrier::Texture { resource, .. } | Barrier::Buffer { resource, .. } => Some(*resource),
            Barrier::Global { .. } => None,
        }
    }

    /// Whether this barrier changes a texture's arrangement.
    pub fn changes_arrangement(&self) -> bool {
        match self {
            Barrier::Texture { from, to, .. } => from != to,
            _ => false,
        }
    }
}
