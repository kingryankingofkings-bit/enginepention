// Pention Engine - pn-jobs
// Requirement: PN-PLT-013, PN-PLT-014, PN-OBJ-017
// Decision:    ADR-0009
//
// Safe Rust over the engine's C++ scheduler.
//
// Rust deliberately does NOT start its own scheduler. Two schedulers on one
// machine oversubscribe the cores and each makes the other's scheduling
// decisions wrong; ADR-0009 gives the C++ side sole ownership of worker
// threads, and this crate is how Rust uses them.
//
// The boundary is crossed once per `parallel_for` call plus once per chunk -
// never once per element. That ratio is what makes the hybrid viable, and it is
// asserted by test rather than assumed.

use core::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};

use pn_jobs_sys as sys;

/// Why a scheduler operation failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobError {
    /// The scheduler could not be created or started.
    StartFailed,
    /// An argument was rejected by the C ABI.
    InvalidArgument,
    /// A chunk body panicked. The panic was contained at the boundary rather
    /// than unwinding into C++ frames, and every remaining chunk still ran.
    BodyPanicked,
}

impl core::fmt::Display for JobError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            JobError::StartFailed => "the job system failed to start",
            JobError::InvalidArgument => "the job system rejected an argument",
            JobError::BodyPanicked => "a parallel body panicked; the panic was contained",
        };
        f.write_str(text)
    }
}

impl std::error::Error for JobError {}

/// Context handed across the boundary as `user_data`.
///
/// Holds a borrow of the caller's closure plus somewhere to record a panic.
/// A panic flag rather than a stored payload: the payload cannot travel back
/// through C, and what the caller needs to know is *that* a body failed.
struct ChunkContext<'a, F> {
    body: &'a F,
    panicked: &'a AtomicBool,
}

/// Trampoline invoked by C++ once per chunk.
///
/// `catch_unwind` is not defensive programming here - it is required. Unwinding
/// out of an `extern "C"` function into C++ frames is undefined behaviour, and
/// a panic in engine code is entirely possible (an index assertion, an
/// arithmetic overflow in a debug build). Containing it turns a crash of
/// undefined character into a reported error.
unsafe extern "C" fn chunk_trampoline<F>(user_data: *mut c_void, begin: u64, end: u64)
where
    F: Fn(u64, u64) + Sync,
{
    if user_data.is_null() {
        return;
    }
    let context = &*(user_data as *const ChunkContext<'_, F>);

    let outcome = catch_unwind(AssertUnwindSafe(|| {
        (context.body)(begin, end);
    }));

    if outcome.is_err() {
        context.panicked.store(true, Ordering::Relaxed);
    }
}

/// A running scheduler owned by the C++ side.
pub struct JobSystem {
    raw: *mut sys::PnJobSystem,
}

// SAFETY: the underlying scheduler is designed for concurrent use - workers
// steal from one another and `parallel_for` may be called from inside a running
// chunk without deadlocking, which is covered by tests on the C++ side. Only
// creation and destruction are single-threaded, and both are tied to `new` and
// `Drop` here, which Rust's ownership rules already serialize.
unsafe impl Send for JobSystem {}
unsafe impl Sync for JobSystem {}

impl JobSystem {
    /// Starts a scheduler with `worker_count` threads, or one per hardware
    /// thread minus the caller when zero.
    pub fn new(worker_count: u32) -> Result<Self, JobError> {
        // SAFETY: no invariants to uphold on entry; a null return is the
        // documented failure signal.
        let raw = unsafe { sys::pn_job_system_create(worker_count) };
        if raw.is_null() {
            return Err(JobError::StartFailed);
        }
        Ok(Self { raw })
    }

    /// Number of worker threads. The calling thread is not counted.
    pub fn worker_count(&self) -> u32 {
        // SAFETY: `raw` is non-null for the lifetime of `self`.
        unsafe { sys::pn_job_system_worker_count(self.raw) }
    }

    /// Splits `0..count` into chunks of at most `grain` and runs `body` on each,
    /// returning once every chunk has completed.
    ///
    /// `body` receives a half-open range and may run on any worker thread, so it
    /// must be `Sync`. It is borrowed rather than moved, so it does not need to
    /// be `'static` and large captures cost nothing at the boundary.
    ///
    /// A panic inside `body` is contained and reported as
    /// [`JobError::BodyPanicked`]; remaining chunks still run, because
    /// abandoning them would leave the caller's data half-processed with no way
    /// to tell which half.
    pub fn parallel_for<F>(&self, count: u64, grain: u64, body: F) -> Result<(), JobError>
    where
        F: Fn(u64, u64) + Sync,
    {
        if count == 0 {
            return Ok(());
        }
        let grain = grain.max(1);

        let panicked = AtomicBool::new(false);
        let context = ChunkContext { body: &body, panicked: &panicked };

        // SAFETY: `context` outlives the call - `pn_job_system_parallel_for`
        // does not return until every chunk has completed, which the C++ side
        // guarantees by waiting on the root job. The pointer is therefore valid
        // for every invocation of the trampoline.
        let status = unsafe {
            sys::pn_job_system_parallel_for(
                self.raw,
                count,
                grain,
                chunk_trampoline::<F>,
                core::ptr::addr_of!(context) as *mut c_void,
            )
        };

        match status {
            sys::PnJobStatus::Ok => {
                if panicked.load(Ordering::Relaxed) {
                    Err(JobError::BodyPanicked)
                } else {
                    Ok(())
                }
            }
            sys::PnJobStatus::StartFailed => Err(JobError::StartFailed),
            _ => Err(JobError::InvalidArgument),
        }
    }
}

impl Drop for JobSystem {
    fn drop(&mut self) {
        // SAFETY: `raw` was produced by pn_job_system_create and is destroyed
        // exactly once - JobSystem is neither Copy nor Clone.
        unsafe { sys::pn_job_system_destroy(self.raw) };
    }
}

/// Runs `body` once over the whole range on the calling thread, through the
/// same boundary as [`JobSystem::parallel_for`].
///
/// Exists so a test can compare a parallel result against a serial reference
/// without changing the code path being measured.
pub fn run_serial<F>(count: u64, body: F) -> Result<(), JobError>
where
    F: Fn(u64, u64) + Sync,
{
    if count == 0 {
        return Ok(());
    }
    let panicked = AtomicBool::new(false);
    let context = ChunkContext { body: &body, panicked: &panicked };

    // SAFETY: as above - the call is synchronous, so `context` outlives it.
    let status = unsafe {
        sys::pn_job_run_serial(
            count,
            chunk_trampoline::<F>,
            core::ptr::addr_of!(context) as *mut c_void,
        )
    };

    match status {
        sys::PnJobStatus::Ok if !panicked.load(Ordering::Relaxed) => Ok(()),
        sys::PnJobStatus::Ok => Err(JobError::BodyPanicked),
        _ => Err(JobError::InvalidArgument),
    }
}
