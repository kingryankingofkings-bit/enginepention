# Pention Engine - module and test declaration helpers
# Requirement: PN-PLT-018 (module boundaries), PN-OPS-007 (test categories)
# Decision:    ADR-0004
#
# pn_add_module() declares one boundary from the ADR-0004 dependency graph.
# Every module links pn::compiler_settings so warning and sanitizer policy is
# uniform and cannot be forgotten per target.

function(pn_add_module NAME)
    cmake_parse_arguments(ARG "INTERFACE" "" "SOURCES;PUBLIC_DEPS;PRIVATE_DEPS" ${ARGN})

    set(target "pn_${NAME}")

    if(ARG_INTERFACE)
        add_library(${target} INTERFACE)
        target_include_directories(${target} INTERFACE
            $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>)
        target_link_libraries(${target} INTERFACE pn::compiler_settings ${ARG_PUBLIC_DEPS})
    else()
        add_library(${target} STATIC ${ARG_SOURCES})
        target_include_directories(${target}
            PUBLIC  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
            PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src)
        target_link_libraries(${target}
            PUBLIC  pn::compiler_settings ${ARG_PUBLIC_DEPS}
            PRIVATE ${ARG_PRIVATE_DEPS})
        set_target_properties(${target} PROPERTIES
            POSITION_INDEPENDENT_CODE ON
            CXX_EXTENSIONS OFF)
    endif()

    add_library(pn::${NAME} ALIAS ${target})
endfunction()

# pn_add_test() declares one test executable and registers it with CTest.
function(pn_add_test NAME)
    cmake_parse_arguments(ARG "" "" "SOURCES;DEPS" ${ARGN})

    if(NOT PN_BUILD_TESTS)
        return()
    endif()

    set(target "pn_test_${NAME}")
    add_executable(${target} ${ARG_SOURCES} ${PN_TESTING_MAIN_SOURCE})
    target_link_libraries(${target} PRIVATE pn::compiler_settings pn::testing ${ARG_DEPS})
    add_test(NAME ${NAME} COMMAND ${target})
    set_tests_properties(${NAME} PROPERTIES TIMEOUT 120)
endfunction()
