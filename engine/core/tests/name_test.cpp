// Pention Engine - core/tests/name_test.cpp
// Requirement: PN-PLT-010
// Decision:    ADR-0004
//
// PN-PLT-010's criterion names three things: interning collision tests, rehash
// tests, and handle staleness detection. The third is already covered by
// engine/core/tests/handle_test.cpp under PN-PLT-011; the first two are here.
//
// The collision tests use a deliberately weak hasher, because forcing a
// collision in the real one is not practical and the behaviour under collision
// is the property most worth pinning: two distinct strings that share a hash
// must stay two distinct names.

#include "pn/core/name.hpp"
#include "pn/core/random.hpp"
#include "pn/core/string.hpp"
#include "pn/testing/test.hpp"

#include <cstdint>
#include <map>
#include <set>
#include <string>
#include <string_view>
#include <vector>

namespace {

using pn::core::BasicNameTable;
using pn::core::Name;
using pn::core::NameTable;
using pn::core::Random;

/// Every string hashes the same. The worst case a table can be given, and the
/// one where a table that trusts its hash silently merges unrelated names.
struct AlwaysCollides {
    constexpr std::uint64_t operator()(std::string_view) const noexcept { return 0; }
};

/// A handful of buckets, so collisions are common but probing still works.
struct FewBuckets {
    constexpr std::uint64_t operator()(std::string_view text) const noexcept {
        return text.size() % 3;
    }
};

}  // namespace

// -------------------------------------------------------------------
// Interning
// -------------------------------------------------------------------

PN_TEST(name, the_same_text_interns_to_the_same_name) {
    NameTable table;
    const Name first = table.intern("albedo_texture");
    const Name second = table.intern("albedo_texture");
    const Name other = table.intern("normal_texture");

    PN_CHECK(first == second);
    PN_CHECK(!(first == other));
    PN_CHECK_EQ(table.size(), std::size_t{2});
    PN_CHECK(table.text(first) == "albedo_texture");
}

PN_TEST(name, a_default_name_is_invalid_and_distinct_from_the_empty_string) {
    NameTable table;
    const Name none;
    PN_CHECK(!none.is_valid());

    const Name empty = table.intern("");
    PN_CHECK(empty.is_valid());
    PN_CHECK(!(empty == none));
    PN_CHECK(table.text(empty).empty());
}

PN_TEST(name, find_does_not_intern) {
    // A lookup that silently interns turns a typo in a config file into a new
    // asset name rather than an error.
    NameTable table;
    table.intern("present");

    PN_CHECK(table.find("present").is_valid());
    PN_CHECK(!table.find("absent").is_valid());
    PN_CHECK_EQ(table.size(), std::size_t{1});
}

PN_TEST(name, comparison_is_an_integer_comparison) {
    // The reason interning exists. If this ever became a string compare, the
    // cost would move back onto every per-object path.
    NameTable table;
    const Name a = table.intern("a_very_long_material_parameter_name_that_would_be_slow");
    const Name b = table.intern("a_very_long_material_parameter_name_that_would_be_slox");

    PN_CHECK(!(a == b));
    PN_CHECK_EQ(a.index() == b.index(), false);
    PN_CHECK(a < b || b < a);
}

// -------------------------------------------------------------------
// Collisions
// -------------------------------------------------------------------

PN_TEST(name, strings_that_share_a_hash_stay_distinct_names) {
    // Every string hashes to zero here. A table that compares only the hash
    // returns the first name for all of them, and one asset quietly becomes
    // another with nothing to see at the point of failure.
    BasicNameTable<AlwaysCollides> table;

    std::vector<Name> names;
    for (int index = 0; index < 200; ++index) {
        names.push_back(table.intern("asset_" + std::to_string(index)));
    }

    PN_REQUIRE_EQ(table.size(), std::size_t{200});
    std::set<std::uint32_t> distinct;
    for (const Name& name : names) {
        distinct.insert(name.index());
    }
    PN_CHECK_EQ(distinct.size(), std::size_t{200});

    for (int index = 0; index < 200; ++index) {
        const std::string expected = "asset_" + std::to_string(index);
        PN_REQUIRE(table.text(names[static_cast<std::size_t>(index)]) == expected);
        PN_REQUIRE(table.find(expected) == names[static_cast<std::size_t>(index)]);
    }
}

PN_TEST(name, colliding_strings_of_equal_length_are_still_distinguished) {
    // Length is the cheap reject before the byte comparison. Two strings of the
    // same length that collide exercise the comparison itself.
    BasicNameTable<AlwaysCollides> table;
    const Name first = table.intern("aaaa");
    const Name second = table.intern("aaab");
    const Name third = table.intern("baaa");

    PN_CHECK(!(first == second));
    PN_CHECK(!(second == third));
    PN_CHECK(table.text(first) == "aaaa");
    PN_CHECK(table.text(second) == "aaab");
    PN_CHECK(table.text(third) == "baaa");
    PN_CHECK(table.intern("aaab") == second);
}

PN_TEST(name, a_weak_hash_still_resolves_every_name) {
    BasicNameTable<FewBuckets> table;
    std::map<std::string, Name> oracle;

    Random random{909, 1};
    for (int index = 0; index < 2000; ++index) {
        std::string text;
        const std::uint64_t length = random.uniform(1, 24);
        for (std::uint64_t character = 0; character < length; ++character) {
            text.push_back(static_cast<char>('a' + random.uniform(0, 25)));
        }

        const Name name = table.intern(text);
        const auto existing = oracle.find(text);
        if (existing == oracle.end()) {
            oracle.emplace(text, name);
        } else {
            PN_REQUIRE(name == existing->second);
        }
    }

    PN_REQUIRE_EQ(table.size(), oracle.size());
    for (const auto& [text, name] : oracle) {
        PN_REQUIRE(table.find(text) == name);
        PN_REQUIRE(table.text(name) == text);
    }
}

// -------------------------------------------------------------------
// Rehashing
// -------------------------------------------------------------------

PN_TEST(name, growth_preserves_every_name_and_its_text) {
    // Enough interning to force many doublings of the bucket array.
    NameTable table;
    std::vector<Name> names;
    names.reserve(20000);
    for (int index = 0; index < 20000; ++index) {
        names.push_back(table.intern("entity_" + std::to_string(index)));
    }

    PN_REQUIRE_EQ(table.size(), std::size_t{20000});
    for (int index = 0; index < 20000; ++index) {
        const std::string expected = "entity_" + std::to_string(index);
        PN_REQUIRE(table.text(names[static_cast<std::size_t>(index)]) == expected);
        PN_REQUIRE(table.intern(expected) == names[static_cast<std::size_t>(index)]);
    }
    // No duplicates were created by the rehash.
    PN_CHECK_EQ(table.size(), std::size_t{20000});
}

PN_TEST(name, text_views_survive_every_later_intern) {
    // The property the chunked text storage exists for. A single growing buffer
    // would invalidate every view handed out before it reallocated, and that
    // failure appears far from its cause - a name printed in a log turning into
    // garbage minutes after it was captured.
    NameTable table;
    const Name early = table.intern("captured_early");
    const std::string_view held = table.text(early);
    const char* address = held.data();

    // Interned text well past a block boundary, so the storage certainly grew.
    for (int index = 0; index < 20000; ++index) {
        table.intern("filler_" + std::to_string(index) + std::string(40, 'x'));
    }

    PN_CHECK(held == "captured_early");
    PN_CHECK(table.text(early).data() == address);
    PN_CHECK(table.text(early) == "captured_early");
}

PN_TEST(name, a_string_larger_than_a_block_is_stored_contiguously) {
    // A long asset path or a serialized blob key. Splitting it across blocks
    // would mean text() could not return a contiguous view at all.
    NameTable table;
    const std::string huge(200000, 'z');
    const Name name = table.intern(huge);

    PN_REQUIRE(name.is_valid());
    PN_CHECK_EQ(table.text(name).size(), huge.size());
    PN_CHECK(table.text(name) == huge);
    PN_CHECK(table.find(huge) == name);
}

PN_TEST(name, the_probe_run_stays_short_as_the_table_grows) {
    // A table that never rehashes still answers correctly, just slower for
    // ever. Only measurement catches it.
    NameTable table;
    for (int index = 0; index < 10000; ++index) {
        table.intern("material_" + std::to_string(index));
    }
    PN_CHECK(table.longest_probe() < 32);
}

// -------------------------------------------------------------------
// String utilities
// -------------------------------------------------------------------

PN_TEST(string, trims_ascii_whitespace_from_both_ends) {
    using pn::core::trim;
    PN_CHECK(trim("  hello  ") == "hello");
    PN_CHECK(trim("\t\n mixed \r\v\f") == "mixed");
    PN_CHECK(trim("").empty());
    PN_CHECK(trim("   ").empty());
    PN_CHECK(trim("none") == "none");
}

PN_TEST(string, case_insensitive_comparison_is_ascii_only) {
    using pn::core::equals_ignore_case;
    PN_CHECK(equals_ignore_case("Albedo", "albedo"));
    PN_CHECK(equals_ignore_case("MIXED_case_01", "mixed_CASE_01"));
    PN_CHECK(!equals_ignore_case("albedo", "albedos"));
    PN_CHECK(!equals_ignore_case("albedo", "normal"));

    // Not locale-aware on purpose: a locale-aware fold makes the same config
    // file parse differently on a Turkish system, where uppercase 'i' is not
    // 'I'. Only the ASCII range folds here, on every machine.
    PN_CHECK(equals_ignore_case("I", "i"));
}

PN_TEST(string, split_preserves_empty_fields) {
    // In a comma-separated record an empty field is a value, and collapsing it
    // shifts every column after it.
    std::vector<std::string_view> fields;
    pn::core::split("a,,b,", ',', [&fields](std::string_view field) {
        fields.push_back(field);
    });

    PN_REQUIRE_EQ(fields.size(), std::size_t{4});
    PN_CHECK(fields[0] == "a");
    PN_CHECK(fields[1].empty());
    PN_CHECK(fields[2] == "b");
    PN_CHECK(fields[3].empty());
}

PN_TEST(string, split_of_a_string_without_the_delimiter_yields_one_field) {
    std::vector<std::string_view> fields;
    pn::core::split("solitary", ',', [&fields](std::string_view field) {
        fields.push_back(field);
    });
    PN_REQUIRE_EQ(fields.size(), std::size_t{1});
    PN_CHECK(fields[0] == "solitary");
}

PN_TEST(string, parses_integers_and_reports_what_it_will_not_parse) {
    using pn::core::ErrorCategory;
    using pn::core::parse_int64;

    PN_CHECK_EQ(parse_int64("0").value(), std::int64_t{0});
    PN_CHECK_EQ(parse_int64("42").value(), std::int64_t{42});
    PN_CHECK_EQ(parse_int64("+42").value(), std::int64_t{42});
    PN_CHECK_EQ(parse_int64("-42").value(), std::int64_t{-42});

    // Every sentinel a parser could return is also a number someone will
    // legitimately write, so these are errors rather than values.
    PN_CHECK(!parse_int64("").has_value());
    PN_CHECK(!parse_int64("-").has_value());
    PN_CHECK(!parse_int64("12x").has_value());
    PN_CHECK(!parse_int64(" 12").has_value());
    PN_CHECK(parse_int64("12x").error().is(ErrorCategory::invalid_argument));
}

PN_TEST(string, integer_parsing_rejects_values_that_do_not_fit) {
    using pn::core::ErrorCategory;
    using pn::core::parse_int64;

    // The exact limits must parse.
    PN_CHECK_EQ(parse_int64("9223372036854775807").value(),
                std::numeric_limits<std::int64_t>::max());
    // And INT64_MIN, whose magnitude is one greater than INT64_MAX - the value
    // that makes a naive negate undefined.
    PN_CHECK_EQ(parse_int64("-9223372036854775808").value(),
                std::numeric_limits<std::int64_t>::min());

    // One past each, which a check written after the multiply would miss
    // because signed overflow is undefined and the optimiser may delete it.
    PN_CHECK(parse_int64("9223372036854775808").error().is(ErrorCategory::out_of_range));
    PN_CHECK(parse_int64("-9223372036854775809").error().is(ErrorCategory::out_of_range));
    PN_CHECK(parse_int64("99999999999999999999999").error().is(ErrorCategory::out_of_range));
}

PN_TEST(string, integer_parsing_is_a_constant_expression) {
    static_assert(pn::core::parse_int64("1234").value() == 1234);
    static_assert(!pn::core::parse_int64("nope").has_value());
    PN_CHECK(true);
}
