// Pention Engine - core/log.hpp
// Requirement: PN-PLT-016 (logging, structured events, assertions with
//              categories and levels)
// Decision:    ADR-0002, ADR-0004

#ifndef PN_CORE_LOG_HPP
#define PN_CORE_LOG_HPP

#include "pn/core/inline_array.hpp"

#include <cstdint>
#include <mutex>
#include <source_location>
#include <span>
#include <string>
#include <string_view>
#include <vector>

namespace pn::core {

enum class LogLevel : std::uint8_t {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
    /// Above every level, so setting a threshold here silences a category.
    Off = 6,
};

constexpr std::string_view to_string(LogLevel level) noexcept {
    switch (level) {
        case LogLevel::Trace:   return "trace";
        case LogLevel::Debug:   return "debug";
        case LogLevel::Info:    return "info";
        case LogLevel::Warning: return "warning";
        case LogLevel::Error:   return "error";
        case LogLevel::Fatal:   return "fatal";
        case LogLevel::Off:     return "off";
    }
    return "unknown";
}

/// One key and value attached to a log record.
///
/// Structured fields rather than a formatted string, and that is the whole
/// point of the design. A message that reads "loaded 1920x1080 in 43ms" has to
/// be parsed back out with a regular expression before anything can chart it or
/// alert on it, and that regular expression breaks the first time someone
/// improves the wording. A field survives rewording.
class LogValue {
public:
    enum class Kind : std::uint8_t { Signed, Unsigned, Real, Boolean, Text };

    LogValue() = default;
    LogValue(std::int64_t value) noexcept : kind_{Kind::Signed}, signed_{value} {}
    LogValue(std::int32_t value) noexcept : kind_{Kind::Signed}, signed_{value} {}
    LogValue(std::uint64_t value) noexcept : kind_{Kind::Unsigned}, unsigned_{value} {}
    LogValue(std::uint32_t value) noexcept : kind_{Kind::Unsigned}, unsigned_{value} {}
    LogValue(double value) noexcept : kind_{Kind::Real}, real_{value} {}
    LogValue(bool value) noexcept : kind_{Kind::Boolean}, boolean_{value} {}
    LogValue(std::string_view value) noexcept : kind_{Kind::Text}, text_{value} {}
    LogValue(const char* value) noexcept : kind_{Kind::Text}, text_{value} {}
    /// Views the string; does not copy it. Safe because a record is emitted
    /// before the end of the statement that built it, so anything named in that
    /// statement is still alive. A sink that keeps the record must copy.
    LogValue(const std::string& value) noexcept : kind_{Kind::Text}, text_{value} {}

    Kind kind() const noexcept { return kind_; }
    std::int64_t as_signed() const noexcept { return signed_; }
    std::uint64_t as_unsigned() const noexcept { return unsigned_; }
    double as_real() const noexcept { return real_; }
    bool as_boolean() const noexcept { return boolean_; }
    std::string_view as_text() const noexcept { return text_; }

    std::string to_text() const;

private:
    Kind kind_ = Kind::Signed;
    std::int64_t signed_ = 0;
    std::uint64_t unsigned_ = 0;
    double real_ = 0.0;
    bool boolean_ = false;
    /// A view, not a copy. Nothing on the emitting path allocates; a sink that
    /// keeps a record must copy what it wants to keep, and the capturing sink
    /// below does.
    std::string_view text_;
};

struct LogField {
    std::string_view name;
    LogValue value;
};

/// The most fields one record can carry.
///
/// A bound rather than a growing list, so emitting a record allocates nothing.
/// A record that wants more than eight fields is usually two records.
inline constexpr std::size_t kMaxLogFields = 8;

struct LogRecord {
    LogLevel level = LogLevel::Info;
    std::string_view category;
    std::string_view message;
    std::span<const LogField> fields;
    std::source_location where{};
};

/// Somewhere records go.
///
/// Every view in a record points at the caller's memory and is valid only for
/// the duration of the call. A sink that stores a record must copy what it
/// keeps; one that formats and writes immediately need not.
class LogSink {
public:
    LogSink() = default;
    LogSink(const LogSink&) = delete;
    LogSink& operator=(const LogSink&) = delete;
    virtual ~LogSink() = default;

    virtual void write(const LogRecord& record) = 0;
};

/// Routes records to sinks, with a threshold per category.
///
/// Sinks are registered by pointer and not owned. A logger that owned its sinks
/// would have to outlive them, and the natural sink - a file the application
/// owns - outlives the logger instead.
class Logger {
public:
    void add_sink(LogSink* sink);
    void remove_sink(LogSink* sink);
    void clear_sinks();

    void set_level(LogLevel level) noexcept { default_level_ = level; }
    LogLevel level() const noexcept { return default_level_; }

    /// Sets a threshold for one category.
    ///
    /// A subsystem can be made verbose without drowning in everything else,
    /// which is the difference between a log that gets read and one that gets
    /// turned off.
    void set_category_level(std::string_view category, LogLevel level);

    LogLevel level_for(std::string_view category) const;

    /// Whether a record at this level and category would reach any sink.
    ///
    /// Checked before the arguments are evaluated, so a disabled trace costs a
    /// comparison rather than the work of computing what it would have said.
    bool enabled(std::string_view category, LogLevel level) const;

    void emit(const LogRecord& record);

    /// How many records this logger has emitted, by level. For a test, and for
    /// a build that wants to fail on any error-level record.
    std::uint64_t count(LogLevel level) const;

private:
    /// Held during dispatch. Logging is not on the frame's hot path, and a
    /// correct log matters more here than a fast one - an asynchronous ring
    /// buffer is a separate piece of work with its own failure modes, and
    /// pretending this one is lock-free would be the wrong trade to make
    /// silently.
    mutable std::mutex mutex_;
    std::vector<LogSink*> sinks_;
    std::vector<std::pair<std::string, LogLevel>> category_levels_;
    LogLevel default_level_ = LogLevel::Info;
    std::uint64_t counts_[7] = {};
};

/// The logger the macros use when none is named.
Logger& default_logger();

/// Accumulates a record's fields and emits it when it goes out of scope.
///
/// The chained form - `.with("width", 1920).with("height", 1080)` - avoids a
/// format string entirely. There is no mismatch between a placeholder and an
/// argument to get wrong, and no parsing to do at the other end.
class LogBuilder {
public:
    LogBuilder(Logger& logger, LogLevel level, std::string_view category,
               std::string_view message, std::source_location where) noexcept
        : logger_{&logger}, level_{level}, category_{category}, message_{message},
          where_{where} {}

    LogBuilder(const LogBuilder&) = delete;
    LogBuilder& operator=(const LogBuilder&) = delete;

    ~LogBuilder() {
        logger_->emit(LogRecord{level_, category_, message_,
                                std::span<const LogField>{fields_.data(), fields_.size()},
                                where_});
    }

    /// Adds a field. Fields past the bound are dropped rather than growing the
    /// record, and `overflowed()` reports that it happened.
    LogBuilder& with(std::string_view name, LogValue value) {
        if (fields_.full()) {
            overflowed_ = true;
            return *this;
        }
        fields_.push_back(LogField{name, value});
        return *this;
    }

    bool overflowed() const noexcept { return overflowed_; }

private:
    Logger* logger_;
    LogLevel level_;
    std::string_view category_;
    std::string_view message_;
    std::source_location where_;
    InlineArray<LogField, kMaxLogFields> fields_;
    bool overflowed_ = false;
};

/// A sink that keeps what it is given, for tests.
///
/// Copies every string out of the record. The views in a record point at the
/// caller's stack and are valid only during the call, so a sink that stored them
/// would hand back dangling views - the exact bug this class exists to not have.
class CapturingLogSink final : public LogSink {
public:
    struct Entry {
        LogLevel level;
        std::string category;
        std::string message;
        std::vector<std::pair<std::string, std::string>> fields;
        std::uint32_t line;
        std::string file;
    };

    void write(const LogRecord& record) override;

    const std::vector<Entry>& entries() const noexcept { return entries_; }
    void clear() noexcept { entries_.clear(); }

    /// The first entry whose message matches, or nullptr.
    const Entry* find(std::string_view message) const;

    std::size_t count(LogLevel level) const;

private:
    std::vector<Entry> entries_;
};

/// A sink that formats records to a stream.
class StreamLogSink final : public LogSink {
public:
    /// `stream` is a `std::FILE*`, taken as `void*` so this header does not drag
    /// `<cstdio>` into everything that logs.
    explicit StreamLogSink(void* stream) noexcept : stream_{stream} {}
    void write(const LogRecord& record) override;

private:
    void* stream_;
};

}  // namespace pn::core

/// The lowest level compiled in. Below it, a log statement is not merely
/// skipped - it is not present, and neither is the work of evaluating its
/// arguments.
#ifndef PN_LOG_MIN_LEVEL
#  ifdef NDEBUG
#    define PN_LOG_MIN_LEVEL 2  // Info
#  else
#    define PN_LOG_MIN_LEVEL 0  // Trace
#  endif
#endif

// The level as a plain integer, so the compile-time comparison is between two
// literals. Comparing `static_cast<int>(LogLevel::Trace) >= 0` instead makes GCC
// warn that the comparison is always true "due to limited range of data type",
// which is correct and unhelpful: the whole point is that it is decided at
// compile time.
#define PN_LOG_DETAIL_LEVEL_Trace 0
#define PN_LOG_DETAIL_LEVEL_Debug 1
#define PN_LOG_DETAIL_LEVEL_Info 2
#define PN_LOG_DETAIL_LEVEL_Warning 3
#define PN_LOG_DETAIL_LEVEL_Error 4
#define PN_LOG_DETAIL_LEVEL_Fatal 5

// The `if (!c) {} else` shape rather than a plain `if (c)`. A bare `if` would
// swallow a following `else` belonging to the caller:
//
//     if (ready) PN_LOG(Info, "a", "b"); else recover();
//
// binds `recover()` to the log statement's hidden `if`, and the caller's
// recovery silently stops running when the log level changes.
#define PN_LOG_DETAIL_TO(logger_, level_, category_, message_)                        \
    if (!(PN_LOG_DETAIL_LEVEL_##level_ >= PN_LOG_MIN_LEVEL &&                         \
          (logger_).enabled((category_), ::pn::core::LogLevel::level_))) {            \
    } else                                                                            \
        ::pn::core::LogBuilder((logger_), ::pn::core::LogLevel::level_, (category_),  \
                               (message_), std::source_location::current())

/// Logs to an explicit logger: `PN_LOG_TO(log, Info, "render", "created device")`.
#define PN_LOG_TO(logger_, level_, category_, message_) \
    PN_LOG_DETAIL_TO(logger_, level_, category_, message_)

/// Logs to the default logger.
#define PN_LOG(level_, category_, message_) \
    PN_LOG_DETAIL_TO(::pn::core::default_logger(), level_, category_, message_)

#endif  // PN_CORE_LOG_HPP
