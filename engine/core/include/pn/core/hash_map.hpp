// Pention Engine - core/hash_map.hpp
// Requirement: PN-PLT-009 (cache-aware containers, bounds-checked in debug)
// Decision:    ADR-0004

#ifndef PN_CORE_HASH_MAP_HPP
#define PN_CORE_HASH_MAP_HPP

#include "pn/core/assert.hpp"
#include "pn/core/hash.hpp"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>
#include <vector>

namespace pn::core {

/// An open-addressing hash map with linear probing.
///
/// ## Why not a node-based map
///
/// The standard library's `unordered_map` is specified in a way that forces
/// separate chaining with stable node addresses, so every lookup is a pointer
/// chase into memory the prefetcher had no reason to fetch. That is the wrong
/// shape for a table read thousands of times a frame. Here keys and values live
/// in one contiguous array and a probe walks forward through it, so a miss
/// usually costs one cache line rather than one per collision.
///
/// ## Why Robin Hood ordering
///
/// Plain linear probing has a failure that only appears at scale: as the table
/// fills, occupied slots clump, and the longest probe run grows far faster than
/// the average. Measured on this table before the change, a hundred thousand
/// integer keys produced a run of **146** slots. Every lookup still returned the
/// right answer, so nothing failed - the table was simply no longer the thing it
/// claimed to be.
///
/// Robin Hood ordering fixes it by keeping the run sorted by probe distance: an
/// entry being inserted displaces any entry that is closer to its ideal slot,
/// and carries that one onward. Nobody gets to sit near home while a newcomer
/// walks past. That bounds the *variance*, which is the quantity that was out of
/// control.
///
/// It also earns a cheaper lookup: probing may stop as soon as the entry sitting
/// in a slot is closer to home than the key being sought, because a key with a
/// larger distance would have displaced it.
///
/// ## Why backward-shift deletion rather than tombstones
///
/// The usual open-addressing erase leaves a tombstone: a slot marked deleted so
/// later probes keep walking past it. Tombstones accumulate. A table that
/// inserts and erases every frame - which describes most engine tables - fills
/// with them until every lookup walks the whole probe run, and the only cure is
/// a rehash triggered by something nobody thought to measure.
///
/// This instead shifts the following run backwards to close the hole, so the
/// invariant "a probe run contains no gaps" holds after every erase and the
/// table's cost does not depend on its history. It is more code, and the code is
/// in one place.
///
/// Robin Hood makes that shift simpler as well as faster. Because distances
/// along a run never decrease, an entry may move back exactly when its distance
/// is not already zero, and the scan may stop at the first entry that cannot
/// move. Under plain linear probing that rule is wrong - the scan has to
/// continue past such an entry - and this file had that bug for one commit.
///
/// ## Not for adversarial keys
///
/// `mix64` is not collision-resistant against chosen input. A table keyed on
/// data an untrusted party controls needs a keyed hash and graceful degradation,
/// neither of which this has.
template <typename Key, typename Value, typename Hasher = Hash<Key>>
class HashMap {
public:
    using key_type = Key;
    using mapped_type = Value;
    using size_type = std::size_t;

    HashMap() = default;

    explicit HashMap(size_type minimum_capacity) { reserve(minimum_capacity); }

    HashMap(const HashMap& other) { copy_from(other); }

    HashMap(HashMap&& other) noexcept
        : control_{std::move(other.control_)},
          slots_{std::move(other.slots_)},
          size_{other.size_},
          mask_{other.mask_} {
        other.control_.clear();
        other.slots_.clear();
        other.size_ = 0;
        other.mask_ = 0;
    }

    HashMap& operator=(const HashMap& other) {
        if (this != &other) {
            clear();
            control_.clear();
            slots_.clear();
            mask_ = 0;
            copy_from(other);
        }
        return *this;
    }

    HashMap& operator=(HashMap&& other) noexcept {
        if (this != &other) {
            clear();
            control_ = std::move(other.control_);
            slots_ = std::move(other.slots_);
            size_ = other.size_;
            mask_ = other.mask_;
            other.control_.clear();
            other.slots_.clear();
            other.size_ = 0;
            other.mask_ = 0;
        }
        return *this;
    }

    ~HashMap() { clear(); }

    size_type size() const noexcept { return size_; }
    bool empty() const noexcept { return size_ == 0; }
    size_type capacity() const noexcept { return control_.size(); }

    /// Ensures the table can hold `count` entries without rehashing.
    void reserve(size_type count) {
        const size_type needed = required_capacity(count);
        if (needed > control_.size()) {
            rehash(needed);
        }
    }

    /// Inserts, leaving an existing value untouched.
    ///
    /// Returns whether an insertion happened, which is the question a caller
    /// asking "is this the first time I've seen this key" is actually asking.
    bool insert(const Key& key, Value value) {
        return emplace_impl(key, std::move(value), false);
    }

    /// Inserts, or overwrites an existing value.
    void insert_or_assign(const Key& key, Value value) {
        (void)emplace_impl(key, std::move(value), true);
    }

    Value* find(const Key& key) noexcept {
        const size_type slot = locate(key);
        return slot == kNotFound ? nullptr : value_at(slot);
    }

    const Value* find(const Key& key) const noexcept {
        const size_type slot = locate(key);
        return slot == kNotFound ? nullptr : value_at(slot);
    }

    bool contains(const Key& key) const noexcept { return locate(key) != kNotFound; }

    /// The value for `key`, aborting in debug if it is absent.
    Value& at(const Key& key) noexcept {
        Value* found = find(key);
        PN_ASSERT_MSG(found != nullptr, "HashMap::at on a missing key");
        return *found;
    }

    /// Removes `key`, reporting whether it was there.
    bool erase(const Key& key) {
        const size_type slot = locate(key);
        if (slot == kNotFound) {
            return false;
        }
        erase_at(slot);
        return true;
    }

    void clear() noexcept {
        if (control_.empty()) {
            size_ = 0;
            return;
        }
        for (size_type index = 0; index < control_.size(); ++index) {
            if (control_[index] != kEmpty) {
                destroy(index);
                control_[index] = kEmpty;
            }
        }
        size_ = 0;
    }

    /// Visits every entry. Order is unspecified and depends on the hash.
    template <typename Visitor>
    void for_each(Visitor&& visit) const {
        for (size_type index = 0; index < control_.size(); ++index) {
            if (control_[index] != kEmpty) {
                visit(*key_at(index), *value_at(index));
            }
        }
    }

    template <typename Visitor>
    void for_each(Visitor&& visit) {
        for (size_type index = 0; index < control_.size(); ++index) {
            if (control_[index] != kEmpty) {
                visit(*key_at(index), *value_at(index));
            }
        }
    }

    /// The longest probe run currently in the table.
    ///
    /// Exposed for tests, not for callers. A table whose worst probe grows with
    /// its history has a deletion bug, and that is invisible from the outside
    /// because every lookup still returns the right answer - just slower, for
    /// ever.
    size_type longest_probe() const noexcept {
        size_type worst = 0;
        for (size_type index = 0; index < control_.size(); ++index) {
            if (control_[index] == kEmpty) {
                continue;
            }
            const size_type distance = distance_of(index);
            worst = distance > worst ? distance : worst;
        }
        return worst;
    }

private:
    static constexpr std::uint8_t kEmpty = 0;
    static constexpr size_type kNotFound = static_cast<size_type>(-1);
    static constexpr size_type kMinimumCapacity = 8;

    struct Slot {
        std::uint64_t hash = 0;
        alignas(Key) std::byte key_storage[sizeof(Key)]{};
        alignas(Value) std::byte value_storage[sizeof(Value)]{};
    };

    /// The top bits of the hash, with the high bit forced so it is never the
    /// empty sentinel.
    ///
    /// A one-byte tag lets a probe reject a slot without touching the key at
    /// all, which for a string key is the difference between a comparison and a
    /// second cache miss.
    static std::uint8_t tag_of(std::uint64_t hash) noexcept {
        return static_cast<std::uint8_t>((hash >> 56) | 0x80U);
    }

    static size_type required_capacity(size_type count) noexcept {
        // Grow at seven eighths. Linear probing degrades sharply as a table
        // approaches full, and the last eighth is where it happens.
        size_type needed = kMinimumCapacity;
        while (count * 8 >= needed * 7) {
            needed *= 2;
        }
        return needed;
    }

    Key* key_at(size_type index) noexcept {
        return std::launder(reinterpret_cast<Key*>(slots_[index].key_storage));
    }
    const Key* key_at(size_type index) const noexcept {
        return std::launder(reinterpret_cast<const Key*>(slots_[index].key_storage));
    }
    Value* value_at(size_type index) noexcept {
        return std::launder(reinterpret_cast<Value*>(slots_[index].value_storage));
    }
    const Value* value_at(size_type index) const noexcept {
        return std::launder(reinterpret_cast<const Value*>(slots_[index].value_storage));
    }

    void construct(size_type index, std::uint64_t hash, Key&& key, Value&& value) {
        ::new (static_cast<void*>(slots_[index].key_storage)) Key(std::move(key));
        ::new (static_cast<void*>(slots_[index].value_storage)) Value(std::move(value));
        slots_[index].hash = hash;
        control_[index] = tag_of(hash);
    }

    void destroy(size_type index) noexcept {
        key_at(index)->~Key();
        value_at(index)->~Value();
    }

    size_type locate(const Key& key) const noexcept {
        if (control_.empty()) {
            return kNotFound;
        }
        return locate_with_hash(key, Hasher{}(key));
    }

    size_type locate_with_hash(const Key& key, std::uint64_t hash) const noexcept {
        const std::uint8_t tag = tag_of(hash);
        size_type index = static_cast<size_type>(hash) & mask_;
        size_type distance = 0;

        // Terminates on an empty slot - the load factor keeps at least one - or
        // earlier, at an entry nearer its ideal slot than we are to ours. Under
        // Robin Hood ordering that entry could not be sitting there if our key
        // were further along, so the search can stop without walking the rest of
        // the run. A miss on a full-ish table is where that saving shows.
        while (control_[index] != kEmpty) {
            if (distance_of(index) < distance) {
                return kNotFound;
            }
            if (control_[index] == tag && slots_[index].hash == hash &&
                *key_at(index) == key) {
                return index;
            }
            index = (index + 1) & mask_;
            ++distance;
        }
        return kNotFound;
    }

    /// Returns whether a new entry was created.
    bool emplace_impl(const Key& key, Value&& value, bool overwrite) {
        if (control_.empty() || (size_ + 1) * 8 >= control_.size() * 7) {
            rehash(control_.empty() ? kMinimumCapacity : control_.size() * 2);
        }

        const std::uint64_t hash = Hasher{}(key);
        const size_type slot = locate_with_hash(key, hash);
        if (slot != kNotFound) {
            if (overwrite) {
                *value_at(slot) = std::move(value);
            }
            return false;
        }

        insert_new(hash, Key{key}, std::move(value));
        ++size_;
        return true;
    }

    /// Places a key known to be absent, displacing entries that sit closer to
    /// their ideal slot than the one being placed.
    void insert_new(std::uint64_t hash, Key key, Value value) {
        size_type index = static_cast<size_type>(hash) & mask_;
        size_type distance = 0;

        for (;;) {
            if (control_[index] == kEmpty) {
                construct(index, hash, std::move(key), std::move(value));
                return;
            }

            const size_type occupant_distance = distance_of(index);
            if (occupant_distance < distance) {
                // The occupant is nearer home than we are, so it yields the
                // slot and we carry it onward. This is the whole of Robin Hood:
                // the entry that has travelled further wins the argument.
                Key displaced_key = std::move(*key_at(index));
                Value displaced_value = std::move(*value_at(index));
                const std::uint64_t displaced_hash = slots_[index].hash;
                destroy(index);

                construct(index, hash, std::move(key), std::move(value));

                key = std::move(displaced_key);
                value = std::move(displaced_value);
                hash = displaced_hash;
                distance = occupant_distance;
            }

            index = (index + 1) & mask_;
            ++distance;
        }
    }

    /// How far the entry in `index` sits from its ideal slot.
    size_type distance_of(size_type index) const noexcept {
        const size_type ideal = static_cast<size_type>(slots_[index].hash) & mask_;
        return (index - ideal) & mask_;
    }

    /// Closes the hole at `slot` by shifting later entries backwards into it.
    ///
    /// Robin Hood ordering means probe distances never decrease along a run, so
    /// an entry may move back exactly when its distance is not already zero, and
    /// the scan may stop at the first entry that cannot move: everything after
    /// it belongs to a run that starts at or after that slot, and none of it can
    /// reach back past the hole.
    ///
    /// This rule is **wrong** under plain linear probing, where an entry sitting
    /// on its ideal slot can be followed by one that started before the hole.
    /// This file used the rule without the invariant for one commit, and the
    /// property test against `std::map` is what found it. The rule is the same;
    /// what changed is that the insert path now earns it.
    void erase_at(size_type slot) noexcept {
        destroy(slot);

        size_type hole = slot;
        for (;;) {
            const size_type next = (hole + 1) & mask_;
            if (control_[next] == kEmpty || distance_of(next) == 0) {
                break;
            }

            ::new (static_cast<void*>(slots_[hole].key_storage))
                Key(std::move(*key_at(next)));
            ::new (static_cast<void*>(slots_[hole].value_storage))
                Value(std::move(*value_at(next)));
            slots_[hole].hash = slots_[next].hash;
            control_[hole] = control_[next];
            destroy(next);

            hole = next;
        }

        control_[hole] = kEmpty;
        --size_;
    }

    void rehash(size_type new_capacity) {
        PN_ASSERT_MSG((new_capacity & (new_capacity - 1)) == 0,
                      "capacity must be a power of two for the mask to work");

        std::vector<std::uint8_t> old_control = std::move(control_);
        std::vector<Slot> old_slots = std::move(slots_);

        control_.assign(new_capacity, kEmpty);
        slots_.clear();
        slots_.resize(new_capacity);
        mask_ = new_capacity - 1;
        size_ = 0;

        for (size_type index = 0; index < old_control.size(); ++index) {
            if (old_control[index] == kEmpty) {
                continue;
            }
            Key* key = std::launder(reinterpret_cast<Key*>(old_slots[index].key_storage));
            Value* value =
                std::launder(reinterpret_cast<Value*>(old_slots[index].value_storage));
            // The stored hash is reused rather than recomputed. For a string key
            // that is the difference between a rehash that walks every byte of
            // every key and one that does not.
            insert_new(old_slots[index].hash, std::move(*key), std::move(*value));
            ++size_;
            key->~Key();
            value->~Value();
        }
    }

    void copy_from(const HashMap& other) {
        reserve(other.size_);
        other.for_each([this](const Key& key, const Value& value) {
            insert_or_assign(key, value);
        });
    }

    std::vector<std::uint8_t> control_;
    std::vector<Slot> slots_;
    size_type size_ = 0;
    size_type mask_ = 0;
};

}  // namespace pn::core

#endif  // PN_CORE_HASH_MAP_HPP
