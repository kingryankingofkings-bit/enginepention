// Pention Engine -  testing/tests/self_test.cpp
// Requirement: PN-PLT-026 -  the test framework verifies its own failure reporting
// Decision:    ADR-0001
//
// A test framework that cannot fail is worse than no test framework, because it
// reports a green suite regardless of the code under test. These tests run
// deliberately failing bodies under observation and assert that the failure was
// actually recorded.
//
// The deliberate-failure bodies below are plain functions, not PN_TEST cases, so
// they are never registered and never run by the top-level runner. They are
// invoked only through run_one() with quiet=true.

#include "pn/testing/test.hpp"

#include <limits>

namespace {

using pn::testing::TestCase;
using pn::testing::TestContext;
using pn::testing::TestOutcome;

void body_all_pass(TestContext& pn_ctx_) {
    PN_CHECK(true);
    PN_CHECK_EQ(2 + 2, 4);
}

void body_one_check_fails(TestContext& pn_ctx_) {
    PN_CHECK(true);
    PN_CHECK(false);       // deliberate
    PN_CHECK(true);        // must still run -  CHECK does not abort
}

void body_require_aborts(TestContext& pn_ctx_) {
    PN_CHECK(true);
    PN_REQUIRE(false);     // deliberate -  must return from here
    PN_CHECK(true);        // must NOT run
}

void body_require_eq_aborts(TestContext& pn_ctx_) {
    PN_REQUIRE_EQ(1, 2);   // deliberate
    PN_CHECK(true);        // must NOT run
}

void body_explicit_fail(TestContext& pn_ctx_) {
    PN_FAIL("deliberate");
}

TestOutcome observe(pn::testing::TestFunction fn) {
    const TestCase probe{"selfcheck", "probe", fn, __FILE__, __LINE__};
    return pn::testing::run_one(probe, /*quiet=*/true);
}

}  // namespace

PN_TEST(framework, passing_checks_are_counted) {
    const TestOutcome outcome = observe(&body_all_pass);
    PN_CHECK(outcome.passed());
    PN_CHECK_EQ(outcome.checks_run, 2u);
    PN_CHECK_EQ(outcome.checks_failed, 0u);
    PN_CHECK(!outcome.aborted);
}

PN_TEST(framework, failing_check_is_recorded_and_does_not_abort) {
    const TestOutcome outcome = observe(&body_one_check_fails);
    PN_CHECK(!outcome.passed());
    PN_CHECK_EQ(outcome.checks_failed, 1u);
    // Three checks must have run: a CHECK failure continues past the failure.
    PN_CHECK_EQ(outcome.checks_run, 3u);
    PN_CHECK(!outcome.aborted);
}

PN_TEST(framework, failing_require_aborts_the_body) {
    const TestOutcome outcome = observe(&body_require_aborts);
    PN_CHECK(!outcome.passed());
    PN_CHECK(outcome.aborted);
    // The check after the failed REQUIRE must not have executed.
    PN_CHECK_EQ(outcome.checks_run, 2u);
    PN_CHECK_EQ(outcome.checks_failed, 1u);
}

PN_TEST(framework, failing_require_eq_aborts_the_body) {
    const TestOutcome outcome = observe(&body_require_eq_aborts);
    PN_CHECK(outcome.aborted);
    PN_CHECK_EQ(outcome.checks_run, 1u);
}

PN_TEST(framework, explicit_fail_is_recorded) {
    const TestOutcome outcome = observe(&body_explicit_fail);
    PN_CHECK(!outcome.passed());
    PN_CHECK_EQ(outcome.checks_failed, 1u);
}

PN_TEST(framework, near_respects_tolerance) {
    PN_CHECK_NEAR(1.0, 1.0 + 1e-9, 1e-6);
    PN_CHECK_NEAR(-5.0f, -5.0f, 0.0f);

    // And must reject a difference outside tolerance.
    PN_CHECK(!pn::testing::detail::within_tolerance(1.0, 1.5, 1e-6));
    PN_CHECK(pn::testing::detail::within_tolerance(1.0, 1.0000001, 1e-6));
}

PN_TEST(framework, near_handles_infinity_without_false_pass) {
    const double inf = std::numeric_limits<double>::infinity();
    // inf vs inf yields a NaN difference, which compares false against any
    // tolerance. Asserting the false result pins the behaviour so a later
    // "simplification" of within_tolerance cannot silently start passing here.
    PN_CHECK(!pn::testing::detail::within_tolerance(inf, inf, 1.0));
    PN_CHECK(!pn::testing::detail::within_tolerance(0.0, inf, 1.0));
}

PN_TEST(framework, describe_formats_streamable_values) {
    PN_CHECK_EQ(pn::testing::detail::describe(42), std::string{"42"});
    PN_CHECK_EQ(pn::testing::detail::describe(true), std::string{"1"});
}

namespace {
struct NotPrintable {
    int value;
    bool operator==(const NotPrintable&) const = default;
};
}  // namespace

PN_TEST(framework, describe_falls_back_for_unprintable_types) {
    PN_CHECK_EQ(pn::testing::detail::describe(NotPrintable{7}),
                std::string{"<not printable>"});
    // A comparison macro must still work on a type with no stream operator.
    PN_CHECK_EQ(NotPrintable{7}, NotPrintable{7});
}

// ---------------------------------------------------------------------------
// Single-evaluation guarantee
//
// Regression tests. The comparison macros originally expanded each operand
// twice - once to compare and once to build the failure message - so any
// argument with a side effect ran twice, in unspecified order. It was found by
// a FrameCounter test whose advance() was called twice per check, and it had
// been silently corrupting side-effecting assertions until then.
//
// These pin the guarantee for every macro that takes operands.
// ---------------------------------------------------------------------------

namespace {

/// Counts how many times it is read.
struct EvaluationCounter {
    int* calls;
    int value;

    [[nodiscard]] int read() const {
        ++(*calls);
        return value;
    }
};

}  // namespace

PN_TEST(framework, check_eq_evaluates_each_operand_exactly_once) {
    int left_calls = 0;
    int right_calls = 0;
    const EvaluationCounter left{&left_calls, 5};
    const EvaluationCounter right{&right_calls, 5};

    PN_CHECK_EQ(left.read(), right.read());

    PN_CHECK_EQ(left_calls, 1);
    PN_CHECK_EQ(right_calls, 1);
}

namespace {

int g_failing_path_calls = 0;

/// Deliberately fails a comparison whose left operand has a side effect, then
/// asserts the operand ran exactly once. Run under observation so the
/// deliberate failure does not fail the suite.
void body_failing_comparison_evaluates_once(TestContext& pn_ctx_) {
    g_failing_path_calls = 0;
    const EvaluationCounter counter{&g_failing_path_calls, 1};
    PN_CHECK_EQ(counter.read(), 999);          // deliberate failure
    PN_CHECK_EQ(g_failing_path_calls, 1);      // exactly one evaluation despite failing
}

}  // namespace

PN_TEST(framework, check_eq_evaluates_once_even_when_the_check_fails) {
    // The failing path is the one that regressed: the second evaluation existed
    // only to format the failure message, so a passing check never exposed it.
    const TestOutcome outcome = observe(&body_failing_comparison_evaluates_once);

    // Exactly one deliberate failure, and the follow-up call-count check passed.
    PN_CHECK_EQ(outcome.checks_failed, 1u);
    PN_CHECK_EQ(outcome.checks_run, 2u);
    PN_CHECK_EQ(g_failing_path_calls, 1);
}

PN_TEST(framework, ordering_comparisons_evaluate_each_operand_once) {
    int calls = 0;
    const EvaluationCounter counter{&calls, 3};

    PN_CHECK_LT(counter.read(), 10);
    PN_CHECK_EQ(calls, 1);

    calls = 0;
    PN_CHECK_GE(counter.read(), 1);
    PN_CHECK_EQ(calls, 1);

    calls = 0;
    PN_CHECK_NE(counter.read(), 99);
    PN_CHECK_EQ(calls, 1);
}

PN_TEST(framework, require_eq_evaluates_each_operand_once) {
    int calls = 0;
    const EvaluationCounter counter{&calls, 7};
    PN_REQUIRE_EQ(counter.read(), 7);
    PN_CHECK_EQ(calls, 1);
}

PN_TEST(framework, near_evaluates_each_operand_once) {
    int calls = 0;
    int tolerance_calls = 0;
    const EvaluationCounter value{&calls, 1};
    const EvaluationCounter tolerance{&tolerance_calls, 1};

    PN_CHECK_NEAR(static_cast<double>(value.read()), 1.0,
                  static_cast<double>(tolerance.read()));
    PN_CHECK_EQ(calls, 1);
    PN_CHECK_EQ(tolerance_calls, 1);
}

PN_TEST(framework, check_evaluates_its_expression_exactly_once) {
    int calls = 0;
    const EvaluationCounter counter{&calls, 1};
    PN_CHECK(counter.read() == 1);
    PN_CHECK_EQ(calls, 1);
}

PN_TEST(framework, a_mutating_expression_advances_exactly_one_step_per_check) {
    // The shape of the original bug, reduced: a counter that advances on read.
    int state = 0;
    const auto advance = [&state] { return ++state; };

    PN_CHECK_EQ(advance(), 1);
    PN_CHECK_EQ(advance(), 2);
    PN_CHECK_EQ(advance(), 3);
    PN_CHECK_EQ(state, 3);
}
