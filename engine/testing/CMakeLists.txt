# Pention Engine - testing module
# Requirement: PN-PLT-026
# Decision:    ADR-0004 (test-only; outside the runtime dependency graph)

pn_add_module(testing
    SOURCES src/test.cpp)

# The default entry point is compiled into each test executable rather than
# archived into the library. A main() inside a static archive is only pulled in
# if the linker resolves it from the startup object, which is true on some
# toolchains and not others; compiling it directly removes the question.
set(PN_TESTING_MAIN_SOURCE "${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp"
    CACHE INTERNAL "Default test entry point")

pn_add_test(testing_self
    SOURCES tests/self_test.cpp
    DEPS    pn::testing)
