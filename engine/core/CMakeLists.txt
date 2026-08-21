# Pention Engine - core module
# Requirement: PN-PLT-004, PN-PLT-006, PN-PLT-009, PN-PLT-010, PN-PLT-007, PN-PLT-011, PN-PLT-027
# Decision:    ADR-0002, ADR-0004, ADR-0007

pn_add_module(core
    SOURCES
        src/memory.cpp
    PUBLIC_DEPS
        pn::math)

pn_add_test(core_expected SOURCES tests/expected_test.cpp DEPS pn::core)
pn_add_test(core_handle   SOURCES tests/handle_test.cpp   DEPS pn::core)
pn_add_test(core_memory   SOURCES tests/memory_test.cpp   DEPS pn::core)
pn_add_test(core_random   SOURCES tests/random_test.cpp   DEPS pn::core)
pn_add_test(core_containers SOURCES tests/containers_test.cpp DEPS pn::core)
pn_add_test(core_collections SOURCES tests/collections_test.cpp DEPS pn::core)
pn_add_test(core_name     SOURCES tests/name_test.cpp     DEPS pn::core)
