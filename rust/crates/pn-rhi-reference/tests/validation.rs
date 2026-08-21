// Pention Engine - reference backend validation tests
// Requirement: PN-RND-003, PN-OPS-007
// Decision:    ADR-0005, ADR-0006
//
// Half of these feed the backend a correctly derived timeline and require a
// clean report. That half alone would be worthless: a backend that reports
// nothing is also clean.
//
// The other half takes a correct timeline, breaks it in one specific way, and
// requires the backend to say so. Those are the tests that establish this is an
// oracle rather than a formality.

use pn_rhi::{
    Access, Arrangement, Barrier, Intent, Pass, PassId, QueueKind, ResourceId,
    ResourceKind, Scope, Stages, derive_barriers,
};
use pn_rhi_reference::{Hazard, ReferenceDevice};

const COLOR: ResourceId = ResourceId(0);
const DEPTH: ResourceId = ResourceId(1);
const POST: ResourceId = ResourceId(2);

fn graphics(name: &str, intents: Vec<Intent>) -> Pass {
    Pass::new(name, QueueKind::Graphics, intents)
}

fn color_write(resource: ResourceId) -> Intent {
    Intent::texture(resource, Stages::COLOR_OUTPUT, Access::COLOR_WRITE, Arrangement::ColorTarget)
}

fn sample(resource: ResourceId, stages: Stages) -> Intent {
    Intent::texture(resource, stages, Access::SAMPLED_READ, Arrangement::ShaderRead)
}

/// A frame that exercises a write, two readers in different stages, a depth
/// target, and a present.
fn a_frame() -> Vec<Pass> {
    vec![
        graphics("depth prepass", vec![Intent::texture(
            DEPTH,
            Stages::DEPTH_TEST,
            Access::DEPTH_WRITE,
            Arrangement::DepthTarget,
        )]),
        graphics("opaque", vec![
            Intent::texture(DEPTH, Stages::DEPTH_TEST, Access::DEPTH_READ, Arrangement::DepthReadOnly),
            color_write(COLOR),
        ]),
        graphics("tone map", vec![sample(COLOR, Stages::FRAGMENT_WORK), color_write(POST)]),
        graphics("histogram", vec![sample(COLOR, Stages::COMPUTE_WORK)]),
        graphics("present", vec![Intent::texture(
            POST,
            Stages::PRESENT,
            Access::PRESENT_READ,
            Arrangement::Presentable,
        )]),
    ]
}

fn fresh_device() -> ReferenceDevice {
    let mut device = ReferenceDevice::new();
    for resource in [COLOR, DEPTH, POST] {
        device.create(resource, ResourceKind::Texture);
    }
    device
}

// -------------------------------------------------------------------
// The derivation and the backend agree on a correct frame
// -------------------------------------------------------------------

#[test]
fn a_derived_timeline_replays_without_hazards() {
    let passes = a_frame();
    let timeline = derive_barriers(&passes).expect("derives");
    let report = fresh_device().replay(&passes, &timeline);
    assert!(report.is_clean(), "{}", report.describe());
}

#[test]
fn the_frame_ends_with_the_presented_image_presentable() {
    // Nothing else checks this. A frame whose last barrier leaves the image as a
    // colour target is a frame the presentation engine rejects.
    let passes = a_frame();
    let timeline = derive_barriers(&passes).expect("derives");
    let mut device = fresh_device();
    let report = device.replay(&passes, &timeline);
    assert!(report.is_clean(), "{}", report.describe());
    assert_eq!(device.arrangement_of(POST), Some(Arrangement::Presentable));
}

#[test]
fn the_trace_records_every_transition_and_pass_in_order() {
    // A clean report from a backend that did nothing is indistinguishable from a
    // clean report from one that checked everything. The trace is the
    // difference.
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
    ];
    let timeline = derive_barriers(&passes).expect("derives");
    let report = fresh_device().replay(&passes, &timeline);

    assert_eq!(
        report.trace.to_text(),
        "0: resource 0 Undefined -> ColorTarget\n\
         0: pass 'write'\n\
         1: resource 0 ColorTarget -> ShaderRead\n\
         1: pass 'read'\n"
    );
    assert_eq!(report.trace.transitions().len(), 2);
}

#[test]
fn an_imported_resource_arrives_with_contents_in_a_declared_arrangement() {
    // A swapchain image the presentation engine just released is not in the same
    // arrangement as a texture the previous frame left as a render target.
    // Assuming either would make the first barrier of every frame unverifiable.
    let passes = vec![graphics("blit source", vec![sample(COLOR, Stages::FRAGMENT_WORK)])];

    let mut device = ReferenceDevice::new();
    device.import(COLOR, ResourceKind::Texture, Arrangement::ShaderRead);

    // Derivation would emit a transition out of Undefined here, because it does
    // not know the resource was imported. Hand-write the timeline that matches
    // reality instead: no transition needed.
    let timeline = pn_rhi::Timeline {
        entries: vec![pn_rhi::derive::PassBarriers { pass: PassId(0), barriers: vec![] }],
        diagnostics: vec![],
    };

    let report = device.replay(&passes, &timeline);
    assert!(report.is_clean(), "{}", report.describe());
}

// -------------------------------------------------------------------
// The backend rejects timelines that are wrong in one specific way
// -------------------------------------------------------------------

#[test]
fn a_missing_barrier_is_reported() {
    // Delete the barrier before the reader. Nothing else changes.
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
    ];
    let mut timeline = derive_barriers(&passes).expect("derives");
    timeline.entries[1].barriers.clear();

    let report = fresh_device().replay(&passes, &timeline);
    assert!(!report.is_clean(), "a missing barrier must not replay clean");
    assert!(
        report.hazards.iter().any(|h| matches!(h, Hazard::WrongArrangement { .. })),
        "{}",
        report.describe()
    );
    assert!(
        report.hazards.iter().any(|h| matches!(h, Hazard::WriteNotVisible { .. })),
        "{}",
        report.describe()
    );
}

#[test]
fn a_barrier_whose_destination_is_too_narrow_is_reported() {
    // This is the exact defect the derivation had: a barrier makes a write
    // visible to the reader's stages, so a reader in a stage nobody synchronised
    // for still races the writer even though the arrangement is right.
    //
    // The arrangement here is deliberately correct, so nothing but the
    // visibility check can catch it.
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read in fragment", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
        graphics("read in compute", vec![sample(COLOR, Stages::COMPUTE_WORK)]),
    ];
    let mut timeline = derive_barriers(&passes).expect("derives");

    // The compute reader's barrier exists and is correct; remove it, leaving the
    // arrangement already right from the fragment reader's transition.
    assert_eq!(timeline.entries[2].barriers.len(), 1);
    timeline.entries[2].barriers.clear();

    let report = fresh_device().replay(&passes, &timeline);
    let visibility: Vec<&Hazard> = report
        .hazards
        .iter()
        .filter(|h| matches!(h, Hazard::WriteNotVisible { .. }))
        .collect();
    assert_eq!(visibility.len(), 1, "{}", report.describe());
    assert!(
        !report.hazards.iter().any(|h| matches!(h, Hazard::WrongArrangement { .. })),
        "the arrangement is right; only visibility is wrong:\n{}",
        report.describe()
    );
}

#[test]
fn a_transition_from_the_wrong_arrangement_is_reported() {
    // The derivation's idea of a resource's state diverging from what the
    // preceding barriers actually did. Silent on hardware until it is not.
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
    ];
    let mut timeline = derive_barriers(&passes).expect("derives");

    if let Barrier::Texture { from, .. } = &mut timeline.entries[1].barriers[0] {
        *from = Arrangement::DepthTarget;
    } else {
        panic!("expected a texture barrier");
    }

    let report = fresh_device().replay(&passes, &timeline);
    assert!(
        report
            .hazards
            .iter()
            .any(|h| matches!(h, Hazard::TransitionFromWrongArrangement { .. })),
        "{}",
        report.describe()
    );
}

#[test]
fn a_write_after_read_with_nothing_between_is_reported() {
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
        graphics("write again", vec![color_write(COLOR)]),
    ];
    let mut timeline = derive_barriers(&passes).expect("derives");
    timeline.entries[2].barriers.clear();

    let report = fresh_device().replay(&passes, &timeline);
    assert!(
        report.hazards.iter().any(|h| matches!(h, Hazard::WriteNotOrdered { .. })),
        "{}",
        report.describe()
    );
}

#[test]
fn an_empty_timeline_for_a_real_frame_is_rejected_everywhere() {
    // The degenerate case that a backend which reports nothing would pass. If
    // this replays clean, none of the other results mean anything.
    let passes = a_frame();
    let empty = pn_rhi::Timeline {
        entries: (0..passes.len())
            .map(|index| pn_rhi::derive::PassBarriers {
                pass: PassId(index as u32),
                barriers: vec![],
            })
            .collect(),
        diagnostics: vec![],
    };

    let report = fresh_device().replay(&passes, &empty);
    assert!(!report.is_clean());
    assert!(report.hazards.len() >= 5, "{}", report.describe());
}

#[test]
fn reading_a_resource_nothing_has_written_is_reported() {
    let passes = vec![graphics("read", vec![sample(COLOR, Stages::FRAGMENT_WORK)])];
    let timeline = derive_barriers(&passes).expect("derives");

    let report = fresh_device().replay(&passes, &timeline);
    assert!(
        report
            .hazards
            .iter()
            .any(|h| matches!(h, Hazard::ReadOfUndefinedContents { .. })),
        "{}",
        report.describe()
    );
}

#[test]
fn a_barrier_naming_an_unregistered_resource_is_reported() {
    let unknown = ResourceId(99);
    let passes = vec![graphics("write", vec![color_write(unknown)])];
    let timeline = derive_barriers(&passes).expect("derives");

    let report = fresh_device().replay(&passes, &timeline);
    assert!(
        report.hazards.iter().any(|h| matches!(h, Hazard::UnknownResource { .. })),
        "{}",
        report.describe()
    );
}

#[test]
fn a_texture_barrier_applied_to_a_buffer_is_reported() {
    let buffer = ResourceId(7);
    let passes = vec![graphics("write", vec![color_write(buffer)])];
    let timeline = derive_barriers(&passes).expect("derives");

    let mut device = ReferenceDevice::new();
    device.create(buffer, ResourceKind::Buffer);

    let report = device.replay(&passes, &timeline);
    assert!(
        report.hazards.iter().any(|h| matches!(h, Hazard::WrongBarrierKind { .. })),
        "{}",
        report.describe()
    );
}

// -------------------------------------------------------------------
// Properties of the checking itself
// -------------------------------------------------------------------

#[test]
fn every_hazard_in_a_frame_is_reported_not_only_the_first() {
    // Three broken passes should send someone round the loop once, not three
    // times.
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read a", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
        graphics("write b", vec![color_write(DEPTH)]),
        graphics("read b", vec![sample(DEPTH, Stages::FRAGMENT_WORK)]),
    ];
    let mut timeline = derive_barriers(&passes).expect("derives");
    timeline.entries[1].barriers.clear();
    timeline.entries[3].barriers.clear();

    let report = fresh_device().replay(&passes, &timeline);
    let resources: Vec<ResourceId> = report
        .hazards
        .iter()
        .filter_map(|hazard| match hazard {
            Hazard::WriteNotVisible { resource, .. } => Some(*resource),
            _ => None,
        })
        .collect();
    assert!(resources.contains(&COLOR), "{}", report.describe());
    assert!(resources.contains(&DEPTH), "{}", report.describe());
}

#[test]
fn a_pass_write_does_not_satisfy_a_read_declared_in_the_same_pass() {
    // Within a pass there is no ordering, so a write cannot make anything
    // visible to a read beside it. Folding effects in as each intent is checked
    // would quietly permit this.
    //
    // The RHI rejects this shape outright, so the timeline is built by hand.
    let passes = vec![Pass::new(
        "both",
        QueueKind::Graphics,
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
    )];
    let timeline = pn_rhi::Timeline {
        entries: vec![pn_rhi::derive::PassBarriers {
            pass: PassId(0),
            barriers: vec![Barrier::Texture {
                resource: COLOR,
                before: Scope::NONE,
                after: Scope::new(Stages::COMPUTE_WORK, Access::STORAGE_WRITE),
                from: Arrangement::Undefined,
                to: Arrangement::Storage,
            }],
        }],
        diagnostics: vec![],
    };

    let report = fresh_device().replay(&passes, &timeline);
    assert!(
        report
            .hazards
            .iter()
            .any(|h| matches!(h, Hazard::ReadOfUndefinedContents { .. })),
        "the write beside it must not count as having happened first:\n{}",
        report.describe()
    );
}

#[test]
fn a_barrier_whose_source_is_too_narrow_to_cover_the_write_is_reported() {
    // Distinct from a missing barrier: the barrier is present, the arrangement
    // change is right, and it is still wrong, because its source scope does not
    // name the write it is supposed to be waiting for. Nothing is made visible.
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
    ];
    let mut timeline = derive_barriers(&passes).expect("derives");

    if let Barrier::Texture { before, .. } = &mut timeline.entries[1].barriers[0] {
        // Waits on the transfer stage, which wrote nothing.
        *before = Scope::new(Stages::TRANSFER, Access::TRANSFER_WRITE);
    } else {
        panic!("expected a texture barrier");
    }

    let report = fresh_device().replay(&passes, &timeline);
    assert!(
        report.hazards.iter().any(|h| matches!(h, Hazard::WriteNotVisible { .. })),
        "{}",
        report.describe()
    );
    assert!(
        !report.hazards.iter().any(|h| matches!(h, Hazard::WrongArrangement { .. })),
        "the arrangement change still happened:\n{}",
        report.describe()
    );
}

#[test]
fn an_over_conservative_barrier_is_accepted() {
    // A barrier wider than necessary costs performance, not correctness, and
    // this backend makes no performance claim. Rejecting it would make the
    // oracle disagree with hardware in the safe direction, which is still
    // disagreeing.
    let passes = vec![
        graphics("write", vec![color_write(COLOR)]),
        graphics("read", vec![sample(COLOR, Stages::FRAGMENT_WORK)]),
    ];
    let mut timeline = derive_barriers(&passes).expect("derives");

    if let Barrier::Texture { before, after, .. } = &mut timeline.entries[1].barriers[0] {
        *before = Scope::new(Stages::ALL, Access(before.access.0 | Access::TRANSFER_WRITE.0));
        *after = Scope::new(Stages::ALL, Access(after.access.0 | Access::STORAGE_READ.0));
    } else {
        panic!("expected a texture barrier");
    }

    let report = fresh_device().replay(&passes, &timeline);
    assert!(report.is_clean(), "{}", report.describe());
}
