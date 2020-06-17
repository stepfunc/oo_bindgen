cmake_minimum_required(VERSION 3.8)

project(foo_c LANGUAGES C)

# Find foo_ffi
set(CMAKE_PREFIX_PATH ${CMAKE_CURRENT_LIST_DIR}/generated/cmake)
find_package(foo REQUIRED)

set(tests
    ./version_tests.c
    ./enum_tests.c
    ./duration_tests.c
    ./string_tests.c
    ./structure_tests.c
    ./callback_tests.c
    ./iterator_tests.c
)

add_executable(foo_c main.c ${tests})
target_link_libraries(foo_c PRIVATE foo)

# Copy the DLL after build
add_custom_command(TARGET foo_c POST_BUILD 
    COMMAND ${CMAKE_COMMAND} -E copy_if_different $<TARGET_FILE:foo> $<TARGET_FILE_DIR:foo_c>
)

enable_testing()
add_test(NAME foo_c COMMAND foo_c)
