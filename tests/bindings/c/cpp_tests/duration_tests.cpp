#include <cassert>

#include "foo.hpp"

static void test_echo_milli_seconds(uint64_t value)
{
    const auto input = std::chrono::milliseconds(value);
    const auto output = foo::DurationEchoFunctions::milliseconds_echo(input);
    assert(input == output);
}

static void test_echo_seconds(uint64_t value)
{
    const auto input = std::chrono::seconds(value);
    const auto output = foo::DurationEchoFunctions::seconds_echo(input);
    assert(input == output);
}

static void test_duration_milli_seconds()
{    
    test_echo_milli_seconds(0);
    test_echo_milli_seconds(2000);
    test_echo_milli_seconds(UINT64_MAX);
}

static void test_duration_seconds()
{
    test_echo_seconds(0);
    test_echo_seconds(2000);
    test_echo_seconds(UINT64_MAX);
}

void duration_tests()
{
    test_duration_milli_seconds();
    test_duration_seconds();
}
