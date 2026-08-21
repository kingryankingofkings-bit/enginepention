// Pention Engine - core/tests/console_test.cpp
// Requirement: PN-PLT-019
// Decision:    ADR-0002, ADR-0004
//
// PN-PLT-019's criterion names two things: a CVar set/get/persist round-trip,
// and command parse errors that are actionable. "Actionable" is the harder one
// to test, so it is tested explicitly: every parse failure must say what was
// wrong and where, and the column is asserted rather than assumed.

#include "pn/core/console.hpp"
#include "pn/testing/test.hpp"

#include <string>
#include <string_view>
#include <vector>

namespace {

using pn::core::Console;
using pn::core::CVar;
using pn::core::CVarFlags;
using pn::core::CVarValue;
using pn::core::ErrorCategory;
using pn::core::Token;
using pn::core::tokenize;

Console make_console() {
    Console console;
    console.register_cvar("r.max_fps", CVarValue{std::int64_t{60}}, "frame rate cap");
    console.register_cvar("r.vsync", CVarValue{true}, "wait for vertical blank");
    console.register_cvar("r.exposure", CVarValue{1.0}, "exposure compensation in stops");
    console.register_cvar("g.profile", CVarValue{std::string{"default"}}, "active profile");
    console.register_cvar("r.adapter", CVarValue{std::string{"auto"}}, "graphics adapter",
                          CVarFlags{.read_only = true, .cheat = false, .persist = false});
    console.register_cvar("g.god_mode", CVarValue{false}, "invulnerability",
                          CVarFlags{.read_only = false, .cheat = true, .persist = false});
    return console;
}

}  // namespace

// -------------------------------------------------------------------
// Tokenizing
// -------------------------------------------------------------------

PN_TEST(console, splits_a_line_into_tokens_with_their_columns) {
    const auto tokens = tokenize("set  r.max_fps 120");
    PN_REQUIRE(tokens.has_value());
    PN_REQUIRE_EQ(tokens.value().size(), std::size_t{3});
    PN_CHECK(tokens.value()[0].text == "set");
    PN_CHECK_EQ(tokens.value()[0].column, std::size_t{0});
    PN_CHECK(tokens.value()[1].text == "r.max_fps");
    PN_CHECK_EQ(tokens.value()[1].column, std::size_t{5});
    PN_CHECK(tokens.value()[2].text == "120");
    PN_CHECK_EQ(tokens.value()[2].column, std::size_t{15});
}

PN_TEST(console, quoted_tokens_keep_their_spaces_and_escapes) {
    const auto tokens = tokenize(R"(say "hello there" "a\"b" "line\nbreak")");
    PN_REQUIRE(tokens.has_value());
    PN_REQUIRE_EQ(tokens.value().size(), std::size_t{4});
    PN_CHECK(tokens.value()[1].text == "hello there");
    PN_CHECK(tokens.value()[1].quoted);
    PN_CHECK(tokens.value()[2].text == "a\"b");
    PN_CHECK(tokens.value()[3].text == "line\nbreak");
}

PN_TEST(console, an_empty_quoted_token_is_a_real_argument) {
    // "" is an argument whose value is empty, which is different from no
    // argument at all. A tokenizer that drops it turns `rename foo ""` into a
    // one-argument call.
    const auto tokens = tokenize(R"(rename foo "")");
    PN_REQUIRE(tokens.has_value());
    PN_REQUIRE_EQ(tokens.value().size(), std::size_t{3});
    PN_CHECK(tokens.value()[2].text.empty());
    PN_CHECK(tokens.value()[2].quoted);
}

PN_TEST(console, comments_run_to_the_end_of_the_line) {
    const auto tokens = tokenize("r.vsync true  # the rest is a note");
    PN_REQUIRE(tokens.has_value());
    PN_REQUIRE_EQ(tokens.value().size(), std::size_t{2});
    PN_CHECK(tokens.value()[1].text == "true");
}

PN_TEST(console, an_unterminated_string_reports_where_it_started) {
    // The column is the opening quote, not the end of the line. The mistake is
    // where the string began, and that is where a reader has to look.
    const auto tokens = tokenize(R"(say "never closed)");
    PN_REQUIRE(!tokens.has_value());
    PN_CHECK(tokens.error().is(ErrorCategory::invalid_argument));
    PN_CHECK(tokens.error().message() == "unterminated quoted string");
    PN_CHECK_EQ(tokens.error().code(), std::uint32_t{4});
}

PN_TEST(console, an_unknown_escape_reports_its_column) {
    const auto tokens = tokenize(R"(say "bad \q escape")");
    PN_REQUIRE(!tokens.has_value());
    PN_CHECK(tokens.error().message() == "unknown escape sequence");
    PN_CHECK_EQ(tokens.error().code(), std::uint32_t{9});
}

// -------------------------------------------------------------------
// Set and get
// -------------------------------------------------------------------

PN_TEST(console, a_variable_reports_its_value_when_named_alone) {
    Console console = make_console();
    PN_REQUIRE(console.execute("r.max_fps").has_value());
    PN_REQUIRE_EQ(console.output().size(), std::size_t{1});
    PN_CHECK(console.output()[0].find("r.max_fps = 60") != std::string::npos);
    PN_CHECK(console.output()[0].find("frame rate cap") != std::string::npos);
}

PN_TEST(console, assignment_converts_to_the_declared_type) {
    Console console = make_console();
    PN_REQUIRE(console.execute("r.max_fps 144").has_value());
    PN_CHECK_EQ(console.find_cvar("r.max_fps")->value.as_integer(), std::int64_t{144});

    PN_REQUIRE(console.execute("r.vsync off").has_value());
    PN_CHECK_EQ(console.find_cvar("r.vsync")->value.as_boolean(), false);

    PN_REQUIRE(console.execute("r.exposure -1.5").has_value());
    PN_CHECK_EQ(console.find_cvar("r.exposure")->value.as_real(), -1.5);

    PN_REQUIRE(console.execute(R"(g.profile "high quality")").has_value());
    PN_CHECK(console.find_cvar("g.profile")->value.as_text() == "high quality");
}

PN_TEST(console, booleans_accept_the_spellings_people_actually_type) {
    Console console = make_console();
    for (std::string_view text : {"true", "TRUE", "on", "yes", "1"}) {
        PN_REQUIRE(console.set_from_text("r.vsync", text).has_value());
        PN_REQUIRE_EQ(console.find_cvar("r.vsync")->value.as_boolean(), true);
    }
    for (std::string_view text : {"false", "Off", "no", "0"}) {
        PN_REQUIRE(console.set_from_text("r.vsync", text).has_value());
        PN_REQUIRE_EQ(console.find_cvar("r.vsync")->value.as_boolean(), false);
    }
}

PN_TEST(console, an_assignment_never_changes_a_variables_type) {
    // A console that let `r.max_fps banana` turn an integer into text would move
    // the failure from the assignment to whatever reads it next frame.
    Console console = make_console();
    const auto result = console.execute("r.max_fps banana");
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
    PN_CHECK_EQ(console.find_cvar("r.max_fps")->value.as_integer(), std::int64_t{60});
    PN_CHECK(console.find_cvar("r.max_fps")->value.type() == pn::core::CVarType::Integer);
}

PN_TEST(console, read_only_and_cheat_variables_refuse_assignment) {
    Console console = make_console();

    const auto locked = console.execute("r.adapter discrete");
    PN_REQUIRE(!locked.has_value());
    PN_CHECK(locked.error().is(ErrorCategory::invalid_state));
    PN_CHECK(console.find_cvar("r.adapter")->value.as_text() == "auto");

    const auto cheat = console.execute("g.god_mode true");
    PN_REQUIRE(!cheat.has_value());
    PN_CHECK(cheat.error().is(ErrorCategory::invalid_state));
    PN_CHECK_EQ(console.find_cvar("g.god_mode")->value.as_boolean(), false);

    console.allow_cheats(true);
    PN_REQUIRE(console.execute("g.god_mode true").has_value());
    PN_CHECK_EQ(console.find_cvar("g.god_mode")->value.as_boolean(), true);
}

// -------------------------------------------------------------------
// Actionable errors
// -------------------------------------------------------------------

PN_TEST(console, an_unknown_name_says_so_and_points_at_it) {
    Console console = make_console();
    const auto result = console.execute("  r.no_such_thing 1");
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::not_found));
    PN_CHECK(result.error().message() == "unknown command or variable");
    PN_CHECK_EQ(result.error().code(), std::uint32_t{2});
}

PN_TEST(console, extra_arguments_to_a_variable_suggest_the_fix) {
    // The mistake is almost always a value with spaces in it, so the message
    // says what to do rather than only what went wrong.
    Console console = make_console();
    const auto result = console.execute("g.profile high quality");
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().message() ==
             "a variable takes one value; quote it if it has spaces");
    PN_CHECK_EQ(result.error().code(), std::uint32_t{15});
    PN_CHECK(console.find_cvar("g.profile")->value.as_text() == "default");
}

PN_TEST(console, a_bad_boolean_lists_what_it_would_have_accepted) {
    Console console = make_console();
    const auto result = console.execute("r.vsync maybe");
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().message() ==
             "expected true/false, on/off, yes/no, or 1/0");
}

PN_TEST(console, an_empty_or_comment_only_line_does_nothing) {
    Console console = make_console();
    PN_CHECK(console.execute("").has_value());
    PN_CHECK(console.execute("   ").has_value());
    PN_CHECK(console.execute("# just a note").has_value());
}

// -------------------------------------------------------------------
// Commands
// -------------------------------------------------------------------

PN_TEST(console, a_command_receives_its_arguments_without_its_own_name) {
    Console console = make_console();
    std::vector<std::string> seen;
    console.register_command("echo", "print the arguments",
                             [&seen](std::span<const Token> arguments) -> pn::core::Expected<void> {
                                 for (const Token& token : arguments) {
                                     seen.push_back(token.text);
                                 }
                                 return {};
                             });

    PN_REQUIRE(console.execute(R"(echo one "two three")").has_value());
    PN_REQUIRE_EQ(seen.size(), std::size_t{2});
    PN_CHECK(seen[0] == "one");
    PN_CHECK(seen[1] == "two three");
}

PN_TEST(console, a_commands_own_error_reaches_the_caller) {
    Console console = make_console();
    console.register_command("needs_two", "wants exactly two arguments",
                             [](std::span<const Token> arguments) -> pn::core::Expected<void> {
                                 if (arguments.size() != 2) {
                                     return pn::core::fail(ErrorCategory::invalid_argument,
                                                           "needs_two expects two arguments");
                                 }
                                 return {};
                             });

    const auto result = console.execute("needs_two only_one");
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().message() == "needs_two expects two arguments");
    PN_CHECK(console.execute("needs_two a b").has_value());
}

PN_TEST(console, a_name_cannot_be_both_a_command_and_a_variable) {
    Console console = make_console();
    PN_CHECK(!console.register_cvar("r.max_fps", CVarValue{std::int64_t{1}}, "duplicate"));
    PN_CHECK(!console.register_command("r.max_fps", "clashes with a variable",
                                       [](std::span<const Token>) -> pn::core::Expected<void> {
                                           return {};
                                       }));
}

PN_TEST(console, registering_many_variables_does_not_dangle_the_index) {
    // Every key in the name index is a view into a string the backing vector
    // owns, so a reallocation invalidates all of them at once. Enough
    // registrations to force several growths, then every one must still resolve.
    Console console;
    for (int index = 0; index < 500; ++index) {
        const std::string name = "var_" + std::to_string(index);
        PN_REQUIRE(console.register_cvar(name, CVarValue{static_cast<std::int64_t>(index)},
                                         "generated"));
    }
    for (int index = 0; index < 500; ++index) {
        const CVar* cvar = console.find_cvar("var_" + std::to_string(index));
        PN_REQUIRE(cvar != nullptr);
        PN_REQUIRE_EQ(cvar->value.as_integer(), static_cast<std::int64_t>(index));
    }
}

// -------------------------------------------------------------------
// Persistence
// -------------------------------------------------------------------

PN_TEST(config, only_changed_persistable_variables_are_written) {
    // Writing every variable would freeze today's defaults into every user's
    // config file: a later release that improves a default would never reach
    // anyone who had launched the game once.
    Console console = make_console();
    PN_REQUIRE(console.execute("r.max_fps 144").has_value());
    console.allow_cheats(true);
    PN_REQUIRE(console.execute("g.god_mode true").has_value());

    const std::string config = console.save_config();
    PN_CHECK(config.find("r.max_fps = 144") != std::string::npos);
    PN_CHECK(config.find("r.vsync") == std::string::npos);      // unchanged
    PN_CHECK(config.find("g.god_mode") == std::string::npos);   // persist = false
    PN_CHECK(config.find("r.adapter") == std::string::npos);    // persist = false
}

PN_TEST(config, the_written_order_depends_only_on_the_names) {
    // A config file that reorders itself makes every diff useless.
    //
    // The order must not depend on registration order either, which is the part
    // that actually varies in practice: which module registers first changes
    // with build configuration and with which plugins are present. An earlier
    // version of this test built both consoles the same way and only varied the
    // order of the *assignments*, so it passed with the sort removed entirely -
    // found by deleting the sort and watching nothing fail.
    Console first;
    first.register_cvar("z.last", CVarValue{std::int64_t{0}}, "");
    first.register_cvar("a.first", CVarValue{std::int64_t{0}}, "");
    first.register_cvar("m.middle", CVarValue{std::int64_t{0}}, "");

    Console second;
    second.register_cvar("m.middle", CVarValue{std::int64_t{0}}, "");
    second.register_cvar("z.last", CVarValue{std::int64_t{0}}, "");
    second.register_cvar("a.first", CVarValue{std::int64_t{0}}, "");

    for (Console* console : {&first, &second}) {
        PN_REQUIRE(console->execute("z.last 3").has_value());
        PN_REQUIRE(console->execute("a.first 1").has_value());
        PN_REQUIRE(console->execute("m.middle 2").has_value());
    }

    const std::string expected = "a.first = 1\nm.middle = 2\nz.last = 3\n";
    PN_CHECK(first.save_config() == expected);
    PN_CHECK(second.save_config() == expected);
}

PN_TEST(config, assignment_order_does_not_change_the_file) {
    Console first = make_console();
    PN_REQUIRE(first.execute("g.profile custom").has_value());
    PN_REQUIRE(first.execute("r.max_fps 30").has_value());
    PN_REQUIRE(first.execute("r.vsync false").has_value());

    Console second = make_console();
    PN_REQUIRE(second.execute("r.vsync false").has_value());
    PN_REQUIRE(second.execute("r.max_fps 30").has_value());
    PN_REQUIRE(second.execute("g.profile custom").has_value());

    PN_CHECK(first.save_config() == second.save_config());
}

PN_TEST(config, set_get_and_persist_round_trip) {
    // The acceptance criterion, end to end.
    Console source = make_console();
    PN_REQUIRE(source.execute("r.max_fps 144").has_value());
    PN_REQUIRE(source.execute("r.vsync false").has_value());
    PN_REQUIRE(source.execute("r.exposure 2.25").has_value());
    PN_REQUIRE(source.execute(R"(g.profile "high quality")").has_value());

    const std::string config = source.save_config();

    Console restored = make_console();
    const Console::LoadReport report = restored.load_config(config);
    PN_CHECK_EQ(report.applied, std::size_t{4});
    PN_CHECK(report.problems.empty());

    PN_CHECK_EQ(restored.find_cvar("r.max_fps")->value.as_integer(), std::int64_t{144});
    PN_CHECK_EQ(restored.find_cvar("r.vsync")->value.as_boolean(), false);
    PN_CHECK_EQ(restored.find_cvar("r.exposure")->value.as_real(), 2.25);
    PN_CHECK(restored.find_cvar("g.profile")->value.as_text() == "high quality");

    // And saving again reproduces the same file, which is what makes the
    // round-trip a round-trip rather than a one-way conversion.
    PN_CHECK(restored.save_config() == config);
}

PN_TEST(config, a_real_value_survives_a_save_and_load_without_drifting) {
    // Seventeen significant digits is the shortest count that round-trips every
    // double. Fewer is prettier and loses the last bits, which shows up as a
    // setting that changes slightly every time it is written.
    Console console = make_console();
    const double awkward = 0.1 + 0.2;  // not representable exactly
    PN_REQUIRE(console.set_from_text("r.exposure", "0.30000000000000004").has_value());
    PN_CHECK_EQ(console.find_cvar("r.exposure")->value.as_real(), awkward);

    Console restored = make_console();
    restored.load_config(console.save_config());
    PN_CHECK_EQ(restored.find_cvar("r.exposure")->value.as_real(), awkward);
}

PN_TEST(config, text_with_quotes_and_backslashes_survives) {
    Console console = make_console();
    PN_REQUIRE(console.set_from_text("g.profile", R"(a "quoted" \path\)").has_value());

    Console restored = make_console();
    restored.load_config(console.save_config());
    PN_CHECK(restored.find_cvar("g.profile")->value.as_text() == R"(a "quoted" \path\)");
}

PN_TEST(config, a_bad_line_does_not_discard_the_rest_of_the_file) {
    // Reporting only the first problem means fixing a config file takes as many
    // attempts as it has mistakes.
    Console console = make_console();
    const Console::LoadReport report = console.load_config(
        "# a comment\n"
        "r.max_fps = 90\n"
        "this line has no equals sign\n"
        "r.vsync = perhaps\n"
        "r.retired_setting = 3\n"
        "r.exposure = 1.5\n");

    PN_CHECK_EQ(report.applied, std::size_t{2});
    PN_REQUIRE_EQ(report.problems.size(), std::size_t{3});
    PN_CHECK_EQ(report.problems[0].first, std::size_t{3});
    PN_CHECK(report.problems[1].second.find("true/false") != std::string::npos);
    PN_CHECK(report.problems[2].second == "unknown variable, ignored");

    PN_CHECK_EQ(console.find_cvar("r.max_fps")->value.as_integer(), std::int64_t{90});
    PN_CHECK_EQ(console.find_cvar("r.exposure")->value.as_real(), 1.5);
}

PN_TEST(config, an_unknown_variable_is_reported_but_not_fatal) {
    // A config file outlives the variables in it. One written by a newer build,
    // or by a build with a module this one lacks, must not be rejected wholesale.
    Console console = make_console();
    const Console::LoadReport report =
        console.load_config("r.from_the_future = 1\nr.max_fps = 75\n");
    PN_CHECK_EQ(report.applied, std::size_t{1});
    PN_REQUIRE_EQ(report.problems.size(), std::size_t{1});
    PN_CHECK_EQ(console.find_cvar("r.max_fps")->value.as_integer(), std::int64_t{75});
}
