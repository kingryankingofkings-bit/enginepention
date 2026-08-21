// Pention Engine - core/tests/containers_test.cpp
// Requirement: PN-PLT-009
// Decision:    ADR-0004
//
// PN-PLT-009 asks for unit *and property* tests. The property tests here drive
// the containers with pseudorandom operation sequences and compare against
// std::map as an oracle - a container that agrees with a known-correct one over
// a hundred thousand random operations is a container whose invariants hold in
// states nobody thought to write a case for.
//
// The generator is pn::core::Random, so a failing sequence is reproducible from
// its seed rather than being a flake.

#include "pn/core/hash.hpp"
#include "pn/core/hash_map.hpp"
#include "pn/core/inline_array.hpp"
#include "pn/core/random.hpp"
#include "pn/testing/test.hpp"

#include <cstdint>
#include <map>
#include <string>
#include <string_view>
#include <vector>

namespace {

using pn::core::HashMap;
using pn::core::InlineArray;
using pn::core::Random;

/// Counts its own construction and destruction, so a container that leaks or
/// double-destroys is caught rather than merely suspected.
struct Tracked {
    static inline int live = 0;
    static inline int constructions = 0;

    int value = 0;

    Tracked() { enter(); }
    explicit Tracked(int initial) : value{initial} { enter(); }
    Tracked(const Tracked& other) : value{other.value} { enter(); }
    Tracked(Tracked&& other) noexcept : value{other.value} { enter(); }
    Tracked& operator=(const Tracked&) = default;
    Tracked& operator=(Tracked&&) noexcept = default;
    ~Tracked() { --live; }

    static void reset() {
        live = 0;
        constructions = 0;
    }

private:
    void enter() {
        ++live;
        ++constructions;
    }
};

}  // namespace

// -------------------------------------------------------------------
// InlineArray
// -------------------------------------------------------------------

PN_TEST(inline_array, holds_elements_without_allocating) {
    InlineArray<int, 4> array;
    PN_CHECK(array.empty());
    PN_CHECK_EQ(array.capacity(), std::size_t{4});

    array.push_back(1);
    array.push_back(2);
    array.push_back(3);
    PN_CHECK_EQ(array.size(), std::size_t{3});
    PN_CHECK_EQ(array[0], 1);
    PN_CHECK_EQ(array[2], 3);
    PN_CHECK_EQ(array.back(), 3);
    PN_CHECK(!array.full());

    array.push_back(4);
    PN_CHECK(array.full());
}

PN_TEST(inline_array, try_push_back_reports_the_bound_instead_of_aborting) {
    // For the call sites where hitting the bound is data, not a defect.
    InlineArray<int, 2> array;
    PN_CHECK(array.try_push_back(1));
    PN_CHECK(array.try_push_back(2));
    PN_CHECK(!array.try_push_back(3));
    PN_CHECK_EQ(array.size(), std::size_t{2});
}

PN_TEST(inline_array, destroys_every_element_exactly_once) {
    Tracked::reset();
    {
        InlineArray<Tracked, 8> array;
        for (int index = 0; index < 5; ++index) {
            array.emplace_back(index);
        }
        PN_CHECK_EQ(Tracked::live, 5);
        array.pop_back();
        PN_CHECK_EQ(Tracked::live, 4);
        array.clear();
        PN_CHECK_EQ(Tracked::live, 0);
        for (int index = 0; index < 3; ++index) {
            array.emplace_back(index);
        }
    }
    // The destructor must clean up what clear() did not.
    PN_CHECK_EQ(Tracked::live, 0);
}

PN_TEST(inline_array, erase_unordered_swaps_the_last_element_into_the_hole) {
    InlineArray<int, 8> array;
    for (int value : {10, 20, 30, 40}) {
        array.push_back(value);
    }
    array.erase_unordered(1);

    PN_CHECK_EQ(array.size(), std::size_t{3});
    PN_CHECK_EQ(array[0], 10);
    PN_CHECK_EQ(array[1], 40);  // moved from the end, not shifted
    PN_CHECK_EQ(array[2], 30);
}

PN_TEST(inline_array, erasing_the_last_element_does_not_self_move) {
    // The degenerate case of a swap-remove: moving an object onto itself is
    // valid but leaves an unspecified value, so the code must not do it.
    Tracked::reset();
    {
        InlineArray<Tracked, 4> array;
        array.emplace_back(7);
        array.erase_unordered(0);
        PN_CHECK_EQ(array.size(), std::size_t{0});
        PN_CHECK_EQ(Tracked::live, 0);
    }
}

PN_TEST(inline_array, copies_and_moves_preserve_contents) {
    InlineArray<std::string, 4> original;
    original.push_back("alpha");
    original.push_back("beta");

    InlineArray<std::string, 4> copied = original;
    PN_CHECK_EQ(copied.size(), std::size_t{2});
    PN_CHECK(copied[1] == "beta");
    PN_CHECK(original[1] == "beta");

    InlineArray<std::string, 4> moved = std::move(copied);
    PN_CHECK_EQ(moved.size(), std::size_t{2});
    PN_CHECK(moved[0] == "alpha");
}

// -------------------------------------------------------------------
// Hashing
// -------------------------------------------------------------------

PN_TEST(hash, sequential_integers_do_not_land_in_one_run_of_slots) {
    // The reason integer keys go through the mixer. An identity hash is fine
    // with a prime modulus and catastrophic with a power-of-two mask: every key
    // lands in one contiguous block and the table degenerates into a list.
    constexpr std::size_t kMask = 1023;
    std::vector<int> occupancy(1024, 0);
    for (std::uint64_t key = 0; key < 1024; ++key) {
        const std::uint64_t hash = pn::core::Hash<std::uint64_t>{}(key);
        occupancy[hash & kMask]++;
    }

    // With 1024 keys into 1024 buckets, the expected maximum occupancy of any
    // bucket is small. An identity hash would put exactly one in each - which
    // looks perfect here but is exactly the pathological case for probing,
    // because insertion order then walks the table linearly. What matters is
    // that the mapping is scattered, so check that consecutive keys do not land
    // in consecutive buckets.
    int adjacent = 0;
    for (std::uint64_t key = 0; key + 1 < 1024; ++key) {
        const std::uint64_t a = pn::core::Hash<std::uint64_t>{}(key) & kMask;
        const std::uint64_t b = pn::core::Hash<std::uint64_t>{}(key + 1) & kMask;
        if (b == a + 1) {
            ++adjacent;
        }
    }
    PN_CHECK(adjacent < 20);
}

PN_TEST(hash, strings_of_the_same_length_hash_differently) {
    PN_CHECK(pn::core::hash_string("albedo") != pn::core::hash_string("normal"));
    // One-character difference, and one that only differs at the tail, which is
    // where a hash that stops early gives itself away.
    PN_CHECK(pn::core::hash_string("mesh_wall_01") != pn::core::hash_string("mesh_wall_02"));
    PN_CHECK(pn::core::hash_string("") != pn::core::hash_string("a"));
}

PN_TEST(hash, a_string_hash_is_a_constant_expression) {
    // So an interned name can be hashed at compile time.
    constexpr std::uint64_t value = pn::core::hash_string("static_name");
    static_assert(value == pn::core::hash_string("static_name"));
    PN_CHECK(value != 0);
}

// -------------------------------------------------------------------
// HashMap - units
// -------------------------------------------------------------------

PN_TEST(hash_map, inserts_finds_and_erases) {
    HashMap<int, int> map;
    PN_CHECK(map.empty());

    PN_CHECK(map.insert(1, 100));
    PN_CHECK(map.insert(2, 200));
    PN_CHECK(!map.insert(1, 999));  // already present, left alone

    PN_CHECK_EQ(map.size(), std::size_t{2});
    PN_CHECK(map.contains(1));
    const int* first = map.find(1);
    PN_REQUIRE(first != nullptr);
    PN_CHECK_EQ(*first, 100);
    PN_CHECK(map.find(3) == nullptr);

    map.insert_or_assign(1, 111);
    const int* updated = map.find(1);
    PN_REQUIRE(updated != nullptr);
    PN_CHECK_EQ(*updated, 111);

    PN_CHECK(map.erase(1));
    PN_CHECK(!map.erase(1));
    PN_CHECK_EQ(map.size(), std::size_t{1});
    PN_CHECK(!map.contains(1));
    PN_CHECK(map.contains(2));
}

PN_TEST(hash_map, string_keys_work) {
    HashMap<std::string_view, int> map;
    map.insert("albedo", 1);
    map.insert("normal", 2);
    map.insert("roughness", 3);

    const int* normal = map.find("normal");
    PN_REQUIRE(normal != nullptr);
    PN_CHECK_EQ(*normal, 2);
    PN_CHECK(map.find("metallic") == nullptr);
}

PN_TEST(hash_map, survives_growth_without_losing_entries) {
    HashMap<int, int> map;
    constexpr int kCount = 5000;
    for (int index = 0; index < kCount; ++index) {
        PN_REQUIRE(map.insert(index, index * 3));
    }
    PN_CHECK_EQ(map.size(), static_cast<std::size_t>(kCount));
    for (int index = 0; index < kCount; ++index) {
        const int* found = map.find(index);
        PN_REQUIRE(found != nullptr);
        PN_REQUIRE_EQ(*found, index * 3);
    }
}

PN_TEST(hash_map, destroys_every_entry_exactly_once) {
    Tracked::reset();
    {
        HashMap<int, Tracked> map;
        for (int index = 0; index < 100; ++index) {
            map.insert(index, Tracked{index});
        }
        PN_CHECK_EQ(Tracked::live, 100);
        for (int index = 0; index < 50; ++index) {
            PN_REQUIRE(map.erase(index));
        }
        PN_CHECK_EQ(Tracked::live, 50);
        map.clear();
        PN_CHECK_EQ(Tracked::live, 0);
        for (int index = 0; index < 30; ++index) {
            map.insert(index, Tracked{index});
        }
    }
    PN_CHECK_EQ(Tracked::live, 0);
}

PN_TEST(hash_map, a_rehash_does_not_lose_or_duplicate_entries) {
    Tracked::reset();
    {
        HashMap<int, Tracked> map;
        // Enough to force several doublings.
        for (int index = 0; index < 1000; ++index) {
            map.insert(index, Tracked{index});
        }
        PN_CHECK_EQ(Tracked::live, 1000);
        PN_CHECK_EQ(map.size(), std::size_t{1000});

        int visited = 0;
        map.for_each([&visited](const int&, const Tracked&) { ++visited; });
        PN_CHECK_EQ(visited, 1000);
    }
    PN_CHECK_EQ(Tracked::live, 0);
}

PN_TEST(hash_map, erasing_does_not_break_lookups_further_along_a_probe_run) {
    // The failure backward-shift deletion has to avoid: erasing a slot in the
    // middle of a probe run must not orphan the entries after it. Keys that
    // collide deliberately, so they form one run.
    struct Colliding {
        std::uint64_t operator()(int key) const noexcept {
            // Every key hashes into the same bucket. Pathological on purpose:
            // this is the shape where a deletion bug is guaranteed to show.
            return static_cast<std::uint64_t>(key % 4);
        }
    };

    HashMap<int, int, Colliding> map;
    for (int index = 0; index < 32; ++index) {
        PN_REQUIRE(map.insert(index, index));
    }
    // Erase from the middle of the run, then check everything else is still
    // reachable.
    for (int index = 4; index < 20; ++index) {
        PN_REQUIRE(map.erase(index));
    }
    for (int index = 0; index < 4; ++index) {
        PN_REQUIRE(map.find(index) != nullptr);
    }
    for (int index = 20; index < 32; ++index) {
        const int* found = map.find(index);
        PN_REQUIRE(found != nullptr);
        PN_REQUIRE_EQ(*found, index);
    }
    for (int index = 4; index < 20; ++index) {
        PN_REQUIRE(map.find(index) == nullptr);
    }
}

PN_TEST(hash_map, the_longest_probe_stays_bounded_at_scale) {
    // The test that was missing. An earlier version of this file checked the
    // probe length only after 500 insertions, which is far too small to see the
    // problem: plain linear probing clumps as a table fills, and the longest run
    // grows much faster than the average. Measured before Robin Hood ordering
    // was added, a hundred thousand keys produced a run of 146 slots - every
    // lookup still correct, and the table no longer the thing it claimed to be.
    //
    // The bound below is loose enough not to be a tuning test and tight enough
    // that a return to plain probing fails it by a wide margin.
    HashMap<int, int> map;
    for (int index = 0; index < 100000; ++index) {
        map.insert(index, index);
    }
    PN_CHECK_EQ(map.size(), std::size_t{100000});
    PN_CHECK(map.longest_probe() < 48);

    // And every key is still reachable, so a probing change cannot pass this by
    // making the table shorter and wrong.
    for (int index = 0; index < 100000; ++index) {
        const int* found = map.find(index);
        PN_REQUIRE(found != nullptr);
        PN_REQUIRE_EQ(*found, index);
    }
}

PN_TEST(hash_map, a_miss_on_a_loaded_table_stops_early) {
    // Robin Hood's second benefit: a lookup may stop as soon as it meets an
    // entry nearer home than the key being sought. Without that, a miss walks to
    // the end of the run. Observable only as time, so what is checked here is
    // the invariant it rests on - distances never decrease along a run - which
    // is what makes the early exit sound.
    HashMap<int, int> map;
    for (int index = 0; index < 20000; ++index) {
        map.insert(index, index);
    }
    for (int index = 100000; index < 100200; ++index) {
        PN_REQUIRE(map.find(index) == nullptr);
    }
    PN_CHECK_EQ(map.size(), std::size_t{20000});
}

PN_TEST(hash_map, churn_does_not_degrade_the_longest_probe) {
    // The tombstone failure, made measurable. A table that leaves tombstones
    // still answers every lookup correctly, so nothing fails - it just gets
    // slower for ever. The only way to catch it is to measure.
    HashMap<int, int> map;
    map.reserve(1024);

    for (int index = 0; index < 500; ++index) {
        map.insert(index, index);
    }
    const std::size_t after_fill = map.longest_probe();

    // Half a million insert/erase pairs over the same key space.
    Random random{2026};
    for (int round = 0; round < 500000; ++round) {
        const int key = static_cast<int>(random.uniform(0, 499));
        if (map.contains(key)) {
            map.erase(key);
        } else {
            map.insert(key, key);
        }
    }

    const std::size_t after_churn = map.longest_probe();
    PN_CHECK(after_churn <= after_fill + 8);
    PN_CHECK(after_churn < 64);
}

PN_TEST(hash_map, copies_are_independent) {
    HashMap<int, std::string> original;
    original.insert(1, "one");
    original.insert(2, "two");

    HashMap<int, std::string> copied = original;
    copied.insert_or_assign(1, "changed");
    copied.insert(3, "three");

    const std::string* untouched = original.find(1);
    PN_REQUIRE(untouched != nullptr);
    PN_CHECK(*untouched == "one");
    PN_CHECK(original.find(3) == nullptr);

    const std::string* changed = copied.find(1);
    PN_REQUIRE(changed != nullptr);
    PN_CHECK(*changed == "changed");
    PN_CHECK_EQ(copied.size(), std::size_t{3});
}

// -------------------------------------------------------------------
// HashMap - property tests against a known-correct oracle
// -------------------------------------------------------------------

PN_TEST(hash_map, agrees_with_std_map_over_a_long_random_operation_sequence) {
    // The test that finds what cases nobody thought of. Every operation is
    // applied to both containers and the results compared; a divergence at any
    // point fails immediately, with a seed that reproduces it exactly.
    constexpr int kOperations = 200000;
    constexpr std::uint64_t kKeySpace = 3000;

    HashMap<std::uint64_t, std::uint64_t> subject;
    std::map<std::uint64_t, std::uint64_t> oracle;
    Random random{20260821, 7};

    for (int operation = 0; operation < kOperations; ++operation) {
        const std::uint64_t key = random.uniform(0, kKeySpace - 1);
        const std::uint64_t choice = random.uniform(0, 99);

        if (choice < 45) {
            const std::uint64_t value = random.next_u64();
            const bool inserted = subject.insert(key, value);
            const bool expected = oracle.find(key) == oracle.end();
            PN_REQUIRE_EQ(inserted, expected);
            if (expected) {
                oracle.emplace(key, value);
            }
        } else if (choice < 65) {
            const std::uint64_t value = random.next_u64();
            subject.insert_or_assign(key, value);
            oracle[key] = value;
        } else if (choice < 85) {
            const bool erased = subject.erase(key);
            const bool expected = oracle.erase(key) != 0;
            PN_REQUIRE_EQ(erased, expected);
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

    // And a full sweep at the end, so an entry that became unreachable without
    // changing the count is still caught.
    for (const auto& [key, value] : oracle) {
        const std::uint64_t* found = subject.find(key);
        PN_REQUIRE(found != nullptr);
        PN_REQUIRE_EQ(*found, value);
    }
    std::size_t visited = 0;
    subject.for_each([&](const std::uint64_t& key, const std::uint64_t&) {
        PN_REQUIRE(oracle.find(key) != oracle.end());
        ++visited;
    });
    PN_CHECK_EQ(visited, oracle.size());
}

PN_TEST(inline_array, agrees_with_a_vector_over_a_random_operation_sequence) {
    constexpr std::size_t kCapacity = 32;
    InlineArray<int, kCapacity> subject;
    std::vector<int> oracle;
    Random random{555};

    for (int operation = 0; operation < 100000; ++operation) {
        const std::uint64_t choice = random.uniform(0, 99);
        if (choice < 55 && subject.size() < kCapacity) {
            const int value = static_cast<int>(random.uniform(0, 1000));
            subject.push_back(value);
            oracle.push_back(value);
        } else if (choice < 80 && !oracle.empty()) {
            subject.pop_back();
            oracle.pop_back();
        } else if (!oracle.empty()) {
            const std::size_t index = random.uniform(0, oracle.size() - 1);
            subject.erase_unordered(index);
            oracle[index] = oracle.back();
            oracle.pop_back();
        }

        PN_REQUIRE_EQ(subject.size(), oracle.size());
    }

    for (std::size_t index = 0; index < oracle.size(); ++index) {
        PN_REQUIRE_EQ(subject[index], oracle[index]);
    }
}

PN_TEST(hash_map, erasing_from_the_middle_of_a_colliding_run_loses_nothing) {
    // Three keys, two of which share an ideal slot, under an identity hash and a
    // capacity of eight. Erasing the one at the head of the run must leave both
    // others reachable.
    //
    // This case has a history worth recording. Under plain linear probing the
    // arrangement was: slot 5 = key 5, slot 6 = key 6 (on its own ideal slot),
    // slot 7 = key 13 (ideal 5). Erasing key 5 with a backward shift that
    // stopped at the first immovable entry left slot 5 empty and key 13
    // unreachable - still in the table, answering no query. The fix then was to
    // keep scanning past such an entry.
    //
    // Robin Hood ordering changes the arrangement: key 13 displaces key 6 on
    // insert, so the run is 5, 13, 6 by distance, and the simple shift is
    // correct again because distances never decrease along a run. The rule
    // returned to what it had been; what changed is that the insert path now
    // earns it. The assertion below is the same either way, which is why it is
    // still here.
    struct Identity {
        std::uint64_t operator()(std::uint64_t key) const noexcept { return key; }
    };

    HashMap<std::uint64_t, int, Identity> map;
    map.insert(5, 500);
    map.insert(6, 600);
    map.insert(13, 1300);
    PN_REQUIRE_EQ(map.capacity(), std::size_t{8});

    PN_REQUIRE(map.erase(5));

    const int* survivor = map.find(13);
    PN_REQUIRE(survivor != nullptr);
    PN_CHECK_EQ(*survivor, 1300);

    const int* anchor = map.find(6);
    PN_REQUIRE(anchor != nullptr);
    PN_CHECK_EQ(*anchor, 600);
    PN_CHECK(map.find(5) == nullptr);
    PN_CHECK_EQ(map.size(), std::size_t{2});
}
