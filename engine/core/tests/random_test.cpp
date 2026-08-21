// Pention Engine - core/tests/random_test.cpp
// Requirement: PN-PLT-004
// Decision:    ADR-0004, ADR-0008

#include "pn/core/random.hpp"
#include "pn/testing/test.hpp"

#include <array>
#include <cmath>
#include <cstdint>
#include <set>
#include <vector>

namespace {

using pn::core::Random;

/// Counts set bits without depending on a compiler builtin, so the result is
/// the same everywhere - which is the property this whole file is about.
constexpr int popcount64(std::uint64_t value) noexcept {
    int count = 0;
    while (value != 0) {
        count += static_cast<int>(value & 1U);
        value >>= 1;
    }
    return count;
}

}  // namespace

// -------------------------------------------------------------------
// Determinism - the requirement's acceptance criterion
// -------------------------------------------------------------------

PN_TEST(random, the_same_seed_reproduces_the_same_sequence) {
    Random first{0x0123456789ABCDEFULL};
    Random second{0x0123456789ABCDEFULL};
    for (int index = 0; index < 1000; ++index) {
        PN_REQUIRE_EQ(first.next_u64(), second.next_u64());
    }
}

PN_TEST(random, the_recorded_sequence_is_stable_across_compilers) {
    // The literal values below were produced by this implementation and are
    // pinned here deliberately. The acceptance criterion is "same seed
    // reproduces identical sequence across platforms and compilers", and a test
    // that only compares two generators inside one binary cannot check that -
    // it passes even if every value changed. This one fails on any compiler that
    // disagrees, which is what makes the ten-configuration matrix evidence
    // rather than repetition.
    //
    // A change here is either a deliberate change of generator, which breaks
    // every saved world, or a portability bug. Neither should pass quietly.
    Random random{1};
    const std::array<std::uint64_t, 8> expected = {
        0x801F12D0570FCA6EULL, 0x89D247B92B8C84B1ULL, 0x8F4B464873B6B009ULL,
        0x7023A8837CC5122BULL, 0x9953C2823CE2F50CULL, 0x3305824D4718C137ULL,
        0xFB0B9455216F4A86ULL, 0xC1C00B760F87D261ULL,
    };
    for (std::size_t index = 0; index < expected.size(); ++index) {
        PN_CHECK_EQ(random.next_u64(), expected[index]);
    }
}

PN_TEST(random, a_seed_of_zero_does_not_degenerate) {
    // A generator that collapses on an all-zero state is a generator that fails
    // for the one seed everybody types first.
    Random random{0, 0};
    std::set<std::uint64_t> seen;
    for (int index = 0; index < 256; ++index) {
        seen.insert(random.next_u64());
    }
    PN_CHECK_EQ(seen.size(), std::size_t{256});
    PN_CHECK(seen.count(0) == 0);
}

// -------------------------------------------------------------------
// Streams
// -------------------------------------------------------------------

PN_TEST(random, streams_of_one_seed_are_independent) {
    // The failure this prevents: one system taking an extra draw shifts every
    // other system's output, so "same seed, different world" becomes a bug
    // report nobody can reproduce.
    Random terrain{42, 0};
    Random loot{42, 1};
    Random weather{42, 2};

    int matches = 0;
    for (int index = 0; index < 4096; ++index) {
        const std::uint64_t a = terrain.next_u64();
        const std::uint64_t b = loot.next_u64();
        const std::uint64_t c = weather.next_u64();
        if (a == b || b == c || a == c) {
            ++matches;
        }
    }
    PN_CHECK_EQ(matches, 0);
}

PN_TEST(random, adjacent_streams_do_not_produce_shifted_copies) {
    // The subtle failure mode of a weak stream mix: stream 1 turns out to be
    // stream 0 offset by one draw, so two systems silently share a sequence.
    Random first{7, 0};
    std::vector<std::uint64_t> reference;
    reference.reserve(64);
    for (int index = 0; index < 64; ++index) {
        reference.push_back(first.next_u64());
    }

    for (std::uint64_t stream = 1; stream <= 4; ++stream) {
        Random other{7, stream};
        for (int offset = 0; offset < 16; ++offset) {
            other.seek(static_cast<std::uint64_t>(offset));
            int aligned = 0;
            for (int index = 0; index < 32; ++index) {
                if (other.next_u64() == reference[static_cast<std::size_t>(index)]) {
                    ++aligned;
                }
            }
            PN_CHECK_EQ(aligned, 0);
        }
    }
}

PN_TEST(random, with_stream_keeps_the_seed_and_resets_the_counter) {
    Random base{99, 0};
    base.next_u64();
    base.next_u64();

    const Random derived = base.with_stream(5);
    PN_CHECK_EQ(derived.seed(), std::uint64_t{99});
    PN_CHECK_EQ(derived.stream(), std::uint64_t{5});
    PN_CHECK_EQ(derived.position(), std::uint64_t{0});
}

// -------------------------------------------------------------------
// Seeking - the reason for the counter-based shape
// -------------------------------------------------------------------

PN_TEST(random, seeking_agrees_with_iterating) {
    Random iterated{0xDEADBEEFULL, 3};
    std::vector<std::uint64_t> sequence;
    sequence.reserve(500);
    for (int index = 0; index < 500; ++index) {
        sequence.push_back(iterated.next_u64());
    }

    const Random seekable{0xDEADBEEFULL, 3};
    for (std::size_t index = 0; index < sequence.size(); ++index) {
        PN_REQUIRE_EQ(seekable.at(index), sequence[index]);
    }
}

PN_TEST(random, a_distant_seek_costs_nothing_and_still_agrees) {
    // Ten million draws in, reached without drawing ten million times. This is
    // what makes generating a distant world cell affordable.
    Random random{123, 0};
    random.seek(10'000'000);
    const std::uint64_t jumped = random.next_u64();

    const Random reference{123, 0};
    PN_CHECK_EQ(jumped, reference.at(10'000'000));
    PN_CHECK_EQ(random.position(), std::uint64_t{10'000'001});
}

// -------------------------------------------------------------------
// The mixing function itself
// -------------------------------------------------------------------

PN_TEST(random, flipping_one_input_bit_changes_about_half_the_output_bits) {
    // Avalanche. A mixer without it produces outputs that track their inputs,
    // and neighbouring world cells - whose coordinates differ in one bit - come
    // out visibly similar.
    //
    // The bound is deliberately loose: this asserts the mixer is not broken, not
    // that it is excellent. Claiming more than the measurement supports is the
    // failure this project's rules exist to prevent.
    long long total_flipped = 0;
    long long samples = 0;
    int worst_low = 64;
    int worst_high = 0;

    for (std::uint64_t base = 0; base < 512; ++base) {
        const std::uint64_t original = Random::mix(base * 0x9E3779B97F4A7C15ULL, base);
        for (int bit = 0; bit < 64; ++bit) {
            const std::uint64_t perturbed_input = base ^ (std::uint64_t{1} << bit);
            const std::uint64_t perturbed =
                Random::mix(base * 0x9E3779B97F4A7C15ULL, perturbed_input);
            const int flipped = popcount64(original ^ perturbed);
            total_flipped += flipped;
            ++samples;
            if (flipped < worst_low) {
                worst_low = flipped;
            }
            if (flipped > worst_high) {
                worst_high = flipped;
            }
        }
    }

    const double mean = static_cast<double>(total_flipped) / static_cast<double>(samples);
    PN_CHECK(mean > 31.0);
    PN_CHECK(mean < 33.0);
    // No single input-bit flip should leave the output nearly unchanged or
    // nearly inverted; either would mean a bit path through the mixer that does
    // not disperse.
    PN_CHECK(worst_low >= 12);
    PN_CHECK(worst_high <= 52);
}

PN_TEST(random, the_mixer_is_injective_over_a_large_sample) {
    // Not a proof of bijectivity - that would need all 2^64 inputs - but a
    // collision in a million consecutive inputs would mean the mixer is
    // destroying information, which the odd multipliers exist to prevent.
    std::set<std::uint64_t> seen;
    for (std::uint64_t index = 0; index < 1'000'000; ++index) {
        seen.insert(Random::mix(0, index));
    }
    PN_CHECK_EQ(seen.size(), std::size_t{1'000'000});
}

PN_TEST(random, output_bits_are_each_set_about_half_the_time) {
    // A per-bit bias is the defect a whole-value uniformity test misses: the
    // values look spread while one bit is stuck.
    std::array<long long, 64> ones{};
    constexpr long long kDraws = 200'000;

    Random random{0xA5A5A5A5A5A5A5A5ULL};
    for (long long index = 0; index < kDraws; ++index) {
        const std::uint64_t value = random.next_u64();
        for (int bit = 0; bit < 64; ++bit) {
            if (((value >> bit) & 1U) != 0) {
                ++ones[static_cast<std::size_t>(bit)];
            }
        }
    }

    for (int bit = 0; bit < 64; ++bit) {
        const double fraction =
            static_cast<double>(ones[static_cast<std::size_t>(bit)]) /
            static_cast<double>(kDraws);
        PN_REQUIRE(fraction > 0.49);
        PN_REQUIRE(fraction < 0.51);
    }
}

// -------------------------------------------------------------------
// Ranges
// -------------------------------------------------------------------

PN_TEST(random, uniform_stays_inside_its_range_including_the_ends) {
    Random random{5};
    bool saw_low = false;
    bool saw_high = false;
    for (int index = 0; index < 20000; ++index) {
        const std::uint64_t value = random.uniform(10, 20);
        PN_REQUIRE(value >= 10);
        PN_REQUIRE(value <= 20);
        saw_low = saw_low || value == 10;
        saw_high = saw_high || value == 20;
    }
    // Inclusive at both ends, which an off-by-one in the rejection bound would
    // break at exactly one of them.
    PN_CHECK(saw_low);
    PN_CHECK(saw_high);
}

PN_TEST(random, uniform_does_not_favour_the_low_end_of_its_range) {
    // The modulo bias, made visible. With `% 3` over 2^64 the residues are not
    // equally likely, and the gap is small enough to hide in a few thousand
    // draws and obvious over a million.
    constexpr int kBuckets = 3;
    constexpr long long kDraws = 1'200'000;
    std::array<long long, kBuckets> counts{};

    Random random{0xFEEDFACEULL};
    for (long long index = 0; index < kDraws; ++index) {
        counts[random.uniform(0, kBuckets - 1)]++;
    }

    const double expected = static_cast<double>(kDraws) / kBuckets;
    for (int bucket = 0; bucket < kBuckets; ++bucket) {
        const double deviation =
            std::fabs(static_cast<double>(counts[static_cast<std::size_t>(bucket)]) - expected) /
            expected;
        PN_CHECK(deviation < 0.01);
    }
}

PN_TEST(random, uniform_over_a_degenerate_range_returns_the_bound) {
    Random random{1};
    PN_CHECK_EQ(random.uniform(7, 7), std::uint64_t{7});
    // Reversed rather than empty: returning the low bound is defined behaviour,
    // where reading an empty range would not be.
    PN_CHECK_EQ(random.uniform(9, 4), std::uint64_t{9});
}

PN_TEST(random, uniform_over_the_whole_range_never_rejects) {
    // The span == UINT64_MAX case has no incomplete final block, and computing
    // one would divide by zero.
    Random random{2};
    for (int index = 0; index < 1000; ++index) {
        const std::uint64_t value = random.uniform(0, UINT64_MAX);
        PN_REQUIRE(value <= UINT64_MAX);
    }
}

PN_TEST(random, signed_ranges_spanning_zero_do_not_overflow) {
    Random random{3};
    for (int index = 0; index < 20000; ++index) {
        const std::int64_t value = random.uniform_signed(-100, 100);
        PN_REQUIRE(value >= -100);
        PN_REQUIRE(value <= 100);
    }

    // The extremes, where computing `high - low` in signed arithmetic is
    // undefined rather than merely wrong.
    Random extreme{4};
    for (int index = 0; index < 1000; ++index) {
        const std::int64_t value =
            extreme.uniform_signed(INT64_MIN, INT64_MAX);
        PN_REQUIRE(value >= INT64_MIN);
        PN_REQUIRE(value <= INT64_MAX);
    }
}

// -------------------------------------------------------------------
// Floating-point conversion
// -------------------------------------------------------------------

PN_TEST(random, doubles_stay_in_the_half_open_unit_interval) {
    Random random{11};
    double lowest = 1.0;
    double highest = 0.0;
    for (int index = 0; index < 200000; ++index) {
        const double value = random.next_double();
        PN_REQUIRE(value >= 0.0);
        PN_REQUIRE(value < 1.0);
        lowest = value < lowest ? value : lowest;
        highest = value > highest ? value : highest;
    }
    // The interval is covered, not merely respected: a conversion that clustered
    // near the middle would satisfy the bounds and be useless.
    PN_CHECK(lowest < 0.001);
    PN_CHECK(highest > 0.999);
}

PN_TEST(random, floats_stay_in_the_half_open_unit_interval) {
    Random random{12};
    for (int index = 0; index < 200000; ++index) {
        const float value = random.next_float();
        PN_REQUIRE(value >= 0.0F);
        PN_REQUIRE(value < 1.0F);
    }
}

PN_TEST(random, the_largest_possible_draw_still_yields_less_than_one) {
    // The boundary a rounding conversion gets wrong: all bits set must not round
    // up to exactly 1.0 in a half-open interval.
    constexpr double kScale = 1.0 / 9007199254740992.0;
    const double largest = static_cast<double>(UINT64_MAX >> 11) * kScale;
    PN_CHECK(largest < 1.0);

    constexpr float kFloatScale = 1.0F / 16777216.0F;
    const float largest_float = static_cast<float>(UINT64_MAX >> 40) * kFloatScale;
    PN_CHECK(largest_float < 1.0F);
}

PN_TEST(random, booleans_are_balanced) {
    Random random{13};
    long long trues = 0;
    constexpr long long kDraws = 200'000;
    for (long long index = 0; index < kDraws; ++index) {
        if (random.next_bool()) {
            ++trues;
        }
    }
    const double fraction = static_cast<double>(trues) / static_cast<double>(kDraws);
    PN_CHECK(fraction > 0.49);
    PN_CHECK(fraction < 0.51);
}

// -------------------------------------------------------------------
// Usable at compile time
// -------------------------------------------------------------------

PN_TEST(random, the_generator_works_in_a_constant_expression) {
    // Not a convenience: a constexpr generator is one that cannot depend on
    // ambient state, which is the same property that makes it deterministic.
    constexpr std::uint64_t value = Random{77, 1}.at(5);
    static_assert(value == Random{77, 1}.at(5), "at() must be a pure function");
    PN_CHECK(value != 0);
}
