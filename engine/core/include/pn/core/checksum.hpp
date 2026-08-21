// Pention Engine - core/checksum.hpp
// Requirement: PN-PLT-020, PN-AST-004
// Decision:    ADR-0004

#ifndef PN_CORE_CHECKSUM_HPP
#define PN_CORE_CHECKSUM_HPP

#include <array>
#include <cstddef>
#include <cstdint>

namespace pn::core {

/// CRC-32, the reflected variant standardised in IEEE 802.3 and used by gzip,
/// PNG and zip.
///
/// Implemented from the published algorithm: a reflected polynomial, an
/// all-ones initial value, and a final complement. Nothing is copied - see the
/// note in PROVENANCE_LEDGER.md on implementing published algorithms from their
/// description.
///
/// **For corruption, not for tampering.** CRC-32 detects the bit errors a disk
/// or a cable produces and is trivial to forge deliberately: an attacker who can
/// edit the payload can edit the checksum to match, and can even craft an edit
/// that leaves the original checksum intact. Content that arrives from an
/// untrusted party needs a cryptographic digest and a signature, which is a
/// different mechanism at a different layer.
namespace detail {

/// The table, built at compile time from the polynomial rather than pasted in.
///
/// A pasted 256-entry table is unreviewable and indistinguishable from a copied
/// one. Generating it from the single constant that defines it makes the whole
/// thing checkable by reading four lines.
consteval std::array<std::uint32_t, 256> make_crc32_table() {
    constexpr std::uint32_t kReflectedPolynomial = 0xEDB88320U;
    std::array<std::uint32_t, 256> table{};
    for (std::uint32_t index = 0; index < 256; ++index) {
        std::uint32_t remainder = index;
        for (int bit = 0; bit < 8; ++bit) {
            remainder = (remainder & 1U) != 0 ? (remainder >> 1) ^ kReflectedPolynomial
                                              : (remainder >> 1);
        }
        table[index] = remainder;
    }
    return table;
}

inline constexpr std::array<std::uint32_t, 256> kCrc32Table = make_crc32_table();

}  // namespace detail

/// Continues a CRC-32 over another block, for data that arrives in pieces.
constexpr std::uint32_t crc32_continue(std::uint32_t running, const void* data,
                                       std::size_t size) noexcept {
    const auto* bytes = static_cast<const unsigned char*>(data);
    std::uint32_t remainder = ~running;
    for (std::size_t index = 0; index < size; ++index) {
        remainder = detail::kCrc32Table[(remainder ^ bytes[index]) & 0xFFU] ^ (remainder >> 8);
    }
    return ~remainder;
}

constexpr std::uint32_t crc32(const void* data, std::size_t size) noexcept {
    return crc32_continue(0, data, size);
}

}  // namespace pn::core

#endif  // PN_CORE_CHECKSUM_HPP
