// Pention Engine - pn-vulkan-sys
// Requirement: PN-RND-001, PN-RND-002
// Decision:    ADR-0009, ADR-0010
//
// Raw Vulkan types and constants. No safety, no ergonomics, no lifetimes: this
// crate is the ABI surface and nothing else. The safe wrapper lives in
// pn-vulkan, which is where invariants belong.
//
// Every item below is machine-generated from the official Khronos registry by
// rust/tools/vkgen. Nothing here is hand-written, and nothing is copied from
// the Vulkan headers or from any binding crate - see DEPENDENCY_BOUNDARY.md
// and PROVENANCE_LEDGER.md.
//
//     python3 build_scripts/fetch_vulkan_registry.py
//     cargo run -p vkgen -- third_party/vk.xml \
//         --emit rust/crates/pn-vulkan-sys/src/generated.rs

#![no_std]

mod generated;

pub use generated::*;
