// Pention Engine - core/tests/expected_test.cpp
// Requirement: PN-PLT-027
// Decision:    ADR-0002

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"
#include "pn/testing/test.hpp"

#include <memory>
#include <string>
#include <type_traits>
#include <utility>

namespace {

using pn::core::Error;
using pn::core::ErrorCategory;
using pn::core::Expected;
using pn::core::Unexpected;

/// Counts destructions so the union's lifetime management can be checked
/// rather than assumed.
struct Tracked {
    static int live;
    int value = 0;

    explicit Tracked(int v) : value(v) { ++live; }
    Tracked(const Tracked& other) : value(other.value) { ++live; }
    Tracked(Tracked&& other) noexcept : value(other.value) { ++live; }
    Tracked& operator=(const Tracked&) = default;
    Tracked& operator=(Tracked&&) = default;
    ~Tracked() { --live; }
};
int Tracked::live = 0;

Expected<int, Error> halve(int value) {
    if (value % 2 != 0) {
        return pn::core::fail(ErrorCategory::invalid_argument, "value is odd");
    }
    return value / 2;
}

Expected<int, Error> quarter_via_try(int value) {
    PN_TRY_ASSIGN(const int once, halve(value));
    PN_TRY_ASSIGN(const int twice, halve(once));
    return twice;
}

Expected<void, Error> require_positive(int value) {
    if (value <= 0) {
        return Unexpected{Error{ErrorCategory::out_of_range, "not positive"}};
    }
    return {};
}

Expected<int, Error> uses_try_void(int value) {
    PN_TRY_VOID(require_positive(value));
    return value * 10;
}

}  // namespace

PN_TEST(expected, holds_a_value) {
    const Expected<int, Error> result = 42;
    PN_REQUIRE(result.has_value());
    PN_CHECK(static_cast<bool>(result));
    PN_CHECK_EQ(result.value(), 42);
}

PN_TEST(expected, holds_an_error) {
    const Expected<int, Error> result =
        pn::core::fail(ErrorCategory::not_found, "nothing here");
    PN_REQUIRE(!result.has_value());
    PN_CHECK(!static_cast<bool>(result));
    PN_CHECK(result.error().is(ErrorCategory::not_found));
    PN_CHECK_EQ(result.error().message(), std::string_view{"nothing here"});
}

PN_TEST(expected, captures_the_source_location_of_the_failure) {
    // The line the Error is constructed on, not the line it is inspected on.
    const int expected_line = __LINE__ + 1;
    const Error error{ErrorCategory::io, "disk fell over"};
    PN_CHECK_EQ(static_cast<int>(error.where().line()), expected_line);
    PN_CHECK(std::string_view{error.where().file_name()}.find("expected_test.cpp") !=
             std::string_view::npos);
}

PN_TEST(expected, value_or_returns_fallback_on_error) {
    const Expected<int, Error> good = 7;
    const Expected<int, Error> bad = pn::core::fail(ErrorCategory::io, "nope");
    PN_CHECK_EQ(good.value_or(-1), 7);
    PN_CHECK_EQ(bad.value_or(-1), -1);
}

PN_TEST(expected, is_trivially_destructible_for_trivial_payloads) {
    // ADR-0002 promises this costs nothing in the common case. A static_assert
    // makes the promise enforceable rather than aspirational.
    static_assert(std::is_trivially_destructible_v<Error>,
                  "Error must stay allocation-free and trivially destructible");
    static_assert(std::is_trivially_destructible_v<Expected<int, Error>>,
                  "Expected<int, Error> must be trivially destructible");
    PN_CHECK(true);
}

PN_TEST(expected, destroys_exactly_one_alternative) {
    Tracked::live = 0;
    {
        const Expected<Tracked, Error> holding{Tracked{1}};
        PN_CHECK_EQ(Tracked::live, 1);
    }
    PN_CHECK_EQ(Tracked::live, 0);

    {
        const Expected<Tracked, Error> failed =
            pn::core::fail(ErrorCategory::io, "no value constructed");
        // No Tracked exists in the error case.
        PN_CHECK_EQ(Tracked::live, 0);
    }
    PN_CHECK_EQ(Tracked::live, 0);
}

PN_TEST(expected, copy_and_move_preserve_the_active_alternative) {
    Tracked::live = 0;
    {
        const Expected<Tracked, Error> original{Tracked{5}};
        const Expected<Tracked, Error> copy = original;
        PN_REQUIRE(copy.has_value());
        PN_CHECK_EQ(copy.value().value, 5);
        PN_CHECK_EQ(Tracked::live, 2);

        Expected<Tracked, Error> source{Tracked{9}};
        const Expected<Tracked, Error> moved = std::move(source);
        PN_REQUIRE(moved.has_value());
        PN_CHECK_EQ(moved.value().value, 9);
    }
    PN_CHECK_EQ(Tracked::live, 0);
}

PN_TEST(expected, supports_move_only_payloads) {
    Expected<std::unique_ptr<int>, Error> owned{std::make_unique<int>(11)};
    PN_REQUIRE(owned.has_value());
    PN_CHECK_EQ(*owned.value(), 11);

    const std::unique_ptr<int> taken = std::move(owned).value();
    PN_REQUIRE(taken != nullptr);
    PN_CHECK_EQ(*taken, 11);
}

PN_TEST(expected, map_transforms_values_and_passes_errors_through) {
    const Expected<int, Error> good = 21;
    const auto doubled = good.map([](int v) { return v * 2; });
    PN_REQUIRE(doubled.has_value());
    PN_CHECK_EQ(doubled.value(), 42);

    const Expected<int, Error> bad = pn::core::fail(ErrorCategory::io, "broken");
    const auto still_bad = bad.map([](int v) { return v * 2; });
    PN_REQUIRE(!still_bad.has_value());
    PN_CHECK(still_bad.error().is(ErrorCategory::io));
}

PN_TEST(expected, and_then_chains_fallible_operations) {
    const Expected<int, Error> start = 8;
    const auto chained = start.and_then(halve).and_then(halve);
    PN_REQUIRE(chained.has_value());
    PN_CHECK_EQ(chained.value(), 2);
}

PN_TEST(expected, and_then_short_circuits_on_the_first_failure) {
    const Expected<int, Error> start = 7;  // odd, so the first halve fails
    const auto chained = start.and_then(halve).and_then(halve);
    PN_REQUIRE(!chained.has_value());
    PN_CHECK(chained.error().is(ErrorCategory::invalid_argument));
    PN_CHECK_EQ(chained.error().message(), std::string_view{"value is odd"});
}

PN_TEST(expected, or_else_can_recover) {
    const Expected<int, Error> bad = pn::core::fail(ErrorCategory::not_found, "absent");
    const auto recovered = bad.or_else([](const Error&) -> Expected<int, Error> { return 0; });
    PN_REQUIRE(recovered.has_value());
    PN_CHECK_EQ(recovered.value(), 0);
}

PN_TEST(expected, try_assign_binds_the_value_on_success) {
    const auto result = quarter_via_try(20);
    PN_REQUIRE(result.has_value());
    PN_CHECK_EQ(result.value(), 5);
}

PN_TEST(expected, try_assign_propagates_the_first_failure) {
    // 10 halves to 5, which is odd, so the second halve fails and PN_TRY_ASSIGN must
    // return that error from quarter_via_try unchanged.
    const auto result = quarter_via_try(10);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
    PN_CHECK_EQ(result.error().message(), std::string_view{"value is odd"});
}

PN_TEST(expected, void_specialization_reports_success_and_failure) {
    PN_CHECK(require_positive(1).has_value());
    const auto bad = require_positive(-1);
    PN_REQUIRE(!bad.has_value());
    PN_CHECK(bad.error().is(ErrorCategory::out_of_range));
}

PN_TEST(expected, try_void_macro_propagates) {
    const auto good = uses_try_void(3);
    PN_REQUIRE(good.has_value());
    PN_CHECK_EQ(good.value(), 30);

    const auto bad = uses_try_void(0);
    PN_REQUIRE(!bad.has_value());
    PN_CHECK(bad.error().is(ErrorCategory::out_of_range));
}

PN_TEST(expected, error_categories_stringify) {
    PN_CHECK_EQ(pn::core::to_string(ErrorCategory::out_of_memory),
                std::string_view{"out_of_memory"});
    PN_CHECK_EQ(pn::core::to_string(ErrorCategory::corrupt_data),
                std::string_view{"corrupt_data"});
}
