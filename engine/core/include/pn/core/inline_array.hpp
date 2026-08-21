// Pention Engine - core/inline_array.hpp
// Requirement: PN-PLT-009 (cache-aware containers, bounds-checked in debug)
// Decision:    ADR-0004, ADR-0007

#ifndef PN_CORE_INLINE_ARRAY_HPP
#define PN_CORE_INLINE_ARRAY_HPP

#include "pn/core/assert.hpp"

#include <cstddef>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

namespace pn::core {

/// A fixed-capacity array whose storage lives inside the object.
///
/// The point is what it does *not* do: no allocation, ever. An engine is full of
/// small collections with a known bound - the barriers before a pass, the joints
/// influencing a vertex, the lights touching a cluster - and giving each one a
/// heap allocation puts a pointer chase and an allocator call on paths that run
/// thousands of times a frame. Storage here is contiguous and adjacent to
/// whatever owns it, so touching the owner usually pulls the contents in too.
///
/// The capacity being fixed is a feature. A container that silently grows hides
/// the moment a bound was wrong; this one reports it, and `try_push_back` lets a
/// caller that expects to hit the bound handle it as data rather than as a bug.
template <typename T, std::size_t Capacity>
class InlineArray {
    static_assert(Capacity > 0, "a zero-capacity array holds nothing and cannot be indexed");

public:
    using value_type = T;
    using size_type = std::size_t;

    constexpr InlineArray() noexcept = default;

    InlineArray(const InlineArray& other) {
        for (size_type index = 0; index < other.size_; ++index) {
            push_back(other[index]);
        }
    }

    InlineArray(InlineArray&& other) noexcept(std::is_nothrow_move_constructible_v<T>) {
        for (size_type index = 0; index < other.size_; ++index) {
            push_back(std::move(other[index]));
        }
        other.clear();
    }

    InlineArray& operator=(const InlineArray& other) {
        if (this != &other) {
            clear();
            for (size_type index = 0; index < other.size_; ++index) {
                push_back(other[index]);
            }
        }
        return *this;
    }

    InlineArray& operator=(InlineArray&& other) noexcept(
        std::is_nothrow_move_constructible_v<T>) {
        if (this != &other) {
            clear();
            for (size_type index = 0; index < other.size_; ++index) {
                push_back(std::move(other[index]));
            }
            other.clear();
        }
        return *this;
    }

    ~InlineArray() { clear(); }

    static constexpr size_type capacity() noexcept { return Capacity; }
    constexpr size_type size() const noexcept { return size_; }
    constexpr bool empty() const noexcept { return size_ == 0; }
    constexpr bool full() const noexcept { return size_ == Capacity; }

    T* data() noexcept { return element(0); }
    const T* data() const noexcept { return element(0); }

    T* begin() noexcept { return data(); }
    T* end() noexcept { return data() + size_; }
    const T* begin() const noexcept { return data(); }
    const T* end() const noexcept { return data() + size_; }

    T& operator[](size_type index) noexcept {
        PN_ASSERT_MSG(index < size_, "InlineArray index out of range");
        return *element(index);
    }

    const T& operator[](size_type index) const noexcept {
        PN_ASSERT_MSG(index < size_, "InlineArray index out of range");
        return *element(index);
    }

    T& front() noexcept {
        PN_ASSERT_MSG(size_ > 0, "front() on an empty InlineArray");
        return *element(0);
    }

    T& back() noexcept {
        PN_ASSERT_MSG(size_ > 0, "back() on an empty InlineArray");
        return *element(size_ - 1);
    }

    /// Appends, aborting in debug if the array is full.
    ///
    /// For the call sites where exceeding the bound means the bound was chosen
    /// wrong, which is most of them.
    template <typename... Args>
    T& emplace_back(Args&&... args) {
        PN_ASSERT_MSG(size_ < Capacity, "InlineArray capacity exceeded");
        T* slot = element(size_);
        ::new (static_cast<void*>(slot)) T(std::forward<Args>(args)...);
        ++size_;
        return *slot;
    }

    T& push_back(const T& value) { return emplace_back(value); }
    T& push_back(T&& value) { return emplace_back(std::move(value)); }

    /// Appends if there is room, reporting whether there was.
    ///
    /// For the call sites where hitting the bound is expected - a fixed-size
    /// queue of pending events, say - and is data rather than a defect.
    [[nodiscard]] bool try_push_back(const T& value) {
        if (full()) {
            return false;
        }
        emplace_back(value);
        return true;
    }

    void pop_back() noexcept {
        PN_ASSERT_MSG(size_ > 0, "pop_back() on an empty InlineArray");
        --size_;
        element(size_)->~T();
    }

    /// Removes the element at `index` by moving the last one into its place.
    ///
    /// Order is not preserved, which is the whole point: it is O(1) where a
    /// shifting erase is O(n). Named so that a caller who needs order cannot
    /// reach for it by accident.
    void erase_unordered(size_type index) noexcept {
        PN_ASSERT_MSG(index < size_, "erase_unordered index out of range");
        if (index != size_ - 1) {
            *element(index) = std::move(*element(size_ - 1));
        }
        pop_back();
    }

    void clear() noexcept {
        while (size_ > 0) {
            pop_back();
        }
    }

private:
    T* element(size_type index) noexcept {
        return std::launder(reinterpret_cast<T*>(&storage_[index * sizeof(T)]));
    }

    const T* element(size_type index) const noexcept {
        return std::launder(reinterpret_cast<const T*>(&storage_[index * sizeof(T)]));
    }

    // Raw bytes rather than `T storage_[Capacity]`: an array of T would require
    // T to be default-constructible and would construct Capacity of them at
    // birth, which for a mostly-empty array is both a constraint and a cost.
    alignas(T) std::byte storage_[sizeof(T) * Capacity]{};
    size_type size_ = 0;
};

}  // namespace pn::core

#endif  // PN_CORE_INLINE_ARRAY_HPP
