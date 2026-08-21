// Pention Engine - core/random.hpp
// Requirement: PN-PLT-004 (deterministic seeded RNG with explicit streams)
// Decision:    ADR-0004 (module boundaries), ADR-0008 (job scheduler)

#ifndef PN_CORE_RANDOM_HPP
#define PN_CORE_RANDOM_HPP

#include "pn/core/hash.hpp"

#include <cstdint>

namespace pn::core {

/// A counter-based pseudorandom generator.
///
/// ## Why counter-based rather than state-advancing
///
/// The usual shape - a mutable state word advanced on every draw - is the wrong
/// shape for this engine, and the reason is the job scheduler (ADR-0008). Work
/// is distributed across worker threads with no guaranteed order, so a shared
/// advancing generator either needs a lock on the hottest path in the frame or
/// produces different results depending on which worker got there first. Giving
/// each job its own generator only moves the problem to seeding them.
///
/// Here the output is a pure function of `(seed, stream, counter)`. Nothing is
/// shared, nothing is locked, and a job that knows its index can compute exactly
/// the values it would have got in any other execution order. The same property
/// makes a sequence seekable: jumping to draw ten million is one multiply, not
/// ten million advances - which is what makes procedural generation of a distant
/// world cell affordable at all.
///
/// ## What is and is not claimed
///
/// Claimed, and asserted by tests: determinism across compilers and build
/// configurations, independence between streams, avalanche in the mixing
/// function, and uniformity over the ranges the API offers.
///
/// **Not** claimed: cryptographic strength, or a pass through any published
/// statistical battery. No such battery has been run here, and repeating a
/// quality claim from another generator's literature would be describing
/// evidence this project does not have. Do not use this to generate keys,
/// tokens, or anything an adversary benefits from predicting.
class Random {
public:
    /// Constructs a generator for one stream of one seed.
    ///
    /// `stream` names an independent sequence. Two streams of the same seed are
    /// unrelated, which is what lets terrain, loot, and weather be re-rolled
    /// independently without one system's extra draw shifting another's output -
    /// the failure that makes "same seed, different world" bug reports
    /// impossible to reproduce.
    constexpr Random(std::uint64_t seed, std::uint64_t stream = 0) noexcept
        : seed_{seed}, stream_{stream}, counter_{0} {}

    /// The next 64 bits.
    constexpr std::uint64_t next_u64() noexcept { return draw(counter_++); }

    /// The next 32 bits, taken from the high half.
    ///
    /// The high half rather than the low: in a multiply-based mixer the high
    /// bits have been influenced by every input bit, and the low ones by fewer.
    constexpr std::uint32_t next_u32() noexcept {
        return static_cast<std::uint32_t>(next_u64() >> 32);
    }

    /// A value in `[0, 1)`.
    ///
    /// Built from 53 bits, which is exactly the precision a `double` has. Using
    /// all 64 would ask the conversion to round, and rounding up at the top of
    /// the range would return 1.0 from a half-open interval.
    constexpr double next_double() noexcept {
        constexpr double kScale = 1.0 / 9007199254740992.0;  // 1 / 2^53
        return static_cast<double>(next_u64() >> 11) * kScale;
    }

    /// A value in `[0, 1)` as a `float`, from 24 bits.
    constexpr float next_float() noexcept {
        constexpr float kScale = 1.0F / 16777216.0F;  // 1 / 2^24
        return static_cast<float>(next_u64() >> 40) * kScale;
    }

    constexpr bool next_bool() noexcept { return (next_u64() >> 63) != 0; }

    /// A value in `[low, high]`, inclusive at both ends.
    ///
    /// Rejection sampling, not `% range`. Modulo makes the low values of the
    /// range more likely whenever the range does not divide 2^64 - a bias that
    /// is invisible in a handful of draws and clearly visible in a loot table
    /// over a million.
    ///
    /// Returns `low` when `high < low`, rather than reading an empty range.
    constexpr std::uint64_t uniform(std::uint64_t low, std::uint64_t high) noexcept {
        if (high <= low) {
            return low;
        }
        const std::uint64_t span = high - low;
        if (span == UINT64_MAX) {
            return next_u64();
        }
        const std::uint64_t count = span + 1;

        // Discard the incomplete final block of the 2^64 range, leaving a
        // region whose size is an exact multiple of `count`.
        const std::uint64_t limit = UINT64_MAX - (UINT64_MAX % count);
        std::uint64_t value = next_u64();
        while (value >= limit) {
            value = next_u64();
        }
        return low + (value % count);
    }

    /// A value in `[low, high]` for signed ranges.
    constexpr std::int64_t uniform_signed(std::int64_t low, std::int64_t high) noexcept {
        if (high <= low) {
            return low;
        }
        // Offsetting through unsigned avoids the overflow that `high - low`
        // suffers when the range spans zero near the type's limits.
        const std::uint64_t span =
            static_cast<std::uint64_t>(high) - static_cast<std::uint64_t>(low);
        const std::uint64_t offset = uniform(0, span);
        return static_cast<std::int64_t>(static_cast<std::uint64_t>(low) + offset);
    }

    /// The value this stream would produce at `index`, without advancing.
    ///
    /// The seek that makes counter-based generation worth its cost.
    constexpr std::uint64_t at(std::uint64_t index) const noexcept { return draw(index); }

    /// Moves the next draw to `index`.
    constexpr void seek(std::uint64_t index) noexcept { counter_ = index; }

    constexpr std::uint64_t position() const noexcept { return counter_; }
    constexpr std::uint64_t seed() const noexcept { return seed_; }
    constexpr std::uint64_t stream() const noexcept { return stream_; }

    /// A generator for a different stream of the same seed.
    constexpr Random with_stream(std::uint64_t stream) const noexcept {
        return Random{seed_, stream};
    }

    /// The mixing function, exposed because it is the whole generator.
    ///
    /// A test that can only observe the class cannot measure avalanche, and a
    /// mixer whose avalanche is never measured is a mixer nobody has checked.
    ///
    /// It lives in `hash.hpp` as `mix64` because a hash table needs exactly the
    /// same property for a different reason: neighbouring keys must land in
    /// unrelated buckets, and consecutive counters must produce unrelated
    /// values. The avalanche measured by this file's tests is what both rely
    /// on.
    static constexpr std::uint64_t mix(std::uint64_t a, std::uint64_t b) noexcept {
        return mix64(a, b);
    }

private:
    constexpr std::uint64_t draw(std::uint64_t index) const noexcept {
        // The stream is folded in first so that two streams differ from the
        // first bit of the first round, rather than only in the low bits of an
        // addition the mixer then has to disperse.
        return mix(mix(seed_, stream_), index);
    }

    std::uint64_t seed_;
    std::uint64_t stream_;
    std::uint64_t counter_;
};

}  // namespace pn::core

#endif  // PN_CORE_RANDOM_HPP
