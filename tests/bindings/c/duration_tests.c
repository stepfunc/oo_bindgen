#include <assert.h>
#include <math.h>

#include "foo.h"

static void test_duration_ms()
{
    uint64_t result = foo_duration_ms_echo(0);
    assert(result == 0);

    result = foo_duration_ms_echo(2000);
    assert(result == 2000);

    result = foo_duration_ms_echo(UINT64_MAX);
    assert(result == UINT64_MAX);
}

static void test_duration_s()
{
    uint64_t result = foo_duration_s_echo(0);
    assert(result == 0);

    result = foo_duration_s_echo(2000);
    assert(result == 2000);

    result = foo_duration_s_echo(UINT64_MAX);
    assert(result == UINT64_MAX);
}

static void test_duration_s_float()
{
    float result = foo_duration_s_float_echo(0.0f);
    assert(fabs(result - 0.0f) < 0.001f);

    result = foo_duration_s_float_echo(2000.5f);
    assert(fabs(result - 2000.5f) < 0.001f);
}

void duration_tests()
{
    test_duration_ms();
    test_duration_s();
    test_duration_s_float();
}
