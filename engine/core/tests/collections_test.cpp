// Pention Engine - core/tests/collections_test.cpp
// Requirement: PN-PLT-009
// Decision:    ADR-0004
//
// The sparse set, ring buffer and bit set. Same shape as containers_test.cpp:
// hand-written cases for the properties worth naming, and property tests
// against standard-library oracles for the states nobody thought to write.

#include "pn/core/bit_set.hpp"
#include "pn/core/random.hpp"
#include "pn/core/ring_buffer.hpp"
#include "pn/core/sparse_set.hpp"
#include "pn/testing/test.hpp"

#include <cstdint>
#include <deque>
#include <map>
#include <string>
#include <vector>

namespace {

using pn::core::BitSet;
using pn::core::Random;
using pn::core::RingBuffer;
using pn::core::SparseSet;

}  // namespace

// -------------------------------------------------------------------
// SparseSet
// -------------------------------------------------------------------

PN_TEST(sparse_set, stores_and_finds_by_key) {
    SparseSet<int> set;
    PN_CHECK(set.empty());

    PN_CHECK(set.insert(10, 100));
    PN_CHECK(set.insert(3, 30));
    PN_CHECK(!set.insert(10, 999));

    PN_CHECK_EQ(set.size(), std::size_t{2});
    const int* found = set.find(10);
    PN_REQUIRE(found != nullptr);
    PN_CHECK_EQ(*found, 100);
    PN_CHECK(set.find(7) == nullptr);
    PN_CHECK(set.contains(3));
}

PN_TEST(sparse_set, values_are_contiguous_which_is_the_whole_point) {
    // A system walking every component must walk memory forwards. If the values
    // were not adjacent, this container would have no reason to exist over a
    // hash map.
    SparseSet<int> set;
    for (std::uint32_t key = 0; key < 8; ++key) {
        set.insert(key * 7, static_cast<int>(key));
    }

    const int* base = set.values();
    for (std::size_t index = 0; index < set.size(); ++index) {
        PN_REQUIRE_EQ(&set[index], base + index);
    }

    int sum = 0;
    for (int value : set) {
        sum += value;
    }
    PN_CHECK_EQ(sum, 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7);
}

PN_TEST(sparse_set, removal_moves_the_last_element_into_the_hole) {
    // And the sparse index must be repaired for the element that moved.
    // Forgetting that is the classic sparse-set bug: the moved element becomes
    // findable at the wrong slot, or not at all.
    SparseSet<std::string> set;
    set.insert(1, "one");
    set.insert(2, "two");
    set.insert(3, "three");

    PN_CHECK(set.remove(1));
    PN_CHECK_EQ(set.size(), std::size_t{2});
    PN_CHECK(set.find(1) == nullptr);

    const std::string* moved = set.find(3);
    PN_REQUIRE(moved != nullptr);
    PN_CHECK(*moved == "three");
    const std::string* untouched = set.find(2);
    PN_REQUIRE(untouched != nullptr);
    PN_CHECK(*untouched == "two");
}

PN_TEST(sparse_set, removing_the_last_element_needs_no_move) {
    SparseSet<int> set;
    set.insert(5, 50);
    set.insert(6, 60);
    PN_CHECK(set.remove(6));
    PN_CHECK_EQ(set.size(), std::size_t{1});
    const int* remaining = set.find(5);
    PN_REQUIRE(remaining != nullptr);
    PN_CHECK_EQ(*remaining, 50);
}

PN_TEST(sparse_set, keys_stay_aligned_with_values_after_removals) {
    SparseSet<int> set;
    for (std::uint32_t key = 0; key < 20; ++key) {
        set.insert(key, static_cast<int>(key) * 10);
    }
    for (std::uint32_t key = 0; key < 20; key += 3) {
        PN_REQUIRE(set.remove(key));
    }

    for (std::size_t index = 0; index < set.size(); ++index) {
        const std::uint32_t key = set.key_at(index);
        PN_REQUIRE_EQ(set[index], static_cast<int>(key) * 10);
    }
}

PN_TEST(sparse_set, the_memory_cost_of_a_sparse_key_space_is_reported) {
    // The trade this container makes, made visible rather than discovered in a
    // memory report six months later.
    SparseSet<int> dense;
    for (std::uint32_t key = 0; key < 100; ++key) {
        dense.insert(key, 0);
    }

    SparseSet<int> scattered;
    scattered.insert(1000000, 0);

    PN_CHECK(dense.sparse_bytes() < 1000);
    PN_CHECK(scattered.sparse_bytes() > 1000000);
    PN_CHECK_EQ(scattered.size(), std::size_t{1});
}

PN_TEST(sparse_set, clearing_leaves_no_key_findable) {
    SparseSet<int> set;
    for (std::uint32_t key = 0; key < 50; ++key) {
        set.insert(key, static_cast<int>(key));
    }
    set.clear();
    PN_CHECK(set.empty());
    for (std::uint32_t key = 0; key < 50; ++key) {
        PN_REQUIRE(!set.contains(key));
    }
    // And it must still be usable, not merely empty.
    PN_CHECK(set.insert(7, 7));
    PN_CHECK(set.contains(7));
}

PN_TEST(sparse_set, agrees_with_std_map_over_a_random_operation_sequence) {
    constexpr int kOperations = 200000;
    constexpr std::uint64_t kKeySpace = 2000;

    SparseSet<std::uint64_t> subject;
    std::map<std::uint32_t, std::uint64_t> oracle;
    Random random{31415, 2};

    for (int operation = 0; operation < kOperations; ++operation) {
        const std::uint32_t key = static_cast<std::uint32_t>(random.uniform(0, kKeySpace - 1));
        const std::uint64_t choice = random.uniform(0, 99);

        if (choice < 50) {
            const std::uint64_t value = random.next_u64();
            const bool inserted = subject.insert(key, value);
            const bool expected = oracle.find(key) == oracle.end();
            PN_REQUIRE_EQ(inserted, expected);
            if (expected) {
                oracle.emplace(key, value);
            }
        } else if (choice < 75) {
            const bool removed = subject.remove(key);
            const bool expected = oracle.erase(key) != 0;
            PN_REQUIRE_EQ(removed, expected);
        } else {
            const std::uint64_t* found = subject.find(key);
            const auto expected = oracle.find(key);
            if (expected == oracle.end()) {
                PN_REQUIRE(found == nullptr);
            } else {
                PN_REQUIRE(found != nullptr);
                PN_REQUIRE_EQ(*found, expected->second);
            }
        }
        PN_REQUIRE_EQ(subject.size(), oracle.size());
    }

    // Every dense slot must still agree with the oracle, which catches an
    // element that survived with a stale sparse index.
    for (std::size_t index = 0; index < subject.size(); ++index) {
        const auto expected = oracle.find(subject.key_at(index));
        PN_REQUIRE(expected != oracle.end());
        PN_REQUIRE_EQ(subject[index], expected->second);
    }
}

// -------------------------------------------------------------------
// RingBuffer
// -------------------------------------------------------------------

PN_TEST(ring_buffer, pushes_and_pops_at_both_ends) {
    RingBuffer<int, 4> ring;
    PN_CHECK(ring.empty());

    ring.push_back(1);
    ring.push_back(2);
    ring.emplace_front(0);
    PN_CHECK_EQ(ring.size(), std::size_t{3});
    PN_CHECK_EQ(ring.front(), 0);
    PN_CHECK_EQ(ring.back(), 2);
    PN_CHECK_EQ(ring[1], 1);

    ring.pop_front();
    PN_CHECK_EQ(ring.front(), 1);
    ring.pop_back();
    PN_CHECK_EQ(ring.back(), 1);
    PN_CHECK_EQ(ring.size(), std::size_t{1});
}

PN_TEST(ring_buffer, wraps_without_losing_order) {
    // The failure a ring buffer has: elements that straddle the wrap point come
    // back in the wrong order, or the wrong count.
    RingBuffer<int, 4> ring;
    for (int value = 0; value < 4; ++value) {
        ring.push_back(value);
    }
    // Drain two and refill, so the live range crosses the end of the storage.
    ring.pop_front();
    ring.pop_front();
    ring.push_back(4);
    ring.push_back(5);

    PN_CHECK_EQ(ring.size(), std::size_t{4});
    PN_CHECK_EQ(ring[0], 2);
    PN_CHECK_EQ(ring[1], 3);
    PN_CHECK_EQ(ring[2], 4);
    PN_CHECK_EQ(ring[3], 5);
}

PN_TEST(ring_buffer, try_push_back_reports_the_bound) {
    RingBuffer<int, 2> ring;
    PN_CHECK(ring.try_push_back(1));
    PN_CHECK(ring.try_push_back(2));
    PN_CHECK(!ring.try_push_back(3));
    PN_CHECK_EQ(ring.size(), std::size_t{2});
}

PN_TEST(ring_buffer, overwriting_push_drops_the_oldest) {
    // For a history window, where the newest matters and the oldest is meant to
    // fall off.
    RingBuffer<int, 4> history;
    for (int value = 0; value < 10; ++value) {
        history.push_back_overwriting(value);
    }
    PN_CHECK_EQ(history.size(), std::size_t{4});
    PN_CHECK_EQ(history[0], 6);
    PN_CHECK_EQ(history[3], 9);
}

PN_TEST(ring_buffer, destroys_every_element_exactly_once) {
    // Values that own memory, so a leak or a double-free shows up under the
    // sanitizers rather than only in a counter.
    RingBuffer<std::string, 8> ring;
    for (int value = 0; value < 6; ++value) {
        ring.push_back(std::string(64, static_cast<char>('a' + value)));
    }
    ring.pop_front();
    ring.pop_back();
    PN_CHECK_EQ(ring.size(), std::size_t{4});
    PN_CHECK(ring.front() == std::string(64, 'b'));

    RingBuffer<std::string, 8> moved = std::move(ring);
    PN_CHECK_EQ(moved.size(), std::size_t{4});
    PN_CHECK(moved.back() == std::string(64, 'e'));
}

PN_TEST(ring_buffer, agrees_with_a_deque_over_a_random_operation_sequence) {
    constexpr std::size_t kCapacity = 16;
    RingBuffer<int, kCapacity> subject;
    std::deque<int> oracle;
    Random random{2718, 4};

    for (int operation = 0; operation < 200000; ++operation) {
        const std::uint64_t choice = random.uniform(0, 99);
        const int value = static_cast<int>(random.uniform(0, 10000));

        if (choice < 35 && oracle.size() < kCapacity) {
            subject.push_back(value);
            oracle.push_back(value);
        } else if (choice < 60 && oracle.size() < kCapacity) {
            subject.emplace_front(value);
            oracle.push_front(value);
        } else if (choice < 80 && !oracle.empty()) {
            subject.pop_front();
            oracle.pop_front();
        } else if (!oracle.empty()) {
            subject.pop_back();
            oracle.pop_back();
        }

        PN_REQUIRE_EQ(subject.size(), oracle.size());
        if (!oracle.empty()) {
            PN_REQUIRE_EQ(subject.front(), oracle.front());
            PN_REQUIRE_EQ(subject.back(), oracle.back());
        }
    }

    for (std::size_t index = 0; index < oracle.size(); ++index) {
        PN_REQUIRE_EQ(subject[index], oracle[index]);
    }
}

// -------------------------------------------------------------------
// BitSet
// -------------------------------------------------------------------

PN_TEST(bit_set, sets_clears_and_tests) {
    BitSet bits{200};
    PN_CHECK_EQ(bits.size(), std::size_t{200});
    PN_CHECK(bits.none());

    bits.set(0);
    bits.set(63);
    bits.set(64);
    bits.set(199);
    PN_CHECK_EQ(bits.count(), std::size_t{4});
    PN_CHECK(bits.test(63));
    PN_CHECK(bits.test(64));
    PN_CHECK(!bits.test(65));
    PN_CHECK(bits.any());

    bits.clear(63);
    PN_CHECK(!bits.test(63));
    PN_CHECK_EQ(bits.count(), std::size_t{3});
}

PN_TEST(bit_set, a_size_that_is_not_a_multiple_of_the_word_counts_correctly) {
    // The bug most test sizes hide. count(), any() and find_next() read whole
    // words, so bits past the end must be cleared - or a set of 65 bits reports
    // 128 after set_all().
    BitSet bits{65};
    bits.set_all();
    PN_CHECK_EQ(bits.count(), std::size_t{65});
    PN_CHECK(bits.all());

    BitSet awkward{1};
    awkward.set_all();
    PN_CHECK_EQ(awkward.count(), std::size_t{1});

    BitSet exact{128};
    exact.set_all();
    PN_CHECK_EQ(exact.count(), std::size_t{128});
}

PN_TEST(bit_set, growing_leaves_the_new_bits_clear) {
    BitSet bits{10};
    bits.set_all();
    bits.resize(100);
    PN_CHECK_EQ(bits.count(), std::size_t{10});
    PN_CHECK(!bits.test(50));
}

PN_TEST(bit_set, find_next_walks_only_the_set_bits) {
    BitSet bits{300};
    for (std::size_t index : {5U, 63U, 64U, 128U, 299U}) {
        bits.set(index);
    }

    std::vector<std::size_t> found;
    for (std::size_t index = bits.find_next(0); index < bits.size();
         index = bits.find_next(index + 1)) {
        found.push_back(index);
    }
    PN_REQUIRE_EQ(found.size(), std::size_t{5});
    PN_CHECK_EQ(found[0], std::size_t{5});
    PN_CHECK_EQ(found[1], std::size_t{63});
    PN_CHECK_EQ(found[2], std::size_t{64});
    PN_CHECK_EQ(found[3], std::size_t{128});
    PN_CHECK_EQ(found[4], std::size_t{299});

    // Past the last set bit, and past the end.
    PN_CHECK_EQ(bits.find_next(300), std::size_t{300});
    BitSet empty{64};
    PN_CHECK_EQ(empty.find_next(0), std::size_t{64});
}

PN_TEST(bit_set, for_each_set_visits_exactly_the_set_bits_in_order) {
    // The operation a culling result is read with. It must cost what the answer
    // costs, and it must not skip a bit at a word boundary.
    BitSet bits{1000};
    std::vector<std::size_t> expected;
    for (std::size_t index = 0; index < 1000; index += 37) {
        bits.set(index);
        expected.push_back(index);
    }

    std::vector<std::size_t> visited;
    bits.for_each_set([&visited](std::size_t index) { visited.push_back(index); });
    PN_REQUIRE_EQ(visited.size(), expected.size());
    for (std::size_t index = 0; index < expected.size(); ++index) {
        PN_REQUIRE_EQ(visited[index], expected[index]);
    }
}

PN_TEST(bit_set, set_operations_behave_like_sets) {
    BitSet left{100};
    BitSet right{100};
    for (std::size_t index = 0; index < 100; index += 2) {
        left.set(index);
    }
    for (std::size_t index = 0; index < 100; index += 3) {
        right.set(index);
    }

    BitSet intersection = left;
    intersection.intersect_with(right);
    // Multiples of six.
    PN_CHECK_EQ(intersection.count(), std::size_t{17});
    PN_CHECK(intersection.test(6));
    PN_CHECK(!intersection.test(4));

    BitSet difference = left;
    difference.subtract(right);
    PN_CHECK(!difference.test(6));
    PN_CHECK(difference.test(4));

    BitSet combined = left;
    combined.union_with(right);
    PN_CHECK(combined.test(3));
    PN_CHECK(combined.test(4));
}

PN_TEST(bit_set, agrees_with_a_vector_of_bool_over_a_random_operation_sequence) {
    constexpr std::size_t kBits = 517;  // deliberately not a multiple of 64
    BitSet subject{kBits};
    std::vector<bool> oracle(kBits, false);
    Random random{1618, 5};

    for (int operation = 0; operation < 200000; ++operation) {
        const std::size_t index = random.uniform(0, kBits - 1);
        const std::uint64_t choice = random.uniform(0, 99);

        if (choice < 40) {
            subject.set(index);
            oracle[index] = true;
        } else if (choice < 70) {
            subject.clear(index);
            oracle[index] = false;
        } else if (choice < 85) {
            subject.flip(index);
            oracle[index] = !oracle[index];
        } else {
            PN_REQUIRE_EQ(subject.test(index), static_cast<bool>(oracle[index]));
        }
    }

    std::size_t expected_count = 0;
    for (std::size_t index = 0; index < kBits; ++index) {
        PN_REQUIRE_EQ(subject.test(index), static_cast<bool>(oracle[index]));
        if (oracle[index]) {
            ++expected_count;
        }
    }
    PN_CHECK_EQ(subject.count(), expected_count);

    // And the two iteration paths must agree with the oracle and each other.
    std::vector<std::size_t> by_visit;
    subject.for_each_set([&by_visit](std::size_t index) { by_visit.push_back(index); });
    std::vector<std::size_t> by_scan;
    for (std::size_t index = subject.find_next(0); index < subject.size();
         index = subject.find_next(index + 1)) {
        by_scan.push_back(index);
    }
    PN_REQUIRE_EQ(by_visit.size(), expected_count);
    PN_REQUIRE_EQ(by_scan.size(), expected_count);
    for (std::size_t index = 0; index < by_visit.size(); ++index) {
        PN_REQUIRE_EQ(by_visit[index], by_scan[index]);
        PN_REQUIRE(oracle[by_visit[index]]);
    }
}
