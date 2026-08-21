// Pention Engine - core/tests/handle_test.cpp
// Requirement: PN-PLT-011
// Decision:    ADR-0003 (docs/conventions.md, "Naming and identity")

#include "pn/core/error.hpp"
#include "pn/core/handle.hpp"
#include "pn/testing/test.hpp"

#include <string>
#include <type_traits>
#include <vector>

namespace {

using pn::core::ErrorCategory;
using pn::core::Handle;
using pn::core::HandlePool;

struct MeshTag {};
struct TextureTag {};

struct Resource {
    std::string name;
    int payload = 0;
};

}  // namespace

PN_TEST(handle, default_handle_is_invalid) {
    const Handle<MeshTag> handle;
    PN_CHECK(!handle.is_valid());
    PN_CHECK_EQ(handle.index(), Handle<MeshTag>::kInvalidIndex);
}

PN_TEST(handle, handles_of_different_tags_are_distinct_types) {
    // The whole point of the tag: a texture handle must not be usable where a
    // mesh handle is expected. If these ever became the same type, an entire
    // class of resource-confusion bug becomes possible.
    static_assert(!std::is_same_v<Handle<MeshTag>, Handle<TextureTag>>,
                  "tagged handles must not collapse to one type");
    static_assert(!std::is_convertible_v<Handle<MeshTag>, Handle<TextureTag>>,
                  "tagged handles must not be implicitly convertible");
    PN_CHECK(true);
}

PN_TEST(handle, insert_then_get_returns_the_value) {
    HandlePool<Resource, MeshTag> pool;
    const auto handle = pool.insert(Resource{"cube", 7});

    PN_REQUIRE(pool.contains(handle));
    const auto resolved = pool.get(handle);
    PN_REQUIRE(resolved.has_value());
    PN_CHECK_EQ(resolved.value()->name, std::string{"cube"});
    PN_CHECK_EQ(resolved.value()->payload, 7);
    PN_CHECK_EQ(pool.size(), 1u);
}

PN_TEST(handle, removed_handle_no_longer_resolves) {
    HandlePool<Resource, MeshTag> pool;
    const auto handle = pool.insert(Resource{"temp", 1});
    PN_REQUIRE(pool.remove(handle).has_value());

    PN_CHECK(!pool.contains(handle));
    const auto resolved = pool.get(handle);
    PN_REQUIRE(!resolved.has_value());
    PN_CHECK(resolved.error().is(ErrorCategory::not_found));
    PN_CHECK_EQ(pool.size(), 0u);
}

PN_TEST(handle, recycled_slot_invalidates_the_old_handle) {
    // This is the defect generational handles exist to prevent. Without the
    // generation counter, `stale` below would resolve to `second` - a
    // use-after-free that reads as a plain logic bug and can survive for months.
    HandlePool<Resource, MeshTag> pool;

    const auto first = pool.insert(Resource{"first", 1});
    PN_REQUIRE(pool.remove(first).has_value());

    const auto second = pool.insert(Resource{"second", 2});

    // The recycled slot has the same index...
    PN_CHECK_EQ(first.index(), second.index());
    // ...but a different generation.
    PN_CHECK_NE(first.generation(), second.generation());

    // So the old handle is detectably stale rather than silently aliasing.
    PN_CHECK(!pool.contains(first));
    const auto stale = pool.get(first);
    PN_REQUIRE(!stale.has_value());
    PN_CHECK(stale.error().is(ErrorCategory::not_found));

    // And the new handle resolves to the new object.
    const auto live = pool.get(second);
    PN_REQUIRE(live.has_value());
    PN_CHECK_EQ(live.value()->name, std::string{"second"});
}

PN_TEST(handle, double_remove_is_reported_not_ignored) {
    // A double free is a real defect. Tolerating it silently hides the defect
    // rather than the symptom.
    HandlePool<Resource, MeshTag> pool;
    const auto handle = pool.insert(Resource{"once", 1});

    PN_REQUIRE(pool.remove(handle).has_value());
    const auto second_remove = pool.remove(handle);
    PN_REQUIRE(!second_remove.has_value());
    PN_CHECK(second_remove.error().is(ErrorCategory::not_found));
}

PN_TEST(handle, never_issued_handle_is_rejected_distinctly) {
    HandlePool<Resource, MeshTag> pool;
    const Handle<MeshTag> never_issued;
    const auto result = pool.get(never_issued);
    PN_REQUIRE(!result.has_value());
    // Distinct from "stale" so diagnostics can tell the two apart.
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
}

PN_TEST(handle, out_of_range_index_is_rejected) {
    HandlePool<Resource, MeshTag> pool;
    static_cast<void>(pool.insert(Resource{"only", 1}));
    const Handle<MeshTag> beyond{999u, 1u};
    const auto result = pool.get(beyond);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::out_of_range));
}

PN_TEST(handle, many_insert_remove_cycles_stay_consistent) {
    // Randomized-ish churn: the invariant is that every handle the test still
    // believes is live resolves, and every handle it has released does not.
    HandlePool<Resource, MeshTag> pool;
    std::vector<Handle<MeshTag>> live;
    std::vector<Handle<MeshTag>> released;

    for (int i = 0; i < 200; ++i) {
        live.push_back(pool.insert(Resource{"r" + std::to_string(i), i}));
    }
    // Release every third handle.
    for (std::size_t i = 0; i < live.size(); i += 3) {
        PN_REQUIRE(pool.remove(live[i]).has_value());
        released.push_back(live[i]);
    }
    // Re-fill, forcing slot recycling.
    for (int i = 0; i < 40; ++i) {
        live.push_back(pool.insert(Resource{"new" + std::to_string(i), i}));
    }

    std::size_t stale_detected = 0;
    for (const auto& handle : released) {
        if (!pool.contains(handle)) {
            ++stale_detected;
        }
    }
    // Every released handle must be detected as stale - none may resolve.
    PN_CHECK_EQ(stale_detected, released.size());
    PN_CHECK_GT(released.size(), 0u);
}

PN_TEST(handle, size_and_capacity_track_recycling) {
    HandlePool<Resource, MeshTag> pool;
    const auto a = pool.insert(Resource{"a", 1});
    const auto b = pool.insert(Resource{"b", 2});
    PN_CHECK_EQ(pool.size(), 2u);
    PN_CHECK_EQ(pool.capacity(), 2u);

    PN_REQUIRE(pool.remove(a).has_value());
    PN_CHECK_EQ(pool.size(), 1u);
    // Capacity does not shrink - the slot is retained for recycling.
    PN_CHECK_EQ(pool.capacity(), 2u);

    static_cast<void>(pool.insert(Resource{"c", 3}));
    PN_CHECK_EQ(pool.size(), 2u);
    PN_CHECK_EQ(pool.capacity(), 2u);  // reused, not grown

    PN_CHECK(pool.contains(b));
}
