// Pention Engine - core/hash.hpp
// Requirement: PN-PLT-009, PN-PLT-010
// Decision:    ADR-0004

#ifndef PN_CORE_HASH_HPP
#define PN_CORE_HASH_HPP

#include <cstddef>
#include <cstdint>
#include <string_view>
#include <type_traits>

namespace pn::core {

/// Mixes two 64-bit words into one, dispersing every input bit.
///
/// One primitive serves two purposes here, and it is the same property in both:
/// a hash table needs neighbouring keys to land in unrelated buckets, and a
/// counter-based generator needs consecutive counters to produce unrelated
/// values. Both are avalanche. `pn::core::Random` is built on this, and its
/// tests measure the avalanche this relies on - flipping any single input bit
/// changes 31 to 33 of the 64 output bits.
///
/// The three multipliers are the fractional bits of the square roots of 2, 3
/// and 5, forced odd. The derivation is stated so it can be recomputed rather
/// than taken on trust: an unexplained magic constant is the fingerprint that
/// makes provenance unverifiable, and a multiplier must be odd or the
/// multiplication is not invertible and information is destroyed.
///
/// Not a cryptographic hash. An attacker who chooses keys can collide this, so
/// it must not be used where key choice is adversarial - see the note on
/// `hash_bytes`.
constexpr std::uint64_t mix64(std::uint64_t a, std::uint64_t b) noexcept {
    constexpr std::uint64_t kRootTwo = 0x6A09E667F3BCC909ULL;
    constexpr std::uint64_t kRootThree = 0xBB67AE8584CAA73BULL;
    constexpr std::uint64_t kRootFive = 0x3C6EF372FE94F82BULL;

    std::uint64_t value = a ^ (b + kRootTwo);
    value ^= value >> 29;
    value *= kRootThree;
    value ^= value >> 32;
    value += b;
    value *= kRootFive;
    value ^= value >> 31;
    value *= kRootTwo;
    value ^= value >> 30;
    return value;
}

/// Hashes a byte range.
///
/// Eight bytes at a time from an unaligned read built out of shifts, not a
/// `memcpy` of a `uint64_t`: the shift form gives the same answer on a
/// big-endian machine, and a hash that differs by endianness makes a serialized
/// table unreadable on another platform.
///
/// **Not collision-resistant against chosen input.** A hash table keyed on data
/// an untrusted party controls - a network packet, a downloaded asset name -
/// needs a keyed hash and a table that degrades gracefully, neither of which
/// this is. That work belongs with the network boundary, not here.
constexpr std::uint64_t hash_bytes(const char* data, std::size_t size,
                                   std::uint64_t seed = 0) noexcept {
    std::uint64_t accumulator = mix64(seed, size);
    std::size_t offset = 0;

    while (offset + 8 <= size) {
        std::uint64_t word = 0;
        for (std::size_t byte = 0; byte < 8; ++byte) {
            word |= static_cast<std::uint64_t>(
                        static_cast<unsigned char>(data[offset + byte]))
                    << (byte * 8);
        }
        accumulator = mix64(accumulator, word);
        offset += 8;
    }

    if (offset < size) {
        std::uint64_t tail = 0;
        for (std::size_t byte = 0; offset + byte < size; ++byte) {
            tail |= static_cast<std::uint64_t>(
                        static_cast<unsigned char>(data[offset + byte]))
                    << (byte * 8);
        }
        accumulator = mix64(accumulator, tail);
    }

    return accumulator;
}

constexpr std::uint64_t hash_string(std::string_view text,
                                    std::uint64_t seed = 0) noexcept {
    return hash_bytes(text.data(), text.size(), seed);
}

/// The default hash for a key type. Specialize for your own types.
template <typename Key>
struct Hash;

template <typename Key>
    requires std::is_integral_v<Key> || std::is_enum_v<Key>
struct Hash<Key> {
    constexpr std::uint64_t operator()(Key key) const noexcept {
        // Through the mixer rather than used directly. An identity hash on
        // sequential integers is fine with a prime modulus and catastrophic
        // with the power-of-two mask an open-addressing table uses: every key
        // lands in one contiguous run.
        return mix64(static_cast<std::uint64_t>(key), 0);
    }
};

template <>
struct Hash<std::string_view> {
    constexpr std::uint64_t operator()(std::string_view key) const noexcept {
        return hash_string(key);
    }
};

template <typename T>
struct Hash<T*> {
    std::uint64_t operator()(T* key) const noexcept {
        // The low bits of a pointer are always zero for an aligned allocation,
        // so an unmixed pointer hash puts everything in one bucket in every
        // (1 << alignment) run of slots.
        return mix64(reinterpret_cast<std::uintptr_t>(key), 0);
    }
};

}  // namespace pn::core

#endif  // PN_CORE_HASH_HPP
