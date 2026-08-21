// Pention Engine - core/string.hpp
// Requirement: PN-PLT-010 (string, interned name, and handle systems)
// Decision:    ADR-0002 (Expected), ADR-0004

#ifndef PN_CORE_STRING_HPP
#define PN_CORE_STRING_HPP

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"

#include <cstddef>
#include <cstdint>
#include <limits>
#include <string_view>

namespace pn::core {

/// ASCII lowercase. Not locale-aware, deliberately.
///
/// Engine identifiers - asset paths, parameter names, console commands - are
/// ASCII by construction, and a locale-aware fold would make the same config
/// file parse differently on a Turkish system, where uppercase `i` is not `I`.
/// That class of bug is very hard to reproduce and very easy to avoid.
constexpr char to_lower_ascii(char character) noexcept {
    return (character >= 'A' && character <= 'Z')
               ? static_cast<char>(character - 'A' + 'a')
               : character;
}

constexpr bool equals_ignore_case(std::string_view left, std::string_view right) noexcept {
    if (left.size() != right.size()) {
        return false;
    }
    for (std::size_t index = 0; index < left.size(); ++index) {
        if (to_lower_ascii(left[index]) != to_lower_ascii(right[index])) {
            return false;
        }
    }
    return true;
}

constexpr bool is_space_ascii(char character) noexcept {
    return character == ' ' || character == '\t' || character == '\n' ||
           character == '\r' || character == '\f' || character == '\v';
}

constexpr std::string_view trim_front(std::string_view text) noexcept {
    std::size_t start = 0;
    while (start < text.size() && is_space_ascii(text[start])) {
        ++start;
    }
    return text.substr(start);
}

constexpr std::string_view trim_back(std::string_view text) noexcept {
    std::size_t end = text.size();
    while (end > 0 && is_space_ascii(text[end - 1])) {
        --end;
    }
    return text.substr(0, end);
}

constexpr std::string_view trim(std::string_view text) noexcept {
    return trim_back(trim_front(text));
}

/// Calls `visit` with each field of `text` separated by `delimiter`.
///
/// Empty fields are preserved. `a,,b` is three fields, not two: in a
/// comma-separated record an empty field is a value, and collapsing it shifts
/// every column after it.
template <typename Visitor>
constexpr void split(std::string_view text, char delimiter, Visitor&& visit) {
    std::size_t start = 0;
    while (true) {
        const std::size_t position = text.find(delimiter, start);
        if (position == std::string_view::npos) {
            visit(text.substr(start));
            return;
        }
        visit(text.substr(start, position - start));
        start = position + 1;
    }
}

/// Parses a signed decimal integer.
///
/// Returns an error rather than a sentinel, because every sentinel a parser
/// could return is also a number someone will legitimately write in a config
/// file. Rejects an empty string, trailing text, and any value outside the
/// type's range - the last of which `strtoll` reports through `errno`, which
/// callers forget to check.
constexpr Expected<std::int64_t> parse_int64(std::string_view text) noexcept {
    if (text.empty()) {
        return fail(ErrorCategory::invalid_argument, "empty string is not a number");
    }

    std::size_t index = 0;
    bool negative = false;
    if (text[0] == '+' || text[0] == '-') {
        negative = text[0] == '-';
        index = 1;
        if (index == text.size()) {
            return fail(ErrorCategory::invalid_argument, "sign with no digits");
        }
    }

    // Accumulated as unsigned so the check happens before the overflow rather
    // than after it: signed overflow is undefined, so a test written as
    // "value * 10 + digit < value" is a test the optimiser is entitled to
    // delete.
    constexpr std::uint64_t kPositiveLimit =
        static_cast<std::uint64_t>(std::numeric_limits<std::int64_t>::max());
    const std::uint64_t limit = negative ? kPositiveLimit + 1 : kPositiveLimit;

    std::uint64_t magnitude = 0;
    for (; index < text.size(); ++index) {
        const char character = text[index];
        if (character < '0' || character > '9') {
            return fail(ErrorCategory::invalid_argument,
                        "unexpected character in number");
        }
        const std::uint64_t digit = static_cast<std::uint64_t>(character - '0');
        if (magnitude > (limit - digit) / 10) {
            return fail(ErrorCategory::out_of_range, "number does not fit in 64 bits");
        }
        magnitude = magnitude * 10 + digit;
    }

    if (negative) {
        // Negating through unsigned, because `-static_cast<int64_t>(magnitude)`
        // is undefined for the one value whose magnitude is INT64_MAX + 1.
        return static_cast<std::int64_t>(~magnitude + 1);
    }
    return static_cast<std::int64_t>(magnitude);
}

}  // namespace pn::core

#endif  // PN_CORE_STRING_HPP
