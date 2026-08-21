// Pention Engine - pn-jobs-sys
// Requirement: PN-PLT-013, PN-OBJ-017
// Decision:    ADR-0009
//
// Raw declarations mirroring engine/jobs/include/pn/jobs/jobs_c_api.h.
//
// This crate is a transcription of that header and nothing more: no safety, no
// abstraction, no convenience. Anything ergonomic belongs in `pn-jobs`, so that
// the set of things actually crossing the boundary stays small enough to audit
// by reading one file against one header.

#![no_std]
#![allow(non_camel_case_types)]

use core::ffi::c_void;

/// Opaque handle to a running scheduler.
#[repr(C)]
pub struct PnJobSystem {
    _private: [u8; 0],
}

/// Status codes. Zero is success; negative values are failures.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnJobStatus {
    Ok = 0,
    NullArgument = -1,
    InvalidArgument = -2,
    StartFailed = -3,
    BodyAborted = -4,
}

/// Body of a parallel loop, invoked once per chunk on a worker thread.
///
/// Must not unwind. A Rust implementation is responsible for containing its own
/// panics before returning; unwinding into C++ frames is undefined behaviour.
pub type PnParallelForBody =
    unsafe extern "C" fn(user_data: *mut c_void, begin: u64, end: u64);

extern "C" {
    pub fn pn_job_system_create(worker_count: u32) -> *mut PnJobSystem;
    pub fn pn_job_system_destroy(system: *mut PnJobSystem);
    pub fn pn_job_system_worker_count(system: *const PnJobSystem) -> u32;
    pub fn pn_job_system_parallel_for(
        system: *mut PnJobSystem,
        count: u64,
        grain: u64,
        body: PnParallelForBody,
        user_data: *mut c_void,
    ) -> PnJobStatus;
    pub fn pn_job_run_serial(
        count: u64,
        body: PnParallelForBody,
        user_data: *mut c_void,
    ) -> PnJobStatus;
}
