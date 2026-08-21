# Pention Engine - platform module
# Requirement: PN-PLT-003, PN-PLT-005, PN-PLT-012
# Decision:    ADR-0004, ADR-0007

find_package(Threads REQUIRED)

pn_add_module(platform
    SOURCES
        src/virtual_memory.cpp
        src/thread.cpp
    PUBLIC_DEPS
        pn::core
        Threads::Threads)

pn_add_test(platform_virtual_memory SOURCES tests/virtual_memory_test.cpp DEPS pn::platform)
pn_add_test(platform_clock          SOURCES tests/clock_test.cpp          DEPS pn::platform)
pn_add_test(platform_thread         SOURCES tests/thread_test.cpp         DEPS pn::platform)
