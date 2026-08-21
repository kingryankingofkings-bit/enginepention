// Pention Engine - scene/tests/world_test.cpp
// Requirement: PN-OBJ-001
// Decision:    ADR-0004
//
// PN-OBJ-001's criterion names two things: add/remove/query under randomized
// churn, and handles that never silently alias. The second is the one that
// matters most and is easiest to get wrong quietly - an aliasing handle returns
// the wrong component rather than failing, so nothing crashes and the bug
// presents as a gameplay oddity weeks later.

#include "pn/core/random.hpp"
#include "pn/scene/world.hpp"
#include "pn/testing/test.hpp"

#include <cstdint>
#include <map>
#include <set>
#include <string>
#include <vector>

namespace {

using pn::core::Random;
using pn::scene::Entity;
using pn::scene::World;

struct Position {
    float x = 0.0F;
    float y = 0.0F;
    float z = 0.0F;
};

struct Velocity {
    float x = 0.0F;
    float y = 0.0F;
    float z = 0.0F;
};

struct Label {
    std::string text;
};

}  // namespace

// -------------------------------------------------------------------
// Entities and handle identity
// -------------------------------------------------------------------

PN_TEST(world, creates_and_destroys_entities) {
    World world;
    PN_CHECK_EQ(world.entity_count(), std::size_t{0});

    const Entity first = world.create();
    const Entity second = world.create();
    PN_CHECK_EQ(world.entity_count(), std::size_t{2});
    PN_CHECK(world.alive(first));
    PN_CHECK(world.alive(second));
    PN_CHECK(!(first == second));

    PN_CHECK(world.destroy(first));
    PN_CHECK(!world.alive(first));
    PN_CHECK(world.alive(second));
    PN_CHECK_EQ(world.entity_count(), std::size_t{1});
}

PN_TEST(world, a_destroyed_handle_never_aliases_the_entity_that_takes_its_slot) {
    // The criterion's second clause, and the failure mode it exists to prevent:
    // a recycled slot handing an old handle someone else's components. Nothing
    // crashes when that happens - the wrong data is simply returned - so it
    // shows up as a gameplay oddity long after the cause.
    World world;
    const Entity original = world.create();
    world.add(original, Position{1.0F, 2.0F, 3.0F});

    PN_REQUIRE(world.destroy(original));
    const Entity recycled = world.create();

    // The slot is reused, which is the point of recycling.
    PN_CHECK_EQ(recycled.index(), original.index());
    // And the generation differs, which is what makes the old handle detectably
    // stale rather than an alias.
    PN_CHECK(recycled.generation() != original.generation());
    PN_CHECK(!(recycled == original));

    PN_CHECK(!world.alive(original));
    PN_CHECK(world.get<Position>(original) == nullptr);
    PN_CHECK(!world.has<Position>(recycled));

    world.add(recycled, Position{9.0F, 9.0F, 9.0F});
    PN_CHECK(world.get<Position>(original) == nullptr);
    PN_REQUIRE(world.get<Position>(recycled) != nullptr);
    PN_CHECK_EQ(world.get<Position>(recycled)->x, 9.0F);
}

PN_TEST(world, a_default_handle_is_not_alive_even_at_index_zero) {
    // A default-constructed handle has an invalid index, and its generation is
    // zero while slots start at one. Both guards matter: an engine that
    // zero-initialises a struct full of entity handles must not find them
    // pointing at whatever was created first.
    World world;
    const Entity first = world.create();
    PN_CHECK_EQ(first.index(), std::uint32_t{0});

    const Entity none;
    PN_CHECK(!world.alive(none));
    PN_CHECK(world.get<Position>(none) == nullptr);
    PN_CHECK(world.add(none, Position{}) == nullptr);
    PN_CHECK(!world.destroy(none));
}

PN_TEST(world, destroying_twice_reports_the_second_attempt) {
    // A double destroy is a real defect; tolerating it silently hides the defect
    // rather than the symptom.
    World world;
    const Entity entity = world.create();
    PN_CHECK(world.destroy(entity));
    PN_CHECK(!world.destroy(entity));
    PN_CHECK_EQ(world.entity_count(), std::size_t{0});
}

PN_TEST(world, generations_keep_rising_across_many_recycles) {
    World world;
    std::vector<Entity> history;
    for (int round = 0; round < 100; ++round) {
        const Entity entity = world.create();
        history.push_back(entity);
        PN_REQUIRE(world.destroy(entity));
    }
    // Every handle ever issued for this slot is distinct, and none of them is
    // alive.
    std::set<std::uint32_t> generations;
    for (const Entity& entity : history) {
        generations.insert(entity.generation());
        PN_REQUIRE(!world.alive(entity));
    }
    PN_CHECK_EQ(generations.size(), std::size_t{100});
}

// -------------------------------------------------------------------
// Components
// -------------------------------------------------------------------

PN_TEST(world, adds_gets_and_removes_components) {
    World world;
    const Entity entity = world.create();

    PN_REQUIRE(world.add(entity, Position{1.0F, 2.0F, 3.0F}) != nullptr);
    PN_CHECK(world.has<Position>(entity));
    PN_CHECK(!world.has<Velocity>(entity));

    Position* position = world.get<Position>(entity);
    PN_REQUIRE(position != nullptr);
    PN_CHECK_EQ(position->y, 2.0F);
    position->y = 20.0F;
    PN_CHECK_EQ(world.get<Position>(entity)->y, 20.0F);

    PN_CHECK(world.remove<Position>(entity));
    PN_CHECK(!world.has<Position>(entity));
    PN_CHECK(!world.remove<Position>(entity));
}

PN_TEST(world, adding_a_component_twice_is_refused_rather_than_overwriting) {
    // A silent overwrite would make add and replace the same call, and the
    // caller who meant add would never learn the entity already had one.
    World world;
    const Entity entity = world.create();

    PN_REQUIRE(world.add(entity, Position{1.0F, 0.0F, 0.0F}) != nullptr);
    PN_CHECK(world.add(entity, Position{7.0F, 0.0F, 0.0F}) == nullptr);
    PN_CHECK_EQ(world.get<Position>(entity)->x, 1.0F);

    PN_REQUIRE(world.replace(entity, Position{7.0F, 0.0F, 0.0F}) != nullptr);
    PN_CHECK_EQ(world.get<Position>(entity)->x, 7.0F);
}

PN_TEST(world, a_dead_entity_accepts_no_components) {
    World world;
    const Entity entity = world.create();
    PN_REQUIRE(world.destroy(entity));

    PN_CHECK(world.add(entity, Position{}) == nullptr);
    PN_CHECK(world.replace(entity, Position{}) == nullptr);
    PN_CHECK(!world.remove<Position>(entity));
    PN_CHECK(!world.has<Position>(entity));
}

PN_TEST(world, destroying_an_entity_removes_every_component_it_had) {
    // Leaving one behind is the leak that makes a component count drift upward
    // over a session, and it is invisible until something iterates.
    World world;
    const Entity entity = world.create();
    world.add(entity, Position{});
    world.add(entity, Velocity{});
    world.add(entity, Label{"named"});

    PN_CHECK_EQ(world.component_count<Position>(), std::size_t{1});
    PN_CHECK_EQ(world.component_count<Label>(), std::size_t{1});

    PN_REQUIRE(world.destroy(entity));
    PN_CHECK_EQ(world.component_count<Position>(), std::size_t{0});
    PN_CHECK_EQ(world.component_count<Velocity>(), std::size_t{0});
    PN_CHECK_EQ(world.component_count<Label>(), std::size_t{0});
}

PN_TEST(world, components_of_one_type_are_contiguous) {
    // The reason for the storage layout. If these were not adjacent, the world
    // would have no advantage over a map from entity to component.
    World world;
    for (int index = 0; index < 16; ++index) {
        const Entity entity = world.create();
        world.add(entity, Position{static_cast<float>(index), 0.0F, 0.0F});
    }

    const std::span<Position> positions = world.components<Position>();
    PN_REQUIRE_EQ(positions.size(), std::size_t{16});
    for (std::size_t index = 0; index + 1 < positions.size(); ++index) {
        PN_REQUIRE_EQ(&positions[index] + 1, &positions[index + 1]);
    }
}

// -------------------------------------------------------------------
// Queries
// -------------------------------------------------------------------

PN_TEST(world, a_query_visits_exactly_the_entities_with_every_named_component) {
    World world;
    const Entity both = world.create();
    world.add(both, Position{});
    world.add(both, Velocity{});

    const Entity only_position = world.create();
    world.add(only_position, Position{});

    const Entity only_velocity = world.create();
    world.add(only_velocity, Velocity{});

    const Entity neither = world.create();
    (void)neither;

    std::vector<Entity> visited;
    world.each<Position, Velocity>(
        [&visited](Entity entity, Position&, Velocity&) { visited.push_back(entity); });

    PN_REQUIRE_EQ(visited.size(), std::size_t{1});
    PN_CHECK(visited[0] == both);
    PN_CHECK_EQ(world.count_with<Position>(), std::size_t{2});
    PN_CHECK_EQ((world.count_with<Position, Velocity>()), std::size_t{1});
}

PN_TEST(world, a_query_hands_the_visitor_references_it_can_write_through) {
    World world;
    for (int index = 0; index < 8; ++index) {
        const Entity entity = world.create();
        world.add(entity, Position{0.0F, 0.0F, 0.0F});
        world.add(entity, Velocity{1.0F, 2.0F, 3.0F});
    }

    world.each<Velocity, Position>([](Entity, Velocity& velocity, Position& position) {
        position.x += velocity.x;
        position.y += velocity.y;
    });

    for (const Position& position : world.components<Position>()) {
        PN_REQUIRE_EQ(position.x, 1.0F);
        PN_REQUIRE_EQ(position.y, 2.0F);
    }
}

PN_TEST(world, a_query_for_a_component_nobody_has_visits_nothing) {
    World world;
    const Entity entity = world.create();
    world.add(entity, Position{});

    std::size_t visits = 0;
    world.each<Velocity>([&visits](Entity, Velocity&) { ++visits; });
    PN_CHECK_EQ(visits, std::size_t{0});
    PN_CHECK_EQ((world.count_with<Velocity, Position>()), std::size_t{0});
}

PN_TEST(world, a_query_yields_a_handle_that_resolves_back_to_the_same_entity) {
    // The handle handed to a visitor has to carry the current generation. One
    // built with a stale generation would be refused by every accessor, and the
    // visitor's only way to touch the rest of the entity is through it.
    World world;
    const Entity created = world.create();
    world.add(created, Position{4.0F, 0.0F, 0.0F});
    world.add(created, Label{"findable"});

    // Captured by reference rather than by an explicit list: the check macros
    // refer to the enclosing test's context, so a visitor that checks anything
    // has to capture it.
    world.each<Position>([&](Entity entity, Position& position) {
        PN_CHECK(entity == created);
        PN_CHECK(world.alive(entity));
        const Label* label = world.get<Label>(entity);
        PN_REQUIRE(label != nullptr);
        PN_CHECK(label->text == "findable");
        PN_CHECK_EQ(position.x, 4.0F);
    });
}

PN_TEST(world, a_query_after_churn_still_sees_only_live_entities) {
    // Slot recycling means a query's handles are built from the slot table. If
    // that were read wrongly, a query would hand out handles for entities that
    // no longer exist.
    World world;
    std::vector<Entity> keep;
    for (int index = 0; index < 50; ++index) {
        const Entity entity = world.create();
        world.add(entity, Position{static_cast<float>(index), 0.0F, 0.0F});
        if (index % 2 == 0) {
            keep.push_back(entity);
        }
    }
    for (int index = 1; index < 50; index += 2) {
        // Destroy the odd ones by looking them up through the query.
        std::vector<Entity> doomed;
        world.each<Position>([&doomed, index](Entity entity, const Position& position) {
            if (static_cast<int>(position.x) == index) {
                doomed.push_back(entity);
            }
        });
        for (const Entity& entity : doomed) {
            PN_REQUIRE(world.destroy(entity));
        }
    }

    std::size_t visited = 0;
    world.each<Position>([&](Entity entity, Position&) {
        PN_REQUIRE(world.alive(entity));
        ++visited;
    });
    PN_CHECK_EQ(visited, keep.size());
    PN_CHECK_EQ(world.entity_count(), keep.size());
}

// -------------------------------------------------------------------
// Randomized churn against an oracle
// -------------------------------------------------------------------

PN_TEST(world, agrees_with_a_reference_model_under_randomized_churn) {
    // The criterion's first clause. Every operation is applied to the world and
    // to a plain map keyed by handle, and the two are compared after each one.
    // A divergence fails immediately, with a seed that reproduces it.
    struct Reference {
        bool alive = true;
        bool has_position = false;
        Position position{};
        bool has_velocity = false;
        Velocity velocity{};
    };

    World world;
    std::map<std::uint64_t, Reference> oracle;
    std::vector<Entity> live;
    Random random{20260821, 21};

    auto key_of = [](Entity entity) {
        return (static_cast<std::uint64_t>(entity.index()) << 32) | entity.generation();
    };

    for (int step = 0; step < 120000; ++step) {
        const std::uint64_t choice = random.uniform(0, 99);

        if (choice < 22 || live.empty()) {
            const Entity entity = world.create();
            oracle[key_of(entity)] = Reference{};
            live.push_back(entity);
            continue;
        }

        const std::size_t slot = random.uniform(0, live.size() - 1);
        const Entity entity = live[slot];

        if (choice < 40) {
            const Position value{static_cast<float>(random.uniform(0, 999)), 0.0F, 0.0F};
            const bool added = world.add(entity, value) != nullptr;
            Reference& reference = oracle[key_of(entity)];
            PN_REQUIRE_EQ(added, !reference.has_position);
            if (added) {
                reference.has_position = true;
                reference.position = value;
            }
        } else if (choice < 55) {
            const Velocity value{static_cast<float>(random.uniform(0, 999)), 0.0F, 0.0F};
            const bool added = world.add(entity, value) != nullptr;
            Reference& reference = oracle[key_of(entity)];
            PN_REQUIRE_EQ(added, !reference.has_velocity);
            if (added) {
                reference.has_velocity = true;
                reference.velocity = value;
            }
        } else if (choice < 68) {
            const bool removed = world.remove<Position>(entity);
            Reference& reference = oracle[key_of(entity)];
            PN_REQUIRE_EQ(removed, reference.has_position);
            reference.has_position = false;
        } else if (choice < 78) {
            const bool removed = world.remove<Velocity>(entity);
            Reference& reference = oracle[key_of(entity)];
            PN_REQUIRE_EQ(removed, reference.has_velocity);
            reference.has_velocity = false;
        } else if (choice < 90) {
            PN_REQUIRE(world.destroy(entity));
            Reference& reference = oracle[key_of(entity)];
            reference.alive = false;
            reference.has_position = false;
            reference.has_velocity = false;
            live[slot] = live.back();
            live.pop_back();
        } else {
            const Reference& reference = oracle[key_of(entity)];
            const Position* position = world.get<Position>(entity);
            PN_REQUIRE_EQ(position != nullptr, reference.has_position);
            if (position != nullptr) {
                PN_REQUIRE_EQ(position->x, reference.position.x);
            }
            const Velocity* velocity = world.get<Velocity>(entity);
            PN_REQUIRE_EQ(velocity != nullptr, reference.has_velocity);
        }

        PN_REQUIRE_EQ(world.entity_count(), live.size());
    }

    // Every handle ever issued, live or not, must resolve exactly as the model
    // says. This is where an aliasing handle would finally be caught: a dead
    // one that resolves to a live entity's component.
    std::size_t position_total = 0;
    std::size_t velocity_total = 0;
    for (const auto& [key, reference] : oracle) {
        const Entity entity{static_cast<std::uint32_t>(key >> 32),
                            static_cast<std::uint32_t>(key & 0xFFFFFFFFU)};
        PN_REQUIRE_EQ(world.alive(entity), reference.alive);
        PN_REQUIRE_EQ(world.get<Position>(entity) != nullptr, reference.has_position);
        PN_REQUIRE_EQ(world.get<Velocity>(entity) != nullptr, reference.has_velocity);
        position_total += reference.has_position ? 1 : 0;
        velocity_total += reference.has_velocity ? 1 : 0;
    }

    PN_CHECK_EQ(world.component_count<Position>(), position_total);
    PN_CHECK_EQ(world.component_count<Velocity>(), velocity_total);
    std::size_t both_in_model = 0;
    for (const auto& [key, reference] : oracle) {
        both_in_model += (reference.has_position && reference.has_velocity) ? 1 : 0;
    }
    PN_CHECK_EQ((world.count_with<Position, Velocity>()), both_in_model);
}

PN_TEST(world, components_that_own_memory_are_destroyed_exactly_once) {
    // Strings rather than a counter, so a leak or a double free is caught by the
    // sanitizers rather than only by arithmetic.
    World world;
    std::vector<Entity> entities;
    for (int index = 0; index < 200; ++index) {
        const Entity entity = world.create();
        world.add(entity, Label{std::string(120, static_cast<char>('a' + index % 26))});
        entities.push_back(entity);
    }
    for (std::size_t index = 0; index < entities.size(); index += 2) {
        PN_REQUIRE(world.destroy(entities[index]));
    }
    PN_CHECK_EQ(world.component_count<Label>(), std::size_t{100});

    std::size_t seen = 0;
    world.each<Label>([&](Entity, Label& label) {
        PN_REQUIRE_EQ(label.text.size(), std::size_t{120});
        ++seen;
    });
    PN_CHECK_EQ(seen, std::size_t{100});
}
