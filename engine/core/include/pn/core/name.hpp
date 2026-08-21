// Pention Engine - core/name.hpp
// Requirement: PN-PLT-010 (string, interned name, and handle systems)
// Decision:    ADR-0004

#ifndef PN_CORE_NAME_HPP
#define PN_CORE_NAME_HPP

#include "pn/core/assert.hpp"
#include "pn/core/hash.hpp"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <string_view>
#include <vector>

namespace pn::core {

/// An interned string, compared as an integer.
///
/// Engines compare names constantly - a material parameter against a shader
/// binding, a bone against a retarget rule, an asset path against a cache key -
/// and doing that with `strcmp` puts a byte loop and a cache miss on paths that
/// run per object per frame. Interning moves the cost to load time: comparing
/// two `Name`s is comparing two integers.
///
/// A `Name` is meaningful only against the table that produced it. Two tables
/// will assign different indices to the same text, so a `Name` from one is not
/// a `Name` from the other. That is why the table is an explicit object rather
/// than a hidden global: a global would make the mistake invisible, and would
/// serialize badly the first time two worlds were loaded at once.
class Name {
public:
    using index_type = std::uint32_t;
    static constexpr index_type kInvalid = static_cast<index_type>(-1);

    constexpr Name() noexcept = default;
    constexpr explicit Name(index_type index) noexcept : index_{index} {}

    constexpr index_type index() const noexcept { return index_; }
    constexpr bool is_valid() const noexcept { return index_ != kInvalid; }

    friend constexpr bool operator==(Name left, Name right) noexcept {
        return left.index_ == right.index_;
    }

    /// Orders by index, which is interning order - **not** alphabetical.
    ///
    /// Useful for putting names in a sorted container or deduplicating a list,
    /// and wrong for anything a person will read. A UI that sorts by this
    /// produces an order that changes when an unrelated asset loads first.
    friend constexpr bool operator<(Name left, Name right) noexcept {
        return left.index_ < right.index_;
    }

private:
    index_type index_ = kInvalid;
};

template <>
struct Hash<Name> {
    constexpr std::uint64_t operator()(Name name) const noexcept {
        return mix64(name.index(), 0);
    }
};

/// The table that owns interned text.
///
/// Templated on the hasher so a test can supply a deliberately colliding one.
/// Collision behaviour is the property most worth testing here and the hardest
/// to reach with the real hash: two distinct strings that share a hash must stay
/// two distinct names, and a table that trusts the hash alone will silently
/// merge them - producing a bug where one asset quietly becomes another.
///
/// **Single-threaded.** Interning mutates the table, and nothing here is
/// synchronized. A build that interns from worker threads needs either a
/// per-thread table merged at a barrier or a concurrent design, and neither is
/// written; saying so is cheaper than a data race discovered in a shipping
/// build.
template <typename Hasher = Hash<std::string_view>>
class BasicNameTable {
public:
    using size_type = std::size_t;

    BasicNameTable() = default;
    BasicNameTable(const BasicNameTable&) = delete;
    BasicNameTable& operator=(const BasicNameTable&) = delete;
    BasicNameTable(BasicNameTable&&) noexcept = default;
    BasicNameTable& operator=(BasicNameTable&&) noexcept = default;

    size_type size() const noexcept { return entries_.size(); }
    bool empty() const noexcept { return entries_.empty(); }

    /// Total bytes of interned text, excluding block slack.
    size_type text_bytes() const noexcept { return text_bytes_; }

    /// Interns `text`, returning the existing name if it is already there.
    Name intern(std::string_view text) {
        const std::uint64_t hash = Hasher{}(text);

        if (!buckets_.empty()) {
            const Name existing = lookup(text, hash);
            if (existing.is_valid()) {
                return existing;
            }
        }

        if (buckets_.empty() || (entries_.size() + 1) * 8 >= buckets_.size() * 7) {
            grow();
        }

        const char* stored = store(text);
        const Name name{static_cast<Name::index_type>(entries_.size())};
        entries_.push_back(Entry{hash, stored, static_cast<std::uint32_t>(text.size())});
        text_bytes_ += text.size();
        place(name.index());
        return name;
    }

    /// The name for `text` if it has been interned, or an invalid name.
    ///
    /// Distinct from `intern` because a lookup that silently interns turns a
    /// typo in a config file into a new asset name rather than an error.
    Name find(std::string_view text) const noexcept {
        if (buckets_.empty()) {
            return Name{};
        }
        return lookup(text, Hasher{}(text));
    }

    /// The text behind a name.
    ///
    /// The returned view stays valid for the table's lifetime, including across
    /// any number of later interns. Text lives in blocks that are allocated once
    /// and never moved, precisely so this is true: a single growing buffer would
    /// invalidate every view handed out before it reallocated, and that failure
    /// appears far from its cause.
    std::string_view text(Name name) const noexcept {
        PN_ASSERT_MSG(name.is_valid(), "text() on an invalid Name");
        PN_ASSERT_MSG(name.index() < entries_.size(), "Name is not from this table");
        const Entry& entry = entries_[name.index()];
        return std::string_view{entry.data, entry.size};
    }

    /// The longest probe run in the index, for tests.
    size_type longest_probe() const noexcept {
        size_type worst = 0;
        for (size_type index = 0; index < buckets_.size(); ++index) {
            if (buckets_[index] == kEmptyBucket) {
                continue;
            }
            const size_type distance = bucket_distance(index);
            worst = distance > worst ? distance : worst;
        }
        return worst;
    }

private:
    static constexpr std::uint32_t kEmptyBucket = static_cast<std::uint32_t>(-1);
    static constexpr size_type kMinimumBuckets = 16;
    static constexpr size_type kBlockBytes = 64 * 1024;

    struct Entry {
        std::uint64_t hash;
        const char* data;
        std::uint32_t size;
    };

    struct Block {
        std::unique_ptr<char[]> data;
        size_type used = 0;
        size_type capacity = 0;
    };

    /// How far the entry in `bucket` sits from its ideal bucket.
    size_type bucket_distance(size_type bucket) const noexcept {
        const size_type ideal =
            static_cast<size_type>(entries_[buckets_[bucket]].hash) & mask_;
        return (bucket - ideal) & mask_;
    }

    Name lookup(std::string_view text, std::uint64_t hash) const noexcept {
        size_type bucket = static_cast<size_type>(hash) & mask_;
        size_type distance = 0;
        while (buckets_[bucket] != kEmptyBucket) {
            // Robin Hood ordering: an entry nearer its ideal bucket than we are
            // to ours could not be sitting here if our text were further along,
            // so the search stops rather than walking the rest of the run.
            if (bucket_distance(bucket) < distance) {
                return Name{};
            }
            const Entry& entry = entries_[buckets_[bucket]];
            // The hash is compared first as a cheap reject, and the bytes are
            // compared always. Trusting the hash alone would merge two distinct
            // strings that happen to collide - one asset quietly becoming
            // another, with nothing to see at the point of failure.
            if (entry.hash == hash && entry.size == text.size() &&
                std::memcmp(entry.data, text.data(), text.size()) == 0) {
                return Name{buckets_[bucket]};
            }
            bucket = (bucket + 1) & mask_;
            ++distance;
        }
        return Name{};
    }

    /// Places an entry index, displacing any entry nearer its ideal bucket.
    ///
    /// Plain linear probing clumps as the table fills, and the longest run grows
    /// far faster than the average - measured at 133 buckets for a hundred
    /// thousand names before this change, against 47 for ten thousand. Every
    /// lookup still returned the right answer, so nothing failed; the table was
    /// simply no longer the thing it claimed to be. Robin Hood ordering keeps
    /// the run sorted by distance, which bounds the variance that was out of
    /// control.
    void place(std::uint32_t entry_index) noexcept {
        size_type bucket = static_cast<size_type>(entries_[entry_index].hash) & mask_;
        size_type distance = 0;
        for (;;) {
            if (buckets_[bucket] == kEmptyBucket) {
                buckets_[bucket] = entry_index;
                return;
            }
            const size_type occupant_distance = bucket_distance(bucket);
            if (occupant_distance < distance) {
                const std::uint32_t displaced = buckets_[bucket];
                buckets_[bucket] = entry_index;
                entry_index = displaced;
                distance = occupant_distance;
            }
            bucket = (bucket + 1) & mask_;
            ++distance;
        }
    }

    void grow() {
        const size_type new_size =
            buckets_.empty() ? kMinimumBuckets : buckets_.size() * 2;
        buckets_.assign(new_size, kEmptyBucket);
        mask_ = new_size - 1;
        // Re-probed from the stored hashes rather than recomputed. For a table
        // of asset paths that is the difference between a rehash that walks
        // every byte of every name and one that does not.
        for (size_type index = 0; index < entries_.size(); ++index) {
            place(static_cast<std::uint32_t>(index));
        }
    }

    /// Copies `text` into stable storage and returns a pointer to it.
    const char* store(std::string_view text) {
        if (blocks_.empty() || blocks_.back().used + text.size() > blocks_.back().capacity) {
            // A string larger than a block gets a block of its own rather than
            // being split; splitting would mean text() could not return a
            // contiguous view.
            const size_type capacity = text.size() > kBlockBytes ? text.size() : kBlockBytes;
            blocks_.push_back(Block{std::make_unique<char[]>(capacity), 0, capacity});
        }
        Block& block = blocks_.back();
        char* destination = block.data.get() + block.used;
        if (!text.empty()) {
            std::memcpy(destination, text.data(), text.size());
        }
        block.used += text.size();
        return destination;
    }

    std::vector<Entry> entries_;
    /// Bucket -> entry index. Open addressing, linear probing; no deletion,
    /// because interning is append-only and a name is never withdrawn.
    std::vector<std::uint32_t> buckets_;
    std::vector<Block> blocks_;
    size_type mask_ = 0;
    size_type text_bytes_ = 0;
};

using NameTable = BasicNameTable<>;

}  // namespace pn::core

#endif  // PN_CORE_NAME_HPP
