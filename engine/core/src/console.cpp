// Pention Engine - core/src/console.cpp
// Requirement: PN-PLT-019
// Decision:    ADR-0002, ADR-0004

#include "pn/core/console.hpp"

#include <algorithm>
#include <cstdio>

namespace pn::core {
namespace {

/// Renders a double so that reading it back gives the same value.
///
/// Seventeen significant digits is the shortest count that round-trips every
/// double. Fewer is prettier and loses the last bits, which for a config file
/// means a value drifting slightly every time it is saved and loaded.
std::string real_to_text(double value) {
    char buffer[40];
    const int written = std::snprintf(buffer, sizeof(buffer), "%.17g", value);
    if (written <= 0) {
        return "0";
    }
    return std::string{buffer, static_cast<std::size_t>(written)};
}

/// Parses a real number without the locale sensitivity of `strtod`.
///
/// A locale that uses a comma for the decimal point would otherwise make the
/// same config file mean different things on different machines - and the
/// failure is silent, because `1,5` parses as `1`.
Expected<double> parse_real(std::string_view text) {
    if (text.empty()) {
        return fail(ErrorCategory::invalid_argument, "empty string is not a number");
    }

    std::size_t index = 0;
    double sign = 1.0;
    if (text[index] == '+' || text[index] == '-') {
        sign = text[index] == '-' ? -1.0 : 1.0;
        ++index;
    }

    bool saw_digit = false;
    double whole = 0.0;
    while (index < text.size() && text[index] >= '0' && text[index] <= '9') {
        whole = whole * 10.0 + static_cast<double>(text[index] - '0');
        ++index;
        saw_digit = true;
    }

    double fraction = 0.0;
    double scale = 1.0;
    if (index < text.size() && text[index] == '.') {
        ++index;
        while (index < text.size() && text[index] >= '0' && text[index] <= '9') {
            scale *= 10.0;
            fraction = fraction * 10.0 + static_cast<double>(text[index] - '0');
            ++index;
            saw_digit = true;
        }
    }

    if (!saw_digit) {
        return fail(ErrorCategory::invalid_argument, "no digits in number");
    }

    double value = whole + fraction / scale;

    if (index < text.size() && (text[index] == 'e' || text[index] == 'E')) {
        ++index;
        PN_TRY_ASSIGN(const std::int64_t exponent, parse_int64(text.substr(index)));
        index = text.size();
        // Repeated multiplication rather than pow, so the result does not depend
        // on which libm is linked. Config exponents are small.
        const std::int64_t steps = exponent < 0 ? -exponent : exponent;
        if (steps > 308) {
            return fail(ErrorCategory::out_of_range, "exponent is out of range");
        }
        for (std::int64_t step = 0; step < steps; ++step) {
            value = exponent < 0 ? value / 10.0 : value * 10.0;
        }
    }

    if (index != text.size()) {
        return fail(ErrorCategory::invalid_argument, "unexpected character in number");
    }
    return sign * value;
}

Expected<bool> parse_boolean(std::string_view text) {
    if (equals_ignore_case(text, "true") || text == "1" || equals_ignore_case(text, "on") ||
        equals_ignore_case(text, "yes")) {
        return true;
    }
    if (equals_ignore_case(text, "false") || text == "0" || equals_ignore_case(text, "off") ||
        equals_ignore_case(text, "no")) {
        return false;
    }
    return fail(ErrorCategory::invalid_argument,
                "expected true/false, on/off, yes/no, or 1/0");
}

}  // namespace

std::string CVarValue::to_text() const {
    switch (type_) {
        case CVarType::Boolean: return boolean_ ? "true" : "false";
        case CVarType::Integer: return std::to_string(integer_);
        case CVarType::Real:    return real_to_text(real_);
        case CVarType::Text:    return text_;
    }
    return {};
}

Expected<std::vector<Token>> tokenize(std::string_view line) {
    std::vector<Token> tokens;
    std::size_t index = 0;

    while (index < line.size()) {
        while (index < line.size() && is_space_ascii(line[index])) {
            ++index;
        }
        if (index >= line.size()) {
            break;
        }
        // A comment runs to the end of the line, so a config file can explain
        // itself.
        if (line[index] == '#') {
            break;
        }

        Token token;
        token.column = index;

        if (line[index] == '"') {
            token.quoted = true;
            ++index;
            bool closed = false;
            while (index < line.size()) {
                if (line[index] == '\\' && index + 1 < line.size()) {
                    const char escaped = line[index + 1];
                    switch (escaped) {
                        case 'n':  token.text.push_back('\n'); break;
                        case 't':  token.text.push_back('\t'); break;
                        case '"':  token.text.push_back('"');  break;
                        case '\\': token.text.push_back('\\'); break;
                        default:
                            return Unexpected{Error{ErrorCategory::invalid_argument,
                                                   "unknown escape sequence",
                                                   static_cast<std::uint32_t>(index)}};
                    }
                    index += 2;
                    continue;
                }
                if (line[index] == '"') {
                    ++index;
                    closed = true;
                    break;
                }
                token.text.push_back(line[index]);
                ++index;
            }
            if (!closed) {
                // The column is the opening quote, not the end of the line: the
                // mistake is where the string started, and that is where a
                // reader has to look.
                return Unexpected{Error{ErrorCategory::invalid_argument,
                                       "unterminated quoted string",
                                       static_cast<std::uint32_t>(token.column)}};
            }
        } else {
            while (index < line.size() && !is_space_ascii(line[index]) && line[index] != '#') {
                token.text.push_back(line[index]);
                ++index;
            }
        }

        tokens.push_back(std::move(token));
    }

    return tokens;
}

bool Console::register_cvar(std::string name, CVarValue default_value,
                            std::string description, CVarFlags flags) {
    if (cvar_index_.contains(name) || command_index_.contains(name)) {
        return false;
    }
    cvars_.push_back(CVar{std::move(name), std::move(description), default_value,
                          std::move(default_value), flags});
    // Rebuilt rather than appended to, because the vector may have reallocated
    // and every key in the index is a view into a string it owns. Appending
    // alone would leave the earlier views dangling - a use-after-free that only
    // appears once enough variables are registered to force a growth.
    reindex_cvars();
    return true;
}

bool Console::register_command(std::string name, std::string description, Handler handler) {
    if (cvar_index_.contains(name) || command_index_.contains(name)) {
        return false;
    }
    (void)description;
    commands_.emplace_back(std::move(name), std::move(handler));
    reindex_commands();
    return true;
}

void Console::reindex_cvars() {
    cvar_index_.clear();
    for (std::size_t index = 0; index < cvars_.size(); ++index) {
        cvar_index_.insert_or_assign(std::string_view{cvars_[index].name}, index);
    }
}

void Console::reindex_commands() {
    command_index_.clear();
    for (std::size_t index = 0; index < commands_.size(); ++index) {
        command_index_.insert_or_assign(std::string_view{commands_[index].first}, index);
    }
}

CVar* Console::find_cvar(std::string_view name) noexcept {
    const std::size_t* slot = cvar_index_.find(name);
    return slot == nullptr ? nullptr : &cvars_[*slot];
}

const CVar* Console::find_cvar(std::string_view name) const noexcept {
    const std::size_t* slot = cvar_index_.find(name);
    return slot == nullptr ? nullptr : &cvars_[*slot];
}

Expected<void> Console::assign(CVar& cvar, std::string_view text) {
    if (cvar.flags.read_only) {
        return fail(ErrorCategory::invalid_state, "variable is read-only");
    }
    if (cvar.flags.cheat && !cheats_allowed_) {
        return fail(ErrorCategory::invalid_state, "variable requires cheats to be enabled");
    }

    switch (cvar.default_value.type()) {
        case CVarType::Boolean: {
            PN_TRY_ASSIGN(const bool value, parse_boolean(text));
            cvar.value = CVarValue{value};
            return {};
        }
        case CVarType::Integer: {
            PN_TRY_ASSIGN(const std::int64_t value, parse_int64(text));
            cvar.value = CVarValue{value};
            return {};
        }
        case CVarType::Real: {
            PN_TRY_ASSIGN(const double value, parse_real(text));
            cvar.value = CVarValue{value};
            return {};
        }
        case CVarType::Text:
            cvar.value = CVarValue{std::string{text}};
            return {};
    }
    return fail(ErrorCategory::internal, "unknown variable type");
}

Expected<void> Console::set_from_text(std::string_view name, std::string_view text) {
    CVar* cvar = find_cvar(name);
    if (cvar == nullptr) {
        return fail(ErrorCategory::not_found, "no such variable");
    }
    return assign(*cvar, text);
}

Expected<void> Console::execute(std::string_view line) {
    output_.clear();
    PN_TRY_ASSIGN(const std::vector<Token> tokens, tokenize(line));
    if (tokens.empty()) {
        return {};
    }

    const std::string_view head = tokens.front().text;

    if (const std::size_t* slot = command_index_.find(head); slot != nullptr) {
        const std::span<const Token> arguments{tokens.data() + 1, tokens.size() - 1};
        return commands_[*slot].second(arguments);
    }

    CVar* cvar = find_cvar(head);
    if (cvar == nullptr) {
        // The column of the offending token, so a caller can point at it.
        return Unexpected{Error{ErrorCategory::not_found, "unknown command or variable",
                               static_cast<std::uint32_t>(tokens.front().column)}};
    }

    if (tokens.size() == 1) {
        output_.push_back(std::string{cvar->name} + " = " + cvar->value.to_text() +
                          "  (default " + cvar->default_value.to_text() + ")  " +
                          std::string{cvar->description});
        return {};
    }
    if (tokens.size() > 2) {
        return Unexpected{Error{ErrorCategory::invalid_argument,
                               "a variable takes one value; quote it if it has spaces",
                               static_cast<std::uint32_t>(tokens[2].column)}};
    }

    return assign(*cvar, tokens[1].text);
}

std::string Console::save_config() const {
    std::vector<const CVar*> saved;
    for (const CVar& cvar : cvars_) {
        if (cvar.flags.persist && !cvar.is_default()) {
            saved.push_back(&cvar);
        }
    }
    // Sorted by name, so two runs that set the same variables produce the same
    // file. A config file that reorders itself makes every diff useless.
    std::sort(saved.begin(), saved.end(),
              [](const CVar* left, const CVar* right) { return left->name < right->name; });

    std::string out;
    for (const CVar* cvar : saved) {
        out += cvar->name;
        out += " = ";
        if (cvar->value.type() == CVarType::Text) {
            out += '"';
            for (char character : cvar->value.as_text()) {
                if (character == '"' || character == '\\') {
                    out += '\\';
                }
                out += character;
            }
            out += '"';
        } else {
            out += cvar->value.to_text();
        }
        out += '\n';
    }
    return out;
}

Console::LoadReport Console::load_config(std::string_view text) {
    LoadReport report;
    std::size_t line_number = 0;

    split(text, '\n', [&](std::string_view raw) {
        ++line_number;
        const std::string_view line = trim(raw);
        if (line.empty() || line.front() == '#') {
            return;
        }

        const std::size_t equals = line.find('=');
        if (equals == std::string_view::npos) {
            report.problems.emplace_back(line_number, "expected 'name = value'");
            return;
        }

        const std::string_view name = trim(line.substr(0, equals));
        const std::string_view raw_value = trim(line.substr(equals + 1));

        CVar* cvar = find_cvar(name);
        if (cvar == nullptr) {
            // Not an error worth refusing the file over. A config file outlives
            // the variables in it: one written by a newer build, or by a build
            // with a module this one does not have, would otherwise be rejected
            // wholesale.
            report.problems.emplace_back(line_number, "unknown variable, ignored");
            return;
        }

        const auto tokens = tokenize(raw_value);
        if (!tokens.has_value() || tokens.value().empty()) {
            report.problems.emplace_back(line_number, "could not read the value");
            return;
        }

        const auto result = assign(*cvar, tokens.value().front().text);
        if (!result.has_value()) {
            report.problems.emplace_back(line_number, std::string{result.error().message()});
            return;
        }
        ++report.applied;
    });

    return report;
}

std::vector<const CVar*> Console::all_cvars() const {
    std::vector<const CVar*> out;
    out.reserve(cvars_.size());
    for (const CVar& cvar : cvars_) {
        out.push_back(&cvar);
    }
    std::sort(out.begin(), out.end(),
              [](const CVar* left, const CVar* right) { return left->name < right->name; });
    return out;
}

}  // namespace pn::core
