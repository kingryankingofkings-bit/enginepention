# Pention Engine - shared compiler configuration
# Requirement: PN-OPS-003 (warning discipline, static analysis, sanitizers)
# Decision:    ADR-0001

add_library(pn_compiler_settings INTERFACE)
add_library(pn::compiler_settings ALIAS pn_compiler_settings)

set(_pn_gnu_like_warnings
    -Wall
    -Wextra
    -Wpedantic
    -Wshadow
    -Wnon-virtual-dtor
    -Wold-style-cast
    -Wcast-align
    -Wunused
    -Woverloaded-virtual
    -Wconversion
    -Wsign-conversion
    -Wdouble-promotion
    -Wformat=2
    -Wimplicit-fallthrough
    -Wnull-dereference)

if(CMAKE_CXX_COMPILER_ID MATCHES "GNU")
    list(APPEND _pn_gnu_like_warnings
        -Wduplicated-cond
        -Wduplicated-branches
        -Wlogical-op
        -Wuseless-cast)
endif()

if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    target_compile_options(pn_compiler_settings INTERFACE ${_pn_gnu_like_warnings})
    if(PN_WARNINGS_AS_ERRORS)
        target_compile_options(pn_compiler_settings INTERFACE -Werror)
    endif()
elseif(MSVC)
    target_compile_options(pn_compiler_settings INTERFACE /W4 /permissive-)
    if(PN_WARNINGS_AS_ERRORS)
        target_compile_options(pn_compiler_settings INTERFACE /WX)
    endif()
endif()

# Exception independence. ADR-0002 requires the tree to compile cleanly with
# exceptions disabled; a dedicated CI configuration proves the claim rather
# than asserting it.
if(PN_NO_EXCEPTIONS)
    if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
        target_compile_options(pn_compiler_settings INTERFACE -fno-exceptions)
    elseif(MSVC)
        target_compile_definitions(pn_compiler_settings INTERFACE _HAS_EXCEPTIONS=0)
    endif()
    target_compile_definitions(pn_compiler_settings INTERFACE PN_NO_EXCEPTIONS=1)
endif()

if(PN_SANITIZE_ADDRESS AND CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    target_compile_options(pn_compiler_settings INTERFACE
        -fsanitize=address,undefined -fno-omit-frame-pointer -fno-sanitize-recover=all)
    target_link_options(pn_compiler_settings INTERFACE -fsanitize=address,undefined)
endif()

if(PN_SANITIZE_THREAD AND CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    target_compile_options(pn_compiler_settings INTERFACE
        -fsanitize=thread -fno-omit-frame-pointer)
    target_link_options(pn_compiler_settings INTERFACE -fsanitize=thread)
endif()
