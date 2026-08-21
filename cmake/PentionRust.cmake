# Pention Engine - Rust integration
# Requirement: PN-OBJ-017, PN-OPS-001
# Decision:    ADR-0009
#
# CMake stays authoritative. It builds the C++ static libraries first, then
# drives cargo with the output directory in PN_NATIVE_LIB_DIR. There is one
# build entry point and one test gate, not two.

find_program(PN_CARGO_EXECUTABLE cargo)

set(PN_RUST_MANIFEST "${PROJECT_SOURCE_DIR}/rust/Cargo.toml")
set(PN_RUST_TARGET_DIR "${CMAKE_BINARY_DIR}/rust-target")

if(NOT PN_CARGO_EXECUTABLE)
    message(STATUS "cargo not found - Rust crates will not be built")
    set(PN_RUST_ENABLED OFF)
elseif(PN_SANITIZE_ADDRESS OR PN_SANITIZE_THREAD)
    # A sanitizer build instruments the C++ archives. Linking those into a Rust
    # binary that was not instrumented the same way produces link errors or,
    # worse, a binary whose runtime checks disagree with each other. Stable Rust
    # has no matching sanitizer support, so the honest option is to skip the
    # Rust half in these configurations rather than to ship a half-instrumented
    # binary and call it covered.
    #
    # The C++ side, which owns all the threading, still gets full sanitizer
    # coverage. What is NOT covered here is the FFI boundary itself under a
    # sanitizer, and that limit is stated rather than glossed.
    message(STATUS "Sanitizer configuration - Rust crates skipped (see cmake/PentionRust.cmake)")
    set(PN_RUST_ENABLED OFF)
else()
    set(PN_RUST_ENABLED ON)
endif()

function(pn_configure_rust)
    if(NOT PN_RUST_ENABLED)
        return()
    endif()

    set(_cargo_env
        "PN_NATIVE_LIB_DIR=${CMAKE_BINARY_DIR}"
        "CARGO_TARGET_DIR=${PN_RUST_TARGET_DIR}")

    # --offline is deliberate and load-bearing, not a convenience: the workspace
    # declares no external dependencies, so any attempt to reach the network is
    # itself a boundary violation and should fail the build.
    add_custom_target(pn_rust ALL
        COMMAND ${CMAKE_COMMAND} -E env ${_cargo_env}
                ${PN_CARGO_EXECUTABLE} build --offline --manifest-path "${PN_RUST_MANIFEST}"
        COMMENT "Building Rust crates against the native libraries"
        VERBATIM
        USES_TERMINAL)

    add_dependencies(pn_rust pn_jobs pn_platform pn_core)

    if(PN_BUILD_TESTS)
        # One cargo invocation runs every crate's tests: the FFI boundary, the
        # registry parser and emitter, and the generated Vulkan bindings.
        add_test(NAME rust_workspace
                 COMMAND ${CMAKE_COMMAND} -E env ${_cargo_env}
                         ${PN_CARGO_EXECUTABLE} test --offline
                         --manifest-path "${PN_RUST_MANIFEST}")
        set_tests_properties(rust_workspace PROPERTIES TIMEOUT 600)
    endif()
endfunction()
