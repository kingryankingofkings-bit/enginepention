// Pention Engine - pn-jobs integration tests
// Requirement: PN-PLT-013, PN-PLT-014, PN-OBJ-017
// Decision:    ADR-0009
//
// These tests exercise the whole hybrid path: Rust closures executed on C++
// worker threads, through the C ABI. They are the evidence that the boundary
// contract in ADR-0009 actually holds, rather than only being written down.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use pn_jobs::{run_serial, JobError, JobSystem};

#[test]
fn scheduler_starts_with_the_requested_worker_count() {
    let system = JobSystem::new(3).expect("scheduler should start");
    assert_eq!(system.worker_count(), 3);
}

#[test]
fn zero_count_does_nothing_and_succeeds() {
    let system = JobSystem::new(2).expect("scheduler should start");
    let calls = AtomicUsize::new(0);
    system
        .parallel_for(0, 16, |_, _| {
            calls.fetch_add(1, Ordering::Relaxed);
        })
        .expect("zero count is not an error");
    assert_eq!(calls.load(Ordering::Relaxed), 0);
}

#[test]
fn every_index_is_visited_exactly_once() {
    const COUNT: u64 = 10_000;
    let visits: Vec<AtomicU64> = (0..COUNT).map(|_| AtomicU64::new(0)).collect();

    let system = JobSystem::new(3).expect("scheduler should start");
    system
        .parallel_for(COUNT, 64, |begin, end| {
            for i in begin..end {
                visits[i as usize].fetch_add(1, Ordering::Relaxed);
            }
        })
        .expect("parallel_for should succeed");

    let wrong = visits
        .iter()
        .filter(|v| v.load(Ordering::Relaxed) != 1)
        .count();
    assert_eq!(wrong, 0, "every index must be visited exactly once");
}

#[test]
fn parallel_result_matches_the_serial_reference() {
    const COUNT: u64 = 50_000;
    let values: Vec<u64> = (1..=COUNT).collect();

    let parallel_total = AtomicU64::new(0);
    let system = JobSystem::new(3).expect("scheduler should start");
    system
        .parallel_for(COUNT, 512, |begin, end| {
            let mut local = 0u64;
            for i in begin..end {
                local += values[i as usize];
            }
            parallel_total.fetch_add(local, Ordering::Relaxed);
        })
        .expect("parallel_for should succeed");

    let serial_total = AtomicU64::new(0);
    run_serial(COUNT, |begin, end| {
        let mut local = 0u64;
        for i in begin..end {
            local += values[i as usize];
        }
        serial_total.fetch_add(local, Ordering::Relaxed);
    })
    .expect("serial run should succeed");

    assert_eq!(
        parallel_total.load(Ordering::Relaxed),
        serial_total.load(Ordering::Relaxed)
    );
    assert_eq!(serial_total.load(Ordering::Relaxed), COUNT * (COUNT + 1) / 2);
}

#[test]
fn boundary_is_crossed_per_chunk_not_per_element() {
    // The claim the hybrid rests on. If a future change makes the boundary
    // fine-grained, this fails loudly rather than quietly costing an order of
    // magnitude in overhead.
    const COUNT: u64 = 50_000;
    const GRAIN: u64 = 512;

    let crossings = AtomicUsize::new(0);
    let system = JobSystem::new(3).expect("scheduler should start");
    system
        .parallel_for(COUNT, GRAIN, |_, _| {
            crossings.fetch_add(1, Ordering::Relaxed);
        })
        .expect("parallel_for should succeed");

    let observed = crossings.load(Ordering::Relaxed) as u64;
    let expected = COUNT.div_ceil(GRAIN);
    assert_eq!(observed, expected, "one crossing per chunk");
    assert!(
        observed < COUNT / 10,
        "crossings ({observed}) must be far below the element count ({COUNT})"
    );
}

#[test]
fn a_panicking_body_is_contained_and_reported() {
    // Unwinding out of an extern "C" function into C++ frames is undefined
    // behaviour. The trampoline catches it, so a panic in engine code becomes a
    // reported error rather than a crash of undefined character.
    //
    // If containment ever regressed, this test would abort the process rather
    // than fail - which is itself an unmistakable signal.
    let system = JobSystem::new(2).expect("scheduler should start");

    let result = system.parallel_for(100, 10, |begin, _end| {
        if begin == 0 {
            panic!("deliberate panic inside a parallel body");
        }
    });

    assert_eq!(result, Err(JobError::BodyPanicked));

    // And the scheduler is still usable afterwards - a contained panic must not
    // poison the pool.
    let total = AtomicU64::new(0);
    system
        .parallel_for(100, 10, |begin, end| {
            total.fetch_add(end - begin, Ordering::Relaxed);
        })
        .expect("the scheduler must remain usable after a contained panic");
    assert_eq!(total.load(Ordering::Relaxed), 100);
}

#[test]
fn remaining_chunks_still_run_when_one_panics() {
    // Abandoning the rest would leave the caller's data half-processed with no
    // way to tell which half. Every chunk runs; the failure is reported once.
    const COUNT: u64 = 1_000;
    const GRAIN: u64 = 10;

    let completed = AtomicUsize::new(0);
    let system = JobSystem::new(3).expect("scheduler should start");

    let result = system.parallel_for(COUNT, GRAIN, |begin, _end| {
        if begin == 0 {
            panic!("deliberate");
        }
        completed.fetch_add(1, Ordering::Relaxed);
    });

    assert_eq!(result, Err(JobError::BodyPanicked));
    let expected_chunks = COUNT.div_ceil(GRAIN) as usize - 1; // all but the panicking one
    assert_eq!(completed.load(Ordering::Relaxed), expected_chunks);
}

#[test]
fn large_captures_cost_nothing_at_the_boundary() {
    // The closure is borrowed, not copied into a fixed payload, so a capture
    // far larger than any inline job payload is fine.
    let big: Vec<u64> = (0..100_000).collect();
    let total = AtomicU64::new(0);

    let system = JobSystem::new(2).expect("scheduler should start");
    system
        .parallel_for(big.len() as u64, 4096, |begin, end| {
            let mut local = 0u64;
            for i in begin..end {
                local += big[i as usize];
            }
            total.fetch_add(local, Ordering::Relaxed);
        })
        .expect("parallel_for should succeed");

    let expected: u64 = big.iter().sum();
    assert_eq!(total.load(Ordering::Relaxed), expected);
}

#[test]
fn nested_parallel_for_does_not_deadlock() {
    // The calling thread participates rather than blocking, so a chunk may
    // itself call parallel_for.
    let system = JobSystem::new(2).expect("scheduler should start");
    let inner = AtomicU64::new(0);

    system
        .parallel_for(8, 1, |_, _| {
            system
                .parallel_for(100, 10, |b, e| {
                    inner.fetch_add(e - b, Ordering::Relaxed);
                })
                .expect("nested parallel_for should succeed");
        })
        .expect("outer parallel_for should succeed");

    assert_eq!(inner.load(Ordering::Relaxed), 800);
}
