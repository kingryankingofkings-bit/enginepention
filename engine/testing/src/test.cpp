// Pention Engine -  testing/test.cpp
// Requirement: PN-PLT-026 -  in-project unit test framework
// Decision:    ADR-0001

#include "pn/testing/test.hpp"

#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <sstream>
#include <cstring>
#include <string_view>
#include <vector>

namespace pn::testing {
namespace {

/// Function-local static so registration order does not depend on the order in
/// which translation units are initialized. A namespace-scope container could
/// be constructed after the first AutoRegister runs.
std::vector<TestCase>& registry() {
    static std::vector<TestCase> tests;
    return tests;
}

constexpr std::string_view kGreen = "\033[32m";
constexpr std::string_view kRed = "\033[31m";
constexpr std::string_view kDim = "\033[2m";
constexpr std::string_view kReset = "\033[0m";

bool colour_enabled() {
    static const bool enabled = [] {
        const char* term = std::getenv("TERM");
        if (std::getenv("NO_COLOR") != nullptr) return false;
        return term != nullptr && std::strcmp(term, "dumb") != 0;
    }();
    return enabled;
}

std::string_view paint(std::string_view code) {
    return colour_enabled() ? code : std::string_view{};
}

void write_out(const std::string& text) {
    std::fwrite(text.data(), 1, text.size(), stdout);
}

/// Trims the repository prefix so failure output stays readable.
std::string_view short_path(const char* path) {
    std::string_view view{path};
    const std::size_t marker = view.rfind("engine/");
    return marker == std::string_view::npos ? view : view.substr(marker);
}

}  // namespace

void TestContext::record_failure(std::string_view expression,
                                 std::string_view detail,
                                 std::source_location where,
                                 bool fatal) {
    ++outcome_.checks_run;
    ++outcome_.checks_failed;
    if (fatal) {
        outcome_.aborted = true;
    }

    if (quiet_) {
        return;
    }

    // Composed through a stream rather than printf: source_location::line()
    // returns uint_least32_t, whose underlying type varies by platform, so no
    // single printf conversion specifier is correct everywhere.
    std::ostringstream message;
    message << "      " << paint(kRed) << (fatal ? "REQUIRE" : "CHECK") << paint(kReset)
            << ' ' << short_path(where.file_name()) << ':' << where.line() << '\n'
            << "        expression: " << expression << '\n';
    if (!detail.empty()) {
        message << "        actual:     " << detail << '\n';
    }
    const std::string text = message.str();
    std::fwrite(text.data(), 1, text.size(), stderr);
}

void register_test(const TestCase& test_case) {
    registry().push_back(test_case);
}

TestOutcome run_one(const TestCase& test_case, bool quiet) {
    TestContext ctx{quiet};
    test_case.function(ctx);
    return ctx.outcome();
}

namespace detail {

bool evaluate(TestContext& ctx,
              bool condition,
              std::string_view expression,
              std::string_view detail,
              std::source_location where,
              bool fatal) {
    if (condition) {
        ctx.record_pass();
        return true;
    }
    ctx.record_failure(expression, detail, where, fatal);
    return false;
}

}  // namespace detail

int run_all(int argc, char** argv) {
    std::string_view filter;
    bool list_only = false;

    for (int i = 1; i < argc; ++i) {
        const std::string_view arg{argv[i]};
        if (arg.starts_with("--filter=")) {
            filter = arg.substr(std::strlen("--filter="));
        } else if (arg == "--list") {
            list_only = true;
        } else {
            std::fprintf(stderr, "unknown argument: %.*s\n",
                         static_cast<int>(arg.size()), arg.data());
            return 2;
        }
    }

    auto& tests = registry();
    std::sort(tests.begin(), tests.end(), [](const TestCase& a, const TestCase& b) {
        const int suite_order = std::strcmp(a.suite, b.suite);
        return suite_order != 0 ? suite_order < 0 : std::strcmp(a.name, b.name) < 0;
    });

    if (list_only) {
        std::ostringstream listing;
        for (const TestCase& test_case : tests) {
            listing << test_case.suite << '.' << test_case.name << '\n';
        }
        write_out(listing.str());
        return 0;
    }

    std::size_t selected = 0;
    std::size_t failed = 0;
    std::size_t total_checks = 0;
    const char* current_suite = nullptr;

    for (const TestCase& test_case : tests) {
        std::string qualified{test_case.suite};
        qualified += '.';
        qualified += test_case.name;

        if (!filter.empty() && qualified.find(filter) == std::string::npos) {
            continue;
        }
        ++selected;

        if (current_suite == nullptr || std::strcmp(current_suite, test_case.suite) != 0) {
            current_suite = test_case.suite;
            std::ostringstream heading;
            heading << '\n' << "  " << paint(kDim) << test_case.suite << paint(kReset) << '\n';
            write_out(heading.str());
        }

        const TestOutcome outcome = run_one(test_case, false);
        total_checks += outcome.checks_run;

        std::ostringstream line;
        if (outcome.passed()) {
            line << "    " << paint(kGreen) << "PASS" << paint(kReset) << ' ' << test_case.name
                 << ' ' << paint(kDim) << '(' << outcome.checks_run << " checks)"
                 << paint(kReset) << '\n';
        } else {
            ++failed;
            line << "    " << paint(kRed) << "FAIL" << paint(kReset) << ' ' << test_case.name
                 << ' ' << paint(kDim) << '(' << outcome.checks_failed << " of "
                 << outcome.checks_run << " checks failed"
                 << (outcome.aborted ? ", aborted" : "") << ')' << paint(kReset) << '\n';
        }
        write_out(line.str());
    }

    std::ostringstream summary;
    summary << '\n' << (failed == 0 ? "PASSED" : "FAILED") << ": "
            << selected << (selected == 1 ? " test, " : " tests, ")
            << total_checks << (total_checks == 1 ? " check, " : " checks, ")
            << failed << (failed == 1 ? " failure" : " failures") << '\n';
    write_out(summary.str());

    if (selected == 0) {
        std::fprintf(stderr,
                     "no tests selected -  treating as failure so an empty run is never green\n");
        return 1;
    }

    return failed == 0 ? 0 : 1;
}

}  // namespace pn::testing
