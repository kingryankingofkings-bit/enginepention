# ADR-0008 - Work-stealing scheduler design

- **Status:** Accepted - **Date:** 2026-08-21 - **Requirements:** PN-PLT-013, PN-PLT-014 - **Addresses:** RSK-M02

## Context

Every subsystem above this one assumes it can parallelise. Retrofitting
threading into code that assumed single-threaded mutable access is the largest
rewrite an engine can undergo (RSK-M02), which is why the scheduler is Tier A
and precedes the subsystems that depend on it rather than following them.

The requirement is a work-stealing job graph with dependencies, priorities, and
cancellation, that scales with core count and has no mutex on a per-frame hot
path.

## Alternatives

**A. A single shared queue behind a mutex.** Simple and obviously correct.
Rejected: every worker contends on one lock on every job acquisition, so
throughput falls as cores are added - the opposite of the requirement. It is
also precisely the "mutex on the per-frame hot path" that has to be removed
later, at which point every subsystem built on it is affected.

**B. Per-worker queues, each behind its own mutex, stolen from with try_lock.**
Genuinely defensible: much simpler than a lock-free deque and contention is
low because the owner and thieves touch opposite ends. Rejected, narrowly,
because the owner still pays an atomic lock acquisition per job even when
uncontended, and job granularity in a renderer is small enough for that to
matter. Recorded rather than dismissed: if the lock-free deque ever proves
troublesome, this is the fallback and it is a local change.

**C. A bounded single-producer, multi-consumer deque per worker, with the owner
at one end and thieves at the other.** Chosen.

## Decision

One bounded deque per worker. The owner pushes and pops at the bottom; thieves
take from the top. Implemented from the published description of the algorithm
(Chase and Lev), from its prose and invariants rather than from any reference
implementation - see PROVENANCE_LEDGER.md.

Four consequential choices within that:

**Bounded, not growable.** Capacity is a budget, as everywhere else in this
engine. A deque that reallocates while thieves are indexing it is a much harder
object to reason about, for a benefit this scheduler does not need: a full deque
means the caller runs the work inline, which is a graceful degradation rather
than a failure.

**The submitting thread owns a deque too, and it is a steal target.** The thread
that calls `start()` gets a slot in the same array as the workers. This is not
symmetry for its own sake - see "What went wrong" below.

**`wait()` executes work rather than blocking.** A job that waits on children it
created must not park its worker, or a pool where several jobs do so deadlocks
with work still queued. The waiting thread runs available jobs until its target
completes.

**Jobs live in a per-worker ring, allocated without locks and recycled.** A job
is 64-byte aligned with a fixed inline payload; captures larger than the payload
are a compile error rather than a silent heap allocation, because an allocation
on the scheduling path is exactly what this design exists to avoid.

## What went wrong, and what it cost to find

Both defects below were found by tests written to look for them, not by
inspection. Both are recorded because the tests that caught them are the reason
to keep them.

**1. Work submitted from the main thread was invisible to the pool.**
`schedule()` pushes to the calling thread's deque, and the steal loop originally
iterated over worker deques only. So every job submitted from the main thread -
the primary use case - sat in a deque no worker could reach, and the main
thread's `wait()` executed all of it itself. Every correctness test still
passed, because the results were right; the parallelism was simply zero.

Caught by `stealing_actually_happens_under_load`, which asserts the steal
counter is non-zero. Without an assertion that the mechanism is *live*, a
scheduler that silently degrades to single-threaded is indistinguishable from
one that works.

**2. `finish()` read `job->parent` after releasing the job's slot.**
Reaching zero on the completion counter is exactly the signal that makes a slot
eligible for reuse; `allocate_job()` then overwrites `function` and `parent`.
Reading `parent` after the decrement therefore raced the owning thread's reuse
of that slot. The fix is to read the parent pointer before the decrement and
treat the job as no longer ours afterwards.

This one is invisible to ordinary testing - it produces a wrong parent pointer
only under a precise interleaving - and was found by ThreadSanitizer, which
reported the read against `allocate_job`'s write directly.

**3. Three tests asserted scheduling outcomes rather than structural
properties.** Not defects in the scheduler - defects in how it was tested, and
worth as much space because they are the more insidious kind.

Each had the same shape. The deque's contended-race test required that both the
owner and a thief win at least once across 2000 rounds. The scheduler's
steal test required the steal counter to be non-zero. And the test written to
*replace* that one asserted `steal_attempts > 0`, with a comment confidently
calling it structural.

None of the three is a property of this code. Which thread wins a
few-instruction race, whether a worker ever runs dry, whether a steal is even
attempted - all are properties of the scheduler and the core count. On CI they
failed in *both directions*: one configuration saw the thief win all 2000
rounds, another saw the owner win all 2000, while the exactly-one-taker
invariant held 2000 of 2000 in each. A real ordering bug cannot produce "always
exactly one winner, but always the same one".

**The rule that actually holds: assert what the caller can observe, never what
the workers happened to do.**

The concerns behind those assertions were legitimate - a test that never
contends proves nothing, and a scheduler that silently degrades to
single-threaded must be caught. Both are now met without depending on timing:

- `last_item_taken_by_the_owner` and `last_item_taken_by_a_thief` drive the
  contended compare-exchange branch down each side deterministically, with no
  second thread at all.
- `work_submitted_from_the_main_thread_is_reachable_by_workers` asserts
  reachability rather than stealing. It deliberately does not call `wait()`,
  because `wait()` executes work on the calling thread and would mask the very
  defect it tests; the main thread spins on a flag with a deadline, so the job
  can only complete if a worker reached it. Before defect 1 was fixed, this
  never completed.

CI now runs the whole suite a second time under `taskset -c 0`. A single core is
the cheapest way to expose this class of defect, and it is the class most likely
to pass on a developer machine and fail on a small runner.

**A note on tool feedback.** TSan initially also reported the publication of the
job payload as a race. That one traced to the deque using a standalone release
fence plus a relaxed store, a formulation the standard makes equivalent to a
release store but which race detectors model imprecisely. It was changed to a
release store: on the most concurrency-critical object in the engine, a
formulation a tool can verify beats an equivalent one it cannot. Distinguishing
the two reports mattered - one was a tooling limitation and one was a real bug,
and treating either as the other would have been wrong.

## Consequences

- A job's captured state must be trivially destructible and fit the payload.
  Both are compile-time errors, not runtime surprises.
- Allocating more than `kJobsPerWorker` live jobs on one worker returns null,
  and callers fall back to inline execution rather than corrupting a slot.
- Cancellation does not propagate to children. Propagating would mean walking a
  structure being mutated concurrently, and half-doing it would be worse than
  not doing it. Stated in the API rather than left to be discovered.
- Idle workers sleep on a condition variable rather than spinning. On a small
  core count, spinning workers steal cycles from the ones doing useful work.

## Risks

| Risk | Mitigation |
|---|---|
| Memory-ordering error in the deque | 20 consecutive ThreadSanitizer runs clean; concurrent tests assert every item is taken exactly once - never duplicated, never lost |
| The owner/thief race on the final item | Dedicated test over 2000 rounds asserting exactly one taker, and asserting both outcomes actually occur so the test is not silently degenerate |
| Job ring wraps while a slot is live | Reuse is refused while the completion count is positive; the caller falls back to inline execution |
| Scheduler silently degrades to single-threaded | Steal counters asserted non-zero under load - the assertion that caught defect 1 |
| Deadlock under nested waiting | `wait()` executes work; a test has a job wait on children it creates |

## Validation plan

Deque: LIFO for the owner, FIFO for thieves, refusal when full, the contended
single-item case, a concurrent drain, and concurrent push against steal - the
last two asserting exact-once delivery across thousands of items.

Scheduler: single job, many jobs, parent/child completion, nested children,
cancellation, `parallel_for` against a serial reference, degenerate inputs, zero
workers, nested waiting, steal counters non-zero, and twenty start/stop cycles.

All of it under ThreadSanitizer and AddressSanitizer, under both compilers.

**Not validated here:** that the scheduler *scales*. Measuring that needs
hardware and a workload, and this host has four cores and no performance budget
to measure against. Scaling is a target, not an achievement, and is recorded as
such.

## Reversal cost

**Moderate.** The deque is replaceable behind its interface - alternative B
above is a local change. The `wait()`-executes-work contract is not: subsystems
will rely on being able to wait from inside a job, and removing that would
change every call site.
