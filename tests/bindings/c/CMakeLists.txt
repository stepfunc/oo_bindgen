cmake_minimum_required(VERSION 3.8)

project(foo_c LANGUAGES C)

# Find foo_ffi
set(CMAKE_PREFIX_PATH ${CMAKE_CURRENT_LIST_DIR}/generated/cmake)
find_package(foo REQUIRED)

# You probably don't need do this, just pick either the dynamic or the static
# one if you are on Linux. Here we do CMake trickery to make it work on the CI.
if(TARGET foo)
    set(foo_target foo)
else()
    set(foo_target foo_static)
endif()

set(tests
    ./version_tests.c
    ./enum_tests.c
    ./duration_tests.c
    ./error_tests.c
    ./string_tests.c
    ./structure_tests.c
    ./callback_tests.c
    ./iterator_tests.c
    ./constant_tests.c
)

add_executable(foo_c main.c ${tests})
target_link_libraries(foo_c PRIVATE ${foo_target})

# Copy the DLL after build (if using dynamic)
if(${foo_target} STREQUAL "foo")
    add_custom_command(TARGET foo_c POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different $<TARGET_FILE:foo> $<TARGET_FILE_DIR:foo_c>
    )
endif()

enable_testing()
add_test(NAME foo_c COMMAND foo_c)
