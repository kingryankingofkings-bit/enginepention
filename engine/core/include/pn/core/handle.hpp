// Pention Engine - core/handle.hpp
// Requirement: PN-PLT-011 (generational handles that detect stale access)
// Decision:    ADR-0003 (docs/conventions.md, "Naming and identity")

#ifndef PN_CORE_HANDLE_HPP
#define PN_CORE_HANDLE_HPP

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"

#include <cstddef>
#include <cstdint>
#include <limits>
#include <utility>
#include <vector>

namespace pn::core {

/// A generational reference to a pooled object.
///
/// The generation is what makes a dangling handle *detectably* stale rather
/// than silently aliasing a recycled slot. Without it, freeing an object and
/// allocating another turns an old handle into a valid-looking reference to
/// unrelated data - a use-after-free that reads as a logic bug and can survive
/// for months. With it, the same mistake is an assertion.
///
/// `Tag` makes handles to different resource kinds distinct types, so a texture
/// handle cannot be passed where a mesh handle is expected.
template <typename Tag>
class Handle {
public:
    using IndexType = std::uint32_t;
    using GenerationType = std::uint32_t;

    static constexpr IndexType kInvalidIndex = std::numeric_limits<IndexType>::max();

    constexpr Handle() = default;
    constexpr Handle(IndexType index, GenerationType generation) noexcept
        : index_(index), generation_(generation) {}

    [[nodiscard]] constexpr IndexType index() const noexcept { return index_; }
    [[nodiscard]] constexpr GenerationType generation() const noexcept { return generation_; }

    /// True if this handle was ever issued. Says nothing about whether the
    /// object it referred to still exists - only the pool can answer that.
    [[nodiscard]] constexpr bool is_valid() const noexcept { return index_ != kInvalidIndex; }

    friend constexpr bool operator==(const Handle&, const Handle&) = default;

private:
    IndexType index_ = kInvalidIndex;
    GenerationType generation_ = 0;
};

/// Slot-based storage handing out generational handles.
///
/// Slots are recycled, and every recycle bumps the slot's generation, so a
/// handle issued before a free never resolves after it.
template <typename T, typename Tag = T>
class HandlePool {
public:
    using HandleType = Handle<Tag>;

    HandlePool() = default;

    explicit HandlePool(std::size_t initial_capacity) { reserve(initial_capacity); }

    void reserve(std::size_t capacity) {
        slots_.reserve(capacity);
        free_list_.reserve(capacity);
    }

    /// Inserts a value and returns a handle to it.
    [[nodiscard]] HandleType insert(T value) {
        if (!free_list_.empty()) {
            const auto index = free_list_.back();
            free_list_.pop_back();
            Slot& slot = slots_[index];
            slot.value = std::move(value);
            slot.occupied = true;
            return HandleType{index, slot.generation};
        }

        slots_.push_back(Slot{std::move(value), 1, true});
        const auto index = static_cast<typename HandleType::IndexType>(slots_.size() - 1);
        return HandleType{index, 1};
    }

    /// True if the handle refers to a currently live object.
    [[nodiscard]] bool contains(HandleType handle) const noexcept {
        if (!handle.is_valid() || handle.index() >= slots_.size()) {
            return false;
        }
        const Slot& slot = slots_[handle.index()];
        return slot.occupied && slot.generation == handle.generation();
    }

    /// Resolves a handle, or explains why it could not be resolved.
    ///
    /// Returns a pointer rather than a reference so the failure case does not
    /// require a sentinel object, and wrapped in Expected so ignoring the
    /// failure is a compile-time diagnostic.
    [[nodiscard]] Expected<T*, Error> get(HandleType handle) noexcept {
        if (!handle.is_valid()) {
            return fail(ErrorCategory::invalid_argument, "handle was never issued");
        }
        if (handle.index() >= slots_.size()) {
            return fail(ErrorCategory::out_of_range, "handle index outside the pool");
        }
        Slot& slot = slots_[handle.index()];
        if (!slot.occupied) {
            return fail(ErrorCategory::not_found, "handle refers to a freed slot");
        }
        if (slot.generation != handle.generation()) {
            // The precise case generations exist to catch.
            return fail(ErrorCategory::not_found, "stale handle: slot was recycled");
        }
        return &slot.value;
    }

    [[nodiscard]] Expected<const T*, Error> get(HandleType handle) const noexcept {
        // Reusing the mutable path would need a const_cast; the duplication is
        // small and keeps the const path honestly const.
        if (!handle.is_valid()) {
            return fail(ErrorCategory::invalid_argument, "handle was never issued");
        }
        if (handle.index() >= slots_.size()) {
            return fail(ErrorCategory::out_of_range, "handle index outside the pool");
        }
        const Slot& slot = slots_[handle.index()];
        if (!slot.occupied) {
            return fail(ErrorCategory::not_found, "handle refers to a freed slot");
        }
        if (slot.generation != handle.generation()) {
            return fail(ErrorCategory::not_found, "stale handle: slot was recycled");
        }
        return &slot.value;
    }

    /// Frees the slot a handle refers to. Freeing an already-freed or stale
    /// handle is reported, not ignored - a double free is a real defect and
    /// silently tolerating it hides the defect rather than the symptom.
    [[nodiscard]] Expected<void, Error> remove(HandleType handle) noexcept {
        if (!contains(handle)) {
            return Unexpected{Error{ErrorCategory::not_found,
                                    "remove() called with a stale or freed handle"}};
        }
        Slot& slot = slots_[handle.index()];
        slot.occupied = false;
        // Bumping on free is what invalidates every outstanding handle to this
        // slot before the slot can be handed out again.
        ++slot.generation;
        free_list_.push_back(handle.index());
        return {};
    }

    [[nodiscard]] std::size_t size() const noexcept {
        return slots_.size() - free_list_.size();
    }
    [[nodiscard]] std::size_t capacity() const noexcept { return slots_.size(); }
    [[nodiscard]] bool empty() const noexcept { return size() == 0; }

    void clear() noexcept {
        slots_.clear();
        free_list_.clear();
    }

private:
    struct Slot {
        T value{};
        typename HandleType::GenerationType generation = 1;
        bool occupied = false;
    };

    std::vector<Slot> slots_;
    std::vector<typename HandleType::IndexType> free_list_;
};

}  // namespace pn::core

#endif  // PN_CORE_HANDLE_HPP
