// Pention Engine - core/serialize.hpp
// Requirement: PN-PLT-020 (deterministic serialization with schema migration)
// Decision:    ADR-0002 (Expected), ADR-0004

#ifndef PN_CORE_SERIALIZE_HPP
#define PN_CORE_SERIALIZE_HPP

#include "pn/core/checksum.hpp"
#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <span>
#include <string_view>
#include <vector>

namespace pn::core {

/// Appends values to a byte buffer in a canonical encoding.
///
/// "Deterministic" here means one thing precisely: the same values written in
/// the same order produce the same bytes, on every platform and every compiler.
/// That is what makes a content-addressed asset cache work at all - if two
/// machines cook the same asset into different bytes, every artefact hashes
/// differently and the cache never hits.
///
/// Two decisions follow from it:
///
///   - **Little-endian, written with shifts.** Not `memcpy` of the native
///     representation, which would produce different bytes on a big-endian
///     host. The shifts cost nothing a compiler will not fold on the common
///     case and remove a whole class of portability bug.
///   - **One encoding per value.** Variable-length integers are emitted in the
///     shortest form, and the reader *rejects* a longer one. Accepting a
///     redundant encoding would mean two different byte strings decode to the
///     same value, and "identical input produces identical bytes" would hold in
///     only one direction.
class ByteWriter {
public:
    ByteWriter() = default;
    explicit ByteWriter(std::size_t reserve_bytes) { bytes_.reserve(reserve_bytes); }

    std::span<const std::byte> data() const noexcept { return bytes_; }
    std::size_t size() const noexcept { return bytes_.size(); }
    void clear() noexcept { bytes_.clear(); }

    void write_u8(std::uint8_t value) { bytes_.push_back(static_cast<std::byte>(value)); }

    void write_u16(std::uint16_t value) {
        write_u8(static_cast<std::uint8_t>(value & 0xFFU));
        write_u8(static_cast<std::uint8_t>((value >> 8) & 0xFFU));
    }

    void write_u32(std::uint32_t value) {
        for (int shift = 0; shift < 32; shift += 8) {
            write_u8(static_cast<std::uint8_t>((value >> shift) & 0xFFU));
        }
    }

    void write_u64(std::uint64_t value) {
        for (int shift = 0; shift < 64; shift += 8) {
            write_u8(static_cast<std::uint8_t>((value >> shift) & 0xFFU));
        }
    }

    void write_i8(std::int8_t value) { write_u8(static_cast<std::uint8_t>(value)); }
    void write_i16(std::int16_t value) { write_u16(static_cast<std::uint16_t>(value)); }
    void write_i32(std::int32_t value) { write_u32(static_cast<std::uint32_t>(value)); }
    void write_i64(std::int64_t value) { write_u64(static_cast<std::uint64_t>(value)); }

    void write_bool(bool value) { write_u8(value ? 1U : 0U); }

    /// Writes the bit pattern, not a formatted number.
    ///
    /// Formatting and re-parsing a float is not round-trip safe unless done very
    /// carefully, and is slower by orders of magnitude. The bit pattern is
    /// exact. A signalling NaN's payload survives too, which is the correct
    /// behaviour for a serializer: it is not the serializer's business to
    /// normalise the caller's data.
    void write_f32(float value) { write_u32(std::bit_cast<std::uint32_t>(value)); }
    void write_f64(double value) { write_u64(std::bit_cast<std::uint64_t>(value)); }

    /// Variable-length unsigned integer, seven bits per byte, shortest form.
    void write_varint(std::uint64_t value) {
        while (value >= 0x80U) {
            write_u8(static_cast<std::uint8_t>((value & 0x7FU) | 0x80U));
            value >>= 7;
        }
        write_u8(static_cast<std::uint8_t>(value));
    }

    /// Variable-length signed integer.
    ///
    /// Zigzag first, so that -1 encodes as one byte rather than ten. A negative
    /// number in two's complement has its high bits set, and a plain varint
    /// would spend a byte on each of them.
    void write_svarint(std::int64_t value) {
        const std::uint64_t magnitude = static_cast<std::uint64_t>(value);
        write_varint((magnitude << 1) ^ static_cast<std::uint64_t>(value >> 63));
    }

    void write_bytes(std::span<const std::byte> block) {
        bytes_.insert(bytes_.end(), block.begin(), block.end());
    }

    /// Length-prefixed text. The length is a varint, so short strings cost one
    /// byte of overhead rather than four.
    void write_string(std::string_view text) {
        write_varint(text.size());
        const auto* raw = reinterpret_cast<const std::byte*>(text.data());
        bytes_.insert(bytes_.end(), raw, raw + text.size());
    }

private:
    std::vector<std::byte> bytes_;
};

/// Reads values back, checking every access against the end of the buffer.
///
/// Every read returns [`Expected`] rather than aborting or reading past the
/// end. Serialized data is the most common untrusted input an engine has - a
/// save file, a downloaded asset, a network packet - and a reader that trusts
/// its input is a reader that turns a corrupt file into an out-of-bounds read.
class ByteReader {
public:
    explicit ByteReader(std::span<const std::byte> bytes) noexcept : bytes_{bytes} {}

    std::size_t offset() const noexcept { return offset_; }
    std::size_t remaining() const noexcept { return bytes_.size() - offset_; }
    bool at_end() const noexcept { return offset_ >= bytes_.size(); }

    Expected<std::uint8_t> read_u8() noexcept {
        if (remaining() < 1) {
            return fail(ErrorCategory::corrupt_data, "read past the end of the buffer");
        }
        return static_cast<std::uint8_t>(bytes_[offset_++]);
    }

    Expected<std::uint16_t> read_u16() noexcept {
        PN_TRY_ASSIGN(const std::uint64_t value, read_fixed(2));
        return static_cast<std::uint16_t>(value);
    }

    Expected<std::uint32_t> read_u32() noexcept {
        PN_TRY_ASSIGN(const std::uint64_t value, read_fixed(4));
        return static_cast<std::uint32_t>(value);
    }

    Expected<std::uint64_t> read_u64() noexcept { return read_fixed(8); }

    Expected<std::int8_t> read_i8() noexcept {
        PN_TRY_ASSIGN(const std::uint8_t value, read_u8());
        return static_cast<std::int8_t>(value);
    }

    Expected<std::int16_t> read_i16() noexcept {
        PN_TRY_ASSIGN(const std::uint16_t value, read_u16());
        return static_cast<std::int16_t>(value);
    }

    Expected<std::int32_t> read_i32() noexcept {
        PN_TRY_ASSIGN(const std::uint32_t value, read_u32());
        return static_cast<std::int32_t>(value);
    }

    Expected<std::int64_t> read_i64() noexcept {
        PN_TRY_ASSIGN(const std::uint64_t value, read_u64());
        return static_cast<std::int64_t>(value);
    }

    /// Reads a boolean, rejecting any byte that is not zero or one.
    ///
    /// Treating every non-zero byte as true would make two different files
    /// decode identically, which breaks the round-trip property the format is
    /// built on - and hides a corrupted byte that happens to land here.
    Expected<bool> read_bool() noexcept {
        PN_TRY_ASSIGN(const std::uint8_t value, read_u8());
        if (value > 1) {
            return fail(ErrorCategory::corrupt_data, "boolean is neither zero nor one");
        }
        return value != 0;
    }

    Expected<float> read_f32() noexcept {
        PN_TRY_ASSIGN(const std::uint32_t value, read_u32());
        return std::bit_cast<float>(value);
    }

    Expected<double> read_f64() noexcept {
        PN_TRY_ASSIGN(const std::uint64_t value, read_u64());
        return std::bit_cast<double>(value);
    }

    /// Reads a variable-length unsigned integer, rejecting a non-shortest form.
    Expected<std::uint64_t> read_varint() noexcept {
        std::uint64_t value = 0;
        int shift = 0;
        for (int index = 0; index < 10; ++index) {
            PN_TRY_ASSIGN(const std::uint8_t byte, read_u8());
            const std::uint64_t payload = byte & 0x7FU;

            if (shift == 63 && payload > 1) {
                return fail(ErrorCategory::corrupt_data, "varint does not fit in 64 bits");
            }
            value |= payload << shift;

            if ((byte & 0x80U) == 0) {
                // The final byte may only be zero when it is the only byte.
                // Anything else is a longer encoding of a value that has a
                // shorter one, and accepting it would let two byte strings mean
                // the same thing.
                if (byte == 0 && index != 0) {
                    return fail(ErrorCategory::corrupt_data, "varint is not in shortest form");
                }
                return value;
            }
            shift += 7;
        }
        return fail(ErrorCategory::corrupt_data, "varint is longer than ten bytes");
    }

    Expected<std::int64_t> read_svarint() noexcept {
        PN_TRY_ASSIGN(const std::uint64_t encoded, read_varint());
        return static_cast<std::int64_t>((encoded >> 1) ^ (~(encoded & 1) + 1));
    }

    /// Reads length-prefixed text as a view into the buffer, without copying.
    ///
    /// The view is valid for as long as the buffer is. That is the right default
    /// for a loader that parses into structures it owns, and a trap for one that
    /// keeps the view after freeing the file - which is why it is said here.
    Expected<std::string_view> read_string() noexcept {
        PN_TRY_ASSIGN(const std::uint64_t length, read_varint());
        if (length > remaining()) {
            return fail(ErrorCategory::corrupt_data, "string length exceeds the buffer");
        }
        const auto* raw = reinterpret_cast<const char*>(bytes_.data() + offset_);
        offset_ += static_cast<std::size_t>(length);
        return std::string_view{raw, static_cast<std::size_t>(length)};
    }

    Expected<std::span<const std::byte>> read_bytes(std::size_t count) noexcept {
        if (count > remaining()) {
            return fail(ErrorCategory::corrupt_data, "block length exceeds the buffer");
        }
        const std::span<const std::byte> block = bytes_.subspan(offset_, count);
        offset_ += count;
        return block;
    }

private:
    Expected<std::uint64_t> read_fixed(std::size_t width) noexcept {
        if (remaining() < width) {
            return fail(ErrorCategory::corrupt_data, "read past the end of the buffer");
        }
        std::uint64_t value = 0;
        for (std::size_t index = 0; index < width; ++index) {
            value |= static_cast<std::uint64_t>(bytes_[offset_ + index]) << (index * 8);
        }
        offset_ += width;
        return value;
    }

    std::span<const std::byte> bytes_;
    std::size_t offset_ = 0;
};

/// A framed, checksummed payload.
///
/// The frame carries what a loader needs before it can trust anything else: a
/// magic number so a wrong file is rejected rather than misread, a format
/// version for the framing itself, a payload version for the schema inside, an
/// explicit length, and a checksum.
///
/// The checksum covers the header as well as the payload. A corrupted length
/// field is the one that turns a bad byte into an out-of-bounds read, so it is
/// the field most worth protecting.
struct Blob {
    /// "PNBL", little-endian.
    static constexpr std::uint32_t kMagic = 0x4C424E50U;
    /// The framing's own version. Changing the frame changes this; changing what
    /// is inside it changes the payload version instead.
    static constexpr std::uint16_t kFormatVersion = 1;
    static constexpr std::size_t kHeaderBytes = 12;
    static constexpr std::size_t kTrailerBytes = 4;

    std::uint16_t payload_version = 0;
    std::span<const std::byte> payload;
};

/// Frames `payload` into `out`.
inline void write_blob(ByteWriter& out, std::uint16_t payload_version,
                       std::span<const std::byte> payload) {
    const std::size_t start = out.size();
    out.write_u32(Blob::kMagic);
    out.write_u16(Blob::kFormatVersion);
    out.write_u16(payload_version);
    out.write_u32(static_cast<std::uint32_t>(payload.size()));
    out.write_bytes(payload);

    const std::span<const std::byte> framed = out.data().subspan(start);
    out.write_u32(crc32(framed.data(), framed.size()));
}

/// Validates a frame and returns the payload inside it.
///
/// Each failure is a distinct category, because a caller's response differs: a
/// wrong magic means the wrong file was opened, an unsupported format version
/// means the tooling is too old, and a checksum mismatch means the bytes are
/// damaged.
inline Expected<Blob> read_blob(std::span<const std::byte> bytes) noexcept {
    if (bytes.size() < Blob::kHeaderBytes + Blob::kTrailerBytes) {
        return fail(ErrorCategory::corrupt_data, "buffer is smaller than an empty blob");
    }

    ByteReader reader{bytes};
    PN_TRY_ASSIGN(const std::uint32_t magic, reader.read_u32());
    if (magic != Blob::kMagic) {
        return fail(ErrorCategory::invalid_argument, "not a Pention blob");
    }

    PN_TRY_ASSIGN(const std::uint16_t format_version, reader.read_u16());
    if (format_version != Blob::kFormatVersion) {
        return fail(ErrorCategory::unsupported, "blob framing version is not supported");
    }

    PN_TRY_ASSIGN(const std::uint16_t payload_version, reader.read_u16());
    PN_TRY_ASSIGN(const std::uint32_t payload_length, reader.read_u32());

    const std::size_t expected =
        Blob::kHeaderBytes + payload_length + Blob::kTrailerBytes;
    if (expected != bytes.size()) {
        return fail(ErrorCategory::corrupt_data, "blob length does not match the buffer");
    }

    // The checksum is computed over everything before it, header included.
    const std::uint32_t actual =
        crc32(bytes.data(), Blob::kHeaderBytes + payload_length);
    ByteReader trailer{bytes.subspan(Blob::kHeaderBytes + payload_length)};
    PN_TRY_ASSIGN(const std::uint32_t stored, trailer.read_u32());
    if (actual != stored) {
        return fail(ErrorCategory::corrupt_data, "blob checksum does not match");
    }

    return Blob{payload_version, bytes.subspan(Blob::kHeaderBytes, payload_length)};
}

}  // namespace pn::core

#endif  // PN_CORE_SERIALIZE_HPP
