// Pention Engine - pn-render-graph/visualize.rs
// Requirement: PN-RND-005
// Decision:    ADR-0005
//
// The pass/resource visualization PN-RND-005's acceptance criterion calls for.
//
// DOT, because it is a text format with a published grammar that this file can
// emit from first principles - no library, and no binary blob a reviewer cannot
// read. The output is deterministic, so a test can assert it exactly; a
// visualization that renders differently on each run cannot be regression
// tested, and one that cannot be regression tested drifts out of agreement with
// the graph it claims to show.

use std::fmt::Write as _;

use pn_rhi::ResourceId;

use crate::compile::Compiled;
use crate::graph::{Graph, PassIndex, Residency};

/// Renders the compiled graph as a DOT digraph.
///
/// Passes appear in execution order with their position, culled passes are
/// shown dashed rather than hidden - a culled pass is usually a mistake, and
/// omitting it from the picture hides the mistake - and each edge is labelled
/// with the resource that creates the dependency.
pub fn to_dot(graph: &Graph, compiled: &Compiled) -> String {
    let mut out = String::with_capacity(1024);

    let _ = writeln!(out, "digraph render_graph {{");
    let _ = writeln!(out, "  rankdir=LR;");
    let _ = writeln!(out, "  node [fontname=\"monospace\"];");
    let _ = writeln!(out);

    let _ = writeln!(out, "  // passes, in execution order");
    for (position, index) in compiled.order.iter().enumerate() {
        let pass = &graph.passes[index.0 as usize];
        let _ = writeln!(
            out,
            "  pass{} [shape=box, label=\"{}: {}\\n{:?}\"];",
            index.0,
            position,
            escape(&pass.name),
            pass.queue
        );
    }
    for index in &compiled.culled {
        let pass = &graph.passes[index.0 as usize];
        let _ = writeln!(
            out,
            "  pass{} [shape=box, style=dashed, label=\"{} (culled)\"];",
            index.0,
            escape(&pass.name)
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "  // resources");
    for (id, declaration) in graph.resources.iter().enumerate() {
        let resource = ResourceId(id as u32);
        let shape = match declaration.residency {
            Residency::Transient => "ellipse",
            Residency::Persistent => "doubleoctagon",
        };
        let lifetime = match compiled.lifetimes.get(&resource) {
            Some(span) => format!("\\n[{}..{}]", span.first, span.last),
            None => "\\n(unused)".to_owned(),
        };
        let _ = writeln!(
            out,
            "  res{id} [shape={shape}, label=\"{}{lifetime}\"];",
            escape(&declaration.name)
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "  // reads and writes");
    for (index, pass) in graph.passes.iter().enumerate() {
        let pass_index = PassIndex(index as u32);
        let live = compiled.order.contains(&pass_index);
        let style = if live { "" } else { " [style=dashed]" };
        for intent in &pass.intents {
            let resource = intent.resource.0;
            if intent.writes() {
                let _ = writeln!(out, "  pass{index} -> res{resource}{style};");
            } else if !intent.access.is_empty() {
                let _ = writeln!(out, "  res{resource} -> pass{index}{style};");
            } else {
                // No access at all: an arrangement change and nothing more.
                let _ = writeln!(
                    out,
                    "  res{resource} -> pass{index} [style=dotted, label=\"arrange\"];"
                );
            }
        }
    }

    let _ = writeln!(out, "}}");
    out
}

/// Escapes the characters DOT treats specially inside a quoted label.
fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}
