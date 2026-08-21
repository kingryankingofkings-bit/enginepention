// Pention Engine - core/src/log.cpp
// Requirement: PN-PLT-016
// Decision:    ADR-0004

#include "pn/core/log.hpp"

#include <algorithm>
#include <cstdio>

namespace pn::core {
namespace {

std::string real_to_text(double value) {
    char buffer[40];
    const int written = std::snprintf(buffer, sizeof(buffer), "%.17g", value);
    return written > 0 ? std::string{buffer, static_cast<std::size_t>(written)}
                       : std::string{"0"};
}

}  // namespace

std::string LogValue::to_text() const {
    switch (kind_) {
        case Kind::Signed:   return std::to_string(signed_);
        case Kind::Unsigned: return std::to_string(unsigned_);
        case Kind::Real:     return real_to_text(real_);
        case Kind::Boolean:  return boolean_ ? "true" : "false";
        case Kind::Text:     return std::string{text_};
    }
    return {};
}

void Logger::add_sink(LogSink* sink) {
    if (sink == nullptr) {
        return;
    }
    const std::lock_guard<std::mutex> guard{mutex_};
    if (std::find(sinks_.begin(), sinks_.end(), sink) == sinks_.end()) {
        sinks_.push_back(sink);
    }
}

void Logger::remove_sink(LogSink* sink) {
    const std::lock_guard<std::mutex> guard{mutex_};
    sinks_.erase(std::remove(sinks_.begin(), sinks_.end(), sink), sinks_.end());
}

void Logger::clear_sinks() {
    const std::lock_guard<std::mutex> guard{mutex_};
    sinks_.clear();
}

void Logger::set_category_level(std::string_view category, LogLevel level) {
    const std::lock_guard<std::mutex> guard{mutex_};
    for (auto& entry : category_levels_) {
        if (entry.first == category) {
            entry.second = level;
            return;
        }
    }
    category_levels_.emplace_back(std::string{category}, level);
}

LogLevel Logger::level_for(std::string_view category) const {
    const std::lock_guard<std::mutex> guard{mutex_};
    for (const auto& entry : category_levels_) {
        if (entry.first == category) {
            return entry.second;
        }
    }
    return default_level_;
}

bool Logger::enabled(std::string_view category, LogLevel level) const {
    return static_cast<int>(level) >= static_cast<int>(level_for(category));
}

void Logger::emit(const LogRecord& record) {
    const std::lock_guard<std::mutex> guard{mutex_};
    ++counts_[static_cast<std::size_t>(record.level)];
    for (LogSink* sink : sinks_) {
        sink->write(record);
    }
}

std::uint64_t Logger::count(LogLevel level) const {
    const std::lock_guard<std::mutex> guard{mutex_};
    return counts_[static_cast<std::size_t>(level)];
}

Logger& default_logger() {
    // Function-local, so it is constructed on first use and cannot be caught by
    // static initialisation order - a logger that other globals want to use
    // during their own construction has to exist by then.
    static Logger logger;
    return logger;
}

void CapturingLogSink::write(const LogRecord& record) {
    Entry entry;
    entry.level = record.level;
    entry.category = std::string{record.category};
    entry.message = std::string{record.message};
    entry.line = record.where.line();
    entry.file = record.where.file_name();
    entry.fields.reserve(record.fields.size());
    for (const LogField& field : record.fields) {
        entry.fields.emplace_back(std::string{field.name}, field.value.to_text());
    }
    entries_.push_back(std::move(entry));
}

const CapturingLogSink::Entry* CapturingLogSink::find(std::string_view message) const {
    for (const Entry& entry : entries_) {
        if (entry.message == message) {
            return &entry;
        }
    }
    return nullptr;
}

std::size_t CapturingLogSink::count(LogLevel level) const {
    std::size_t total = 0;
    for (const Entry& entry : entries_) {
        if (entry.level == level) {
            ++total;
        }
    }
    return total;
}

void StreamLogSink::write(const LogRecord& record) {
    auto* stream = static_cast<std::FILE*>(stream_);
    if (stream == nullptr) {
        return;
    }

    std::string line;
    line += '[';
    line += to_string(record.level);
    line += "] ";
    if (!record.category.empty()) {
        line += record.category;
        line += ": ";
    }
    line += record.message;

    for (const LogField& field : record.fields) {
        line += ' ';
        line += field.name;
        line += '=';
        // Text values are quoted so a value containing a space cannot be read
        // back as two fields. A log nobody can parse is a log nobody can search.
        if (field.value.kind() == LogValue::Kind::Text) {
            line += '"';
            for (char character : field.value.as_text()) {
                if (character == '"' || character == '\\') {
                    line += '\\';
                }
                line += character;
            }
            line += '"';
        } else {
            line += field.value.to_text();
        }
    }

    // Source location last, so the message reads first.
    line += "  (";
    line += record.where.file_name();
    line += ':';
    line += std::to_string(record.where.line());
    line += ")\n";

    std::fwrite(line.data(), 1, line.size(), stream);
}

}  // namespace pn::core
