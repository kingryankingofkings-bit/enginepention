// Pention Engine - scene/serialize.hpp
// Requirement: PN-PLT-021 (object identity and reference repair across load)
// Decision:    ADR-0004
//
// A scene is written on top of the canonical byte format (PN-PLT-020), which
// already gives framing, a checksum, and shortest-form varints. What this adds
// is the part that format cannot know about: which bytes are a *reference to
// another object*, and what those bytes have to become on the way back in.
//
// The problem reference repair solves is not encoding. An entity handle is two
// integers and encodes fine. It is that both integers are meaningless in the
// world that reads them - the slot index names a slot in the world that wrote
// the file, and the generation counts recycles that never happened in the world
// reading it. Written verbatim, a reference to entity A comes back as a handle
// to whatever now occupies slot A, or to nothing, and neither failure announces
// itself.

#ifndef PN_SCENE_SERIALIZE_HPP
#define PN_SCENE_SERIALIZE_HPP

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"
#include "pn/core/serialize.hpp"
#include "pn/scene/world.hpp"

#include <cstddef>
#include <cstdint>
#include <span>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace pn::scene {

/// How a component type turns into bytes and back.
///
/// Specialize for each component type that should be saved. A component with no
/// specialization is not saved and is not an error: a velocity that is
/// recomputed on load, or an editor-only selection flag, has no business in the
/// file, and requiring an empty specialization for each would make "not saved"
/// look like an oversight.
///
///     template <>
///     struct ComponentCodec<Position> {
///         static void write(const Position& value, SceneWriter& out);
///         static core::Expected<Position> read(SceneReader& in);
///     };
template <typename T>
struct ComponentCodec;

class SceneWriter;
class SceneReader;

/// The scene payload's schema version, inside the blob's own framing.
inline constexpr std::uint16_t kSceneFormatVersion = 1;

/// Writes one component's fields.
///
/// Field encoding is the caller's: this only adds `write_ref`, because a
/// reference is the one field a component cannot encode correctly by itself.
class SceneWriter {
public:
    core::ByteWriter& bytes() noexcept { return *bytes_; }

    /// Writes a reference to `target`.
    ///
    /// An entity that is not in the save set - dead, or belonging to another
    /// world - is written as "no reference". A dangling reference is dropped
    /// rather than preserved, because the alternative is a handle that resolves
    /// to whatever later occupies the slot, which is the aliasing failure the
    /// generational handle exists to prevent, reintroduced through the file.
    void write_ref(Entity target) {
        // The aliveness test is load-bearing, not a formality. A component may
        // still hold a handle to an entity that was destroyed, and that handle's
        // slot index may since have been reused by a live entity. Mapping the
        // index alone would write the *new* entity's ordinal, and the file would
        // then contain the exact aliasing the generational handle exists to
        // prevent - laundered through a save.
        const std::uint32_t index = target.index();
        if (!world_->alive(target) || index >= ordinals_->size()) {
            bytes_->write_varint(0);
            return;
        }
        // Stored as ordinal + 1 so that zero can mean "no reference" without
        // costing a separate present/absent byte on every reference.
        bytes_->write_varint((*ordinals_)[index]);
    }

private:
    friend class SceneCodec;
    SceneWriter(const World& world, core::ByteWriter& bytes,
                const std::vector<std::uint32_t>& ordinals) noexcept
        : world_{&world}, bytes_{&bytes}, ordinals_{&ordinals} {}

    const World* world_;
    core::ByteWriter* bytes_;
    /// Slot index -> ordinal + 1, or zero for a slot not in the save set.
    const std::vector<std::uint32_t>* ordinals_;
};

/// Reads one component's fields.
class SceneReader {
public:
    core::ByteReader& bytes() noexcept { return *bytes_; }

    /// Reads a reference, resolved against the entities this load created.
    ///
    /// An out-of-range ordinal is a corrupt file rather than a missing entity,
    /// and is reported as one: every ordinal a well-formed file contains was
    /// written from the same entity table that precedes it.
    core::Expected<Entity> read_ref() {
        PN_TRY_ASSIGN(const std::uint64_t stored, bytes_->read_varint());
        if (stored == 0) {
            return Entity{};
        }
        if (stored > by_ordinal_->size()) {
            return core::fail(core::ErrorCategory::corrupt_data,
                              "scene reference names an entity the file does not contain");
        }
        return (*by_ordinal_)[stored - 1];
    }

private:
    friend class SceneCodec;
    SceneReader(core::ByteReader& bytes, const std::vector<Entity>& by_ordinal) noexcept
        : bytes_{&bytes}, by_ordinal_{&by_ordinal} {}

    core::ByteReader* bytes_;
    const std::vector<Entity>* by_ordinal_;
};

/// The set of component types a scene file may contain, and how to move each
/// one across the boundary.
///
/// A type's key in the file is the name registered here, not
/// [`component_id`]. Component ids are assigned in first-use order, so they
/// depend on which types a binary touches and when - a file keyed on them would
/// be readable only by the build that wrote it, and would be *misread* rather
/// than rejected by any other, because the ids are all valid, just for different
/// types.
class SceneCodec {
public:
    /// Registers `T` under `name`.
    ///
    /// Returns false if the name or the type is already registered. Two types
    /// sharing a name would make the file ambiguous; one type under two names
    /// would make the save non-deterministic.
    template <typename T>
    bool register_component(std::string_view name) {
        if (name.empty() || find_by_name(name) != nullptr) {
            return false;
        }
        const ComponentId id = component_id<T>();
        for (const TypeEntry& entry : types_) {
            if (entry.id == id) {
                return false;
            }
        }

        types_.push_back(TypeEntry{
            std::string{name},
            id,
            &save_type<T>,
            &load_type<T>,
        });
        // Sorted by name at registration rather than at save time, so the file
        // depends on which types are registered and not on the order a
        // particular build happened to register them in. Which module registers
        // first changes with build configuration; the file must not.
        for (std::size_t position = types_.size() - 1; position > 0; --position) {
            if (types_[position - 1].name <= types_[position].name) {
                break;
            }
            std::swap(types_[position - 1], types_[position]);
        }
        return true;
    }

    std::size_t registered_type_count() const noexcept { return types_.size(); }

    /// Writes `world` into `out` as a framed blob.
    ///
    /// Deterministic: the same live entities holding the same component values
    /// produce the same bytes, whatever sequence of edits produced them. That
    /// is what makes save/load/save-again a usable test - it compares the whole
    /// file rather than the fields somebody remembered to check.
    void save(const World& world, core::ByteWriter& out) const {
        // Slot index -> ordinal + 1. Sized to cover every live slot; a slot that
        // is not live keeps its zero and reads as "not in the save set".
        std::vector<std::uint32_t> ordinals;
        std::vector<Entity> in_order;
        world.for_each_entity([&](Entity entity) {
            const std::size_t index = entity.index();
            if (index >= ordinals.size()) {
                ordinals.resize(index + 1, 0);
            }
            in_order.push_back(entity);
            ordinals[index] = static_cast<std::uint32_t>(in_order.size());
        });

        core::ByteWriter payload;
        payload.write_varint(in_order.size());
        for (const Entity& entity : in_order) {
            payload.write_varint(world.persistent_id(entity));
        }

        payload.write_varint(types_.size());
        for (const TypeEntry& entry : types_) {
            payload.write_string(entry.name);

            // Each type's records are built into a scratch buffer first so the
            // block can be prefixed with its length. That is what lets a reader
            // skip a type it does not know instead of refusing the file - a
            // scene saved by a build with one more component type still loads,
            // minus that component.
            core::ByteWriter block;
            std::size_t instances = 0;
            entry.save(world, ordinals, block, instances);

            payload.write_varint(instances);
            payload.write_varint(block.size());
            payload.write_bytes(block.data());
        }

        core::write_blob(out, kSceneFormatVersion, payload.data());
    }

    /// Reads a scene into `world`, which must be empty.
    ///
    /// Loading into a populated world is refused rather than merged: merging
    /// needs an identity that survives across worlds, and the per-world counter
    /// behind [`PersistentId`] is not one. Refusing is the honest version of
    /// that limitation.
    core::Expected<std::size_t> load(std::span<const std::byte> bytes, World& world) const {
        if (world.entity_count() != 0) {
            return core::fail(core::ErrorCategory::invalid_argument,
                              "a scene loads into an empty world; merging is not supported");
        }

        PN_TRY_ASSIGN(const core::Blob blob, core::read_blob(bytes));
        if (blob.payload_version != kSceneFormatVersion) {
            return core::fail(core::ErrorCategory::unsupported,
                              "scene payload version is not supported");
        }

        core::ByteReader reader{blob.payload};
        PN_TRY_ASSIGN(const std::uint64_t entity_count, reader.read_varint());

        // Every entity first, then every component. A component may reference an
        // entity that appears later in the file, and a single pass would have to
        // either patch afterwards or forbid the forward reference. Creating the
        // entities up front costs one pass and removes the whole question.
        std::vector<Entity> by_ordinal;
        by_ordinal.reserve(entity_count);
        for (std::uint64_t position = 0; position < entity_count; ++position) {
            PN_TRY_ASSIGN(const std::uint64_t id, reader.read_varint());
            const Entity entity = world.create_with_id(id);
            if (!entity.is_valid()) {
                return core::fail(core::ErrorCategory::corrupt_data,
                                  "scene contains a duplicate or zero entity identity");
            }
            by_ordinal.push_back(entity);
        }

        PN_TRY_ASSIGN(const std::uint64_t type_count, reader.read_varint());
        std::size_t components_read = 0;
        for (std::uint64_t position = 0; position < type_count; ++position) {
            PN_TRY_ASSIGN(const std::string_view name, reader.read_string());
            PN_TRY_ASSIGN(const std::uint64_t instances, reader.read_varint());
            PN_TRY_ASSIGN(const std::uint64_t block_length, reader.read_varint());
            if (block_length > reader.remaining()) {
                return core::fail(core::ErrorCategory::corrupt_data,
                                  "scene component block runs past the end of the payload");
            }
            PN_TRY_ASSIGN(const std::span<const std::byte> block, reader.read_bytes(block_length));

            const TypeEntry* entry = find_by_name(name);
            if (entry == nullptr) {
                // Not an error. A file written by a build that knows one more
                // component type still loads here, without it.
                continue;
            }

            core::ByteReader block_reader{block};
            PN_TRY_VOID(entry->load(world, by_ordinal, block_reader, instances));
            if (!block_reader.at_end()) {
                return core::fail(core::ErrorCategory::corrupt_data,
                                  "scene component block has bytes left over");
            }
            components_read += instances;
        }

        if (!reader.at_end()) {
            return core::fail(core::ErrorCategory::corrupt_data,
                              "scene payload has bytes left over");
        }
        return components_read;
    }

private:
    using SaveFn = void (*)(const World&, const std::vector<std::uint32_t>&, core::ByteWriter&,
                            std::size_t&);
    using LoadFn = core::Expected<void> (*)(World&, const std::vector<Entity>&, core::ByteReader&,
                                            std::uint64_t);

    struct TypeEntry {
        std::string name;
        ComponentId id;
        SaveFn save;
        LoadFn load;
    };

    template <typename T>
    static void save_type(const World& world, const std::vector<std::uint32_t>& ordinals,
                          core::ByteWriter& out, std::size_t& instances) {
        // Gathered and sorted by ordinal rather than written in storage order.
        // Storage order depends on the history of inserts and removes, so two
        // worlds holding identical data would otherwise write different files
        // and the byte-for-byte round-trip test would be checking nothing.
        std::vector<std::pair<std::uint32_t, const T*>> records;
        world.for_each_component<T>([&](Entity entity, const T& value) {
            const std::size_t index = entity.index();
            if (index < ordinals.size() && ordinals[index] != 0) {
                records.emplace_back(ordinals[index], &value);
            }
        });
        for (std::size_t position = 1; position < records.size(); ++position) {
            std::pair<std::uint32_t, const T*> held = records[position];
            std::size_t slot = position;
            while (slot > 0 && records[slot - 1].first > held.first) {
                records[slot] = records[slot - 1];
                --slot;
            }
            records[slot] = held;
        }

        SceneWriter writer{world, out, ordinals};
        for (const auto& [ordinal, value] : records) {
            out.write_varint(static_cast<std::uint64_t>(ordinal));
            ComponentCodec<T>::write(*value, writer);
        }
        instances = records.size();
    }

    template <typename T>
    static core::Expected<void> load_type(World& world, const std::vector<Entity>& by_ordinal,
                                          core::ByteReader& in, std::uint64_t instances) {
        SceneReader scene_reader{in, by_ordinal};
        for (std::uint64_t position = 0; position < instances; ++position) {
            PN_TRY_ASSIGN(const std::uint64_t ordinal, in.read_varint());
            if (ordinal == 0 || ordinal > by_ordinal.size()) {
                return core::fail(core::ErrorCategory::corrupt_data,
                                  "scene component names an entity the file does not contain");
            }
            PN_TRY_ASSIGN(T value, ComponentCodec<T>::read(scene_reader));
            if (world.add(by_ordinal[ordinal - 1], std::move(value)) == nullptr) {
                return core::fail(core::ErrorCategory::corrupt_data,
                                  "scene gives one entity the same component twice");
            }
        }
        return {};
    }

    const TypeEntry* find_by_name(std::string_view name) const noexcept {
        for (const TypeEntry& entry : types_) {
            if (entry.name == name) {
                return &entry;
            }
        }
        return nullptr;
    }

    /// Kept sorted by name. Linear search: a scene has tens of component types,
    /// not thousands, and a hash map here would be slower to build than the
    /// scans it saves.
    std::vector<TypeEntry> types_;
};

}  // namespace pn::scene

#endif  // PN_SCENE_SERIALIZE_HPP
