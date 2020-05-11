#include <assert.h>
#include <string.h>

#include "foo.h"

static void test_enum_zero_to_five()
{
    EnumZeroToFive value = Two;
    EnumZeroToFive result = enum_zero_to_five_echo(value);
    assert(result == value);
}

void enum_tests()
{
    test_enum_zero_to_five();
}
