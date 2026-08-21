// Pention Engine - core/tests/serialize_test.cpp
// Requirement: PN-PLT-020
// Decision:    ADR-0002, ADR-0004
//
// PN-PLT-020's criterion names two things: a golden-data test, and an
// old-version payload that migrates and round-trips. Both are here.
//
// The golden data is a committed byte array. A round-trip test alone proves
// only that the reader and the writer agree with each other, which they would
// even if both changed - and a format that silently changes is a format that
// cannot read yesterday's save file.

#include "pn/core/checksum.hpp"
#include "pn/core/random.hpp"
#include "pn/core/serialize.hpp"
#include "pn/testing/test.hpp"

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <limits>
#include <span>
#include <string>
#include <string_view>
#include <vector>

namespace {

using pn::core::Blob;
using pn::core::ByteReader;
using pn::core::ByteWriter;
using pn::core::ErrorCategory;
using pn::core::Random;

std::vector<std::byte> to_bytes(std::span<const std::byte> span) {
    return std::vector<std::byte>{span.begin(), span.end()};
}

std::vector<std::byte> from_hex(std::string_view hex) {
    std::vector<std::byte> out;
    for (std::size_t index = 0; index + 1 < hex.size(); index += 2) {
        auto digit = [](char character) -> unsigned {
            if (character >= '0' && character <= '9') {
                return static_cast<unsigned>(character - '0');
            }
            return static_cast<unsigned>(character - 'a' + 10);
        };
        out.push_back(static_cast<std::byte>(digit(hex[index]) * 16 + digit(hex[index + 1])));
    }
    return out;
}

std::string to_hex(std::span<const std::byte> bytes) {
    std::string out;
    for (std::byte byte : bytes) {
        const unsigned value = static_cast<unsigned>(byte);
        out.push_back("0123456789abcdef"[value >> 4]);
        out.push_back("0123456789abcdef"[value & 0xFU]);
    }
    return out;
}

}  // namespace

// -------------------------------------------------------------------
// Checksum
// -------------------------------------------------------------------

PN_TEST(checksum, matches_the_published_crc32_check_values) {
    // The value published alongside the algorithm for the string "123456789",
    // which is what makes this implementation checkable rather than merely
    // self-consistent.
    PN_CHECK_EQ(pn::core::crc32("123456789", 9), std::uint32_t{0xCBF43926U});
    PN_CHECK_EQ(pn::core::crc32("", 0), std::uint32_t{0});
    PN_CHECK_EQ(pn::core::crc32("a", 1), std::uint32_t{0xE8B7BE43U});
}

PN_TEST(checksum, a_running_checksum_matches_one_taken_in_a_single_pass) {
    // Data arrives in pieces - a stream, a file read in blocks - and the pieces
    // must not change the answer.
    const std::string whole = "the quick brown fox jumps over the lazy dog";
    const std::uint32_t once = pn::core::crc32(whole.data(), whole.size());

    std::uint32_t running = 0;
    for (std::size_t offset = 0; offset < whole.size(); offset += 7) {
        const std::size_t count = std::min<std::size_t>(7, whole.size() - offset);
        running = pn::core::crc32_continue(running, whole.data() + offset, count);
    }
    PN_CHECK_EQ(running, once);
}

PN_TEST(checksum, a_single_flipped_bit_changes_the_result) {
    std::string data = "payload";
    const std::uint32_t original = pn::core::crc32(data.data(), data.size());
    data[3] = static_cast<char>(data[3] ^ 0x01);
    PN_CHECK(pn::core::crc32(data.data(), data.size()) != original);
}

// -------------------------------------------------------------------
// Round-trips
// -------------------------------------------------------------------

PN_TEST(serialize, fixed_width_values_round_trip) {
    ByteWriter writer;
    writer.write_u8(0xABU);
    writer.write_u16(0x1234U);
    writer.write_u32(0xDEADBEEFU);
    writer.write_u64(0x0123456789ABCDEFULL);
    writer.write_i8(-128);
    writer.write_i16(std::numeric_limits<std::int16_t>::min());
    writer.write_i32(std::numeric_limits<std::int32_t>::min());
    writer.write_i64(std::numeric_limits<std::int64_t>::min());
    writer.write_bool(true);
    writer.write_bool(false);

    ByteReader reader{writer.data()};
    PN_CHECK_EQ(reader.read_u8().value(), std::uint8_t{0xABU});
    PN_CHECK_EQ(reader.read_u16().value(), std::uint16_t{0x1234U});
    PN_CHECK_EQ(reader.read_u32().value(), std::uint32_t{0xDEADBEEFU});
    PN_CHECK_EQ(reader.read_u64().value(), std::uint64_t{0x0123456789ABCDEFULL});
    PN_CHECK_EQ(reader.read_i8().value(), std::int8_t{-128});
    PN_CHECK_EQ(reader.read_i16().value(), std::numeric_limits<std::int16_t>::min());
    PN_CHECK_EQ(reader.read_i32().value(), std::numeric_limits<std::int32_t>::min());
    PN_CHECK_EQ(reader.read_i64().value(), std::numeric_limits<std::int64_t>::min());
    PN_CHECK_EQ(reader.read_bool().value(), true);
    PN_CHECK_EQ(reader.read_bool().value(), false);
    PN_CHECK(reader.at_end());
}

PN_TEST(serialize, multi_byte_values_are_little_endian_whatever_the_host_is) {
    // Written with shifts rather than a memcpy of the native representation, so
    // a big-endian host produces the same bytes. Asserting the layout is what
    // makes that claim checkable on a machine that is little-endian anyway.
    ByteWriter writer;
    writer.write_u32(0x11223344U);
    PN_CHECK(to_hex(writer.data()) == "44332211");

    ByteWriter wide;
    wide.write_u64(0x0102030405060708ULL);
    PN_CHECK(to_hex(wide.data()) == "0807060504030201");
}

PN_TEST(serialize, floating_point_values_round_trip_exactly) {
    // The bit pattern, not a formatted number: formatting and re-parsing is not
    // round-trip safe unless done very carefully, and is far slower.
    ByteWriter writer;
    for (float value : {0.0F, -0.0F, 1.0F, -1.5F, 3.4028235e38F, 1.4e-45F}) {
        writer.write_f32(value);
    }
    writer.write_f64(3.141592653589793);
    writer.write_f64(std::numeric_limits<double>::infinity());

    ByteReader reader{writer.data()};
    PN_CHECK_EQ(reader.read_f32().value(), 0.0F);
    // Negative zero must survive as negative zero, which `== 0.0F` would not
    // catch: the sign bit is data.
    const float negative_zero = reader.read_f32().value();
    PN_CHECK(std::signbit(negative_zero));
    PN_CHECK_EQ(reader.read_f32().value(), 1.0F);
    PN_CHECK_EQ(reader.read_f32().value(), -1.5F);
    PN_CHECK_EQ(reader.read_f32().value(), 3.4028235e38F);
    PN_CHECK_EQ(reader.read_f32().value(), 1.4e-45F);
    PN_CHECK_EQ(reader.read_f64().value(), 3.141592653589793);
    PN_CHECK(std::isinf(reader.read_f64().value()));
}

PN_TEST(serialize, strings_round_trip_including_empty_and_embedded_nulls) {
    ByteWriter writer;
    writer.write_string("");
    writer.write_string("albedo");
    writer.write_string(std::string_view{"a\0b", 3});
    writer.write_string(std::string(300, 'x'));  // length needs two varint bytes

    ByteReader reader{writer.data()};
    PN_CHECK(reader.read_string().value().empty());
    PN_CHECK(reader.read_string().value() == "albedo");
    const std::string_view embedded = reader.read_string().value();
    PN_CHECK_EQ(embedded.size(), std::size_t{3});
    PN_CHECK_EQ(embedded[1], '\0');
    PN_CHECK_EQ(reader.read_string().value().size(), std::size_t{300});
    PN_CHECK(reader.at_end());
}

PN_TEST(serialize, varints_use_the_shortest_form) {
    ByteWriter writer;
    writer.write_varint(0);
    PN_CHECK_EQ(writer.size(), std::size_t{1});

    ByteWriter boundary;
    boundary.write_varint(127);
    PN_CHECK_EQ(boundary.size(), std::size_t{1});
    boundary.write_varint(128);
    PN_CHECK_EQ(boundary.size(), std::size_t{3});

    ByteWriter widest;
    widest.write_varint(std::numeric_limits<std::uint64_t>::max());
    PN_CHECK_EQ(widest.size(), std::size_t{10});
}

PN_TEST(serialize, signed_varints_do_not_spend_ten_bytes_on_small_negatives) {
    // Zigzag first. A negative number in two's complement has its high bits set,
    // and a plain varint spends a byte on each of them.
    ByteWriter writer;
    writer.write_svarint(-1);
    PN_CHECK_EQ(writer.size(), std::size_t{1});

    ByteWriter values;
    for (std::int64_t value : {std::int64_t{0}, std::int64_t{-1}, std::int64_t{1},
                               std::int64_t{-1000000},
                               std::numeric_limits<std::int64_t>::min(),
                               std::numeric_limits<std::int64_t>::max()}) {
        values.write_svarint(value);
    }
    ByteReader reader{values.data()};
    PN_CHECK_EQ(reader.read_svarint().value(), std::int64_t{0});
    PN_CHECK_EQ(reader.read_svarint().value(), std::int64_t{-1});
    PN_CHECK_EQ(reader.read_svarint().value(), std::int64_t{1});
    PN_CHECK_EQ(reader.read_svarint().value(), std::int64_t{-1000000});
    PN_CHECK_EQ(reader.read_svarint().value(), std::numeric_limits<std::int64_t>::min());
    PN_CHECK_EQ(reader.read_svarint().value(), std::numeric_limits<std::int64_t>::max());
    PN_CHECK(reader.at_end());
}

// -------------------------------------------------------------------
// Rejecting input that is not canonical or not there
// -------------------------------------------------------------------

PN_TEST(serialize, a_non_shortest_varint_is_rejected) {
    // 0x80 0x00 decodes to zero, and so does 0x00. Accepting both would mean two
    // byte strings mean the same thing, and "identical input produces identical
    // bytes" would hold in only one direction.
    const std::vector<std::byte> redundant = from_hex("8000");
    ByteReader reader{redundant};
    const auto result = reader.read_varint();
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::corrupt_data));

    // The shortest form of the same value is accepted.
    const std::vector<std::byte> canonical = from_hex("00");
    ByteReader good{canonical};
    PN_CHECK_EQ(good.read_varint().value(), std::uint64_t{0});
}

PN_TEST(serialize, an_overlong_varint_is_rejected) {
    // Eleven continuation bytes, and ten whose payload exceeds 64 bits.
    const std::vector<std::byte> too_long = from_hex("8080808080808080808080");
    ByteReader reader{too_long};
    PN_CHECK(!reader.read_varint().has_value());

    const std::vector<std::byte> too_wide = from_hex("80808080808080808002");
    ByteReader wide{too_wide};
    const auto result = wide.read_varint();
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::corrupt_data));
}

PN_TEST(serialize, a_boolean_that_is_neither_zero_nor_one_is_rejected) {
    const std::vector<std::byte> odd = from_hex("02");
    ByteReader reader{odd};
    PN_CHECK(!reader.read_bool().has_value());
}

PN_TEST(serialize, reading_past_the_end_is_an_error_not_a_crash) {
    // Serialized data is the most common untrusted input an engine has.
    ByteWriter writer;
    writer.write_u16(7);
    ByteReader reader{writer.data()};

    PN_CHECK(reader.read_u32().has_value() == false);
    PN_CHECK(reader.read_u16().has_value());
    PN_CHECK(!reader.read_u8().has_value());
    PN_CHECK(reader.at_end());
}

PN_TEST(serialize, a_string_length_beyond_the_buffer_is_rejected) {
    // The shape a fuzzer finds first: a plausible header and a length field
    // pointing past the end.
    const std::vector<std::byte> lying = from_hex("ff01");  // length 255, one byte of data
    ByteReader reader{lying};
    const auto result = reader.read_string();
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::corrupt_data));
}

// -------------------------------------------------------------------
// Blob framing
// -------------------------------------------------------------------

PN_TEST(blob, frames_and_unframes_a_payload) {
    ByteWriter payload;
    payload.write_string("scene");
    payload.write_u32(42);

    ByteWriter framed;
    pn::core::write_blob(framed, 3, payload.data());

    const auto blob = pn::core::read_blob(framed.data());
    PN_REQUIRE(blob.has_value());
    PN_CHECK_EQ(blob.value().payload_version, std::uint16_t{3});

    ByteReader reader{blob.value().payload};
    PN_CHECK(reader.read_string().value() == "scene");
    PN_CHECK_EQ(reader.read_u32().value(), std::uint32_t{42});
}

PN_TEST(blob, an_empty_payload_is_valid) {
    ByteWriter framed;
    pn::core::write_blob(framed, 1, {});
    const auto blob = pn::core::read_blob(framed.data());
    PN_REQUIRE(blob.has_value());
    PN_CHECK(blob.value().payload.empty());
}

PN_TEST(blob, each_way_of_being_wrong_reports_a_different_category) {
    // A caller's response differs: a wrong magic means the wrong file was
    // opened, an unsupported version means the tooling is too old, and a
    // checksum mismatch means the bytes are damaged. One error category for all
    // three would make them indistinguishable at the call site.
    ByteWriter payload;
    payload.write_u32(1);
    ByteWriter framed;
    pn::core::write_blob(framed, 1, payload.data());
    const std::vector<std::byte> good = to_bytes(framed.data());

    std::vector<std::byte> wrong_magic = good;
    wrong_magic[0] = static_cast<std::byte>('X');
    PN_CHECK(pn::core::read_blob(wrong_magic).error().is(ErrorCategory::invalid_argument));

    std::vector<std::byte> wrong_format = good;
    wrong_format[4] = static_cast<std::byte>(99);
    PN_CHECK(pn::core::read_blob(wrong_format).error().is(ErrorCategory::unsupported));

    std::vector<std::byte> corrupted = good;
    corrupted[Blob::kHeaderBytes] = static_cast<std::byte>(
        static_cast<unsigned char>(corrupted[Blob::kHeaderBytes]) ^ 0x01U);
    PN_CHECK(pn::core::read_blob(corrupted).error().is(ErrorCategory::corrupt_data));

    std::vector<std::byte> truncated = good;
    truncated.pop_back();
    PN_CHECK(pn::core::read_blob(truncated).error().is(ErrorCategory::corrupt_data));

    PN_CHECK(pn::core::read_blob(std::span<const std::byte>{}).error().is(
        ErrorCategory::corrupt_data));
}

PN_TEST(blob, a_corrupted_length_field_is_caught_by_the_checksum) {
    // The field that turns a bad byte into an out-of-bounds read, which is why
    // the checksum covers the header and not only the payload.
    ByteWriter payload;
    for (int index = 0; index < 64; ++index) {
        payload.write_u8(static_cast<std::uint8_t>(index));
    }
    ByteWriter framed;
    pn::core::write_blob(framed, 1, payload.data());

    std::vector<std::byte> tampered = to_bytes(framed.data());
    tampered[8] = static_cast<std::byte>(0xFFU);  // first byte of the length
    const auto result = pn::core::read_blob(tampered);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::corrupt_data));
}

// -------------------------------------------------------------------
// Golden data and schema migration
// -------------------------------------------------------------------

namespace {

/// Version 1 of a made-up record: a name and a count.
struct MaterialV1 {
    std::string_view name;
    std::uint32_t pass_count;
};

/// Version 2 adds a roughness. Old files do not have it, and must not be
/// rejected for that.
struct Material {
    std::string_view name;
    std::uint32_t pass_count;
    float roughness;

    static constexpr std::uint16_t kVersion = 2;
    static constexpr float kDefaultRoughness = 0.5F;
};

void write_v1(ByteWriter& out, const MaterialV1& material) {
    out.write_string(material.name);
    out.write_varint(material.pass_count);
}

void write_current(ByteWriter& out, const Material& material) {
    out.write_string(material.name);
    out.write_varint(material.pass_count);
    out.write_f32(material.roughness);
}

/// Reads either version, filling in what an older one did not carry.
pn::core::Expected<Material> read_material(ByteReader& reader, std::uint16_t version) {
    if (version == 0 || version > Material::kVersion) {
        return pn::core::fail(ErrorCategory::unsupported, "unknown material version");
    }
    Material material{};
    PN_TRY_ASSIGN(material.name, reader.read_string());
    PN_TRY_ASSIGN(const std::uint64_t passes, reader.read_varint());
    material.pass_count = static_cast<std::uint32_t>(passes);
    if (version >= 2) {
        PN_TRY_ASSIGN(material.roughness, reader.read_f32());
    } else {
        // The migration. A default here is a decision about the old data's
        // meaning, not a placeholder: it says what a file written before the
        // field existed should be taken to have meant.
        material.roughness = Material::kDefaultRoughness;
    }
    return material;
}

}  // namespace

PN_TEST(golden, a_committed_byte_sequence_still_decodes) {
    // The bytes below were produced by this format and are pinned deliberately.
    // A round-trip test proves only that the reader and writer agree with each
    // other, which they would even if both changed together - and a format that
    // silently changes is a format that cannot read yesterday's save file.
    //
    // A failure here is either a deliberate format change, which breaks every
    // stored asset and every save, or an accident. Neither should pass quietly.
    constexpr std::string_view kGoldenV1 =
        "504e424c010001000f000000"        // magic "PNBL", format 1, payload 1, length 15
        "0d6772616e6974655f726f756768"    // varint 13, then "granite_rough"
        "04"                              // varint 4, the pass count
        "d3b25aec";                       // crc32 over the header and payload

    const std::vector<std::byte> bytes = from_hex(kGoldenV1);
    const auto blob = pn::core::read_blob(bytes);
    PN_REQUIRE(blob.has_value());
    PN_CHECK_EQ(blob.value().payload_version, std::uint16_t{1});

    ByteReader reader{blob.value().payload};
    const auto material = read_material(reader, blob.value().payload_version);
    PN_REQUIRE(material.has_value());
    PN_CHECK(material.value().name == "granite_rough");
    PN_CHECK_EQ(material.value().pass_count, std::uint32_t{4});
    PN_CHECK_EQ(material.value().roughness, Material::kDefaultRoughness);
}

PN_TEST(golden, writing_the_same_values_reproduces_the_committed_bytes) {
    // The other direction. If this passes and the decode above fails, the reader
    // changed; if this fails and the decode passes, the writer did.
    ByteWriter payload;
    write_v1(payload, MaterialV1{"granite_rough", 4});
    ByteWriter framed;
    pn::core::write_blob(framed, 1, payload.data());

    PN_CHECK(to_hex(framed.data()) ==
             "504e424c010001000f0000000d6772616e6974655f726f75676804d3b25aec");
}

PN_TEST(golden, a_version_one_payload_migrates_and_round_trips_as_version_two) {
    // The acceptance criterion, end to end: read an old payload, fill in the
    // field it never had, write it out at the current version, and read it back
    // unchanged.
    const std::vector<std::byte> old_bytes = from_hex(
        "504e424c010001000f0000000d6772616e6974655f726f75676804d3b25aec");

    const auto old_blob = pn::core::read_blob(old_bytes);
    PN_REQUIRE(old_blob.has_value());
    ByteReader old_reader{old_blob.value().payload};
    const auto migrated = read_material(old_reader, old_blob.value().payload_version);
    PN_REQUIRE(migrated.has_value());

    ByteWriter payload;
    write_current(payload, migrated.value());
    ByteWriter framed;
    pn::core::write_blob(framed, Material::kVersion, payload.data());

    const auto new_blob = pn::core::read_blob(framed.data());
    PN_REQUIRE(new_blob.has_value());
    PN_CHECK_EQ(new_blob.value().payload_version, Material::kVersion);

    ByteReader new_reader{new_blob.value().payload};
    const auto reloaded = read_material(new_reader, new_blob.value().payload_version);
    PN_REQUIRE(reloaded.has_value());
    PN_CHECK(reloaded.value().name == migrated.value().name);
    PN_CHECK_EQ(reloaded.value().pass_count, migrated.value().pass_count);
    PN_CHECK_EQ(reloaded.value().roughness, migrated.value().roughness);
    PN_CHECK(new_reader.at_end());
}

PN_TEST(golden, a_version_from_the_future_is_refused_rather_than_guessed) {
    ByteWriter payload;
    payload.write_string("unknown");
    ByteWriter framed;
    pn::core::write_blob(framed, 99, payload.data());

    const auto blob = pn::core::read_blob(framed.data());
    PN_REQUIRE(blob.has_value());  // the framing is fine; the schema is not
    ByteReader reader{blob.value().payload};
    const auto result = read_material(reader, blob.value().payload_version);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::unsupported));
}

// -------------------------------------------------------------------
// Determinism, over many values
// -------------------------------------------------------------------

PN_TEST(serialize, every_value_round_trips_and_encodes_identically_twice) {
    Random random{20260821, 11};

    for (int round = 0; round < 20000; ++round) {
        const std::uint64_t value = random.next_u64();
        const std::int64_t signed_value = static_cast<std::int64_t>(random.next_u64());
        const double real = std::bit_cast<double>(random.next_u64());
        const std::size_t length = random.uniform(0, 40);
        std::string text;
        for (std::size_t index = 0; index < length; ++index) {
            text.push_back(static_cast<char>(random.uniform(0, 255)));
        }

        auto encode = [&] {
            ByteWriter writer;
            writer.write_varint(value);
            writer.write_svarint(signed_value);
            writer.write_f64(real);
            writer.write_string(text);
            return to_bytes(writer.data());
        };

        const std::vector<std::byte> first = encode();
        PN_REQUIRE(first == encode());

        ByteReader reader{first};
        PN_REQUIRE_EQ(reader.read_varint().value(), value);
        PN_REQUIRE_EQ(reader.read_svarint().value(), signed_value);
        // Compared as bits, because a NaN generated at random is not equal to
        // itself and the serializer's job is to preserve the pattern.
        PN_REQUIRE_EQ(std::bit_cast<std::uint64_t>(reader.read_f64().value()),
                      std::bit_cast<std::uint64_t>(real));
        PN_REQUIRE(reader.read_string().value() == text);
        PN_REQUIRE(reader.at_end());
    }
}
