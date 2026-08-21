// Pention Engine - scene/tests/serialize_test.cpp
// Requirement: PN-PLT-021
// Decision:    ADR-0004
//
// The criterion has two clauses and the second one is the interesting half:
// cross-referencing objects survive save/load *and rename*. A reference that
// survives a round trip but breaks when the target is renamed is keyed on the
// name, which is the mistake this requirement exists to rule out.
//
// The tests keep one adversarial case in view throughout: a component holding a
// handle to an entity that has been destroyed, whose slot has since been reused
// by a different entity. Written naively that reference comes back pointing at
// the wrong object, and nothing about the file or the load reports it.

#include "pn/core/random.hpp"
#include "pn/scene/serialize.hpp"
#include "pn/scene/world.hpp"
#include "pn/testing/test.hpp"

#include <algorithm>
#include <cstdint>
#include <map>
#include <string>
#include <vector>

namespace {

using pn::core::ByteWriter;
using pn::core::Random;
using pn::scene::Entity;
using pn::scene::PersistentId;
using pn::scene::SceneCodec;
using pn::scene::SceneReader;
using pn::scene::SceneWriter;
using pn::scene::World;

struct Transform {
    double x = 0.0;
    double y = 0.0;
    double z = 0.0;
};

/// The renameable thing. A name is a property of an object, not its identity -
/// which is the whole point of the second clause.
struct Tag {
    std::string text;
};

/// Two references, so a component holding several is exercised and so the
/// "target destroyed" case can be checked beside a live one in the same record.
struct Link {
    Entity primary;
    Entity secondary;
    std::int32_t weight = 0;
};

/// Registered by nobody, on purpose: a component type with no codec is not
/// saved and is not an error.
struct EditorOnly {
    bool selected = false;
};

}  // namespace

namespace pn::scene {

template <>
struct ComponentCodec<Transform> {
    static void write(const Transform& value, SceneWriter& out) {
        out.bytes().write_f64(value.x);
        out.bytes().write_f64(value.y);
        out.bytes().write_f64(value.z);
    }
    static core::Expected<Transform> read(SceneReader& in) {
        Transform value;
        PN_TRY_ASSIGN(value.x, in.bytes().read_f64());
        PN_TRY_ASSIGN(value.y, in.bytes().read_f64());
        PN_TRY_ASSIGN(value.z, in.bytes().read_f64());
        return value;
    }
};

template <>
struct ComponentCodec<Tag> {
    static void write(const Tag& value, SceneWriter& out) { out.bytes().write_string(value.text); }
    static core::Expected<Tag> read(SceneReader& in) {
        PN_TRY_ASSIGN(const std::string_view text, in.bytes().read_string());
        return Tag{std::string{text}};
    }
};

template <>
struct ComponentCodec<Link> {
    static void write(const Link& value, SceneWriter& out) {
        out.write_ref(value.primary);
        out.write_ref(value.secondary);
        out.bytes().write_i32(value.weight);
    }
    static core::Expected<Link> read(SceneReader& in) {
        Link value;
        PN_TRY_ASSIGN(value.primary, in.read_ref());
        PN_TRY_ASSIGN(value.secondary, in.read_ref());
        PN_TRY_ASSIGN(value.weight, in.bytes().read_i32());
        return value;
    }
};

}  // namespace pn::scene

namespace {

SceneCodec make_codec() {
    SceneCodec codec;
    codec.register_component<Transform>("pn.Transform");
    codec.register_component<Tag>("pn.Tag");
    codec.register_component<Link>("pn.Link");
    return codec;
}

}  // namespace

// -------------------------------------------------------------------
// Identity
// -------------------------------------------------------------------

PN_TEST(scene_identity, every_entity_gets_an_identity_and_no_two_share_one) {
    World world;
    std::vector<PersistentId> seen;
    for (int index = 0; index < 200; ++index) {
        const Entity entity = world.create();
        const PersistentId id = world.persistent_id(entity);
        PN_REQUIRE(id != pn::scene::kNoPersistentId);
        seen.push_back(id);
    }
    std::vector<PersistentId> sorted = seen;
    std::sort(sorted.begin(), sorted.end());
    PN_CHECK(std::adjacent_find(sorted.begin(), sorted.end()) == sorted.end());
}

PN_TEST(scene_identity, an_identity_is_never_reused_when_a_slot_is) {
    // The property that separates identity from a handle. The slot comes back;
    // the identity does not, so a saved reference to the destroyed object cannot
    // silently resolve to its replacement.
    World world;
    const Entity first = world.create();
    const PersistentId first_id = world.persistent_id(first);
    PN_REQUIRE(world.destroy(first));

    const Entity second = world.create();
    PN_CHECK_EQ(second.index(), first.index());
    PN_CHECK(world.persistent_id(second) != first_id);
    PN_CHECK(!world.find_by_persistent_id(first_id).is_valid());
    PN_CHECK(world.find_by_persistent_id(world.persistent_id(second)) == second);
}

PN_TEST(scene_identity, a_dead_entity_has_no_identity_to_report) {
    World world;
    const Entity entity = world.create();
    PN_REQUIRE(world.persistent_id(entity) != pn::scene::kNoPersistentId);
    PN_REQUIRE(world.destroy(entity));
    PN_CHECK_EQ(world.persistent_id(entity), pn::scene::kNoPersistentId);
    PN_CHECK_EQ(world.persistent_id(Entity{}), pn::scene::kNoPersistentId);
}

PN_TEST(scene_identity, restoring_an_identity_advances_the_counter_past_it) {
    // A loader creates entities with the ids the file names. If the counter kept
    // going from where it was, the next entity created after a load would be
    // handed an id the file already used.
    World world;
    const Entity restored = world.create_with_id(900);
    PN_REQUIRE(restored.is_valid());
    PN_CHECK(world.next_persistent_id() > 900);

    const Entity fresh = world.create();
    PN_CHECK(world.persistent_id(fresh) > 900);
    PN_CHECK(world.persistent_id(fresh) != 900);
}

PN_TEST(scene_identity, a_duplicate_or_zero_identity_is_refused) {
    World world;
    PN_REQUIRE(world.create_with_id(7).is_valid());
    PN_CHECK(!world.create_with_id(7).is_valid());
    PN_CHECK(!world.create_with_id(pn::scene::kNoPersistentId).is_valid());
    PN_CHECK_EQ(world.entity_count(), std::size_t{1});
}

// -------------------------------------------------------------------
// Round trip
// -------------------------------------------------------------------

PN_TEST(scene_serialize, a_saved_scene_round_trips) {
    const SceneCodec codec = make_codec();

    World original;
    const Entity anchor = original.create();
    original.add(anchor, Tag{"anchor"});
    original.add(anchor, Transform{1.5, -2.25, 3.75});

    const Entity satellite = original.create();
    original.add(satellite, Tag{"satellite"});
    original.add(satellite, Transform{10.0, 20.0, 30.0});
    original.add(satellite, Link{anchor, Entity{}, 42});

    ByteWriter saved;
    codec.save(original, saved);

    World loaded;
    const auto count = codec.load(saved.data(), loaded);
    PN_REQUIRE(count.has_value());
    // Two tags, two transforms, one link.
    PN_CHECK_EQ(count.value(), std::size_t{5});
    PN_REQUIRE_EQ(loaded.entity_count(), std::size_t{2});

    const Entity loaded_anchor = loaded.find_by_persistent_id(original.persistent_id(anchor));
    const Entity loaded_satellite =
        loaded.find_by_persistent_id(original.persistent_id(satellite));
    PN_REQUIRE(loaded_anchor.is_valid());
    PN_REQUIRE(loaded_satellite.is_valid());

    const Tag* tag = loaded.get<Tag>(loaded_anchor);
    PN_REQUIRE(tag != nullptr);
    PN_CHECK_EQ(tag->text, std::string{"anchor"});

    const Transform* transform = loaded.get<Transform>(loaded_satellite);
    PN_REQUIRE(transform != nullptr);
    PN_CHECK_EQ(transform->x, 10.0);
    PN_CHECK_EQ(transform->z, 30.0);

    const Link* link = loaded.get<Link>(loaded_satellite);
    PN_REQUIRE(link != nullptr);
    // The reference repaired: it names the loaded anchor, whose handle is not
    // required to equal the original's.
    PN_CHECK(link->primary == loaded_anchor);
    PN_CHECK(!link->secondary.is_valid());
    PN_CHECK_EQ(link->weight, 42);
}

PN_TEST(scene_serialize, saving_a_loaded_scene_reproduces_the_file_byte_for_byte) {
    // The strongest form of the round-trip claim, and the one that does not
    // depend on remembering which fields to compare.
    const SceneCodec codec = make_codec();

    World original;
    Random random{20260821, 30};
    std::vector<Entity> entities;
    for (int index = 0; index < 300; ++index) {
        const Entity entity = original.create();
        entities.push_back(entity);
        if (random.uniform(0, 2) != 0) {
            original.add(entity, Transform{static_cast<double>(index), 0.5, -1.25});
        }
        if (random.uniform(0, 1) == 0) {
            original.add(entity, Tag{"entity-" + std::to_string(index)});
        }
        if (random.uniform(0, 3) == 0) {
            original.add(entity, Link{entities[random.uniform(0, entities.size() - 1)], Entity{},
                                      static_cast<std::int32_t>(index)});
        }
    }
    // Churn, so the dense storage order stops matching the entity order and the
    // file's determinism has to come from the sort rather than from luck.
    for (std::size_t index = 0; index < entities.size(); index += 3) {
        PN_REQUIRE(original.destroy(entities[index]));
    }

    ByteWriter first;
    codec.save(original, first);

    World loaded;
    PN_REQUIRE(codec.load(first.data(), loaded).has_value());

    ByteWriter second;
    codec.save(loaded, second);

    PN_REQUIRE_EQ(second.size(), first.size());
    const std::span<const std::byte> left = first.data();
    const std::span<const std::byte> right = second.data();
    for (std::size_t index = 0; index < left.size(); ++index) {
        PN_REQUIRE_EQ(static_cast<int>(left[index]), static_cast<int>(right[index]));
    }
}

PN_TEST(scene_serialize, two_worlds_with_the_same_content_write_the_same_file) {
    // The round-trip test above cannot catch a file that depends on storage
    // order, because it compares a world against its own reload and the reload
    // inherits that order. It passed with the record sort removed. This is the
    // property that actually needs holding: the file is a function of the
    // content, not of the edit history that produced it.
    //
    // Both worlds end with the same five entities, the same identities, and the
    // same components. They differ only in that one of them had a component
    // removed and put back, which moves that entity to the end of the dense
    // array while leaving the world's contents unchanged.
    const SceneCodec codec = make_codec();

    World settled;
    World churned;
    std::vector<Entity> settled_entities;
    std::vector<Entity> churned_entities;
    for (int index = 0; index < 5; ++index) {
        settled_entities.push_back(settled.create());
        churned_entities.push_back(churned.create());
    }
    for (int index = 0; index < 5; ++index) {
        const Transform value{static_cast<double>(index), 0.25, -0.5};
        settled.add(settled_entities[static_cast<std::size_t>(index)], value);
        churned.add(churned_entities[static_cast<std::size_t>(index)], value);
    }

    // The only difference: entity 1's component is removed and re-added, which
    // swaps the last element into its slot and appends it at the end.
    const Transform moved = *churned.get<Transform>(churned_entities[1]);
    PN_REQUIRE(churned.remove<Transform>(churned_entities[1]));
    PN_REQUIRE(churned.add(churned_entities[1], moved) != nullptr);

    PN_REQUIRE_EQ(settled.component_count<Transform>(), churned.component_count<Transform>());
    PN_REQUIRE_EQ(settled.persistent_id(settled_entities[1]),
                  churned.persistent_id(churned_entities[1]));

    ByteWriter left;
    ByteWriter right;
    codec.save(settled, left);
    codec.save(churned, right);

    PN_REQUIRE_EQ(right.size(), left.size());
    for (std::size_t index = 0; index < left.data().size(); ++index) {
        PN_REQUIRE_EQ(static_cast<int>(left.data()[index]), static_cast<int>(right.data()[index]));
    }
}

PN_TEST(scene_serialize, registration_order_does_not_change_the_file) {
    // The console's config file had this exact hole: a test that varied only the
    // order of assignments, not the order of registration. Which module
    // registers a component type first changes with build configuration.
    World world;
    const Entity entity = world.create();
    world.add(entity, Tag{"one"});
    world.add(entity, Transform{1.0, 2.0, 3.0});
    world.add(entity, Link{entity, Entity{}, 5});

    SceneCodec forward;
    forward.register_component<Transform>("pn.Transform");
    forward.register_component<Tag>("pn.Tag");
    forward.register_component<Link>("pn.Link");

    SceneCodec backward;
    backward.register_component<Link>("pn.Link");
    backward.register_component<Tag>("pn.Tag");
    backward.register_component<Transform>("pn.Transform");

    ByteWriter left;
    ByteWriter right;
    forward.save(world, left);
    backward.save(world, right);

    PN_REQUIRE_EQ(left.size(), right.size());
    for (std::size_t index = 0; index < left.data().size(); ++index) {
        PN_REQUIRE_EQ(static_cast<int>(left.data()[index]), static_cast<int>(right.data()[index]));
    }
}

// -------------------------------------------------------------------
// Reference repair
// -------------------------------------------------------------------

PN_TEST(scene_serialize, a_reference_survives_the_target_being_renamed) {
    // The criterion's second clause. Renaming happens on both sides of the save
    // here: once before, once after, and the reference is unmoved by either -
    // because it was never keyed on the name.
    const SceneCodec codec = make_codec();

    World original;
    const Entity target = original.create();
    original.add(target, Tag{"original-name"});
    const Entity holder = original.create();
    original.add(holder, Link{target, Entity{}, 1});

    original.replace(target, Tag{"renamed-before-save"});

    ByteWriter saved;
    codec.save(original, saved);

    World loaded;
    PN_REQUIRE(codec.load(saved.data(), loaded).has_value());

    const Entity loaded_holder = loaded.find_by_persistent_id(original.persistent_id(holder));
    const Entity loaded_target = loaded.find_by_persistent_id(original.persistent_id(target));
    PN_REQUIRE(loaded_holder.is_valid());
    PN_REQUIRE(loaded_target.is_valid());

    const Link* link = loaded.get<Link>(loaded_holder);
    PN_REQUIRE(link != nullptr);
    PN_CHECK(link->primary == loaded_target);
    PN_CHECK_EQ(loaded.get<Tag>(loaded_target)->text, std::string{"renamed-before-save"});

    loaded.replace(loaded_target, Tag{"renamed-after-load"});
    PN_CHECK(loaded.get<Link>(loaded_holder)->primary == loaded_target);
    PN_CHECK(loaded.get<Tag>(loaded_target) != nullptr);
}

PN_TEST(scene_serialize, a_reference_to_a_destroyed_entity_does_not_become_its_replacement) {
    // The failure this whole mechanism exists to prevent, arranged deliberately:
    // the destroyed target's slot is reused by a different entity before the
    // save. A reference written as a raw slot index comes back pointing at the
    // replacement, and neither the file nor the load says a word about it.
    const SceneCodec codec = make_codec();

    World original;
    const Entity doomed = original.create();
    original.add(doomed, Tag{"doomed"});
    const Entity holder = original.create();
    original.add(holder, Link{doomed, Entity{}, 9});

    PN_REQUIRE(original.destroy(doomed));
    const Entity replacement = original.create();
    original.add(replacement, Tag{"replacement"});
    PN_REQUIRE_EQ(replacement.index(), doomed.index());
    // The stale handle is still sitting in the component, which is the whole
    // premise: nothing rewrites components when an entity dies.
    PN_REQUIRE(original.get<Link>(holder)->primary == doomed);

    ByteWriter saved;
    codec.save(original, saved);

    World loaded;
    PN_REQUIRE(codec.load(saved.data(), loaded).has_value());

    const Entity loaded_holder = loaded.find_by_persistent_id(original.persistent_id(holder));
    const Entity loaded_replacement =
        loaded.find_by_persistent_id(original.persistent_id(replacement));
    PN_REQUIRE(loaded_holder.is_valid());
    PN_REQUIRE(loaded_replacement.is_valid());

    const Link* link = loaded.get<Link>(loaded_holder);
    PN_REQUIRE(link != nullptr);
    PN_CHECK(!link->primary.is_valid());
    PN_CHECK(!(link->primary == loaded_replacement));
}

PN_TEST(scene_serialize, references_survive_a_churned_world_entity_by_entity) {
    // Every reference in a world built by random churn, checked after the round
    // trip against what it pointed at before it. A spot check on a handful of
    // links would miss an off-by-one in the ordinal mapping that only shows up
    // once slots stop being dense.
    const SceneCodec codec = make_codec();

    World original;
    Random random{20260821, 31};
    std::vector<Entity> live;
    for (int step = 0; step < 4000; ++step) {
        const std::uint64_t choice = random.uniform(0, 99);
        if (live.empty() || choice < 55) {
            const Entity entity = original.create();
            original.add(entity, Tag{"e" + std::to_string(step)});
            if (!live.empty() && random.uniform(0, 1) == 0) {
                const Entity primary = live[random.uniform(0, live.size() - 1)];
                const Entity secondary = live[random.uniform(0, live.size() - 1)];
                original.add(entity, Link{primary, secondary, static_cast<std::int32_t>(step)});
            }
            live.push_back(entity);
            continue;
        }
        const std::size_t slot = random.uniform(0, live.size() - 1);
        PN_REQUIRE(original.destroy(live[slot]));
        live[slot] = live.back();
        live.pop_back();
    }

    // What every live link pointed at, by identity, before the save. Identity
    // rather than handle, because the handles are about to change.
    struct Expectation {
        bool has_link = false;
        PersistentId primary = 0;
        PersistentId secondary = 0;
        std::int32_t weight = 0;
        std::string tag;
    };
    std::map<PersistentId, Expectation> expected;
    original.for_each_entity([&](Entity entity) {
        Expectation record;
        record.tag = original.get<Tag>(entity)->text;
        if (const Link* link = original.get<Link>(entity); link != nullptr) {
            record.has_link = true;
            record.primary = original.persistent_id(link->primary);
            record.secondary = original.persistent_id(link->secondary);
            record.weight = link->weight;
        }
        expected.emplace(original.persistent_id(entity), record);
    });
    PN_REQUIRE_EQ(expected.size(), original.entity_count());

    ByteWriter saved;
    codec.save(original, saved);
    World loaded;
    PN_REQUIRE(codec.load(saved.data(), loaded).has_value());
    PN_REQUIRE_EQ(loaded.entity_count(), original.entity_count());

    std::size_t checked = 0;
    std::size_t links_seen = 0;
    std::size_t dropped = 0;
    for (const auto& [id, record] : expected) {
        const Entity entity = loaded.find_by_persistent_id(id);
        PN_REQUIRE(entity.is_valid());
        PN_REQUIRE_EQ(loaded.get<Tag>(entity)->text, record.tag);

        const Link* link = loaded.get<Link>(entity);
        PN_REQUIRE_EQ(link != nullptr, record.has_link);
        if (link != nullptr) {
            ++links_seen;
            PN_REQUIRE_EQ(loaded.persistent_id(link->primary), record.primary);
            PN_REQUIRE_EQ(loaded.persistent_id(link->secondary), record.secondary);
            PN_REQUIRE_EQ(link->weight, record.weight);
            dropped += (record.primary == 0 ? std::size_t{1} : std::size_t{0}) +
                       (record.secondary == 0 ? std::size_t{1} : std::size_t{0});
        }
        ++checked;
    }
    PN_CHECK_EQ(checked, expected.size());
    // Both outcomes have to be present for this to be evidence: links that were
    // repaired, and links whose target had died and were correctly dropped.
    PN_CHECK(links_seen > 0);
    PN_CHECK(dropped > 0);
}

// -------------------------------------------------------------------
// Registration and rejection
// -------------------------------------------------------------------

PN_TEST(scene_serialize, a_component_with_no_codec_is_not_saved_and_is_not_an_error) {
    const SceneCodec codec = make_codec();
    World original;
    const Entity entity = original.create();
    original.add(entity, Tag{"kept"});
    original.add(entity, EditorOnly{true});

    ByteWriter saved;
    codec.save(original, saved);
    World loaded;
    PN_REQUIRE(codec.load(saved.data(), loaded).has_value());

    const Entity restored = loaded.find_by_persistent_id(original.persistent_id(entity));
    PN_REQUIRE(restored.is_valid());
    PN_CHECK(loaded.get<Tag>(restored) != nullptr);
    PN_CHECK(loaded.get<EditorOnly>(restored) == nullptr);
}

PN_TEST(scene_serialize, a_type_the_reader_does_not_know_is_skipped_rather_than_refused) {
    // Forward compatibility, which is why each type's block carries its length.
    World original;
    const Entity entity = original.create();
    original.add(entity, Tag{"kept"});
    original.add(entity, Transform{1.0, 2.0, 3.0});

    const SceneCodec writer_codec = make_codec();
    ByteWriter saved;
    writer_codec.save(original, saved);

    SceneCodec older;
    older.register_component<Tag>("pn.Tag");

    World loaded;
    const auto result = older.load(saved.data(), loaded);
    PN_REQUIRE(result.has_value());
    const Entity restored = loaded.find_by_persistent_id(original.persistent_id(entity));
    PN_REQUIRE(restored.is_valid());
    PN_CHECK(loaded.get<Tag>(restored) != nullptr);
    PN_CHECK(loaded.get<Transform>(restored) == nullptr);
}

PN_TEST(scene_serialize, a_name_or_a_type_cannot_be_registered_twice) {
    SceneCodec codec;
    PN_CHECK(codec.register_component<Transform>("pn.Transform"));
    PN_CHECK(!codec.register_component<Tag>("pn.Transform"));
    PN_CHECK(!codec.register_component<Transform>("pn.SomethingElse"));
    PN_CHECK(!codec.register_component<Tag>(""));
    PN_CHECK_EQ(codec.registered_type_count(), std::size_t{1});
}

PN_TEST(scene_serialize, loading_into_a_populated_world_is_refused) {
    const SceneCodec codec = make_codec();
    World original;
    original.add(original.create(), Tag{"one"});
    ByteWriter saved;
    codec.save(original, saved);

    World occupied;
    occupied.create();
    const auto result = codec.load(saved.data(), occupied);
    PN_CHECK(!result.has_value());
    PN_CHECK_EQ(result.error().category(), pn::core::ErrorCategory::invalid_argument);
}

PN_TEST(scene_serialize, a_damaged_file_is_reported_rather_than_loaded) {
    const SceneCodec codec = make_codec();
    World original;
    const Entity entity = original.create();
    original.add(entity, Tag{"one"});
    original.add(entity, Transform{1.0, 2.0, 3.0});

    ByteWriter saved;
    codec.save(original, saved);
    std::vector<std::byte> bytes{saved.data().begin(), saved.data().end()};

    // Every single-byte corruption in the file, one at a time. The checksum
    // covers the header as well as the payload, so every one of them must be
    // caught - and any that is not is a hole in the framing rather than a
    // tolerable outcome.
    std::size_t rejected = 0;
    for (std::size_t index = 0; index < bytes.size(); ++index) {
        std::vector<std::byte> damaged = bytes;
        damaged[index] ^= std::byte{0x5A};
        World loaded;
        if (!codec.load(damaged, loaded).has_value()) {
            ++rejected;
        }
    }
    PN_CHECK_EQ(rejected, bytes.size());
}

PN_TEST(scene_serialize, an_empty_world_round_trips) {
    const SceneCodec codec = make_codec();
    World original;
    ByteWriter saved;
    codec.save(original, saved);

    World loaded;
    const auto count = codec.load(saved.data(), loaded);
    PN_REQUIRE(count.has_value());
    PN_CHECK_EQ(count.value(), std::size_t{0});
    PN_CHECK_EQ(loaded.entity_count(), std::size_t{0});
}
