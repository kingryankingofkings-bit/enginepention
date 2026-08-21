// Pention Engine - core/sparse_set.hpp
// Requirement: PN-PLT-009 (cache-aware containers), PN-OBJ-002
// Decision:    ADR-0004

#ifndef PN_CORE_SPARSE_SET_HPP
#define PN_CORE_SPARSE_SET_HPP

#include "pn/core/assert.hpp"

#include <cstddef>
#include <cstdint>
#include <utility>
#include <vector>

namespace pn::core {

/// A set of integer-keyed values with contiguous iteration.
///
/// The container an entity system is built on, and the reason is what iteration
/// costs. A hash map keyed on entity id gives O(1) lookup and scattered
/// iteration: walking every component touches slots in hash order, which is
/// effectively random order, and the prefetcher can do nothing with it. Here the
/// values live in a dense array in insertion order, so a system that walks all
/// of them walks memory forwards.
///
/// The trade is memory. The sparse index is one `uint32_t` per *id*, not per
/// stored value, so a set holding ten entities with ids near a million costs
/// four megabytes. That is the right trade when ids are dense and the wrong one
/// when they are not, and it is stated rather than discovered: `sparse_bytes()`
/// reports it.
template <typename T>
class SparseSet {
public:
    using key_type = std::uint32_t;
    using size_type = std::size_t;

    static constexpr key_type kInvalid = static_cast<key_type>(-1);

    SparseSet() = default;

    size_type size() const noexcept { return dense_keys_.size(); }
    bool empty() const noexcept { return dense_keys_.empty(); }

    /// Bytes held by the sparse index. Grows with the largest id, not the count.
    size_type sparse_bytes() const noexcept { return sparse_.size() * sizeof(key_type); }

    bool contains(key_type key) const noexcept {
        return key < sparse_.size() && sparse_[key] != kInvalid;
    }

    /// Inserts, leaving an existing value untouched. Reports whether it inserted.
    bool insert(key_type key, T value) {
        if (contains(key)) {
            return false;
        }
        if (key >= sparse_.size()) {
            sparse_.resize(static_cast<size_type>(key) + 1, kInvalid);
        }
        sparse_[key] = static_cast<key_type>(dense_keys_.size());
        dense_keys_.push_back(key);
        values_.push_back(std::move(value));
        return true;
    }

    void insert_or_assign(key_type key, T value) {
        if (T* existing = find(key)) {
            *existing = std::move(value);
            return;
        }
        insert(key, std::move(value));
    }

    T* find(key_type key) noexcept {
        return contains(key) ? &values_[sparse_[key]] : nullptr;
    }

    const T* find(key_type key) const noexcept {
        return contains(key) ? &values_[sparse_[key]] : nullptr;
    }

    T& at(key_type key) noexcept {
        T* found = find(key);
        PN_ASSERT_MSG(found != nullptr, "SparseSet::at on an absent key");
        return *found;
    }

    /// Removes `key`, reporting whether it was present.
    ///
    /// The last element is moved into the hole, so the dense array stays
    /// contiguous. Iteration order therefore changes on removal - which is why
    /// nothing here promises an order, and why a caller that needs one must
    /// keep it itself rather than discover the hard way that removal shuffled it.
    bool remove(key_type key) {
        if (!contains(key)) {
            return false;
        }
        const key_type slot = sparse_[key];
        const key_type last = static_cast<key_type>(dense_keys_.size() - 1);

        if (slot != last) {
            values_[slot] = std::move(values_[last]);
            dense_keys_[slot] = dense_keys_[last];
            sparse_[dense_keys_[slot]] = slot;
        }

        values_.pop_back();
        dense_keys_.pop_back();
        sparse_[key] = kInvalid;
        return true;
    }

    void clear() noexcept {
        for (key_type key : dense_keys_) {
            sparse_[key] = kInvalid;
        }
        dense_keys_.clear();
        values_.clear();
    }

    /// The values, contiguous. This is the whole point of the container.
    T* values() noexcept { return values_.data(); }
    const T* values() const noexcept { return values_.data(); }

    /// The key for each value, in the same order.
    const key_type* keys() const noexcept { return dense_keys_.data(); }

    T* begin() noexcept { return values_.data(); }
    T* end() noexcept { return values_.data() + values_.size(); }
    const T* begin() const noexcept { return values_.data(); }
    const T* end() const noexcept { return values_.data() + values_.size(); }

    T& operator[](size_type dense_index) noexcept {
        PN_ASSERT_MSG(dense_index < values_.size(), "SparseSet dense index out of range");
        return values_[dense_index];
    }

    key_type key_at(size_type dense_index) const noexcept {
        PN_ASSERT_MSG(dense_index < dense_keys_.size(),
                      "SparseSet dense index out of range");
        return dense_keys_[dense_index];
    }

private:
    /// Indexed by key; holds the dense position or `kInvalid`.
    std::vector<key_type> sparse_;
    /// The key for each dense slot, so removal can repair the sparse index.
    std::vector<key_type> dense_keys_;
    std::vector<T> values_;
};

}  // namespace pn::core

#endif  // PN_CORE_SPARSE_SET_HPP
