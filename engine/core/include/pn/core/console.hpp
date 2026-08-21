// Pention Engine - core/console.hpp
// Requirement: PN-PLT-019 (configuration, console variables, command execution)
// Decision:    ADR-0002 (Expected), ADR-0004

#ifndef PN_CORE_CONSOLE_HPP
#define PN_CORE_CONSOLE_HPP

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"
#include "pn/core/hash_map.hpp"
#include "pn/core/string.hpp"

#include <cstddef>
#include <cstdint>
#include <functional>
#include <span>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace pn::core {

/// What a console variable holds.
enum class CVarType : std::uint8_t { Boolean, Integer, Real, Text };

constexpr std::string_view to_string(CVarType type) noexcept {
    switch (type) {
        case CVarType::Boolean: return "boolean";
        case CVarType::Integer: return "integer";
        case CVarType::Real:    return "real";
        case CVarType::Text:    return "text";
    }
    return "unknown";
}

/// A console variable's value.
///
/// A small tagged union rather than a string that is parsed on every read. A
/// variable read once per frame per object should not re-parse its own text, and
/// a type that is checked at assignment cannot be wrong at the point of use.
class CVarValue {
public:
    CVarValue() = default;
    explicit CVarValue(bool value) : type_{CVarType::Boolean}, boolean_{value} {}
    explicit CVarValue(std::int64_t value) : type_{CVarType::Integer}, integer_{value} {}
    explicit CVarValue(double value) : type_{CVarType::Real}, real_{value} {}
    explicit CVarValue(std::string value) : type_{CVarType::Text}, text_{std::move(value)} {}

    CVarType type() const noexcept { return type_; }
    bool as_boolean() const noexcept { return boolean_; }
    std::int64_t as_integer() const noexcept { return integer_; }
    double as_real() const noexcept { return real_; }
    const std::string& as_text() const noexcept { return text_; }

    bool operator==(const CVarValue& other) const noexcept {
        if (type_ != other.type_) {
            return false;
        }
        switch (type_) {
            case CVarType::Boolean: return boolean_ == other.boolean_;
            case CVarType::Integer: return integer_ == other.integer_;
            // Compared by value, so a variable set to the same number it started
            // with is not persisted. NaN is not equal to itself, which means a
            // NaN default would always be written out - correct, if surprising,
            // and no engine setting should have one.
            case CVarType::Real:    return real_ == other.real_;
            case CVarType::Text:    return text_ == other.text_;
        }
        return false;
    }

    /// The canonical text form, which is what a config file stores.
    std::string to_text() const;

private:
    CVarType type_ = CVarType::Boolean;
    bool boolean_ = false;
    std::int64_t integer_ = 0;
    double real_ = 0.0;
    std::string text_;
};

/// How a variable may be used.
struct CVarFlags {
    /// Set at startup and fixed thereafter - a resolution chosen before the
    /// device was created, say.
    bool read_only = false;
    /// Refused unless cheats are enabled, so a shipped build cannot be talked
    /// into god mode through the console.
    bool cheat = false;
    /// Written to the config file when it differs from its default.
    bool persist = true;
};

/// A named, typed, documented setting.
struct CVar {
    std::string name;
    std::string description;
    CVarValue value;
    CVarValue default_value;
    CVarFlags flags;

    bool is_default() const noexcept { return value == default_value; }
};

/// One token of a command line, with where it came from.
struct Token {
    std::string text;
    /// Zero-based column of the token's first character in the original line.
    std::size_t column = 0;
    /// True when the token was written in quotes, so `""` is a real empty
    /// argument rather than nothing at all.
    bool quoted = false;
};

/// Splits a command line into tokens.
///
/// Errors carry the column in [`Error::code`], because [`Error`] holds a
/// non-owning `string_view` message and cannot build one that names the
/// position. A message that says only "parse error" is not actionable; a
/// message plus a column is.
Expected<std::vector<Token>> tokenize(std::string_view line);

/// The registry of variables and commands, and the thing that runs a line.
class Console {
public:
    /// A command's handler. Arguments exclude the command name itself.
    using Handler = std::function<Expected<void>(std::span<const Token>)>;

    /// Registers a variable. Returns false if the name is already taken.
    bool register_cvar(std::string name, CVarValue default_value, std::string description,
                       CVarFlags flags = {});

    bool register_command(std::string name, std::string description, Handler handler);

    CVar* find_cvar(std::string_view name) noexcept;
    const CVar* find_cvar(std::string_view name) const noexcept;

    /// Assigns from text, converting to the variable's declared type.
    ///
    /// The type is never changed by an assignment. A console that let
    /// `set maxfps banana` turn an integer into text would move the failure from
    /// the assignment to whatever reads it next frame.
    Expected<void> set_from_text(std::string_view name, std::string_view text);

    /// Runs one line: a command invocation, or `name value` to assign, or `name`
    /// to report a variable's current value.
    Expected<void> execute(std::string_view line);

    /// Whether cheat-flagged variables may be changed.
    void allow_cheats(bool allowed) noexcept { cheats_allowed_ = allowed; }
    bool cheats_allowed() const noexcept { return cheats_allowed_; }

    /// Output produced by the last executed line, for a UI or a test to read.
    const std::vector<std::string>& output() const noexcept { return output_; }
    void clear_output() noexcept { output_.clear(); }

    /// Writes the variables worth saving, in a stable order.
    ///
    /// Only those that are flagged to persist **and** differ from their default.
    /// Writing every variable would freeze today's defaults into every user's
    /// config file: a later release that improves a default would not reach
    /// anyone who had ever launched the game.
    std::string save_config() const;

    /// Applies a config file, collecting every problem rather than stopping.
    ///
    /// A single bad line should not discard the rest of someone's settings, and
    /// reporting only the first means fixing a config file takes as many
    /// attempts as it has mistakes.
    struct LoadReport {
        std::size_t applied = 0;
        /// One entry per rejected line: its number, and why.
        std::vector<std::pair<std::size_t, std::string>> problems;
    };
    LoadReport load_config(std::string_view text);

    std::vector<const CVar*> all_cvars() const;

private:
    Expected<void> assign(CVar& cvar, std::string_view text);
    /// Rebuilds the name indices after the backing vectors may have moved.
    ///
    /// Every key in the index is a `string_view` into a string the vector owns,
    /// so a reallocation leaves them all dangling. Appending to the index alone
    /// would be a use-after-free that appears only once enough registrations
    /// force a growth - late, and far from its cause.
    void reindex_cvars();
    void reindex_commands();

    std::vector<CVar> cvars_;
    HashMap<std::string_view, std::size_t> cvar_index_;
    std::vector<std::pair<std::string, Handler>> commands_;
    HashMap<std::string_view, std::size_t> command_index_;
    std::vector<std::string> output_;
    bool cheats_allowed_ = false;
};

}  // namespace pn::core

#endif  // PN_CORE_CONSOLE_HPP
