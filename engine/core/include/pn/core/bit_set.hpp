// Pention Engine - core/bit_set.hpp
// Requirement: PN-PLT-009 (cache-aware containers)
// Decision:    ADR-0004

#ifndef PN_CORE_BIT_SET_HPP
#define PN_CORE_BIT_SET_HPP

#include "pn/core/assert.hpp"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <vector>

namespace pn::core {

/// A resizable array of bits.
///
/// The representation a visibility set wants: one bit per object, sixty-four to
/// a word, so a hundred thousand objects fit in twelve kilobytes and a whole
/// frame's culling result stays in L1.
///
/// The operation that matters is [`for_each_set`], which visits only the bits
/// that are set. A loop over every index costs the same whether one object is
/// visible or all of them; this costs what the answer costs. For a culling
/// result that is usually the difference between a hundred thousand iterations
/// and a few hundred.
///
/// Not `std::vector<bool>`: its `operator[]` returns a proxy, it offers no way
/// to reach the underlying words, and so every bulk operation - counting,
/// intersecting, finding the next set bit - degrades to a bit at a time.
class BitSet {
public:
    using size_type = std::size_t;

    BitSet() = default;
    explicit BitSet(size_type bits) { resize(bits); }

    size_type size() const noexcept { return bits_; }
    bool empty() const noexcept { return bits_ == 0; }

    /// Resizes, clearing any new bits.
    void resize(size_type bits) {
        words_.resize((bits + kBitsPerWord - 1) / kBitsPerWord, 0);
        bits_ = bits;
        trim();
    }

    void set(size_type index) noexcept {
        PN_ASSERT_MSG(index < bits_, "BitSet index out of range");
        words_[index / kBitsPerWord] |= word_bit(index);
    }

    void clear(size_type index) noexcept {
        PN_ASSERT_MSG(index < bits_, "BitSet index out of range");
        words_[index / kBitsPerWord] &= ~word_bit(index);
    }

    void set(size_type index, bool value) noexcept {
        if (value) {
            set(index);
        } else {
            clear(index);
        }
    }

    void flip(size_type index) noexcept {
        PN_ASSERT_MSG(index < bits_, "BitSet index out of range");
        words_[index / kBitsPerWord] ^= word_bit(index);
    }

    bool test(size_type index) const noexcept {
        PN_ASSERT_MSG(index < bits_, "BitSet index out of range");
        return (words_[index / kBitsPerWord] & word_bit(index)) != 0;
    }

    void set_all() noexcept {
        for (std::uint64_t& word : words_) {
            word = ~std::uint64_t{0};
        }
        trim();
    }

    void clear_all() noexcept {
        for (std::uint64_t& word : words_) {
            word = 0;
        }
    }

    size_type count() const noexcept {
        size_type total = 0;
        for (std::uint64_t word : words_) {
            total += static_cast<size_type>(std::popcount(word));
        }
        return total;
    }

    bool any() const noexcept {
        for (std::uint64_t word : words_) {
            if (word != 0) {
                return true;
            }
        }
        return false;
    }

    bool none() const noexcept { return !any(); }

    bool all() const noexcept { return count() == bits_; }

    /// The index of the first set bit at or after `from`, or `size()` if there
    /// is none.
    size_type find_next(size_type from) const noexcept {
        if (from >= bits_) {
            return bits_;
        }
        size_type word_index = from / kBitsPerWord;
        // Mask off the bits before `from` in the first word, so the scan can
        // then treat every word identically.
        std::uint64_t word = words_[word_index] & (~std::uint64_t{0} << (from % kBitsPerWord));
        while (true) {
            if (word != 0) {
                return word_index * kBitsPerWord +
                       static_cast<size_type>(std::countr_zero(word));
            }
            ++word_index;
            if (word_index >= words_.size()) {
                return bits_;
            }
            word = words_[word_index];
        }
    }

    /// Visits every set bit, in increasing order.
    ///
    /// Costs what the answer costs, not what the domain costs. Clearing the
    /// lowest set bit with `word & (word - 1)` steps straight to the next one
    /// rather than testing the sixty-three that are not set.
    template <typename Visitor>
    void for_each_set(Visitor&& visit) const {
        for (size_type word_index = 0; word_index < words_.size(); ++word_index) {
            std::uint64_t word = words_[word_index];
            while (word != 0) {
                const size_type offset = static_cast<size_type>(std::countr_zero(word));
                visit(word_index * kBitsPerWord + offset);
                word &= word - 1;
            }
        }
    }

    /// In-place intersection. Sizes must match.
    void intersect_with(const BitSet& other) noexcept {
        PN_ASSERT_MSG(bits_ == other.bits_, "BitSet sizes must match");
        for (size_type index = 0; index < words_.size(); ++index) {
            words_[index] &= other.words_[index];
        }
    }

    void union_with(const BitSet& other) noexcept {
        PN_ASSERT_MSG(bits_ == other.bits_, "BitSet sizes must match");
        for (size_type index = 0; index < words_.size(); ++index) {
            words_[index] |= other.words_[index];
        }
    }

    /// Removes every bit set in `other`.
    void subtract(const BitSet& other) noexcept {
        PN_ASSERT_MSG(bits_ == other.bits_, "BitSet sizes must match");
        for (size_type index = 0; index < words_.size(); ++index) {
            words_[index] &= ~other.words_[index];
        }
    }

    bool operator==(const BitSet& other) const noexcept {
        return bits_ == other.bits_ && words_ == other.words_;
    }

private:
    static constexpr size_type kBitsPerWord = 64;

    static constexpr std::uint64_t word_bit(size_type index) noexcept {
        return std::uint64_t{1} << (index % kBitsPerWord);
    }

    /// Clears the bits past `bits_` in the final word.
    ///
    /// They are unreachable through the public interface, but `count`, `any`
    /// and `find_next` read whole words. Leaving them set would make a set of
    /// 65 bits report 128 - a bug that only appears when the size is not a
    /// multiple of 64, which most test sizes are.
    void trim() noexcept {
        const size_type remainder = bits_ % kBitsPerWord;
        if (remainder != 0 && !words_.empty()) {
            words_.back() &= (std::uint64_t{1} << remainder) - 1;
        }
    }

    std::vector<std::uint64_t> words_;
    size_type bits_ = 0;
};

}  // namespace pn::core

#endif  // PN_CORE_BIT_SET_HPP
