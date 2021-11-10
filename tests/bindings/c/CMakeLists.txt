cmake_minimum_required(VERSION 3.8)

project(foo_c LANGUAGES C CXX)

# Find foo_ffi
if(WIN32)
    set(CMAKE_PREFIX_PATH ${CMAKE_CURRENT_LIST_DIR}/generated/x86_64-pc-windows-msvc/cmake)
endif()
if(UNIX)
    set(CMAKE_PREFIX_PATH ${CMAKE_CURRENT_LIST_DIR}/generated/x86_64-unknown-linux-gnu/cmake)
endif()
find_package(foo REQUIRED)

set(c_test_files
        c_tests/main.c
        c_tests/callback_tests.c
        c_tests/constant_tests.c
        c_tests/enum_tests.c
        c_tests/error_tests.c
        c_tests/duration_tests.c
        c_tests/iterator_tests.c
        c_tests/string_tests.c
        c_tests/structure_tests.c
        c_tests/universal_tests.c
        c_tests/version_tests.c
)

set(cpp_test_files
        ${FOO_CPP_FILE}
        cpp_tests/main.cpp
        cpp_tests/callback_tests.cpp
        cpp_tests/class_tests.cpp
        cpp_tests/collection_tests.cpp
        cpp_tests/constant_tests.cpp
        cpp_tests/enum_tests.cpp
        cpp_tests/error_tests.cpp
        cpp_tests/duration_tests.cpp
        cpp_tests/iterator_tests.cpp
        cpp_tests/string_tests.cpp
        cpp_tests/structure_tests.cpp
        cpp_tests/universal_tests.cpp
        cpp_tests/version_tests.cpp
)

add_executable(foo_c_tests ${c_test_files})
target_link_libraries(foo_c_tests PRIVATE foo)

# Copy the DLL after build
add_custom_command(TARGET foo_c_tests POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different $<TARGET_FILE:foo> $<TARGET_FILE_DIR:foo_c_tests>
)

add_executable(foo_cpp_tests ${cpp_test_files})
target_link_libraries(foo_cpp_tests PRIVATE foo)

enable_testing()
add_test(NAME foo_c_tests COMMAND foo_c_tests)
add_test(NAME foo_cpp_tests COMMAND foo_cpp_tests)
