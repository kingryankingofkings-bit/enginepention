// Pention Engine - jobs/tests/deque_test.cpp
// Requirement: PN-PLT-013
// Decision:    ADR-0008
//
// The concurrent tests here assert one invariant above all others: every item
// pushed is taken exactly once. Not "roughly once" and not "at least once" - a
// duplicate means two workers run the same job, and a loss means a job silently
// never runs and whatever waits on it hangs. Both are catastrophic and neither
// is visible in a debugger.

#include "pn/jobs/work_stealing_deque.hpp"
#include "pn/testing/test.hpp"

#include <atomic>
#include <cstdint>
#include <thread>
#include <vector>

namespace {

using pn::jobs::WorkStealingDeque;

struct Item {
    int value = 0;
};

}  // namespace

PN_TEST(deque, capacity_rounds_up_to_a_power_of_two) {
    PN_CHECK_EQ(WorkStealingDeque<Item>{1}.capacity(), 1u);
    PN_CHECK_EQ(WorkStealingDeque<Item>{5}.capacity(), 8u);
    PN_CHECK_EQ(WorkStealingDeque<Item>{64}.capacity(), 64u);
    PN_CHECK_EQ(WorkStealingDeque<Item>{100}.capacity(), 128u);
}

PN_TEST(deque, empty_deque_yields_nothing) {
    WorkStealingDeque<Item> deque{16};
    PN_CHECK(deque.pop_bottom() == nullptr);
    PN_CHECK(deque.steal_top() == nullptr);
    PN_CHECK(deque.empty_approx());
}

PN_TEST(deque, owner_pops_in_lifo_order) {
    // The owner takes from the bottom, so its own work is LIFO - which keeps
    // recently pushed, cache-warm work on the thread that produced it.
    WorkStealingDeque<Item> deque{16};
    std::vector<Item> items(4);
    for (int i = 0; i < 4; ++i) {
        items[static_cast<std::size_t>(i)].value = i;
        PN_REQUIRE(deque.push_bottom(&items[static_cast<std::size_t>(i)]));
    }

    for (int expected = 3; expected >= 0; --expected) {
        Item* item = deque.pop_bottom();
        PN_REQUIRE(item != nullptr);
        PN_CHECK_EQ(item->value, expected);
    }
    PN_CHECK(deque.pop_bottom() == nullptr);
}

PN_TEST(deque, thieves_take_in_fifo_order) {
    // Thieves take from the top, so they get the oldest work - which is the
    // least likely to be cache-warm for the owner and the most likely to be a
    // large subtree worth stealing.
    WorkStealingDeque<Item> deque{16};
    std::vector<Item> items(4);
    for (int i = 0; i < 4; ++i) {
        items[static_cast<std::size_t>(i)].value = i;
        PN_REQUIRE(deque.push_bottom(&items[static_cast<std::size_t>(i)]));
    }

    for (int expected = 0; expected < 4; ++expected) {
        Item* item = deque.steal_top();
        PN_REQUIRE(item != nullptr);
        PN_CHECK_EQ(item->value, expected);
    }
    PN_CHECK(deque.steal_top() == nullptr);
}

PN_TEST(deque, push_fails_when_full_rather_than_overwriting) {
    WorkStealingDeque<Item> deque{8};
    std::vector<Item> items(16);
    std::size_t pushed = 0;
    for (std::size_t i = 0; i < items.size(); ++i) {
        if (deque.push_bottom(&items[i])) {
            ++pushed;
        }
    }
    PN_CHECK_EQ(pushed, deque.capacity());

    // Everything pushed is still retrievable - a full deque must refuse, never
    // silently drop an already-accepted job.
    std::size_t drained = 0;
    while (deque.pop_bottom() != nullptr) {
        ++drained;
    }
    PN_CHECK_EQ(drained, pushed);
}

PN_TEST(deque, single_item_is_taken_exactly_once_by_owner_or_thief) {
    // The contended case that the seq_cst fences exist for. Repeated many times
    // because the race window is narrow.
    constexpr int kRounds = 2000;
    int owner_wins = 0;
    int thief_wins = 0;

    for (int round = 0; round < kRounds; ++round) {
        WorkStealingDeque<Item> deque{4};
        Item item{round};
        PN_REQUIRE(deque.push_bottom(&item));

        // Both threads rendezvous immediately before the contended operation.
        // Without this the owner reliably wins, because thread creation costs
        // far more than the operation itself - and under a sanitizer it costs
        // enough that the race never happens at all, leaving the test asserting
        // nothing while still passing.
        std::atomic<int> ready{0};
        std::atomic<Item*> stolen{nullptr};

        std::thread thief{[&deque, &stolen, &ready] {
            ready.fetch_add(1, std::memory_order_acq_rel);
            while (ready.load(std::memory_order_acquire) < 2) {
                // spin: the window being tested is a few instructions wide
            }
            stolen.store(deque.steal_top(), std::memory_order_release);
        }};

        ready.fetch_add(1, std::memory_order_acq_rel);
        while (ready.load(std::memory_order_acquire) < 2) {
        }

        Item* popped = deque.pop_bottom();
        thief.join();

        Item* taken_by_thief = stolen.load(std::memory_order_acquire);

        // Exactly one of them got it. Both is a duplicate execution; neither is
        // a lost job.
        const int takers = (popped != nullptr ? 1 : 0) + (taken_by_thief != nullptr ? 1 : 0);
        PN_REQUIRE_EQ(takers, 1);

        if (popped != nullptr) {
            ++owner_wins;
        } else {
            ++thief_wins;
        }
    }

    // Exactly one taker per round is the correctness claim, and it is asserted
    // per round above. Every round is accounted for.
    PN_CHECK_EQ(owner_wins + thief_wins, kRounds);

    // NOTE: which side wins is deliberately NOT asserted.
    //
    // An earlier version required both outcomes to occur at least once, on the
    // reasoning that a test which never contends proves nothing. That reasoning
    // is sound but the assertion was not: which thread wins a few-instruction
    // race is a property of the scheduler and the core count, not of the deque.
    // It failed on CI in BOTH directions - one configuration saw the thief win
    // all 2000 rounds, another saw the owner win all 2000 - while the
    // exactly-one-taker invariant held 2000 of 2000 in each. A real ordering bug
    // cannot produce "always exactly one winner, but always the same one".
    //
    // The concern it was trying to address is now met deterministically, by
    // last_item_taken_by_the_owner and last_item_taken_by_a_thief below, which
    // drive the contended branch down each side without depending on timing.
}

PN_TEST(deque, last_item_taken_by_the_owner) {
    // Drives the same branch the concurrent race resolves - in pop_bottom,
    // top == bottom, so the owner must settle the claim with a
    // compare-exchange - but deterministically, with no second thread and no
    // dependence on scheduling.
    WorkStealingDeque<Item> deque{4};
    Item item{42};
    PN_REQUIRE(deque.push_bottom(&item));

    Item* taken = deque.pop_bottom();
    PN_REQUIRE(taken != nullptr);
    PN_CHECK_EQ(taken->value, 42);

    // Having lost the item, nobody else may take it.
    PN_CHECK(deque.steal_top() == nullptr);
    PN_CHECK(deque.pop_bottom() == nullptr);

    // And the deque is reusable afterwards - the CAS must leave the indices
    // consistent, not merely empty.
    Item next{7};
    PN_REQUIRE(deque.push_bottom(&next));
    Item* again = deque.pop_bottom();
    PN_REQUIRE(again != nullptr);
    PN_CHECK_EQ(again->value, 7);
}

PN_TEST(deque, last_item_taken_by_a_thief) {
    // The mirror image: a thief takes the only item, and the owner must then
    // find nothing rather than handing out the same item twice.
    WorkStealingDeque<Item> deque{4};
    Item item{99};
    PN_REQUIRE(deque.push_bottom(&item));

    Item* stolen = deque.steal_top();
    PN_REQUIRE(stolen != nullptr);
    PN_CHECK_EQ(stolen->value, 99);

    PN_CHECK(deque.pop_bottom() == nullptr);
    PN_CHECK(deque.steal_top() == nullptr);

    Item next{13};
    PN_REQUIRE(deque.push_bottom(&next));
    Item* again = deque.steal_top();
    PN_REQUIRE(again != nullptr);
    PN_CHECK_EQ(again->value, 13);
}

PN_TEST(deque, concurrent_drain_takes_every_item_exactly_once) {
    constexpr std::size_t kItemCount = 4096;
    constexpr int kThiefCount = 3;

    WorkStealingDeque<Item> deque{kItemCount};
    std::vector<Item> items(kItemCount);
    for (std::size_t i = 0; i < kItemCount; ++i) {
        items[i].value = static_cast<int>(i);
    }

    // Counts how many times each item was taken. Any entry other than 1 at the
    // end is a correctness failure.
    std::vector<std::atomic<int>> taken(kItemCount);
    for (auto& counter : taken) {
        counter.store(0, std::memory_order_relaxed);
    }

    for (std::size_t i = 0; i < kItemCount; ++i) {
        PN_REQUIRE(deque.push_bottom(&items[i]));
    }

    std::atomic<bool> go{false};
    std::atomic<std::size_t> total_taken{0};

    const auto record = [&taken, &total_taken](Item* item) {
        taken[static_cast<std::size_t>(item->value)].fetch_add(1, std::memory_order_relaxed);
        total_taken.fetch_add(1, std::memory_order_relaxed);
    };

    std::vector<std::thread> thieves;
    thieves.reserve(kThiefCount);
    for (int t = 0; t < kThiefCount; ++t) {
        thieves.emplace_back([&] {
            while (!go.load(std::memory_order_acquire)) {
            }
            // Keep trying until the owner has finished and the deque is drained;
            // a null result may mean "lost the race", not "empty".
            int idle_rounds = 0;
            while (idle_rounds < 1000) {
                if (Item* item = deque.steal_top(); item != nullptr) {
                    record(item);
                    idle_rounds = 0;
                } else {
                    ++idle_rounds;
                    std::this_thread::yield();
                }
            }
        });
    }

    go.store(true, std::memory_order_release);
    int idle_rounds = 0;
    while (idle_rounds < 1000) {
        if (Item* item = deque.pop_bottom(); item != nullptr) {
            record(item);
            idle_rounds = 0;
        } else {
            ++idle_rounds;
            std::this_thread::yield();
        }
    }

    for (std::thread& thief : thieves) {
        thief.join();
    }

    PN_CHECK_EQ(total_taken.load(), kItemCount);

    std::size_t duplicates = 0;
    std::size_t missing = 0;
    for (std::size_t i = 0; i < kItemCount; ++i) {
        const int count = taken[i].load(std::memory_order_relaxed);
        if (count > 1) {
            ++duplicates;
        } else if (count == 0) {
            ++missing;
        }
    }
    PN_CHECK_EQ(duplicates, 0u);
    PN_CHECK_EQ(missing, 0u);
}

PN_TEST(deque, owner_can_push_while_thieves_steal) {
    // Push and steal overlap in time, which is the normal steady state of a
    // scheduler under load, not just the drain case above.
    constexpr std::size_t kItemCount = 8192;
    WorkStealingDeque<Item> deque{1024};

    std::vector<Item> items(kItemCount);
    for (std::size_t i = 0; i < kItemCount; ++i) {
        items[i].value = static_cast<int>(i);
    }

    std::vector<std::atomic<int>> taken(kItemCount);
    for (auto& counter : taken) {
        counter.store(0, std::memory_order_relaxed);
    }

    std::atomic<bool> producing{true};
    std::atomic<std::size_t> total_taken{0};

    std::vector<std::thread> thieves;
    for (int t = 0; t < 2; ++t) {
        thieves.emplace_back([&] {
            while (producing.load(std::memory_order_acquire) ||
                   !deque.empty_approx()) {
                if (Item* item = deque.steal_top(); item != nullptr) {
                    taken[static_cast<std::size_t>(item->value)]
                        .fetch_add(1, std::memory_order_relaxed);
                    total_taken.fetch_add(1, std::memory_order_relaxed);
                } else {
                    std::this_thread::yield();
                }
            }
        });
    }

    std::size_t pushed = 0;
    while (pushed < kItemCount) {
        if (deque.push_bottom(&items[pushed])) {
            ++pushed;
        } else if (Item* item = deque.pop_bottom(); item != nullptr) {
            // Full: consume one of our own to make room.
            taken[static_cast<std::size_t>(item->value)].fetch_add(1, std::memory_order_relaxed);
            total_taken.fetch_add(1, std::memory_order_relaxed);
        }
    }
    producing.store(false, std::memory_order_release);

    // Drain whatever is left.
    while (Item* item = deque.pop_bottom()) {
        taken[static_cast<std::size_t>(item->value)].fetch_add(1, std::memory_order_relaxed);
        total_taken.fetch_add(1, std::memory_order_relaxed);
    }

    for (std::thread& thief : thieves) {
        thief.join();
    }

    std::size_t duplicates = 0;
    std::size_t missing = 0;
    for (std::size_t i = 0; i < kItemCount; ++i) {
        const int count = taken[i].load(std::memory_order_relaxed);
        if (count > 1) {
            ++duplicates;
        } else if (count == 0) {
            ++missing;
        }
    }
    PN_CHECK_EQ(duplicates, 0u);
    PN_CHECK_EQ(missing, 0u);
    PN_CHECK_EQ(total_taken.load(), kItemCount);
}
