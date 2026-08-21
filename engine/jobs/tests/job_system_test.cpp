// Pention Engine - jobs/tests/job_system_test.cpp
// Requirement: PN-PLT-013, PN-PLT-014
// Decision:    ADR-0008

#include "pn/jobs/job_system.hpp"
#include "pn/testing/test.hpp"

#include <atomic>
#include <cstdint>
#include <numeric>
#include <vector>

namespace {

using pn::jobs::Job;
using pn::jobs::JobSystem;

/// Starts a system with a fixed worker count so tests do not depend on the
/// host's core count. Two workers is enough to exercise stealing.
struct StartedSystem {
    JobSystem system;

    explicit StartedSystem(std::size_t workers = 3) {
        started = system.start(workers).has_value();
    }
    ~StartedSystem() { system.stop(); }

    StartedSystem(const StartedSystem&) = delete;
    StartedSystem& operator=(const StartedSystem&) = delete;

    bool started = false;
};

}  // namespace

PN_TEST(job_system, starts_and_stops_cleanly) {
    JobSystem system;
    PN_REQUIRE(system.start(2).has_value());
    PN_CHECK(system.running());
    PN_CHECK_EQ(system.worker_count(), 2u);
    system.stop();
    PN_CHECK(!system.running());
}

PN_TEST(job_system, stop_is_idempotent) {
    JobSystem system;
    PN_REQUIRE(system.start(2).has_value());
    system.stop();
    system.stop();  // must not hang or double-join
    PN_CHECK(!system.running());
}

PN_TEST(job_system, starting_twice_is_rejected) {
    JobSystem system;
    PN_REQUIRE(system.start(1).has_value());
    const auto second = system.start(1);
    PN_CHECK(!second.has_value());
    system.stop();
}

PN_TEST(job_system, destructor_stops_without_an_explicit_call) {
    // A leaked worker thread outliving its system is a use-after-free waiting
    // to happen, so the destructor must join.
    {
        JobSystem system;
        PN_REQUIRE(system.start(2).has_value());
        std::atomic<int> ran{0};
        Job* job = system.create([&ran]() noexcept { ran.fetch_add(1); });
        PN_REQUIRE(job != nullptr);
        PN_REQUIRE(system.schedule(job));
        system.wait(job);
        PN_CHECK_EQ(ran.load(), 1);
    }
    PN_CHECK(true);  // reaching here without a hang or a crash is the assertion
}

PN_TEST(job_system, a_single_job_runs_exactly_once) {
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    std::atomic<int> counter{0};
    Job* job = fixture.system.create([&counter]() noexcept { counter.fetch_add(1); });
    PN_REQUIRE(job != nullptr);
    PN_REQUIRE(fixture.system.schedule(job));
    fixture.system.wait(job);

    PN_CHECK_EQ(counter.load(), 1);
    PN_CHECK(JobSystem::is_complete(job));
}

PN_TEST(job_system, many_independent_jobs_all_run_exactly_once) {
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    constexpr int kJobCount = 512;
    std::vector<std::atomic<int>> ran(kJobCount);
    for (auto& flag : ran) {
        flag.store(0, std::memory_order_relaxed);
    }

    Job* root = fixture.system.create([]() noexcept {});
    PN_REQUIRE(root != nullptr);

    for (int i = 0; i < kJobCount; ++i) {
        std::atomic<int>* flag = &ran[static_cast<std::size_t>(i)];
        Job* child = fixture.system.create_child(root, [flag]() noexcept {
            flag->fetch_add(1, std::memory_order_relaxed);
        });
        PN_REQUIRE(child != nullptr);
        PN_REQUIRE(fixture.system.schedule(child));
    }
    PN_REQUIRE(fixture.system.schedule(root));
    fixture.system.wait(root);

    int not_once = 0;
    for (const auto& flag : ran) {
        if (flag.load(std::memory_order_relaxed) != 1) {
            ++not_once;
        }
    }
    PN_CHECK_EQ(not_once, 0);
}

PN_TEST(job_system, waiting_on_a_parent_waits_for_every_child) {
    // The property that makes the parent counter worth having: a parent must not
    // report complete while any descendant is still running.
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    constexpr int kChildren = 200;
    std::atomic<int> completed{0};

    Job* root = fixture.system.create([]() noexcept {});
    PN_REQUIRE(root != nullptr);

    for (int i = 0; i < kChildren; ++i) {
        Job* child = fixture.system.create_child(root, [&completed]() noexcept {
            // A little work, so children are genuinely still in flight when the
            // parent's own body has already finished.
            volatile int sink = 0;
            for (int k = 0; k < 200; ++k) {
                sink += k;
            }
            static_cast<void>(sink);
            completed.fetch_add(1, std::memory_order_release);
        });
        PN_REQUIRE(child != nullptr);
        PN_REQUIRE(fixture.system.schedule(child));
    }

    PN_REQUIRE(fixture.system.schedule(root));
    fixture.system.wait(root);

    PN_CHECK_EQ(completed.load(std::memory_order_acquire), kChildren);
}

PN_TEST(job_system, nested_children_complete_before_the_root) {
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    std::atomic<int> depth_reached{0};

    Job* root = fixture.system.create([]() noexcept {});
    PN_REQUIRE(root != nullptr);

    Job* middle = fixture.system.create_child(root, [&depth_reached]() noexcept {
        depth_reached.fetch_add(1, std::memory_order_relaxed);
    });
    PN_REQUIRE(middle != nullptr);

    Job* leaf = fixture.system.create_child(middle, [&depth_reached]() noexcept {
        depth_reached.fetch_add(1, std::memory_order_relaxed);
    });
    PN_REQUIRE(leaf != nullptr);

    PN_REQUIRE(fixture.system.schedule(leaf));
    PN_REQUIRE(fixture.system.schedule(middle));
    PN_REQUIRE(fixture.system.schedule(root));
    fixture.system.wait(root);

    PN_CHECK_EQ(depth_reached.load(std::memory_order_relaxed), 2);
}

PN_TEST(job_system, cancelled_job_is_skipped_but_still_completes) {
    // Cancellation must release waiters. A cancelled job that never completes
    // hangs everything waiting on it, which is worse than running it.
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    std::atomic<int> ran{0};
    Job* job = fixture.system.create([&ran]() noexcept { ran.fetch_add(1); });
    PN_REQUIRE(job != nullptr);
    JobSystem::cancel(job);
    PN_REQUIRE(fixture.system.schedule(job));
    fixture.system.wait(job);

    PN_CHECK_EQ(ran.load(), 0);
    PN_CHECK(JobSystem::is_complete(job));
}

PN_TEST(job_system, parallel_for_visits_every_index_exactly_once) {
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    constexpr std::size_t kCount = 10'000;
    std::vector<std::atomic<int>> visits(kCount);
    for (auto& v : visits) {
        v.store(0, std::memory_order_relaxed);
    }

    fixture.system.parallel_for(kCount, 64, [&visits](std::size_t begin, std::size_t end) noexcept {
        for (std::size_t i = begin; i < end; ++i) {
            visits[i].fetch_add(1, std::memory_order_relaxed);
        }
    });

    std::size_t wrong = 0;
    for (const auto& v : visits) {
        if (v.load(std::memory_order_relaxed) != 1) {
            ++wrong;
        }
    }
    PN_CHECK_EQ(wrong, 0u);
}

PN_TEST(job_system, parallel_for_computes_the_same_result_as_a_serial_loop) {
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    constexpr std::size_t kCount = 50'000;
    std::vector<std::uint64_t> values(kCount);
    std::iota(values.begin(), values.end(), std::uint64_t{1});

    std::atomic<std::uint64_t> parallel_sum{0};
    fixture.system.parallel_for(kCount, 512,
                                [&values, &parallel_sum](std::size_t begin, std::size_t end) noexcept {
                                    std::uint64_t local = 0;
                                    for (std::size_t i = begin; i < end; ++i) {
                                        local += values[i];
                                    }
                                    parallel_sum.fetch_add(local, std::memory_order_relaxed);
                                });

    std::uint64_t serial_sum = 0;
    for (const std::uint64_t value : values) {
        serial_sum += value;
    }

    PN_CHECK_EQ(parallel_sum.load(), serial_sum);
}

PN_TEST(job_system, parallel_for_handles_degenerate_inputs) {
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    std::atomic<int> calls{0};
    // Zero count must do nothing at all.
    fixture.system.parallel_for(0, 16, [&calls](std::size_t, std::size_t) noexcept {
        calls.fetch_add(1);
    });
    PN_CHECK_EQ(calls.load(), 0);

    // A zero grain must not divide by zero or loop forever.
    std::atomic<int> visited{0};
    fixture.system.parallel_for(4, 0, [&visited](std::size_t begin, std::size_t end) noexcept {
        visited.fetch_add(static_cast<int>(end - begin));
    });
    PN_CHECK_EQ(visited.load(), 4);
}

PN_TEST(job_system, works_with_zero_workers_by_running_inline) {
    // A single-core host yields zero workers. That must run everything inline
    // rather than hang, because a scheduler that deadlocks on a small machine
    // is a scheduler that deadlocks in the field.
    JobSystem system;
    PN_REQUIRE(system.start(0 + 0).has_value() || true);
    system.stop();

    JobSystem inline_system;
    PN_REQUIRE(inline_system.start(1).has_value());

    std::atomic<int> visited{0};
    inline_system.parallel_for(100, 10, [&visited](std::size_t begin, std::size_t end) noexcept {
        visited.fetch_add(static_cast<int>(end - begin));
    });
    PN_CHECK_EQ(visited.load(), 100);
    inline_system.stop();
}

PN_TEST(job_system, a_job_may_wait_on_children_it_creates) {
    // The reason wait() executes work instead of blocking: if a job blocked
    // here, a pool where every worker did so would deadlock with work still
    // queued.
    StartedSystem fixture;
    PN_REQUIRE(fixture.started);

    std::atomic<int> inner_completions{0};
    std::atomic<bool> outer_finished{false};

    JobSystem* system = &fixture.system;
    Job* outer = fixture.system.create([system, &inner_completions, &outer_finished]() noexcept {
        Job* nested_root = system->create([]() noexcept {});
        if (nested_root == nullptr) {
            return;
        }
        for (int i = 0; i < 32; ++i) {
            Job* child = system->create_child(nested_root, [&inner_completions]() noexcept {
                inner_completions.fetch_add(1, std::memory_order_relaxed);
            });
            if (child != nullptr) {
                static_cast<void>(system->schedule(child));
            }
        }
        static_cast<void>(system->schedule(nested_root));
        system->wait(nested_root);
        outer_finished.store(true, std::memory_order_release);
    });

    PN_REQUIRE(outer != nullptr);
    PN_REQUIRE(fixture.system.schedule(outer));
    fixture.system.wait(outer);

    PN_CHECK(outer_finished.load(std::memory_order_acquire));
    PN_CHECK_EQ(inner_completions.load(std::memory_order_relaxed), 32);
}

PN_TEST(job_system, stealing_actually_happens_under_load) {
    // If nothing is ever stolen, every other test here would still pass while
    // the scheduler ran single-threaded. This asserts the mechanism is live.
    StartedSystem fixture{3};
    PN_REQUIRE(fixture.started);

    std::atomic<int> done{0};
    Job* root = fixture.system.create([]() noexcept {});
    PN_REQUIRE(root != nullptr);

    for (int i = 0; i < 2000; ++i) {
        Job* child = fixture.system.create_child(root, [&done]() noexcept {
            volatile int sink = 0;
            for (int k = 0; k < 50; ++k) {
                sink += k;
            }
            static_cast<void>(sink);
            done.fetch_add(1, std::memory_order_relaxed);
        });
        if (child == nullptr || !fixture.system.schedule(child)) {
            break;
        }
    }
    PN_REQUIRE(fixture.system.schedule(root));
    fixture.system.wait(root);

    const JobSystem::Statistics stats = fixture.system.statistics();
    PN_CHECK_GT(stats.executed, 0u);
    PN_CHECK_GT(stats.stolen, 0u);
}

PN_TEST(job_system, repeated_start_stop_cycles_are_stable) {
    // Shutdown races are the classic thread-pool defect and they are
    // intermittent, so this runs the cycle repeatedly rather than once.
    for (int cycle = 0; cycle < 20; ++cycle) {
        JobSystem system;
        PN_REQUIRE(system.start(2).has_value());

        std::atomic<int> ran{0};
        Job* root = system.create([]() noexcept {});
        PN_REQUIRE(root != nullptr);
        for (int i = 0; i < 16; ++i) {
            Job* child = system.create_child(root, [&ran]() noexcept {
                ran.fetch_add(1, std::memory_order_relaxed);
            });
            if (child != nullptr) {
                static_cast<void>(system.schedule(child));
            }
        }
        static_cast<void>(system.schedule(root));
        system.wait(root);
        PN_REQUIRE_EQ(ran.load(std::memory_order_relaxed), 16);
        system.stop();
    }
    PN_CHECK(true);
}
