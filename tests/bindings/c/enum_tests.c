#include <assert.h>
#include <string.h>

#include "foo.h"

static void test_enum_zero_to_five()
{
    foo_enum_zero_to_five_t value = foo_EnumZeroToFive_Zero;
    foo_enum_zero_to_five_t result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 0);

    value = foo_EnumZeroToFive_One;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 1);

    value = foo_EnumZeroToFive_Two;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 2);

    value = foo_EnumZeroToFive_Three;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 3);

    value = foo_EnumZeroToFive_Four;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 4);

    value = foo_EnumZeroToFive_Five;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 5);
}

static void test_enum_one_to_six()
{
    foo_enum_one_to_six_t value = foo_EnumOneToSix_One;
    foo_enum_one_to_six_t result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 1);

    value = foo_EnumOneToSix_Two;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 2);

    value = foo_EnumOneToSix_Three;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 3);

    value = foo_EnumOneToSix_Four;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 4);

    value = foo_EnumOneToSix_Five;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 5);

    value = foo_EnumOneToSix_Six;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 6);
}

static void test_enum_disjoint()
{
    foo_enum_disjoint_t value = foo_EnumDisjoint_Five;
    foo_enum_disjoint_t result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 5);

    value = foo_EnumDisjoint_One;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 1);

    value = foo_EnumDisjoint_Twenty;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 20);

    value = foo_EnumDisjoint_Four;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 4);

    value = foo_EnumDisjoint_Seven;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 7);

    value = foo_EnumDisjoint_Two;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 2);
}

static void test_enum_single()
{
    foo_enum_single_t value = foo_EnumSingle_Single;
    foo_enum_single_t result = foo_enum_single_echo(value);
    assert(result == value);
    assert(result == 0);
}

static void test_enum_to_string()
{
    assert(strcmp("Two", foo_EnumZeroToFive_to_string(foo_EnumZeroToFive_Two)) == 0);
    assert(strcmp("Five", foo_EnumDisjoint_to_string(foo_EnumDisjoint_Five)) == 0);
    assert(strcmp("Single", foo_EnumSingle_to_string(foo_EnumSingle_Single)) == 0);
    assert(strcmp("", foo_EnumSingle_to_string((foo_enum_single_t)foo_EnumZeroToFive_Four)) == 0);
}

void enum_tests()
{
    test_enum_zero_to_five();
    test_enum_one_to_six();
    test_enum_disjoint();
    test_enum_single();
    test_enum_to_string();
}
