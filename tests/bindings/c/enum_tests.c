#include <assert.h>
#include <string.h>

#include "foo.h"

static void test_enum_zero_to_five()
{
    EnumZeroToFive value = EnumZeroToFive_Zero;
    EnumZeroToFive result = enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 0);

    value = EnumZeroToFive_One;
    result = enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 1);

    value = EnumZeroToFive_Two;
    result = enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 2);

    value = EnumZeroToFive_Three;
    result = enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 3);

    value = EnumZeroToFive_Four;
    result = enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 4);

    value = EnumZeroToFive_Five;
    result = enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 5);
}

static void test_enum_one_to_six()
{
    EnumOneToSix value = EnumOneToSix_One;
    EnumOneToSix result = enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 1);

    value = EnumOneToSix_Two;
    result = enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 2);

    value = EnumOneToSix_Three;
    result = enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 3);

    value = EnumOneToSix_Four;
    result = enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 4);

    value = EnumOneToSix_Five;
    result = enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 5);

    value = EnumOneToSix_Six;
    result = enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 6);
}

static void test_enum_disjoint()
{
    EnumDisjoint value = EnumDisjoint_Five;
    EnumDisjoint result = enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 5);

    value = EnumDisjoint_One;
    result = enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 1);

    value = EnumDisjoint_Twenty;
    result = enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 20);

    value = EnumDisjoint_Four;
    result = enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 4);

    value = EnumDisjoint_Seven;
    result = enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 7);

    value = EnumDisjoint_Two;
    result = enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 2);
}

void enum_tests()
{
    test_enum_zero_to_five();
    test_enum_one_to_six();
    test_enum_disjoint();
}
