// Pention Engine - scene/world.hpp
// Requirement: PN-OBJ-001 (data-oriented entity/component storage with stable
//              handles)
// Decision:    ADR-0004

#ifndef PN_SCENE_WORLD_HPP
#define PN_SCENE_WORLD_HPP

#include "pn/core/assert.hpp"
#include "pn/core/handle.hpp"
#include "pn/core/sparse_set.hpp"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <span>
#include <tuple>
#include <utility>
#include <vector>

namespace pn::scene {

/// Distinguishes an entity handle from every other kind.
struct EntityTag {};

/// A generational reference to an entity.
///
/// The same [`pn::core::Handle`] the rest of the engine uses, so the property
/// that makes a stale reference detectable rather than silently aliasing is the
/// one already verified under PN-PLT-011 rather than a second implementation of
/// it.
using Entity = core::Handle<EntityTag>;

using ComponentId = std::uint32_t;

namespace detail {

/// Hands out one identifier per component type, in first-use order.
inline ComponentId next_component_id() noexcept {
    static ComponentId next = 0;
    return next++;
}

}  // namespace detail

/// The identifier for `T`, stable for the life of the process.
///
/// Assigned on first use, so it depends on which types a given binary touches
/// and in what order. That is fine for storage indices and **not** fine for a
/// serialized file: writing a component identifier to disk would produce a save
/// that only the build that wrote it can read. Serialization keys on a stable
/// name instead.
template <typename T>
ComponentId component_id() noexcept {
    static const ComponentId id = detail::next_component_id();
    return id;
}

/// Entities and their components.
///
/// Components of one type live in a [`pn::core::SparseSet`], so a system that
/// walks all of them walks contiguous memory. Entity slots are recycled and
/// every recycle bumps a generation, so a handle held across a destroy resolves
/// to nothing rather than to whatever took the slot.
///
/// **Single-threaded.** Nothing here is synchronized. Parallel system scheduling
/// from declared component access is PN-OBJ-004 and is not written; saying so is
/// cheaper than a data race found in a shipping build.
class World {
public:
    World() = default;
    World(const World&) = delete;
    World& operator=(const World&) = delete;
    World(World&&) noexcept = default;
    World& operator=(World&&) noexcept = default;

    /// Creates an entity, reusing a freed slot when there is one.
    Entity create() {
        if (!free_list_.empty()) {
            const std::uint32_t index = free_list_.back();
            free_list_.pop_back();
            slots_[index].alive = true;
            ++live_count_;
            return Entity{index, slots_[index].generation};
        }
        slots_.push_back(Slot{1, true});
        ++live_count_;
        return Entity{static_cast<std::uint32_t>(slots_.size() - 1), 1};
    }

    /// True if the handle refers to an entity that still exists.
    bool alive(Entity entity) const noexcept {
        if (!entity.is_valid() || entity.index() >= slots_.size()) {
            return false;
        }
        const Slot& slot = slots_[entity.index()];
        return slot.alive && slot.generation == entity.generation();
    }

    /// Destroys an entity and every component it holds.
    ///
    /// Returns whether there was anything to destroy, so a double destroy is
    /// visible to a caller that cares rather than silently tolerated.
    bool destroy(Entity entity) {
        if (!alive(entity)) {
            return false;
        }
        const std::uint32_t index = entity.index();
        for (auto& storage : storages_) {
            if (storage != nullptr) {
                storage->remove(index);
            }
        }
        slots_[index].alive = false;
        // Bumping on destroy is what invalidates every outstanding handle to
        // this slot before the slot can be handed out again.
        ++slots_[index].generation;
        free_list_.push_back(index);
        --live_count_;
        return true;
    }

    std::size_t entity_count() const noexcept { return live_count_; }

    /// Adds a component, returning a pointer to it.
    ///
    /// Returns nullptr if the entity is not alive, or already has one. A silent
    /// overwrite would make `add` and `replace` the same call, and the caller
    /// who meant `add` would never learn the entity already had one.
    template <typename T>
    T* add(Entity entity, T value) {
        if (!alive(entity)) {
            return nullptr;
        }
        auto& set = storage_for<T>();
        if (!set.insert(entity.index(), std::move(value))) {
            return nullptr;
        }
        return set.find(entity.index());
    }

    /// Adds or overwrites.
    template <typename T>
    T* replace(Entity entity, T value) {
        if (!alive(entity)) {
            return nullptr;
        }
        auto& set = storage_for<T>();
        set.insert_or_assign(entity.index(), std::move(value));
        return set.find(entity.index());
    }

    template <typename T>
    T* get(Entity entity) noexcept {
        if (!alive(entity)) {
            return nullptr;
        }
        Storage<T>* storage = find_storage<T>();
        return storage == nullptr ? nullptr : storage->set.find(entity.index());
    }

    template <typename T>
    const T* get(Entity entity) const noexcept {
        if (!alive(entity)) {
            return nullptr;
        }
        const Storage<T>* storage = find_storage<T>();
        return storage == nullptr ? nullptr : storage->set.find(entity.index());
    }

    template <typename T>
    bool has(Entity entity) const noexcept {
        return get<T>(entity) != nullptr;
    }

    template <typename T>
    bool remove(Entity entity) {
        if (!alive(entity)) {
            return false;
        }
        Storage<T>* storage = find_storage<T>();
        return storage != nullptr && storage->set.remove(entity.index());
    }

    template <typename T>
    std::size_t component_count() const noexcept {
        const Storage<T>* storage = find_storage<T>();
        return storage == nullptr ? 0 : storage->set.size();
    }

    /// The components of one type, contiguous. The reason for the storage
    /// layout, and what a system that ignores entity identity should use.
    template <typename T>
    std::span<T> components() noexcept {
        Storage<T>* storage = find_storage<T>();
        if (storage == nullptr) {
            return {};
        }
        return std::span<T>{storage->set.values(), storage->set.size()};
    }

    /// Visits every entity that has all of the named components.
    ///
    /// Iteration is driven by the **first** named type, so name the rarest one
    /// first: with a thousand cameras and a million transforms,
    /// `each<Camera, Transform>` tests a thousand entities and
    /// `each<Transform, Camera>` tests a million for the same answer. Choosing
    /// the smallest automatically is PN-OBJ-002's job and is not done here.
    ///
    /// The visitor must not create or destroy entities, or add or remove
    /// components of the driving type: doing so moves the dense array underneath
    /// the loop. Removing is the tempting one, and it is exactly the one that
    /// swaps an unvisited element into a slot already passed.
    template <typename First, typename... Rest, typename Visitor>
    void each(Visitor&& visit) {
        Storage<First>* driver = find_storage<First>();
        if (driver == nullptr) {
            return;
        }
        // The other storages are resolved **into locals** before the loop, and
        // the loop uses those locals. Calling find_storage again inside the loop
        // would look the same to a reader and not to a compiler: each call reads
        // a vector element, so GCC cannot carry the null check across, and at
        // -O3 it reports the dereference as potentially null - correctly, since
        // nothing in the code connects the two calls.
        auto others = std::make_tuple(find_storage<Rest>()...);
        const bool all_present = std::apply(
            [](auto*... storages) { return (... && (storages != nullptr)); }, others);
        if (!all_present) {
            return;
        }

        const std::size_t count = driver->set.size();
        for (std::size_t position = 0; position < count; ++position) {
            const std::uint32_t index = driver->set.key_at(position);

            // One lookup per component, kept and reused. Testing membership and
            // then looking the component up again would search twice for the
            // same answer.
            auto found = std::apply(
                [index](auto*... storages) {
                    return std::make_tuple(storages->set.find(index)...);
                },
                others);
            const bool complete = std::apply(
                [](auto*... pointers) { return (... && (pointers != nullptr)); }, found);
            if (!complete) {
                continue;
            }

            const Entity entity{index, slots_[index].generation};
            std::apply(
                [&](auto*... pointers) {
                    visit(entity, driver->set[position], *pointers...);
                },
                found);
        }
    }

    /// How many entities have all of the named components.
    template <typename First, typename... Rest>
    std::size_t count_with() {
        std::size_t total = 0;
        each<First, Rest...>([&total](Entity, First&, Rest&...) { ++total; });
        return total;
    }

private:
    struct Slot {
        /// Starts at one, so a default-constructed handle - generation zero -
        /// never matches a live slot even at index zero.
        std::uint32_t generation = 1;
        bool alive = false;
    };

    struct StorageBase {
        StorageBase() = default;
        StorageBase(const StorageBase&) = delete;
        StorageBase& operator=(const StorageBase&) = delete;
        virtual ~StorageBase() = default;
        virtual void remove(std::uint32_t index) = 0;
    };

    template <typename T>
    struct Storage final : StorageBase {
        core::SparseSet<T> set;
        void remove(std::uint32_t index) override { set.remove(index); }
    };

    template <typename T>
    Storage<T>* find_storage() const noexcept {
        const ComponentId id = component_id<T>();
        if (id >= storages_.size() || storages_[id] == nullptr) {
            return nullptr;
        }
        return static_cast<Storage<T>*>(storages_[id].get());
    }

    template <typename T>
    core::SparseSet<T>& storage_for() {
        const ComponentId id = component_id<T>();
        if (id >= storages_.size()) {
            storages_.resize(static_cast<std::size_t>(id) + 1);
        }
        if (storages_[id] == nullptr) {
            storages_[id] = std::make_unique<Storage<T>>();
        }
        return static_cast<Storage<T>*>(storages_[id].get())->set;
    }

    std::vector<Slot> slots_;
    std::vector<std::uint32_t> free_list_;
    /// Indexed by component id. Mutable because `find_storage` is used from
    /// const accessors and only reads.
    mutable std::vector<std::unique_ptr<StorageBase>> storages_;
    std::size_t live_count_ = 0;
};

}  // namespace pn::scene

#endif  // PN_SCENE_WORLD_HPP
