// Pention Engine -  testing/test.hpp
// Requirement: PN-PLT-026 -  in-project unit test framework
// Decision:    ADR-0001, ADR-0002
//
// Third-party test frameworks (GoogleTest, Catch2, doctest and equivalents) are
// prohibited by the custom-build boundary; see DEPENDENCY_BOUNDARY.md. This
// framework is written for this project.
//
// Design constraints that shaped it:
//   * Must work with exceptions disabled (ADR-0002), so a failed fatal
//     assertion returns from the test body rather than throwing.
//   * Must capture source location for every failure.
//   * Must be able to test itself, which requires running a test body under
//     controlled observation rather than only through the top-level runner.

#ifndef PN_TESTING_TEST_HPP
#define PN_TESTING_TEST_HPP

#include <cstddef>
#include <source_location>
#include <sstream>
#include <string>
#include <string_view>

namespace pn::testing {

/// Outcome of a single test case.
struct TestOutcome {
    std::size_t checks_run = 0;
    std::size_t checks_failed = 0;
    bool aborted = false;  ///< a fatal assertion returned early

    [[nodiscard]] bool passed() const noexcept {
        return checks_failed == 0 && !aborted;
    }
};

/// Per-test state threaded through the assertion macros.
///
/// `quiet` suppresses failure printing. The framework's own self-tests use it
/// to run a deliberately failing body and assert that the failure was recorded,
/// without polluting the output of a passing run.
class TestContext {
public:
    TestContext() = default;
    explicit TestContext(bool quiet) noexcept : quiet_(quiet) {}

    void record_pass() noexcept { ++outcome_.checks_run; }

    void record_failure(std::string_view expression,
                        std::string_view detail,
                        std::source_location where,
                        bool fatal);

    [[nodiscard]] const TestOutcome& outcome() const noexcept { return outcome_; }
    [[nodiscard]] bool quiet() const noexcept { return quiet_; }

private:
    TestOutcome outcome_{};
    bool quiet_ = false;
};

using TestFunction = void (*)(TestContext&);

struct TestCase {
    const char* suite;
    const char* name;
    TestFunction function;
    const char* file;
    int line;
};

/// Registers a test case. Called from a namespace-scope static initializer, so
/// registration happens before main() without any explicit list to maintain.
void register_test(const TestCase& test_case);

/// Runs one test case in isolation and returns its outcome. Public because the
/// framework's self-tests need it; ordinary tests never call it.
[[nodiscard]] TestOutcome run_one(const TestCase& test_case, bool quiet);

/// Runs every registered test. Returns a process exit code: 0 on success.
///
/// Recognised arguments:
///   --filter=<substring>   run only tests whose "suite.name" contains it
///   --list                 print registered tests and exit
[[nodiscard]] int run_all(int argc, char** argv);

namespace detail {

/// Registers a test at static-initialization time.
struct AutoRegister {
    explicit AutoRegister(const TestCase& test_case) { register_test(test_case); }
};

/// Converts any testable expression to bool without a cast.
///
/// A `static_cast<bool>` in the assertion macro trips -Wuseless-cast when the
/// expression is already bool, and that warning is worth keeping enabled for
/// ordinary code. Branching instead of casting keeps both.
template <typename T>
[[nodiscard]] constexpr bool truth(T&& value) {
    if (value) {
        return true;
    }
    return false;
}

/// Records the outcome of one assertion. Returns the condition, so the calling
/// macro can decide whether to return from the test body.
bool evaluate(TestContext& ctx,
              bool condition,
              std::string_view expression,
              std::string_view detail,
              std::source_location where,
              bool fatal);

/// Formats a value for a failure message, falling back to a placeholder for
/// types with no stream insertion operator.
template <typename T>
std::string describe(const T& value) {
    if constexpr (requires(std::ostringstream& os, const T& v) { os << v; }) {
        std::ostringstream os;
        os << value;
        return os.str();
    } else {
        return "<not printable>";
    }
}

/// Compares two values, evaluating each exactly once.
///
/// The operands arrive as function parameters, so the caller's expressions are
/// evaluated before the call and never again. An earlier version of this
/// framework expanded each operand twice inside the macro - once for the
/// comparison and once for the failure message - which double-called anything
/// with a side effect, and did so in unspecified order, so the reported values
/// did not even match the ones compared.
template <typename T>
[[nodiscard]] bool within_tolerance(T lhs, T rhs, T tolerance) noexcept {
    const T difference = lhs > rhs ? lhs - rhs : rhs - lhs;
    return difference <= tolerance;
}

template <typename T>
[[nodiscard]] std::string describe_tolerance(T lhs, T rhs, T tolerance) {
    const T difference = lhs > rhs ? lhs - rhs : rhs - lhs;
    std::ostringstream os;
    os.precision(17);
    os << lhs << " vs " << rhs << " (difference " << difference
       << ", tolerance " << tolerance << ')';
    return os.str();
}

template <typename A, typename B>
std::string describe_comparison(const A& lhs, const B& rhs, std::string_view op) {
    std::string out;
    out += describe(lhs);
    out += ' ';
    out += op;
    out += ' ';
    out += describe(rhs);
    return out;
}

template <typename A, typename B, typename Compare>
bool compare_values(TestContext& ctx,
                    const A& lhs,
                    const B& rhs,
                    Compare compare,
                    std::string_view expression,
                    std::string_view failure_op,
                    std::source_location where,
                    bool fatal) {
    if (compare(lhs, rhs)) {
        ctx.record_pass();
        return true;
    }
    ctx.record_failure(expression, describe_comparison(lhs, rhs, failure_op), where, fatal);
    return false;
}

/// Absolute-tolerance comparison, evaluating each operand exactly once.
template <typename T>
bool compare_near(TestContext& ctx,
                  T lhs,
                  T rhs,
                  T tolerance,
                  std::string_view expression,
                  std::source_location where) {
    if (within_tolerance(lhs, rhs, tolerance)) {
        ctx.record_pass();
        return true;
    }
    ctx.record_failure(expression, describe_tolerance(lhs, rhs, tolerance), where, false);
    return false;
}


}  // namespace detail
}  // namespace pn::testing

// ---------------------------------------------------------------------------
// Test declaration
// ---------------------------------------------------------------------------

#define PN_TEST_DETAIL_CONCAT_(a, b) a##b
#define PN_TEST_DETAIL_CONCAT(a, b) PN_TEST_DETAIL_CONCAT_(a, b)

/// Declares a test case. The body receives an implicit `pn_ctx_` used by the
/// assertion macros.
#define PN_TEST(suite_, name_)                                                       \
    static void PN_TEST_DETAIL_CONCAT(pn_test_body_, PN_TEST_DETAIL_CONCAT(          \
        suite_, PN_TEST_DETAIL_CONCAT(_, name_)))(                                    \
            [[maybe_unused]] ::pn::testing::TestContext& pn_ctx_);                    \
    namespace {                                                                       \
    const ::pn::testing::detail::AutoRegister PN_TEST_DETAIL_CONCAT(                  \
        pn_test_reg_, PN_TEST_DETAIL_CONCAT(suite_, PN_TEST_DETAIL_CONCAT(_, name_))) \
        {::pn::testing::TestCase{                                                     \
            #suite_, #name_,                                                          \
            &PN_TEST_DETAIL_CONCAT(pn_test_body_, PN_TEST_DETAIL_CONCAT(              \
                suite_, PN_TEST_DETAIL_CONCAT(_, name_))),                            \
            __FILE__, __LINE__}};                                                     \
    }                                                                                 \
    static void PN_TEST_DETAIL_CONCAT(pn_test_body_, PN_TEST_DETAIL_CONCAT(           \
        suite_, PN_TEST_DETAIL_CONCAT(_, name_)))(                                     \
            [[maybe_unused]] ::pn::testing::TestContext& pn_ctx_)

// ---------------------------------------------------------------------------
// Assertions
//
// PN_CHECK*  records a failure and continues, so one run reports every problem.
// PN_REQUIRE* records a failure and returns, for preconditions whose failure
// would make the rest of the test meaningless or unsafe.
// ---------------------------------------------------------------------------

#define PN_CHECK(expr_)                                                          \
    ::pn::testing::detail::evaluate(pn_ctx_,                                      \
                                    ::pn::testing::detail::truth(expr_), #expr_,  \
                                    {}, std::source_location::current(), false)

#define PN_REQUIRE(expr_)                                                            \
    do {                                                                             \
        if (!::pn::testing::detail::evaluate(pn_ctx_,                                 \
                                             ::pn::testing::detail::truth(expr_),      \
                                             #expr_, {},                               \
                                             std::source_location::current(), true)) { \
            return;                                                                   \
        }                                                                             \
    } while (false)

#define PN_TEST_DETAIL_BINARY(lhs_, rhs_, op_, opname_, fatal_)                       \
    ::pn::testing::detail::compare_values(                                            \
        pn_ctx_, (lhs_), (rhs_),                                                       \
        [](const auto& pn_lhs_, const auto& pn_rhs_) { return pn_lhs_ op_ pn_rhs_; },  \
        #lhs_ " " #op_ " " #rhs_, opname_,                                             \
        std::source_location::current(), fatal_)

#define PN_CHECK_EQ(lhs_, rhs_) PN_TEST_DETAIL_BINARY(lhs_, rhs_, ==, "!=", false)
#define PN_CHECK_NE(lhs_, rhs_) PN_TEST_DETAIL_BINARY(lhs_, rhs_, !=, "==", false)
#define PN_CHECK_LT(lhs_, rhs_) PN_TEST_DETAIL_BINARY(lhs_, rhs_, <, ">=", false)
#define PN_CHECK_LE(lhs_, rhs_) PN_TEST_DETAIL_BINARY(lhs_, rhs_, <=, ">", false)
#define PN_CHECK_GT(lhs_, rhs_) PN_TEST_DETAIL_BINARY(lhs_, rhs_, >, "<=", false)
#define PN_CHECK_GE(lhs_, rhs_) PN_TEST_DETAIL_BINARY(lhs_, rhs_, >=, "<", false)

#define PN_REQUIRE_EQ(lhs_, rhs_)                                             \
    do {                                                                      \
        if (!PN_TEST_DETAIL_BINARY(lhs_, rhs_, ==, "!=", true)) return;       \
    } while (false)

/// Absolute-tolerance floating-point comparison.
///
/// Deliberately absolute rather than relative: the engine's precision
/// requirements are stated in absolute units (millimetres of position error at
/// a given distance), so tests assert in those terms. Relative comparison would
/// obscure exactly the property being measured.
#define PN_CHECK_NEAR(lhs_, rhs_, tol_)                                              \
    ::pn::testing::detail::compare_near(pn_ctx_, (lhs_), (rhs_), (tol_),              \
                                        #lhs_ " ~= " #rhs_,                           \
                                        std::source_location::current())

#define PN_FAIL(message_)                                                     \
    ::pn::testing::detail::evaluate(pn_ctx_, false, "PN_FAIL", (message_),    \
                                    std::source_location::current(), false)


#endif  // PN_TESTING_TEST_HPP
