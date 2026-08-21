// Pention Engine - barrier derivation tests
// Requirement: PN-RND-003
// Decision:    ADR-0005, ADR-0006
//
// ADR-0006's validation plan calls for hand-computed expected barrier timelines
// asserted exactly, and for a deliberately hazardous pass sequence to be
// rejected. Both are here.
//
// What these cannot establish is that the derived barriers satisfy a real
// driver. ADR-0006 requires the hardware debug layer with synchronization
// validation to be silent before any of this is VERIFIED, and there is no
// hardware here (BLOCK-002, BLOCK-004).

use pn_rhi::barrier::Scope;
use pn_rhi::derive::Diagnostic;
use pn_rhi::{
    Access, Arrangement, Barrier, DerivationError, Intent, Pass, PassId, QueueKind, ResourceId,
    Stages, derive_barriers,
};

const COLOR: ResourceId = ResourceId(1);
const DEPTH: ResourceId = ResourceId(2);
const BUFFER: ResourceId = ResourceId(3);

fn graphics(name: impl Into<String>, intents: Vec<Intent>) -> Pass {
    Pass::new(name, QueueKind::Graphics, intents)
}

// -------------------------------------------------------------------
// The cases a naive implementation gets wrong
// -------------------------------------------------------------------

fn read(name: &str, stages: Stages) -> Pass {
    graphics(
        name,
        vec![Intent::texture(COLOR, stages, Access::SAMPLED_READ, Arrangement::ShaderRead)],
    )
}

fn write_color(name: &str) -> Pass {
    graphics(
        name,
        vec![Intent::texture(
            COLOR,
            Stages::COLOR_OUTPUT,
            Access::COLOR_WRITE,
            Arrangement::ColorTarget,
        )],
    )
}

#[test]
fn a_second_reader_in_a_stage_already_synchronised_for_needs_no_barrier() {
    // Same stage, same access, same arrangement: the first barrier already made
    // the write visible here. An implementation that emits a barrier per reader
    // serialises every sampler in the frame.
    let timeline = derive_barriers(&[
        write_color("write"),
        read("read a", Stages::FRAGMENT_WORK),
        read("read b", Stages::FRAGMENT_WORK),
    ])
    .expect("no hazard");

    assert_eq!(timeline.before(PassId(1)).len(), 1, "read after write needs one barrier");
    assert!(timeline.before(PassId(2)).is_empty(), "already visible to this stage");
}

#[test]
fn a_second_reader_in_a_new_stage_still_needs_a_barrier() {
    // The tempting simplification is "read after read is always free". It is
    // not: the first barrier made the write visible to the fragment stage, and
    // nothing has made it visible to compute. The arrangement is unchanged, so
    // an implementation keyed on arrangement alone emits nothing here and the
    // compute reader races the writer.
    let timeline = derive_barriers(&[
        write_color("write"),
        read("read in fragment", Stages::FRAGMENT_WORK),
        read("read in compute", Stages::COMPUTE_WORK),
    ])
    .expect("no hazard");

    match timeline.before(PassId(2)) {
        [Barrier::Texture { before, after, from, to, .. }] => {
            assert_eq!(*before, Scope::new(Stages::COLOR_OUTPUT, Access::COLOR_WRITE));
            assert_eq!(*after, Scope::new(Stages::COMPUTE_WORK, Access::SAMPLED_READ));
            assert_eq!(from, to, "no arrangement change, but still a visibility barrier");
        }
        other => panic!("expected one texture barrier, got {other:?}"),
    }
}

#[test]
fn a_writer_waits_for_every_reader_since_the_last_write_not_only_the_latest() {
    // Two readers, then a writer. Waiting only for the most recent reader is a
    // write-after-read hazard against the other one, and it is invisible until
    // the two readers happen to overlap.
    let passes = vec![
        graphics(
            "write",
            vec![Intent::texture(
                COLOR,
                Stages::COLOR_OUTPUT,
                Access::COLOR_WRITE,
                Arrangement::ColorTarget,
            )],
        ),
        graphics(
            "sample in fragment",
            vec![Intent::texture(
                COLOR,
                Stages::FRAGMENT_WORK,
                Access::SAMPLED_READ,
                Arrangement::ShaderRead,
            )],
        ),
        graphics(
            "sample in compute",
            vec![Intent::texture(
                COLOR,
                Stages::COMPUTE_WORK,
                Access::SAMPLED_READ,
                Arrangement::ShaderRead,
            )],
        ),
        graphics(
            "write again",
            vec![Intent::texture(
                COLOR,
                Stages::COLOR_OUTPUT,
                Access::COLOR_WRITE,
                Arrangement::ColorTarget,
            )],
        ),
    ];

    let timeline = derive_barriers(&passes).expect("no hazard");
    let barriers = timeline.before(PassId(3));
    assert_eq!(barriers.len(), 1);

    match barriers[0] {
        Barrier::Texture { before, after, from, to, .. } => {
            assert!(
                before.stages.contains(Stages::FRAGMENT_WORK),
                "dropped the fragment reader"
            );
            assert!(
                before.stages.contains(Stages::COMPUTE_WORK),
                "dropped the compute reader"
            );
            assert!(before.access.contains(Access::SAMPLED_READ));
            assert_eq!(after, Scope::new(Stages::COLOR_OUTPUT, Access::COLOR_WRITE));
            assert_eq!(from, Arrangement::ShaderRead);
            assert_eq!(to, Arrangement::ColorTarget);
        }
        other => panic!("expected a texture barrier, got {other:?}"),
    }
}

#[test]
fn a_first_use_transitions_out_of_undefined() {
    // A texture nobody has written starts undefined, and the first use must say
    // so - the hardware is allowed to discard rather than convert, which is only
    // correct if the source really is undefined.
    let timeline = derive_barriers(&[graphics(
        "first use",
        vec![Intent::texture(
            COLOR,
            Stages::COLOR_OUTPUT,
            Access::COLOR_WRITE,
            Arrangement::ColorTarget,
        )],
    )])
    .expect("no hazard");

    match timeline.before(PassId(0)) {
        [Barrier::Texture { from, to, before, .. }] => {
            assert_eq!(*from, Arrangement::Undefined);
            assert_eq!(*to, Arrangement::ColorTarget);
            assert_eq!(*before, Scope::NONE, "nothing has happened yet to wait for");
        }
        other => panic!("expected one texture barrier, got {other:?}"),
    }
}

#[test]
fn a_buffer_barrier_carries_no_arrangement() {
    // Buffers have no arrangement. The barrier kind reflects that structurally
    // rather than by convention, so there is no field to fill in wrongly.
    let passes = vec![
        graphics(
            "fill",
            vec![Intent::buffer(BUFFER, Stages::COMPUTE_WORK, Access::STORAGE_WRITE)],
        ),
        graphics(
            "draw indirect",
            vec![Intent::buffer(BUFFER, Stages::INDIRECT_FETCH, Access::INDIRECT_ARGUMENTS)],
        ),
    ];

    let timeline = derive_barriers(&passes).expect("no hazard");
    match timeline.before(PassId(1)) {
        [Barrier::Buffer { resource, before, after }] => {
            assert_eq!(*resource, BUFFER);
            assert_eq!(*before, Scope::new(Stages::COMPUTE_WORK, Access::STORAGE_WRITE));
            assert_eq!(
                *after,
                Scope::new(Stages::INDIRECT_FETCH, Access::INDIRECT_ARGUMENTS)
            );
        }
        other => panic!("expected one buffer barrier, got {other:?}"),
    }
    assert_eq!(
        timeline.barrier_count(),
        1,
        "the first write to a buffer waits for nothing, so it needs no barrier"
    );
    assert!(!timeline.barriers()[0].changes_arrangement());
}

#[test]
fn depth_written_then_sampled_moves_through_read_only() {
    // The depth prepass case: written for testing, then sampled while still
    // bound. DepthReadOnly exists so that the second use does not have to give
    // up depth testing, and merging it into DepthTarget would forfeit that.
    let passes = vec![
        graphics(
            "prepass",
            vec![Intent::texture(
                DEPTH,
                Stages::DEPTH_TEST,
                Access::DEPTH_WRITE,
                Arrangement::DepthTarget,
            )],
        ),
        graphics(
            "shading",
            vec![Intent::texture(
                DEPTH,
                Stages::FRAGMENT_WORK | Stages::DEPTH_TEST,
                Access::SAMPLED_READ | Access::DEPTH_READ,
                Arrangement::DepthReadOnly,
            )],
        ),
    ];

    let timeline = derive_barriers(&passes).expect("no hazard");
    match timeline.before(PassId(1)) {
        [Barrier::Texture { from, to, before, after, .. }] => {
            assert_eq!(*from, Arrangement::DepthTarget);
            assert_eq!(*to, Arrangement::DepthReadOnly);
            assert_eq!(*before, Scope::new(Stages::DEPTH_TEST, Access::DEPTH_WRITE));
            assert!(after.access.contains(Access::SAMPLED_READ));
        }
        other => panic!("expected one texture barrier, got {other:?}"),
    }
}

// -------------------------------------------------------------------
// A whole timeline, hand-computed
// -------------------------------------------------------------------

#[test]
fn a_four_pass_frame_derives_the_hand_computed_timeline() {
    // Depth prepass, opaque shading, post-process, present. Every barrier below
    // was worked out by hand from the intents before the code was run.
    let post = ResourceId(4);
    let passes = vec![
        graphics(
            "depth prepass",
            vec![Intent::texture(
                DEPTH,
                Stages::DEPTH_TEST,
                Access::DEPTH_WRITE,
                Arrangement::DepthTarget,
            )],
        ),
        graphics(
            "opaque",
            vec![
                Intent::texture(
                    DEPTH,
                    Stages::DEPTH_TEST,
                    Access::DEPTH_READ,
                    Arrangement::DepthReadOnly,
                ),
                Intent::texture(
                    COLOR,
                    Stages::COLOR_OUTPUT,
                    Access::COLOR_WRITE,
                    Arrangement::ColorTarget,
                ),
            ],
        ),
        graphics(
            "tone map",
            vec![
                Intent::texture(
                    COLOR,
                    Stages::FRAGMENT_WORK,
                    Access::SAMPLED_READ,
                    Arrangement::ShaderRead,
                ),
                Intent::texture(
                    post,
                    Stages::COLOR_OUTPUT,
                    Access::COLOR_WRITE,
                    Arrangement::ColorTarget,
                ),
            ],
        ),
        graphics(
            "present",
            vec![Intent::texture(
                post,
                Stages::PRESENT,
                Access::PRESENT_READ,
                Arrangement::Presentable,
            )],
        ),
    ];

    let timeline = derive_barriers(&passes).expect("no hazard");

    // Pass 0: depth from Undefined to DepthTarget, nothing to wait for.
    assert_eq!(timeline.before(PassId(0)).len(), 1);
    // Pass 1: depth to DepthReadOnly, and colour from Undefined.
    assert_eq!(timeline.before(PassId(1)).len(), 2);
    // Pass 2: colour to ShaderRead, and the post target from Undefined.
    assert_eq!(timeline.before(PassId(2)).len(), 2);
    // Pass 3: the post target to Presentable.
    assert_eq!(timeline.before(PassId(3)).len(), 1);

    assert_eq!(timeline.barrier_count(), 6);
    assert!(timeline.diagnostics.is_empty());

    // Every barrier in this frame is a texture barrier, and all but the two
    // first-use ones wait on real work.
    let waiting: usize = timeline
        .barriers()
        .iter()
        .filter(|barrier| match barrier {
            Barrier::Texture { before, .. } => !before.stages.is_empty(),
            _ => panic!("expected only texture barriers"),
        })
        .count();
    assert_eq!(waiting, 3, "three barriers wait; three are first-use transitions");
}

// -------------------------------------------------------------------
// Hazards, which are rejected rather than papered over
// -------------------------------------------------------------------

#[test]
fn a_buffer_that_names_an_arrangement_is_rejected() {
    // Ignoring it would let the author believe a transition happened.
    let mut intent = Intent::buffer(BUFFER, Stages::COMPUTE_WORK, Access::STORAGE_WRITE);
    intent.arrangement = Arrangement::ShaderRead;

    let error = derive_barriers(&[graphics("bad", vec![intent])]).expect_err("must be rejected");
    assert!(matches!(error, DerivationError::BufferHasNoArrangement { .. }), "{error}");
}

#[test]
fn an_arrangement_that_cannot_serve_the_access_is_rejected() {
    // Writing colour into a texture arranged for sampling is expressible and
    // guaranteed wrong.
    let error = derive_barriers(&[graphics(
        "bad",
        vec![Intent::texture(
            COLOR,
            Stages::COLOR_OUTPUT,
            Access::COLOR_WRITE,
            Arrangement::ShaderRead,
        )],
    )])
    .expect_err("must be rejected");
    assert!(
        matches!(error, DerivationError::ArrangementCannotServeAccess { .. }),
        "{error}"
    );
}

#[test]
fn a_transfer_queue_cannot_produce_a_colour_target() {
    // Queue-type compatibility is enforced at derivation, not discovered at
    // submission - ADR-0006 requires the stricter of the two APIs' rules.
    let error = derive_barriers(&[Pass::new(
        "copy queue",
        QueueKind::Transfer,
        vec![Intent::texture(
            COLOR,
            Stages::COLOR_OUTPUT,
            Access::COLOR_WRITE,
            Arrangement::ColorTarget,
        )],
    )])
    .expect_err("must be rejected");
    assert!(matches!(error, DerivationError::QueueCannotArrange { .. }), "{error}");
}

#[test]
fn a_compute_queue_may_still_use_storage_and_transfer_arrangements() {
    // The queue rule must not be so strict that async compute becomes
    // impossible; a rule that rejects legal work is as wrong as one that admits
    // illegal work.
    let timeline = derive_barriers(&[Pass::new(
        "async compute",
        QueueKind::Compute,
        vec![Intent::texture(
            COLOR,
            Stages::COMPUTE_WORK,
            Access::STORAGE_WRITE,
            Arrangement::Storage,
        )],
    )])
    .expect("compute may arrange for storage");
    assert_eq!(timeline.barrier_count(), 1);
}

#[test]
fn one_pass_declaring_two_arrangements_for_one_resource_is_rejected() {
    // A barrier separates passes. Within one pass there is nothing to separate,
    // so no barrier can satisfy both declarations.
    let error = derive_barriers(&[graphics(
        "bad",
        vec![
            Intent::texture(
                COLOR,
                Stages::COLOR_OUTPUT,
                Access::COLOR_WRITE,
                Arrangement::ColorTarget,
            ),
            Intent::texture(
                COLOR,
                Stages::FRAGMENT_WORK,
                Access::SAMPLED_READ,
                Arrangement::ShaderRead,
            ),
        ],
    )])
    .expect_err("must be rejected");
    assert!(
        matches!(error, DerivationError::ConflictingArrangementsInOnePass { .. }),
        "{error}"
    );
}

#[test]
fn one_pass_reading_and_writing_the_same_resource_is_rejected() {
    // Same arrangement this time, so the conflict is the ordering, not the
    // layout: within a pass there is no order to impose between the two.
    let error = derive_barriers(&[graphics(
        "bad",
        vec![
            Intent::texture(
                COLOR,
                Stages::COMPUTE_WORK,
                Access::STORAGE_WRITE,
                Arrangement::Storage,
            ),
            Intent::texture(
                COLOR,
                Stages::COMPUTE_WORK,
                Access::STORAGE_READ,
                Arrangement::Storage,
            ),
        ],
    )])
    .expect_err("must be rejected");
    assert!(matches!(error, DerivationError::ReadAndWriteInOnePass { .. }), "{error}");
}

#[test]
fn an_access_with_no_stage_that_performs_it_is_rejected() {
    // Compute stage, colour-write access. No hardware does this, and the
    // barrier it would derive reads plausibly while synchronising the wrong
    // thing.
    let error = derive_barriers(&[graphics(
        "bad",
        vec![Intent::texture(
            COLOR,
            Stages::COMPUTE_WORK,
            Access::COLOR_WRITE,
            Arrangement::ColorTarget,
        )],
    )])
    .expect_err("must be rejected");
    assert!(matches!(error, DerivationError::AccessWithoutStage { .. }), "{error}");
}

#[test]
fn an_intent_with_no_stage_at_all_is_rejected() {
    let error = derive_barriers(&[graphics(
        "bad",
        vec![Intent::buffer(BUFFER, Stages::NONE, Access::STORAGE_WRITE)],
    )])
    .expect_err("must be rejected");
    assert!(matches!(error, DerivationError::NoStagesDeclared { .. }), "{error}");
}

// -------------------------------------------------------------------
// Diagnostics, which do not stop derivation
// -------------------------------------------------------------------

#[test]
fn declaring_every_stage_is_reported_but_not_rejected() {
    // A capture or debug pass may genuinely touch everything, so this is not an
    // error. It is reported because the usual cause is a pass author widening
    // the declaration until a hazard went away.
    let timeline = derive_barriers(&[graphics(
        "capture",
        vec![Intent::texture(
            COLOR,
            Stages::ALL,
            Access::SAMPLED_READ,
            Arrangement::ShaderRead,
        )],
    )])
    .expect("not an error");

    assert_eq!(timeline.diagnostics.len(), 1);
    assert!(matches!(
        timeline.diagnostics[0],
        Diagnostic::BlanketStages { pass: PassId(0), resource: COLOR }
    ));
}

// -------------------------------------------------------------------
// Properties that should hold for any input
// -------------------------------------------------------------------

#[test]
fn every_pass_gets_an_entry_even_when_it_needs_no_barrier() {
    // The timeline is indexed by pass, so a pass with nothing to do must still
    // appear. Compacting it would misalign every later index.
    let passes = vec![
        graphics(
            "write",
            vec![Intent::texture(
                COLOR,
                Stages::COLOR_OUTPUT,
                Access::COLOR_WRITE,
                Arrangement::ColorTarget,
            )],
        ),
        graphics("nothing", vec![]),
        graphics(
            "read",
            vec![Intent::texture(
                COLOR,
                Stages::FRAGMENT_WORK,
                Access::SAMPLED_READ,
                Arrangement::ShaderRead,
            )],
        ),
    ];

    let timeline = derive_barriers(&passes).expect("no hazard");
    assert_eq!(timeline.entries.len(), 3);
    assert!(timeline.before(PassId(1)).is_empty());
    assert_eq!(timeline.entries[2].pass, PassId(2));
}

#[test]
fn an_empty_frame_derives_nothing() {
    let timeline = derive_barriers(&[]).expect("no hazard");
    assert_eq!(timeline.barrier_count(), 0);
    assert!(timeline.entries.is_empty());
}

#[test]
fn a_long_chain_of_readers_costs_exactly_one_barrier() {
    // Sixteen readers after one writer. A per-pass barrier would emit sixteen;
    // the derivation should emit one, and this is the assertion that would
    // catch a regression to per-pass emission.
    let mut passes = vec![graphics(
        "write",
        vec![Intent::texture(
            COLOR,
            Stages::COLOR_OUTPUT,
            Access::COLOR_WRITE,
            Arrangement::ColorTarget,
        )],
    )];
    for index in 0..16 {
        passes.push(graphics(
            format!("read {index}"),
            vec![Intent::texture(
                COLOR,
                Stages::FRAGMENT_WORK,
                Access::SAMPLED_READ,
                Arrangement::ShaderRead,
            )],
        ));
    }

    let timeline = derive_barriers(&passes).expect("no hazard");
    assert_eq!(timeline.barrier_count(), 2, "one first-use transition, one read-after-write");
}
