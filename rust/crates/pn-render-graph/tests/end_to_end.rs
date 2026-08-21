// Pention Engine - render graph validated against the reference backend
// Requirement: PN-RND-003, PN-RND-005
// Decision:    ADR-0005, ADR-0006
//
// The other test files check the graph and the backend separately. These run a
// graph all the way through - declaration, culling, ordering, barrier
// derivation - and hand the result to the backend, which reconstructs each
// resource's state from the barriers alone and checks the passes against it.
//
// A clean report here means the derived barriers really do put every resource
// where the pass that uses it needs it. That is the property ADR-0006's
// validation plan asks for, minus the hardware half it also asks for.

use pn_render_graph::compile::compile;
use pn_render_graph::{Graph, ResourceHandle};
use pn_rhi::{Access, Arrangement, Pass, QueueKind, ResourceKind, Stages};
use pn_rhi_reference::ReferenceDevice;

fn ordered_passes(graph: &Graph, compiled: &pn_render_graph::compile::Compiled) -> Vec<Pass> {
    compiled
        .order
        .iter()
        .map(|index| graph.pass_declaration(*index))
        .collect()
}

/// Replays a compiled graph, treating every declared resource as created empty.
fn validate(graph: &Graph, compiled: &pn_render_graph::compile::Compiled) -> String {
    let mut device = ReferenceDevice::new();
    for index in 0..graph.resource_count() {
        let handle = ResourceHandle(index as u32);
        device.create(handle.id(), graph.resource_kind(handle));
    }
    let report = device.replay(&ordered_passes(graph, compiled), &compiled.timeline);
    if report.is_clean() {
        String::new()
    } else {
        report.describe()
    }
}

const COLOR_WRITE: (Stages, Access, Arrangement) =
    (Stages::COLOR_OUTPUT, Access::COLOR_WRITE, Arrangement::ColorTarget);
const SAMPLE: (Stages, Access, Arrangement) =
    (Stages::FRAGMENT_WORK, Access::SAMPLED_READ, Arrangement::ShaderRead);

#[test]
fn a_deferred_shaped_frame_validates_end_to_end() {
    // Depth prepass, three g-buffer targets, a lighting pass that samples all of
    // them, tone mapping, and a present. Enough shape that a derivation working
    // by coincidence would be caught.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let depth = graph.transient_texture("depth");
    let albedo = graph.transient_texture("albedo");
    let normal = graph.transient_texture("normal");
    let lit = graph.transient_texture("lit");

    graph.pass("depth prepass", QueueKind::Graphics)
        .texture(depth, Stages::DEPTH_TEST, Access::DEPTH_WRITE, Arrangement::DepthTarget)
        .build();
    graph.pass("gbuffer", QueueKind::Graphics)
        .texture(depth, Stages::DEPTH_TEST, Access::DEPTH_READ, Arrangement::DepthReadOnly)
        .texture(albedo, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .texture(normal, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("lighting", QueueKind::Graphics)
        .texture(albedo, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(normal, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(depth, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(lit, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("tone map", QueueKind::Graphics)
        .texture(lit, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order.len(), 4);
    assert_eq!(validate(&graph, &compiled), "", "the backend rejected the derived barriers");
}

#[test]
fn a_frame_with_a_compute_reader_validates_end_to_end() {
    // The shape that caught the derivation's read-after-read defect: two readers
    // of one write, in different pipeline stages.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let scene = graph.transient_texture("scene");

    graph.pass("render", QueueKind::Graphics)
        .texture(scene, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("histogram", QueueKind::Graphics)
        .texture(scene, Stages::COMPUTE_WORK, Access::SAMPLED_READ, Arrangement::ShaderRead)
        .build()
        ;
    graph.pass("tone map", QueueKind::Graphics)
        .texture(scene, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(validate(&graph, &compiled), "");
}

#[test]
fn a_frame_that_reuses_a_target_validates_end_to_end() {
    // Ping-pong: two passes write the same target with a reader between them, so
    // the second write is a genuine write-after-read.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let scratch = graph.transient_texture("scratch");

    graph.pass("blur horizontal", QueueKind::Graphics)
        .texture(scratch, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("sample once", QueueKind::Graphics)
        .texture(scratch, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("blur vertical", QueueKind::Graphics)
        .texture(scratch, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("sample again", QueueKind::Graphics)
        .texture(scratch, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order.len(), 4);
    assert_eq!(validate(&graph, &compiled), "");
}

#[test]
fn a_culled_pass_leaves_the_surviving_frame_valid() {
    // Culling removes passes after the intents were declared. If the barriers
    // were derived against the uncut list, the survivors would be separated by
    // barriers referring to a pass that no longer runs.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let scene = graph.transient_texture("scene");
    let orphan = graph.transient_texture("orphan");

    graph.pass("render", QueueKind::Graphics)
        .texture(scene, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("dead", QueueKind::Graphics)
        .texture(orphan, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("present", QueueKind::Graphics)
        .texture(scene, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order.len(), 2);
    assert_eq!(compiled.culled.len(), 1);
    assert_eq!(validate(&graph, &compiled), "");
}

#[test]
fn a_buffer_alongside_textures_validates_end_to_end() {
    // Buffers take a different barrier kind, and a backend that applied a
    // texture barrier to one would say so.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let counts = graph.transient_buffer("counts");

    graph.pass("count", QueueKind::Graphics)
        .buffer(counts, Stages::COMPUTE_WORK, Access::STORAGE_WRITE)
        .build();
    graph.pass("draw indirect", QueueKind::Graphics)
        .buffer(counts, Stages::INDIRECT_FETCH, Access::INDIRECT_ARGUMENTS)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(validate(&graph, &compiled), "");
    assert_eq!(graph.resource_kind(counts), ResourceKind::Buffer);
}
