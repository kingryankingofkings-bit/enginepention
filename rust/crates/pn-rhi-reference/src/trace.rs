// Pention Engine - pn-rhi-reference/trace.rs
// Requirement: PN-RND-003, PN-OPS-007
// Decision:    ADR-0005
//
// The inspectable record ADR-0005 calls for. Every barrier and every pass is
// logged in the order it was executed, so a test can assert what happened rather
// than only whether it was rejected.
//
// A test that only checks "no hazards" passes for a timeline that does nothing
// at all. The trace is what makes the difference visible.

use std::fmt::Write as _;

use pn_rhi::{Arrangement, Barrier, ResourceId};

/// One thing the backend did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A barrier ran, moving a texture between arrangements.
    Transition { position: usize, resource: ResourceId, from: Arrangement, to: Arrangement },
    /// A barrier ran that ordered accesses without moving anything.
    Synchronize { position: usize, resource: Option<ResourceId> },
    /// A pass ran.
    Pass { position: usize, name: String },
}

/// The ordered record of a replay.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Trace {
    pub events: Vec<Event>,
}

impl Trace {
    pub(crate) fn record_barrier(&mut self, position: usize, barrier: &Barrier) {
        let event = match barrier {
            Barrier::Texture { resource, from, to, .. } if from != to => Event::Transition {
                position,
                resource: *resource,
                from: *from,
                to: *to,
            },
            Barrier::Texture { resource, .. } | Barrier::Buffer { resource, .. } => {
                Event::Synchronize { position, resource: Some(*resource) }
            }
            Barrier::Global { .. } => Event::Synchronize { position, resource: None },
        };
        self.events.push(event);
    }

    pub(crate) fn record_pass(&mut self, position: usize, name: &str) {
        self.events.push(Event::Pass { position, name: name.to_owned() });
    }

    /// Arrangement changes only, which is usually what a reader of a trace
    /// wants: they are the events with a cost and the ones a wrong derivation
    /// gets wrong first.
    pub fn transitions(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| matches!(event, Event::Transition { .. }))
            .collect()
    }

    /// A stable one-line-per-event rendering, for asserting a whole replay.
    pub fn to_text(&self) -> String {
        let mut out = String::with_capacity(self.events.len() * 48);
        for event in &self.events {
            match event {
                Event::Transition { position, resource, from, to } => {
                    let _ = writeln!(out, "{position}: resource {} {from} -> {to}", resource.0);
                }
                Event::Synchronize { position, resource: Some(resource) } => {
                    let _ = writeln!(out, "{position}: resource {} sync", resource.0);
                }
                Event::Synchronize { position, resource: None } => {
                    let _ = writeln!(out, "{position}: global sync");
                }
                Event::Pass { position, name } => {
                    let _ = writeln!(out, "{position}: pass '{name}'");
                }
            }
        }
        out
    }
}
