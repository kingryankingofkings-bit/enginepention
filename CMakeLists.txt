# Pention Engine - root build configuration
# Requirement: PN-OPS-001, PN-OPS-002, PN-OPS-003
# Decision:    ADR-0001 (C++23, CMake + Ninja as external orchestration only)
#
# CMake and Ninja are build-time orchestration tools. They are not engine
# components and no engine code depends on them at runtime. See
# DEPENDENCY_BOUNDARY.md.

cmake_minimum_required(VERSION 3.24)

project(Pention
    VERSION 0.0.1
    DESCRIPTION "Clean-room custom-built 3D sandbox game engine"
    LANGUAGES CXX)

if(PROJECT_SOURCE_DIR STREQUAL PROJECT_BINARY_DIR)
    message(FATAL_ERROR "In-source builds are not permitted. Use a separate build directory.")
endif()

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "" FORCE)
endif()

# ---------------------------------------------------------------------------
# Options
# ---------------------------------------------------------------------------

option(PN_BUILD_TESTS       "Build the engine test suite"                  ON)
option(PN_WARNINGS_AS_ERRORS "Treat compiler warnings as errors"           ON)
option(PN_NO_EXCEPTIONS     "Compile without exception support"            OFF)
option(PN_SANITIZE_ADDRESS  "Enable AddressSanitizer + UndefinedBehaviour" OFF)
option(PN_SANITIZE_THREAD   "Enable ThreadSanitizer"                       OFF)

if(PN_SANITIZE_ADDRESS AND PN_SANITIZE_THREAD)
    message(FATAL_ERROR "ASan and TSan cannot be enabled together.")
endif()

list(APPEND CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake")
include(PentionCompilerSettings)
include(PentionModule)
include(PentionRust)

# ---------------------------------------------------------------------------
# Modules, in dependency order (ADR-0004)
# ---------------------------------------------------------------------------

# Test-only infrastructure. Sits outside the runtime dependency graph: nothing
# that ships may depend on it, and it depends on nothing itself.
if(PN_BUILD_TESTS)
    enable_testing()
    add_subdirectory(engine/testing)
endif()

add_subdirectory(engine/math)
add_subdirectory(engine/core)
add_subdirectory(engine/platform)
add_subdirectory(engine/jobs)

# Rust crates sit above the C++ modules and link against them (ADR-0009).
pn_configure_rust()

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------

message(STATUS "")
message(STATUS "Pention ${PROJECT_VERSION}")
message(STATUS "  compiler        : ${CMAKE_CXX_COMPILER_ID} ${CMAKE_CXX_COMPILER_VERSION}")
message(STATUS "  build type      : ${CMAKE_BUILD_TYPE}")
message(STATUS "  tests           : ${PN_BUILD_TESTS}")
message(STATUS "  warnings->errors: ${PN_WARNINGS_AS_ERRORS}")
message(STATUS "  no-exceptions   : ${PN_NO_EXCEPTIONS}")
message(STATUS "  asan+ubsan      : ${PN_SANITIZE_ADDRESS}")
message(STATUS "  tsan            : ${PN_SANITIZE_THREAD}")
message(STATUS "  rust crates     : ${PN_RUST_ENABLED}")
message(STATUS "")
