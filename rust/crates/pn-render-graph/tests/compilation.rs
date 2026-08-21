// Pention Engine - render graph compilation tests
// Requirement: PN-RND-005
// Decision:    ADR-0005, ADR-0006
//
// PN-RND-005's acceptance criterion is "hazard cases detected by test; graph
// emits a pass/resource visualization". Both halves are here.
//
// What is not here is any claim that a compiled graph records correctly on
// hardware. There is no hardware (BLOCK-002). What these establish is that the
// ordering, culling, lifetime, and hazard logic is right - which is a question
// about a directed acyclic graph, and answerable here.

use pn_render_graph::compile::compile;
use pn_render_graph::{CompileError, Diagnostic, Graph, to_dot};
use pn_rhi::{Access, Arrangement, QueueKind, Stages};

const COLOR_WRITE: (Stages, Access, Arrangement) =
    (Stages::COLOR_OUTPUT, Access::COLOR_WRITE, Arrangement::ColorTarget);
const SAMPLE: (Stages, Access, Arrangement) =
    (Stages::FRAGMENT_WORK, Access::SAMPLED_READ, Arrangement::ShaderRead);

// -------------------------------------------------------------------
// Ordering
// -------------------------------------------------------------------

#[test]
fn declaration_order_versions_resources() {
    // Declaring a consumer before its producer does not reorder them: it means
    // the consumer reads whatever the resource held on entry to the graph. For a
    // transient that is undefined, so it is a hazard, not an authoring
    // convenience the graph silently fixes.
    //
    // The alternative - binding a read to a writer declared after it - would
    // mean moving a declaration silently changed which frame's contents a pass
    // sampled. Under this rule, moving a declaration is either a no-op or a
    // visible error.
    let mut graph = Graph::new();
    let target = graph.persistent_texture("target");
    let intermediate = graph.transient_texture("intermediate");

    graph.pass("consumer", QueueKind::Graphics)
        .texture(intermediate, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(target, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("producer", QueueKind::Graphics)
        .texture(intermediate, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    match compile(&graph) {
        Err(CompileError::ReadOfUnwrittenResource { pass, resource }) => {
            assert_eq!(pass, "consumer");
            assert_eq!(resource, "intermediate");
        }
        other => panic!("expected the read to be rejected, got {other:?}"),
    }
}

#[test]
fn a_producer_declared_first_is_ordered_first() {
    // The same two passes the other way round. This is the arrangement that
    // means what the author intended, and it compiles.
    let mut graph = Graph::new();
    let target = graph.persistent_texture("target");
    let intermediate = graph.transient_texture("intermediate");

    let producer = graph.pass("producer", QueueKind::Graphics)
        .texture(intermediate, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    let consumer = graph.pass("consumer", QueueKind::Graphics)
        .texture(intermediate, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(target, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order, vec![producer, consumer]);
}

#[test]
fn ordering_is_deterministic_across_runs() {
    // Without a stable tie-break the order depends on iteration order, and every
    // barrier-timeline assertion in this suite becomes flaky rather than wrong -
    // which is far more expensive to diagnose.
    let build = || {
        let mut graph = Graph::new();
        let screen = graph.persistent_texture("screen");
        let a = graph.transient_texture("a");
        let b = graph.transient_texture("b");
        graph.pass("write a", QueueKind::Graphics)
            .texture(a, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
            .build();
        graph.pass("write b", QueueKind::Graphics)
            .texture(b, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
            .build();
        graph.pass("combine", QueueKind::Graphics)
            .texture(a, SAMPLE.0, SAMPLE.1, SAMPLE.2)
            .texture(b, SAMPLE.0, SAMPLE.1, SAMPLE.2)
            .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
            .build();
        graph
    };

    let first = compile(&build()).expect("compiles");
    for _ in 0..8 {
        let again = compile(&build()).expect("compiles");
        assert_eq!(first.order, again.order);
        assert_eq!(first.timeline, again.timeline);
    }
}

#[test]
fn two_passes_that_look_mutually_dependent_are_not_a_cycle() {
    // Each pass reads what the other writes. Under declaration-order versioning
    // this is not circular: 'a' reads the contents 'right' held on entry, and
    // 'b' reads what 'a' wrote to 'left'. Every dependency edge runs forward, so
    // the edge set cannot contain a cycle at all.
    //
    // Both resources are persistent, because reading a transient nobody has
    // written yet is separately a hazard.
    let mut graph = Graph::new();
    let left = graph.persistent_texture("left");
    let right = graph.persistent_texture("right");

    let a = graph.pass("a", QueueKind::Graphics)
        .texture(right, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(left, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    let b = graph.pass("b", QueueKind::Graphics)
        .texture(left, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(right, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("not a cycle under this semantic");
    assert_eq!(compiled.order, vec![a, b]);
}

// -------------------------------------------------------------------
// Culling
// -------------------------------------------------------------------

#[test]
fn a_pass_nothing_consumes_is_culled() {
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let orphan = graph.transient_texture("orphan");

    let wanted = graph.pass("wanted", QueueKind::Graphics)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    let dead = graph.pass("dead", QueueKind::Graphics)
        .texture(orphan, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order, vec![wanted]);
    assert_eq!(compiled.culled, vec![dead]);
    assert!(compiled
        .diagnostics
        .iter()
        .any(|d| matches!(d, Diagnostic::PassCulled { pass } if pass == "dead")));
}

#[test]
fn culling_propagates_backwards_through_a_chain() {
    // Three passes feeding each other, none reaching anything that outlives the
    // graph. Culling only the last would leave two passes producing input for a
    // pass that no longer exists - which is why liveness is iterated to a fixed
    // point rather than computed in one sweep.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let a = graph.transient_texture("a");
    let b = graph.transient_texture("b");

    let kept = graph.pass("kept", QueueKind::Graphics)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("chain 1", QueueKind::Graphics)
        .texture(a, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("chain 2", QueueKind::Graphics)
        .texture(a, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(b, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("chain 3", QueueKind::Graphics)
        .texture(b, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order, vec![kept]);
    assert_eq!(compiled.culled.len(), 3);
}

#[test]
fn a_chain_that_reaches_a_persistent_resource_survives_entirely() {
    // The same shape as the culled chain, differing only in where it ends. If
    // this were also culled, culling would be discarding real work.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let a = graph.transient_texture("a");
    let b = graph.transient_texture("b");

    graph.pass("first", QueueKind::Graphics)
        .texture(a, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("second", QueueKind::Graphics)
        .texture(a, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(b, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("third", QueueKind::Graphics)
        .texture(b, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order.len(), 3);
    assert!(compiled.culled.is_empty());
}

#[test]
fn a_pinned_pass_survives_even_with_no_consumer() {
    // A readback or a query is consumed outside the graph's model. Without the
    // pin it looks exactly like dead work.
    let mut graph = Graph::new();
    let scratch = graph.transient_texture("scratch");

    let readback = graph.pass("readback", QueueKind::Graphics)
        .pinned()
        .texture(scratch, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order, vec![readback]);
    assert!(compiled.culled.is_empty());
}

// -------------------------------------------------------------------
// Hazards
// -------------------------------------------------------------------

#[test]
fn reading_a_transient_resource_nothing_writes_is_rejected() {
    // Its contents are whatever was in memory. This is the hazard that produces
    // a picture that is correct on the machine it was authored on.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let never_written = graph.transient_texture("never written");

    graph.pass("reader", QueueKind::Graphics)
        .texture(never_written, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    match compile(&graph) {
        Err(CompileError::ReadOfUnwrittenResource { pass, resource }) => {
            assert_eq!(pass, "reader");
            assert_eq!(resource, "never written");
        }
        other => panic!("expected the hazard to be rejected, got {other:?}"),
    }
}

#[test]
fn reading_a_persistent_resource_nothing_in_the_graph_writes_is_fine() {
    // The mirror of the previous test, and the reason it cannot simply reject
    // every unwritten read: a history buffer or a streamed texture was filled
    // before this graph ran.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let history = graph.persistent_texture("history");

    graph.pass("reader", QueueKind::Graphics)
        .texture(history, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("a persistent resource arrives filled");
    assert_eq!(compiled.order.len(), 1);
}

#[test]
fn using_a_buffer_as_a_texture_is_rejected() {
    // The two produce different barrier kinds, so this is not a naming slip.
    let mut graph = Graph::new();
    let buffer = graph.persistent_buffer("counts");

    // Reach past the builder deliberately: the typed API would not permit this,
    // and the check exists for the untyped paths that will eventually feed the
    // graph from data.
    let mut pass = graph.pass("confused", QueueKind::Graphics);
    pass = pass.texture(buffer, SAMPLE.0, SAMPLE.1, SAMPLE.2);
    pass.build();

    match compile(&graph) {
        Err(CompileError::ResourceKindMismatch { pass, resource, .. }) => {
            assert_eq!(pass, "confused");
            assert_eq!(resource, "counts");
        }
        other => panic!("expected a kind mismatch, got {other:?}"),
    }
}

#[test]
fn a_hazard_from_barrier_derivation_surfaces_as_a_compile_error() {
    // The graph does not re-implement the RHI's rules; it must not swallow them
    // either. A transfer queue cannot produce a colour target.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");

    graph.pass("copy queue", QueueKind::Transfer)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    assert!(matches!(compile(&graph), Err(CompileError::Derivation(_))));
}

// -------------------------------------------------------------------
// Diagnostics
// -------------------------------------------------------------------

#[test]
fn a_write_that_a_later_write_discards_is_reported() {
    // Both passes survive - the first may have other outputs - but this output
    // is thrown away, and silence about it is how a frame ends up computing
    // something twice.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let scratch = graph.transient_texture("scratch");

    graph.pass("first write", QueueKind::Graphics)
        .texture(scratch, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("second write", QueueKind::Graphics)
        .texture(scratch, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("reader", QueueKind::Graphics)
        .texture(scratch, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order.len(), 3, "neither writer is culled");
    assert!(
        compiled.diagnostics.iter().any(|d| matches!(
            d,
            Diagnostic::OverwrittenBeforeRead { discarded, by, .. }
                if discarded == "first write" && by == "second write"
        )),
        "{:?}",
        compiled.diagnostics
    );
}

// -------------------------------------------------------------------
// Lifetimes
// -------------------------------------------------------------------

#[test]
fn a_lifetime_spans_first_to_last_touch_in_execution_order() {
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let early = graph.transient_texture("early");
    let late = graph.transient_texture("late");

    graph.pass("produce early", QueueKind::Graphics)
        .texture(early, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("consume early, produce late", QueueKind::Graphics)
        .texture(early, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(late, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("consume late", QueueKind::Graphics)
        .texture(late, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");

    let early_span = compiled.lifetimes[&early.id()];
    let late_span = compiled.lifetimes[&late.id()];
    assert_eq!((early_span.first, early_span.last), (0, 1));
    assert_eq!((late_span.first, late_span.last), (1, 2));

    // They share pass 1, so their memory cannot be shared. Getting this off by
    // one is aliasing that survives every test that does not place two
    // resources exactly end to end.
    assert!(early_span.overlaps(&late_span));
    assert!(compiled.disjoint_from(early.id()).is_empty());
}

#[test]
fn resources_that_do_not_overlap_are_reported_as_aliasable() {
    // The foundation PN-RND-006 will build on. Reported now, unused now.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let first = graph.transient_texture("first");
    let second = graph.transient_texture("second");

    graph.pass("produce first", QueueKind::Graphics)
        .texture(first, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("consume first", QueueKind::Graphics)
        .texture(first, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("produce second", QueueKind::Graphics)
        .texture(second, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("consume second", QueueKind::Graphics)
        .texture(second, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    assert_eq!(compiled.order.len(), 4);
    assert_eq!(compiled.disjoint_from(first.id()), vec![second.id()]);
    assert_eq!(compiled.disjoint_from(second.id()), vec![first.id()]);
}

// -------------------------------------------------------------------
// Barriers, through the graph
// -------------------------------------------------------------------

#[test]
fn barriers_are_indexed_by_execution_position_not_declaration_order() {
    // The bug this guards against compiles, runs, and puts every barrier before
    // the wrong pass.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let intermediate = graph.transient_texture("intermediate");

    // A dead pass is declared between the two, so culling makes execution
    // position and declaration index genuinely differ.
    graph.pass("producer", QueueKind::Graphics)
        .texture(intermediate, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    let orphan = graph.transient_texture("orphan");
    graph.pass("dead", QueueKind::Graphics)
        .texture(orphan, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("consumer", QueueKind::Graphics)
        .texture(intermediate, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");

    // Position 0 is the producer: one first-use transition for `intermediate`.
    assert_eq!(compiled.barriers_before(0).len(), 1);
    // Position 1 is the consumer, which is declaration index 2: `intermediate`
    // to shader-read, and `screen` out of undefined. Indexing the timeline by
    // declaration index would read the culled pass's (empty) entry here.
    assert_eq!(compiled.barriers_before(1).len(), 2);
    assert_eq!(compiled.barrier_count(), 3);
    assert_eq!(compiled.order.len(), 2, "the dead pass is gone");
}

// -------------------------------------------------------------------
// Visualization
// -------------------------------------------------------------------

#[test]
fn the_visualization_shows_passes_resources_edges_and_lifetimes() {
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let scratch = graph.transient_texture("scratch");
    let orphan = graph.transient_texture("orphan");

    graph.pass("produce", QueueKind::Graphics)
        .texture(scratch, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("present", QueueKind::Graphics)
        .texture(scratch, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("dead", QueueKind::Graphics)
        .texture(orphan, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    let dot = to_dot(&graph, &compiled);

    assert!(dot.starts_with("digraph render_graph {"));
    assert!(dot.trim_end().ends_with('}'));

    // Passes carry their execution position, which is the thing a reader of the
    // picture actually wants to know.
    assert!(dot.contains("label=\"0: produce"), "{dot}");
    assert!(dot.contains("label=\"1: present"), "{dot}");

    // A culled pass is drawn, not hidden. A culled pass is usually a mistake,
    // and omitting it from the picture hides the mistake.
    assert!(dot.contains("dead (culled)"), "{dot}");

    // Persistent and transient resources are distinguishable at a glance.
    assert!(dot.contains("doubleoctagon, label=\"screen"), "{dot}");
    assert!(dot.contains("ellipse, label=\"scratch"), "{dot}");
    assert!(dot.contains("scratch\\n[0..1]"), "{dot}");

    // Direction encodes read versus write.
    assert!(dot.contains("pass0 -> res1;"), "produce writes scratch:\n{dot}");
    assert!(dot.contains("res1 -> pass1;"), "present reads scratch:\n{dot}");
}

#[test]
fn the_visualization_is_byte_for_byte_stable() {
    // A picture that renders differently each run cannot be regression tested,
    // and one that cannot be regression tested drifts out of agreement with the
    // graph it claims to show.
    let build = || {
        let mut graph = Graph::new();
        let screen = graph.persistent_texture("screen");
        let scratch = graph.transient_texture("scratch");
        graph.pass("produce", QueueKind::Graphics)
            .texture(scratch, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
            .build();
        graph.pass("present", QueueKind::Graphics)
            .texture(scratch, SAMPLE.0, SAMPLE.1, SAMPLE.2)
            .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
            .build();
        graph
    };

    let graph = build();
    let reference = to_dot(&graph, &compile(&graph).expect("compiles"));
    for _ in 0..8 {
        let graph = build();
        assert_eq!(to_dot(&graph, &compile(&graph).expect("compiles")), reference);
    }
}

#[test]
fn an_empty_graph_compiles_to_nothing_and_still_renders() {
    let graph = Graph::new();
    let compiled = compile(&graph).expect("compiles");
    assert!(compiled.order.is_empty());
    assert_eq!(compiled.barrier_count(), 0);
    assert!(to_dot(&graph, &compiled).contains("digraph render_graph"));
}


#[test]
fn execution_order_is_the_surviving_passes_in_declaration_order() {
    // Today this holds because declaration-order versioning makes every
    // dependency edge run forward. The test exists to name that assumption: if
    // explicit resource versioning is ever added, reordering becomes possible
    // and this test should fail rather than the order quietly changing under
    // every barrier assertion in the suite.
    let mut graph = Graph::new();
    let screen = graph.persistent_texture("screen");
    let a = graph.transient_texture("a");
    let b = graph.transient_texture("b");
    let orphan = graph.transient_texture("orphan");

    graph.pass("p0", QueueKind::Graphics)
        .texture(a, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("dead", QueueKind::Graphics)
        .texture(orphan, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("p2", QueueKind::Graphics)
        .texture(a, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(b, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();
    graph.pass("p3", QueueKind::Graphics)
        .texture(b, SAMPLE.0, SAMPLE.1, SAMPLE.2)
        .texture(screen, COLOR_WRITE.0, COLOR_WRITE.1, COLOR_WRITE.2)
        .build();

    let compiled = compile(&graph).expect("compiles");
    let mut ascending = compiled.order.clone();
    ascending.sort();
    assert_eq!(compiled.order, ascending);
    assert_eq!(compiled.order.len(), 3);
}
