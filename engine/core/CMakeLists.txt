# Pention Engine - core module
# Requirement: PN-PLT-011, PN-PLT-027
# Decision:    ADR-0002, ADR-0004

pn_add_module(core INTERFACE
    PUBLIC_DEPS pn::math)

pn_add_test(core_expected SOURCES tests/expected_test.cpp DEPS pn::core)
pn_add_test(core_handle   SOURCES tests/handle_test.cpp   DEPS pn::core)
