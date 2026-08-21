// Pention Engine - core/error.hpp
// Requirement: PN-PLT-027 (fallible-return convention), PN-PLT-016 (diagnostics)
// Decision:    ADR-0002

#ifndef PN_CORE_ERROR_HPP
#define PN_CORE_ERROR_HPP

#include <cstdint>
#include <source_location>
#include <string_view>

namespace pn::core {

/// Broad classification of a failure.
///
/// Coarse on purpose. The category answers "who should handle this", which is
/// the question a caller several frames up actually needs answered; the
/// specific code answers "what exactly happened" for diagnostics.
enum class ErrorCategory : std::uint16_t {
    none = 0,
    invalid_argument,   ///< the caller passed something the API cannot accept
    out_of_range,       ///< an index or key outside the valid domain
    not_found,          ///< a lookup that legitimately found nothing
    already_exists,     ///< a create that collided with an existing entry
    out_of_memory,      ///< an allocation failed
    io,                 ///< a filesystem or device operation failed
    corrupt_data,       ///< input parsed but is internally inconsistent
    unsupported,        ///< a capability this build or device does not provide
    invalid_state,      ///< the operation is valid but not right now
    internal,           ///< an engine invariant was violated
};

[[nodiscard]] constexpr std::string_view to_string(ErrorCategory category) noexcept {
    switch (category) {
        case ErrorCategory::none:             return "none";
        case ErrorCategory::invalid_argument: return "invalid_argument";
        case ErrorCategory::out_of_range:     return "out_of_range";
        case ErrorCategory::not_found:        return "not_found";
        case ErrorCategory::already_exists:   return "already_exists";
        case ErrorCategory::out_of_memory:    return "out_of_memory";
        case ErrorCategory::io:               return "io";
        case ErrorCategory::corrupt_data:     return "corrupt_data";
        case ErrorCategory::unsupported:      return "unsupported";
        case ErrorCategory::invalid_state:    return "invalid_state";
        case ErrorCategory::internal:         return "internal";
    }
    return "unknown";
}

/// The engine's default error payload.
///
/// Deliberately allocation-free and trivially copyable. An error type that
/// allocates cannot be returned from an out-of-memory path, which is precisely
/// the path that most needs to report a failure. The message is a
/// `string_view` over a string literal with static storage duration, never an
/// owned buffer.
class Error {
public:
    constexpr Error() = default;

    constexpr Error(ErrorCategory category,
                    std::string_view message,
                    std::source_location where = std::source_location::current()) noexcept
        : category_(category), message_(message), where_(where) {}

    constexpr Error(ErrorCategory category,
                    std::string_view message,
                    std::uint32_t code,
                    std::source_location where = std::source_location::current()) noexcept
        : category_(category), code_(code), message_(message), where_(where) {}

    [[nodiscard]] constexpr ErrorCategory category() const noexcept { return category_; }
    [[nodiscard]] constexpr std::uint32_t code() const noexcept { return code_; }
    [[nodiscard]] constexpr std::string_view message() const noexcept { return message_; }
    [[nodiscard]] constexpr const std::source_location& where() const noexcept { return where_; }

    [[nodiscard]] constexpr bool is(ErrorCategory category) const noexcept {
        return category_ == category;
    }

private:
    ErrorCategory category_ = ErrorCategory::internal;
    std::uint32_t code_ = 0;
    std::string_view message_ = "unspecified error";
    std::source_location where_{};
};

}  // namespace pn::core

#endif  // PN_CORE_ERROR_HPP
