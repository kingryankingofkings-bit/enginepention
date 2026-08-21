/* Pention Engine - jobs/jobs_c_api.h
 * Requirement: PN-PLT-013, PN-OBJ-017
 * Decision:    ADR-0009 (hybrid Rust/C++; this header is the boundary contract)
 *
 * C ABI over the C++ job system, so Rust can use the engine's one scheduler
 * rather than starting a second one that competes for the same cores.
 *
 * This header IS the contract. Per ADR-0009, nothing crosses the boundary that
 * is not declared here, and every entry point states its crossing frequency -
 * the whole viability of the hybrid rests on those frequencies staying low.
 *
 * Rules this header obeys:
 *   - plain data only: primitives, opaque pointers, function pointers
 *   - the allocating side frees: pn_job_system_create pairs with _destroy
 *   - nothing unwinds across the boundary in either direction
 */

#ifndef PN_JOBS_C_API_H
#define PN_JOBS_C_API_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Opaque handle to a running scheduler. */
typedef struct PnJobSystem PnJobSystem;

/** Status codes. Zero is success; negative values are failures. */
typedef enum PnJobStatus {
    PN_JOB_OK = 0,
    PN_JOB_ERROR_NULL_ARGUMENT = -1,
    PN_JOB_ERROR_INVALID_ARGUMENT = -2,
    PN_JOB_ERROR_START_FAILED = -3,
    PN_JOB_ERROR_BODY_ABORTED = -4
} PnJobStatus;

/** Body of a parallel loop, invoked once per chunk on a worker thread.
 *
 * `begin` is inclusive, `end` exclusive. The callee must not unwind: a Rust
 * implementation wraps its body so a panic becomes a reported failure rather
 * than an unwind into C++ frames, which is undefined behaviour.
 *
 * Crossing frequency: once per chunk, so `count / grain` times per loop - not
 * once per index.
 */
typedef void (*PnParallelForBody)(void* user_data, uint64_t begin, uint64_t end);

/** Creates and starts a scheduler.
 *
 * @param worker_count worker threads to start; 0 means one per hardware thread
 *                     minus the calling thread.
 * @return NULL on failure.
 *
 * Crossing frequency: once per process, or once per test.
 */
PnJobSystem* pn_job_system_create(uint32_t worker_count);

/** Stops every worker and releases the scheduler. Passing NULL is a no-op.
 *
 * Crossing frequency: once per process.
 */
void pn_job_system_destroy(PnJobSystem* system);

/** Number of worker threads. Returns 0 for NULL.
 *
 * Crossing frequency: rare; configuration and diagnostics only.
 */
uint32_t pn_job_system_worker_count(const PnJobSystem* system);

/** Splits [0, count) into chunks of at most `grain` and runs `body` on each,
 * returning once every chunk has completed.
 *
 * The calling thread participates rather than blocking, so calling this from
 * inside another parallel body does not deadlock.
 *
 * Crossing frequency: ONCE per call, plus one callback per chunk. A
 * 50,000-element loop at grain 512 crosses about 98 times, not 50,000. Keeping
 * that ratio is what makes the hybrid viable; see ADR-0009.
 */
PnJobStatus pn_job_system_parallel_for(PnJobSystem* system,
                                       uint64_t count,
                                       uint64_t grain,
                                       PnParallelForBody body,
                                       void* user_data);

/** Runs `body` once for the whole range, on the calling thread.
 *
 * Exists so a caller can exercise the same code path without a scheduler, and
 * so tests can compare a parallel result against a serial reference through an
 * identical interface.
 *
 * Crossing frequency: once per call.
 */
PnJobStatus pn_job_run_serial(uint64_t count, PnParallelForBody body, void* user_data);

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* PN_JOBS_C_API_H */
