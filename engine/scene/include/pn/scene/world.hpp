// Pention Engine - scene/world.hpp
// Requirement: PN-OBJ-001 (data-oriented entity/component storage with stable
//              handles), PN-OBJ-002 (query acceleration),
//              PN-PLT-021 (object identity across load)
// Decision:    ADR-0004

#ifndef PN_SCENE_WORLD_HPP
#define PN_SCENE_WORLD_HPP

#include "pn/core/assert.hpp"
#include "pn/core/handle.hpp"
#include "pn/core/hash_map.hpp"
#include "pn/core/sparse_set.hpp"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <array>
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

/// An entity's identity, as distinct from its handle.
///
/// A handle names a *slot*: it is small, it is what systems pass around, and it
/// is meaningless outside the world that issued it - the same slot index means
/// a different entity in a world built a different way. A persistent id names
/// the *object*: it is assigned once, never reused, and is what a saved file
/// records so that a reference written on one run can be resolved on the next.
///
/// Unique within one world. Two worlds each assign from one, so ids from
/// different worlds collide; merging saved scenes into one world (PN-OBJ-007)
/// needs an identity wider than a per-world counter and does not have one yet.
/// Saying so is cheaper than a subscene silently overwriting its parent.
using PersistentId = std::uint64_t;

/// The identity no entity has. Written into a saved reference that pointed
/// outside the save set.
inline constexpr PersistentId kNoPersistentId = 0;

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
    Entity create() { return create_with_id(next_persistent_id_); }

    /// Creates an entity carrying a specified identity.
    ///
    /// For a loader restoring a saved scene, which must reproduce the ids the
    /// file records rather than mint new ones - otherwise a reference saved
    /// against an id resolves to nothing. Returns an invalid handle if the id
    /// is zero or already in use, because silently renumbering the entity would
    /// break exactly the references this exists to preserve.
    Entity create_with_id(PersistentId id) {
        if (id == kNoPersistentId || by_persistent_id_.contains(id)) {
            return Entity{};
        }

        std::uint32_t index = 0;
        if (!free_list_.empty()) {
            index = free_list_.back();
            free_list_.pop_back();
            slots_[index].alive = true;
        } else {
            slots_.push_back(Slot{1, true});
            persistent_ids_.push_back(kNoPersistentId);
            index = static_cast<std::uint32_t>(slots_.size() - 1);
        }

        persistent_ids_[index] = id;
        by_persistent_id_.insert(id, index);
        // Never handed out twice, whether it was minted here or restored from a
        // file: a load that brought in id 900 must not let the next create()
        // mint 3 and then 900 again.
        if (id >= next_persistent_id_) {
            next_persistent_id_ = id + 1;
        }
        ++live_count_;
        return Entity{index, slots_[index].generation};
    }

    /// The identity of `entity`, or [`kNoPersistentId`] if it is not alive.
    PersistentId persistent_id(Entity entity) const noexcept {
        return alive(entity) ? persistent_ids_[entity.index()] : kNoPersistentId;
    }

    /// The entity carrying `id`, or an invalid handle if none does.
    ///
    /// The direction a loader needs: a file stores identities, and every
    /// reference in it has to become a handle.
    Entity find_by_persistent_id(PersistentId id) const noexcept {
        const std::uint32_t* index = by_persistent_id_.find(id);
        if (index == nullptr) {
            return Entity{};
        }
        return Entity{*index, slots_[*index].generation};
    }

    /// The id the next [`create`] will assign. For tests and for diagnostics.
    PersistentId next_persistent_id() const noexcept { return next_persistent_id_; }

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
        by_persistent_id_.erase(persistent_ids_[index]);
        // Cleared rather than left behind, so a slot between destroy and reuse
        // does not report the identity of the object that has gone.
        persistent_ids_[index] = kNoPersistentId;
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

    /// Visits every live entity, in slot-index order.
    ///
    /// The order is the world's own, not an insertion order, and it is what
    /// makes a save deterministic: the same set of live entities enumerates the
    /// same way regardless of the sequence of creates and destroys that
    /// produced it.
    template <typename Visitor>
    void for_each_entity(Visitor&& visit) const {
        for (std::size_t index = 0; index < slots_.size(); ++index) {
            if (slots_[index].alive) {
                visit(Entity{static_cast<std::uint32_t>(index), slots_[index].generation});
            }
        }
    }

    /// Visits every entity holding a `T`, with the component, in storage order.
    ///
    /// Read-only, and does not touch the query statistics: this is enumeration,
    /// not a query, and a save walking every component type should not look
    /// like the last thing a system asked for.
    template <typename T, typename Visitor>
    void for_each_component(Visitor&& visit) const {
        const Storage<T>* storage = find_storage<T>();
        if (storage == nullptr) {
            return;
        }
        const core::SparseSet<T>& set = storage->set;
        for (std::size_t position = 0; position < set.size(); ++position) {
            const std::uint32_t index = set.key_at(position);
            visit(Entity{index, slots_[index].generation}, set.values()[position]);
        }
    }

    /// What the last query did. For diagnostics and for tests.
    struct QueryStats {
        /// Entities the query looked at: the size of the storage it was driven
        /// by.
        std::size_t examined = 0;
        /// Entities that had every named component and were visited.
        std::size_t matched = 0;
    };

    /// Statistics from the most recent [`each`] or [`count_with`].
    const QueryStats& last_query() const noexcept { return last_query_; }

    /// Visits every entity that has all of the named components.
    ///
    /// Iteration is driven by whichever named storage is **smallest**, chosen at
    /// run time, so the cost is set by the rarest component and not by how many
    /// entities exist. With a thousand cameras among a million transforms, a
    /// query for both examines a thousand entities whichever order they are
    /// named in.
    ///
    /// That is the whole of the acceleration, and it is worth being plain about
    /// what it is not: this is not an archetype table. Entities are not grouped
    /// by their exact component set, so a query still tests membership per
    /// candidate. What it buys is that the candidate set is the smallest one
    /// available rather than the first one named - which is what the requirement
    /// asks for, and considerably less machinery than archetypes for the same
    /// property.
    ///
    /// The visitor receives components in the order they are named, not the
    /// order they are stored in. The driver being chosen at run time must not
    /// leak into the call site.
    ///
    /// The visitor must not create or destroy entities, or add or remove
    /// components of any named type: doing so moves the dense arrays underneath
    /// the loop. Removing is the tempting one, and it is exactly the one that
    /// swaps an unvisited element into a slot already passed.
    template <typename... Ts, typename Visitor>
    void each(Visitor&& visit) {
        static_assert(sizeof...(Ts) > 0, "a query must name at least one component type");
        last_query_ = QueryStats{};

        // Resolved into locals before the loop, and the loop uses those locals.
        // Calling find_storage again inside would look the same to a reader and
        // not to a compiler: each call reads a vector element, so the null check
        // cannot be carried across, and GCC at -O3 reports the dereference as
        // potentially null - correctly, since nothing connects the two calls.
        auto storages = std::make_tuple(find_storage<Ts>()...);
        const bool all_present = std::apply(
            [](auto*... entries) { return (... && (entries != nullptr)); }, storages);
        if (!all_present) {
            return;
        }

        const std::array<const StorageBase*, sizeof...(Ts)> bases = std::apply(
            [](auto*... entries) {
                return std::array<const StorageBase*, sizeof...(Ts)>{
                    static_cast<const StorageBase*>(entries)...};
            },
            storages);

        const StorageBase* driver = bases[0];
        for (const StorageBase* candidate : bases) {
            if (candidate->size() < driver->size()) {
                driver = candidate;
            }
        }

        const std::size_t count = driver->size();
        last_query_.examined = count;

        for (std::size_t position = 0; position < count; ++position) {
            const std::uint32_t index = driver->key_at(position);

            // One lookup per component, kept and reused. Testing membership and
            // then looking the component up again would search twice for the
            // same answer.
            auto found = std::apply(
                [index](auto*... entries) {
                    return std::make_tuple(entries->set.find(index)...);
                },
                storages);
            const bool complete = std::apply(
                [](auto*... pointers) { return (... && (pointers != nullptr)); }, found);
            if (!complete) {
                continue;
            }

            ++last_query_.matched;
            const Entity entity{index, slots_[index].generation};
            std::apply([&](auto*... pointers) { visit(entity, *pointers...); }, found);
        }
    }

    /// How many entities have all of the named components.
    template <typename... Ts>
    std::size_t count_with() {
        each<Ts...>([](Entity, Ts&...) {});
        return last_query_.matched;
    }

private:
    struct Slot {
        /// Starts at one, so a default-constructed handle - generation zero -
        /// never matches a live slot even at index zero.
        std::uint32_t generation = 1;
        bool alive = false;
    };

    /// The type-erased half of a component storage.
    ///
    /// `size` and `key_at` exist so a query can be driven by whichever storage
    /// turns out to be smallest, which is only known at run time. Without them
    /// the driver would have to be chosen at compile time, which is to say
    /// guessed by the caller.
    struct StorageBase {
        StorageBase() = default;
        StorageBase(const StorageBase&) = delete;
        StorageBase& operator=(const StorageBase&) = delete;
        virtual ~StorageBase() = default;
        virtual void remove(std::uint32_t index) = 0;
        virtual std::size_t size() const noexcept = 0;
        virtual std::uint32_t key_at(std::size_t position) const noexcept = 0;
    };

    template <typename T>
    struct Storage final : StorageBase {
        core::SparseSet<T> set;
        void remove(std::uint32_t index) override { set.remove(index); }
        std::size_t size() const noexcept override { return set.size(); }
        std::uint32_t key_at(std::size_t position) const noexcept override {
            return set.key_at(position);
        }
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
    /// Parallel to `slots_`. A separate array rather than a member of `Slot`
    /// because `alive` and `generation` are read by every handle validation and
    /// the identity is read only by save, load, and lookup.
    std::vector<PersistentId> persistent_ids_;
    core::HashMap<PersistentId, std::uint32_t> by_persistent_id_;
    PersistentId next_persistent_id_ = 1;
    std::vector<std::uint32_t> free_list_;
    /// Indexed by component id. Mutable because `find_storage` is used from
    /// const accessors and only reads.
    mutable std::vector<std::unique_ptr<StorageBase>> storages_;
    std::size_t live_count_ = 0;
    QueryStats last_query_{};
};

}  // namespace pn::scene

#endif  // PN_SCENE_WORLD_HPP
