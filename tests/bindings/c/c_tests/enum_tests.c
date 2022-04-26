#include <assert.h>
#include <string.h>

#include "foo.h"

static void test_enum_zero_to_five()
{
    foo_enum_zero_to_five_t value = FOO_ENUM_ZERO_TO_FIVE_ZERO;
    foo_enum_zero_to_five_t result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 0);

    value = FOO_ENUM_ZERO_TO_FIVE_ONE;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 1);

    value = FOO_ENUM_ZERO_TO_FIVE_TWO;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 2);

    value = FOO_ENUM_ZERO_TO_FIVE_THREE;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 3);

    value = FOO_ENUM_ZERO_TO_FIVE_FOUR;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 4);

    value = FOO_ENUM_ZERO_TO_FIVE_FIVE;
    result = foo_enum_zero_to_five_echo(value);
    assert(result == value);
    assert(result == 5);
}

static void test_enum_one_to_six()
{
    foo_enum_one_to_six_t value = FOO_ENUM_ONE_TO_SIX_ONE;
    foo_enum_one_to_six_t result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 1);

    value = FOO_ENUM_ONE_TO_SIX_TWO;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 2);

    value = FOO_ENUM_ONE_TO_SIX_THREE;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 3);

    value = FOO_ENUM_ONE_TO_SIX_FOUR;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 4);

    value = FOO_ENUM_ONE_TO_SIX_FIVE;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 5);

    value = FOO_ENUM_ONE_TO_SIX_SIX;
    result = foo_enum_one_to_six_echo(value);
    assert(result == value);
    assert(result == 6);
}

static void test_enum_disjoint()
{
    foo_enum_disjoint_t value = FOO_ENUM_DISJOINT_FIVE;
    foo_enum_disjoint_t result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 5);

    value = FOO_ENUM_DISJOINT_ONE;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 1);

    value = FOO_ENUM_DISJOINT_TWENTY;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 20);

    value = FOO_ENUM_DISJOINT_FOUR;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 4);

    value = FOO_ENUM_DISJOINT_SEVEN;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 7);

    value = FOO_ENUM_DISJOINT_TWO;
    result = foo_enum_disjoint_echo(value);
    assert(result == value);
    assert(result == 2);
}

static void test_enum_single()
{
    foo_enum_single_t value = FOO_ENUM_SINGLE_SINGLE;
    foo_enum_single_t result = foo_enum_single_echo(value);
    assert(result == value);
    assert(result == 0);
}

static void test_enum_to_string()
{
    assert(strcmp("two", foo_enum_zero_to_five_to_string(FOO_ENUM_ZERO_TO_FIVE_TWO)) == 0);
    assert(strcmp("five", foo_enum_disjoint_to_string(FOO_ENUM_DISJOINT_FIVE)) == 0);
    assert(strcmp("single", foo_enum_single_to_string(FOO_ENUM_SINGLE_SINGLE)) == 0);
    assert(strcmp("unknown enum_single value", foo_enum_single_to_string((foo_enum_single_t)FOO_ENUM_ZERO_TO_FIVE_FOUR)) == 0);
}

void enum_tests()
{
    test_enum_zero_to_five();
    test_enum_one_to_six();
    test_enum_disjoint();
    test_enum_single();
    test_enum_to_string();
}
