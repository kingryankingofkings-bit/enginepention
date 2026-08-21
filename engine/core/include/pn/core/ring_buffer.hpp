// Pention Engine - core/ring_buffer.hpp
// Requirement: PN-PLT-009 (cache-aware containers)
// Decision:    ADR-0004

#ifndef PN_CORE_RING_BUFFER_HPP
#define PN_CORE_RING_BUFFER_HPP

#include "pn/core/assert.hpp"

#include <cstddef>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

namespace pn::core {

/// A fixed-capacity double-ended queue in a contiguous block.
///
/// For the queues an engine keeps between systems - pending input events,
/// frames in flight, a history window for temporal reconstruction. All of them
/// have a known bound and all of them are pushed and popped every frame, which
/// is the shape a node-based deque handles worst: an allocation per chunk, and
/// a pointer chase to find the chunk.
///
/// Capacity is a power of two so the wrap is a mask rather than a modulo. That
/// is not micro-optimisation - `%` on a runtime value is a division, and a
/// division in a per-event loop is thirty-odd cycles for arithmetic that should
/// be one.
template <typename T, std::size_t Capacity>
class RingBuffer {
    static_assert(Capacity > 0, "a zero-capacity ring holds nothing");
    static_assert((Capacity & (Capacity - 1)) == 0,
                  "capacity must be a power of two so the wrap is a mask, not a division");

public:
    using value_type = T;
    using size_type = std::size_t;

    RingBuffer() = default;
    RingBuffer(const RingBuffer& other) { copy_from(other); }
    RingBuffer(RingBuffer&& other) noexcept { move_from(other); }

    RingBuffer& operator=(const RingBuffer& other) {
        if (this != &other) {
            clear();
            copy_from(other);
        }
        return *this;
    }

    RingBuffer& operator=(RingBuffer&& other) noexcept {
        if (this != &other) {
            clear();
            move_from(other);
        }
        return *this;
    }

    ~RingBuffer() { clear(); }

    static constexpr size_type capacity() noexcept { return Capacity; }
    size_type size() const noexcept { return size_; }
    bool empty() const noexcept { return size_ == 0; }
    bool full() const noexcept { return size_ == Capacity; }

    /// Appends at the back, aborting in debug if full.
    template <typename... Args>
    T& emplace_back(Args&&... args) {
        PN_ASSERT_MSG(size_ < Capacity, "RingBuffer capacity exceeded");
        T* slot = element((head_ + size_) & kMask);
        ::new (static_cast<void*>(slot)) T(std::forward<Args>(args)...);
        ++size_;
        return *slot;
    }

    T& push_back(const T& value) { return emplace_back(value); }
    T& push_back(T&& value) { return emplace_back(std::move(value)); }

    /// Appends if there is room, reporting whether there was.
    [[nodiscard]] bool try_push_back(const T& value) {
        if (full()) {
            return false;
        }
        emplace_back(value);
        return true;
    }

    /// Appends at the back, discarding the front element if full.
    ///
    /// For a history window, where the newest item matters and the oldest is
    /// meant to fall off. Named so a caller cannot reach for it expecting the
    /// bound to be enforced.
    T& push_back_overwriting(const T& value) {
        if (full()) {
            pop_front();
        }
        return emplace_back(value);
    }

    template <typename... Args>
    T& emplace_front(Args&&... args) {
        PN_ASSERT_MSG(size_ < Capacity, "RingBuffer capacity exceeded");
        const size_type slot_index = (head_ + Capacity - 1) & kMask;
        T* slot = element(slot_index);
        ::new (static_cast<void*>(slot)) T(std::forward<Args>(args)...);
        head_ = slot_index;
        ++size_;
        return *slot;
    }

    void pop_front() noexcept {
        PN_ASSERT_MSG(size_ > 0, "pop_front() on an empty RingBuffer");
        element(head_)->~T();
        head_ = (head_ + 1) & kMask;
        --size_;
    }

    void pop_back() noexcept {
        PN_ASSERT_MSG(size_ > 0, "pop_back() on an empty RingBuffer");
        --size_;
        element((head_ + size_) & kMask)->~T();
    }

    T& front() noexcept {
        PN_ASSERT_MSG(size_ > 0, "front() on an empty RingBuffer");
        return *element(head_);
    }

    T& back() noexcept {
        PN_ASSERT_MSG(size_ > 0, "back() on an empty RingBuffer");
        return *element((head_ + size_ - 1) & kMask);
    }

    /// Indexed from the front, so index zero is the oldest element.
    T& operator[](size_type index) noexcept {
        PN_ASSERT_MSG(index < size_, "RingBuffer index out of range");
        return *element((head_ + index) & kMask);
    }

    const T& operator[](size_type index) const noexcept {
        PN_ASSERT_MSG(index < size_, "RingBuffer index out of range");
        return *element((head_ + index) & kMask);
    }

    void clear() noexcept {
        while (size_ > 0) {
            pop_front();
        }
        head_ = 0;
    }

private:
    static constexpr size_type kMask = Capacity - 1;

    T* element(size_type index) noexcept {
        return std::launder(reinterpret_cast<T*>(&storage_[index * sizeof(T)]));
    }

    const T* element(size_type index) const noexcept {
        return std::launder(reinterpret_cast<const T*>(&storage_[index * sizeof(T)]));
    }

    void copy_from(const RingBuffer& other) {
        for (size_type index = 0; index < other.size_; ++index) {
            push_back(other[index]);
        }
    }

    void move_from(RingBuffer& other) {
        for (size_type index = 0; index < other.size_; ++index) {
            emplace_back(std::move(other[index]));
        }
        other.clear();
    }

    alignas(T) std::byte storage_[sizeof(T) * Capacity]{};
    size_type head_ = 0;
    size_type size_ = 0;
};

}  // namespace pn::core

#endif  // PN_CORE_RING_BUFFER_HPP
