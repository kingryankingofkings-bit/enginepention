// Pention Engine - core/tests/log_test.cpp
// Requirement: PN-PLT-016
// Decision:    ADR-0002, ADR-0004
//
// PN-PLT-016's criterion names two things: log capture in tests, and an
// assertion that fires with its source location. Both are here.
//
// The second needed a change to make it testable at all: an assertion that only
// ever aborts cannot be observed by the process it aborts. The handler is now
// replaceable, which is also what a crash reporter needs.

// Both compile-time gates are forced on for the bulk of this file, so the
// behaviour is exercised in every configuration rather than only in Debug. A
// Release build otherwise compiles the trace statements and the assertions away,
// and a test that silently checks nothing is worse than one that fails.
//
// Each gate has its own test at the bottom of the file, where it is turned off
// deliberately and the *absence* of an effect is what is checked.
#define PN_ENABLE_ASSERTS 1
#define PN_LOG_MIN_LEVEL 0

#include "pn/core/assert.hpp"
#include "pn/core/log.hpp"
#include "pn/testing/test.hpp"

#include <cstdio>
#include <string>
#include <string_view>
#include <thread>
#include <vector>

namespace {

using pn::core::AssertionFailure;
using pn::core::CapturingLogSink;
using pn::core::Logger;
using pn::core::LogLevel;

/// Installs a sink for the duration of a scope and takes it away again.
///
/// A test that leaves a sink attached feeds every later test's records into its
/// own capture, and the failure appears in whichever test happens to run next.
class ScopedSink {
public:
    ScopedSink(Logger& logger, pn::core::LogSink& sink) : logger_{&logger}, sink_{&sink} {
        logger_->add_sink(sink_);
    }
    ScopedSink(const ScopedSink&) = delete;
    ScopedSink& operator=(const ScopedSink&) = delete;
    ~ScopedSink() { logger_->remove_sink(sink_); }

private:
    Logger* logger_;
    pn::core::LogSink* sink_;
};

}  // namespace

// -------------------------------------------------------------------
// Capture
// -------------------------------------------------------------------

PN_TEST(log, a_record_reaches_a_sink_with_its_level_category_and_message) {
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    PN_LOG_TO(logger, Info, "render", "created device");

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    PN_CHECK(sink.entries()[0].level == LogLevel::Info);
    PN_CHECK(sink.entries()[0].category == "render");
    PN_CHECK(sink.entries()[0].message == "created device");
}

PN_TEST(log, a_record_carries_its_source_location) {
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    const std::uint32_t line = static_cast<std::uint32_t>(__LINE__) + 1;
    PN_LOG_TO(logger, Warning, "assets", "texture missing");

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    PN_CHECK_EQ(sink.entries()[0].line, line);
    PN_CHECK(sink.entries()[0].file.find("log_test.cpp") != std::string::npos);
}

PN_TEST(log, structured_fields_survive_a_rewording_of_the_message) {
    // The whole point of fields. "loaded 1920x1080 in 43ms" has to be parsed
    // back out with a regular expression before anything can chart it, and that
    // expression breaks the first time someone improves the wording.
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    PN_LOG_TO(logger, Info, "render", "swapchain created")
        .with("width", 1920)
        .with("height", 1080)
        .with("vsync", true)
        .with("scale", 1.5)
        .with("adapter", "reference");

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    const auto& fields = sink.entries()[0].fields;
    PN_REQUIRE_EQ(fields.size(), std::size_t{5});
    PN_CHECK(fields[0].first == "width");
    PN_CHECK(fields[0].second == "1920");
    PN_CHECK(fields[2].second == "true");
    PN_CHECK(fields[3].second == "1.5");
    PN_CHECK(fields[4].second == "reference");
}

PN_TEST(log, fields_past_the_bound_are_dropped_and_reported) {
    // A bound rather than a growing list, so emitting allocates nothing. Silence
    // about the overflow would make a record that quietly lost data look like a
    // record that never had it.
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    bool overflowed = false;
    {
        pn::core::LogBuilder builder{logger, LogLevel::Info, "test", "too many",
                                     std::source_location::current()};
        for (int index = 0; index < 12; ++index) {
            builder.with("field", index);
        }
        overflowed = builder.overflowed();
    }

    PN_CHECK(overflowed);
    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    PN_CHECK_EQ(sink.entries()[0].fields.size(), pn::core::kMaxLogFields);
}

PN_TEST(log, a_sink_that_stores_a_record_copies_what_it_keeps) {
    // Every view in a record points at the caller's memory and lives only for
    // the duration of the call. A sink that stored the views would hand back
    // dangling ones - which is why the message below is built in a scope that
    // ends before the check.
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    {
        const std::string message(200, 'm');
        const std::string value(200, 'v');
        PN_LOG_TO(logger, Info, "scoped", message).with("field", value);
    }

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    PN_CHECK_EQ(sink.entries()[0].message.size(), std::size_t{200});
    PN_CHECK(sink.entries()[0].message == std::string(200, 'm'));
    PN_REQUIRE_EQ(sink.entries()[0].fields.size(), std::size_t{1});
    PN_CHECK(sink.entries()[0].fields[0].second == std::string(200, 'v'));
}

// -------------------------------------------------------------------
// Levels and categories
// -------------------------------------------------------------------

PN_TEST(log, records_below_the_threshold_do_not_reach_a_sink) {
    Logger logger;
    logger.set_level(LogLevel::Warning);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    PN_LOG_TO(logger, Info, "render", "not important enough");
    PN_LOG_TO(logger, Warning, "render", "important enough");
    PN_LOG_TO(logger, Error, "render", "more than important enough");

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{2});
    PN_CHECK(sink.entries()[0].message == "important enough");
    PN_CHECK(sink.entries()[1].message == "more than important enough");
}

PN_TEST(log, a_category_can_be_made_verbose_without_the_rest) {
    // The difference between a log that gets read and one that gets turned off.
    Logger logger;
    logger.set_level(LogLevel::Error);
    logger.set_category_level("physics", LogLevel::Trace);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    PN_LOG_TO(logger, Trace, "physics", "solver iteration");
    PN_LOG_TO(logger, Trace, "render", "draw call");
    PN_LOG_TO(logger, Error, "render", "device lost");

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{2});
    PN_CHECK(sink.entries()[0].category == "physics");
    PN_CHECK(sink.entries()[1].message == "device lost");
}

PN_TEST(log, a_category_can_also_be_silenced) {
    Logger logger;
    logger.set_level(LogLevel::Trace);
    logger.set_category_level("noisy", LogLevel::Off);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    PN_LOG_TO(logger, Fatal, "noisy", "even this is suppressed");
    PN_LOG_TO(logger, Trace, "quiet", "but this is not");

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    PN_CHECK(sink.entries()[0].category == "quiet");
}

PN_TEST(log, a_disabled_record_does_not_evaluate_its_arguments) {
    // A trace statement that computes what it would have said, and then throws
    // it away, is a trace statement that costs money in a shipping build.
    Logger logger;
    logger.set_level(LogLevel::Error);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    int evaluations = 0;
    auto expensive = [&evaluations]() -> int {
        ++evaluations;
        return 42;
    };

    PN_LOG_TO(logger, Info, "render", "suppressed").with("value", expensive());
    PN_CHECK_EQ(evaluations, 0);

    PN_LOG_TO(logger, Error, "render", "emitted").with("value", expensive());
    PN_CHECK_EQ(evaluations, 1);
}

PN_TEST(log, counts_are_kept_per_level) {
    // So a build can fail on any error-level record without a sink of its own.
    Logger logger;
    logger.set_level(LogLevel::Trace);

    PN_LOG_TO(logger, Info, "a", "one");
    PN_LOG_TO(logger, Error, "a", "two");
    PN_LOG_TO(logger, Error, "b", "three");

    PN_CHECK_EQ(logger.count(LogLevel::Info), std::uint64_t{1});
    PN_CHECK_EQ(logger.count(LogLevel::Error), std::uint64_t{2});
    PN_CHECK_EQ(logger.count(LogLevel::Warning), std::uint64_t{0});
}

PN_TEST(log, a_sink_can_be_removed_and_stops_receiving) {
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;

    logger.add_sink(&sink);
    PN_LOG_TO(logger, Info, "a", "before");
    logger.remove_sink(&sink);
    PN_LOG_TO(logger, Info, "a", "after");

    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    PN_CHECK(sink.entries()[0].message == "before");
}

PN_TEST(log, adding_the_same_sink_twice_does_not_double_its_records) {
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;
    logger.add_sink(&sink);
    logger.add_sink(&sink);

    PN_LOG_TO(logger, Info, "a", "once");
    PN_CHECK_EQ(sink.entries().size(), std::size_t{1});
    logger.clear_sinks();
}

// -------------------------------------------------------------------
// Concurrency
// -------------------------------------------------------------------

PN_TEST(log, records_from_many_threads_all_arrive) {
    // The job system means logging happens from worker threads. Under
    // ThreadSanitizer this is also the test that says the dispatch is
    // synchronized rather than hoped to be.
    Logger logger;
    logger.set_level(LogLevel::Trace);
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    constexpr int kThreads = 4;
    constexpr int kPerThread = 200;
    std::vector<std::thread> threads;
    for (int index = 0; index < kThreads; ++index) {
        threads.emplace_back([&logger, index] {
            for (int record = 0; record < kPerThread; ++record) {
                PN_LOG_TO(logger, Info, "worker", "tick")
                    .with("thread", index)
                    .with("record", record);
            }
        });
    }
    for (std::thread& thread : threads) {
        thread.join();
    }

    PN_CHECK_EQ(sink.entries().size(), std::size_t{kThreads * kPerThread});
    PN_CHECK_EQ(logger.count(LogLevel::Info), std::uint64_t{kThreads * kPerThread});
}

// -------------------------------------------------------------------
// Formatting
// -------------------------------------------------------------------

PN_TEST(log, the_stream_sink_quotes_text_so_a_line_can_be_parsed_back) {
    // A value containing a space would otherwise read back as two fields. A log
    // nobody can parse is a log nobody can search.
    std::FILE* stream = std::tmpfile();
    PN_REQUIRE(stream != nullptr);

    Logger logger;
    logger.set_level(LogLevel::Trace);
    pn::core::StreamLogSink sink{stream};
    ScopedSink attached{logger, sink};

    PN_LOG_TO(logger, Warning, "assets", "load failed")
        .with("path", "a path with spaces")
        .with("attempts", 3);

    std::fflush(stream);
    std::fseek(stream, 0, SEEK_SET);
    std::string contents;
    char buffer[512];
    while (std::fgets(buffer, sizeof(buffer), stream) != nullptr) {
        contents += buffer;
    }
    std::fclose(stream);

    PN_CHECK(contents.find("[warning]") != std::string::npos);
    PN_CHECK(contents.find("assets: load failed") != std::string::npos);
    PN_CHECK(contents.find("path=\"a path with spaces\"") != std::string::npos);
    PN_CHECK(contents.find("attempts=3") != std::string::npos);
    PN_CHECK(contents.find("log_test.cpp:") != std::string::npos);
}

// -------------------------------------------------------------------
// Assertions
// -------------------------------------------------------------------

namespace {

/// Where a capturing assertion handler puts what it saw.
struct CapturedAssertion {
    bool fired = false;
    std::string expression;
    std::string message;
    std::string file;
    std::uint32_t line = 0;
};

CapturedAssertion g_captured;

void capture_assertion(const AssertionFailure& failure) {
    g_captured.fired = true;
    g_captured.expression = failure.expression != nullptr ? failure.expression : "";
    g_captured.message = failure.message != nullptr ? failure.message : "";
    g_captured.file = failure.where.file_name();
    g_captured.line = failure.where.line();
}

/// Installs the capturing handler and restores the previous one.
///
/// Leaving a capturing handler installed disarms every assertion in every test
/// that runs afterwards, which would turn a real failure into silence.
class ScopedAssertionCapture {
public:
    ScopedAssertionCapture() : previous_{pn::core::set_assertion_handler(&capture_assertion)} {
        g_captured = CapturedAssertion{};
    }
    ScopedAssertionCapture(const ScopedAssertionCapture&) = delete;
    ScopedAssertionCapture& operator=(const ScopedAssertionCapture&) = delete;
    ~ScopedAssertionCapture() { pn::core::set_assertion_handler(previous_); }

private:
    pn::core::AssertionHandler previous_;
};

}  // namespace

PN_TEST(assertion, a_failing_assertion_reports_its_expression_and_source_location) {
    ScopedAssertionCapture capture;

    const int value = 3;
    const std::uint32_t line = static_cast<std::uint32_t>(__LINE__) + 1;
    PN_ASSERT(value > 10);

    PN_REQUIRE(g_captured.fired);
    PN_CHECK(g_captured.expression == "value > 10");
    PN_CHECK_EQ(g_captured.line, line);
    PN_CHECK(g_captured.file.find("log_test.cpp") != std::string::npos);
}

PN_TEST(assertion, a_message_is_carried_when_one_is_given) {
    ScopedAssertionCapture capture;

    const std::size_t index = 9;
    const std::size_t size = 4;
    PN_ASSERT_MSG(index < size, "index out of range");

    PN_REQUIRE(g_captured.fired);
    PN_CHECK(g_captured.expression == "index < size");
    PN_CHECK(g_captured.message == "index out of range");
}

PN_TEST(assertion, a_passing_assertion_does_not_fire) {
    ScopedAssertionCapture capture;
    const int value = 42;
    PN_ASSERT(value == 42);
    PN_CHECK(!g_captured.fired);
}

PN_TEST(assertion, the_handler_is_restored_when_the_scope_ends) {
    // The property that keeps one test from disarming the next.
    const pn::core::AssertionHandler before = pn::core::set_assertion_handler(nullptr);
    pn::core::set_assertion_handler(before);
    {
        ScopedAssertionCapture capture;
        PN_ASSERT(false);
        PN_CHECK(g_captured.fired);
    }
    const pn::core::AssertionHandler after = pn::core::set_assertion_handler(nullptr);
    pn::core::set_assertion_handler(after);
    PN_CHECK(before == after);
}


// -------------------------------------------------------------------
// The compile-time gates, turned off on purpose
// -------------------------------------------------------------------

// The log macros read PN_LOG_MIN_LEVEL where they are expanded, so redefining it
// here changes what the statements below compile to without touching anything
// above.
#undef PN_LOG_MIN_LEVEL
#define PN_LOG_MIN_LEVEL 4  // Error

PN_TEST(log, statements_below_the_compiled_minimum_are_not_present_at_all) {
    // Not "skipped at runtime": absent. A trace statement that still calls
    // enabled() and evaluates nothing is cheap; one that is not compiled costs
    // nothing at all, and in a shipping build that is the difference the setting
    // exists to make.
    Logger logger;
    logger.set_level(LogLevel::Trace);  // permissive at runtime, so only the
                                        // compile-time gate can suppress
    CapturingLogSink sink;
    ScopedSink attached{logger, sink};

    int evaluations = 0;
    auto counted = [&evaluations]() -> int {
        ++evaluations;
        return 1;
    };

    PN_LOG_TO(logger, Info, "gated", "compiled out").with("value", counted());
    PN_LOG_TO(logger, Warning, "gated", "also compiled out").with("value", counted());
    PN_CHECK_EQ(evaluations, 0);
    PN_CHECK(sink.entries().empty());

    PN_LOG_TO(logger, Error, "gated", "kept").with("value", counted());
    PN_CHECK_EQ(evaluations, 1);
    PN_REQUIRE_EQ(sink.entries().size(), std::size_t{1});
    PN_CHECK(sink.entries()[0].message == "kept");
}

// Re-include the assertion header with the gate off. Its macros live outside its
// include guard precisely so this works, which is how the disabled form gets
// tested at all rather than by writing a copy of it here.
#undef PN_ENABLE_ASSERTS
#define PN_ENABLE_ASSERTS 0
#include "pn/core/assert.hpp"

PN_TEST(assertion, a_disabled_assertion_does_not_evaluate_its_expression) {
    // The Release contract. An assertion that still called its condition would
    // put the cost of every check into a build that has decided not to make
    // them - and worse, would make a condition with a side effect behave
    // differently between configurations.
    ScopedAssertionCapture capture;

    int evaluations = 0;
    auto counted = [&evaluations]() -> bool {
        ++evaluations;
        return false;
    };

    PN_ASSERT(counted());
    PN_ASSERT_MSG(counted(), "not evaluated either");

    PN_CHECK_EQ(evaluations, 0);
    PN_CHECK(!g_captured.fired);
}
