# Pention Engine - scene module
# Requirement: PN-OBJ-001, PN-OBJ-002, PN-PLT-021
# Decision:    ADR-0004

# Header-only for now: the world is a template all the way down, because a
# component storage's element type is only known at the call site. An INTERFACE
# target says that honestly rather than compiling an empty translation unit to
# look like the others.
pn_add_module(scene INTERFACE
    PUBLIC_DEPS pn::jobs)

pn_add_test(scene_world SOURCES tests/world_test.cpp DEPS pn::scene)
pn_add_test(scene_serialize SOURCES tests/serialize_test.cpp DEPS pn::scene)
