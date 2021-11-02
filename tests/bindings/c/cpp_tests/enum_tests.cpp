#include <cassert>
#include <cstring>

#include "foo.hpp"

static void test_enum_zero_to_five(foo::EnumZeroToFive value, int x)
{
    const auto result = foo::EnumEchoFunctions::enum_zero_to_five_echo(value);
    assert(result == value);
    assert(static_cast<int>(result) == x);
}

static void test_enum_zero_to_five()
{
    test_enum_zero_to_five(foo::EnumZeroToFive::zero, 0);
    test_enum_zero_to_five(foo::EnumZeroToFive::one, 1);
    test_enum_zero_to_five(foo::EnumZeroToFive::two, 2);
    test_enum_zero_to_five(foo::EnumZeroToFive::three, 3);
    test_enum_zero_to_five(foo::EnumZeroToFive::four, 4);
    test_enum_zero_to_five(foo::EnumZeroToFive::five, 5);
}

static void test_enum_one_to_six_value(foo::EnumOneToSix value, int x)
{
    const auto result = foo::EnumEchoFunctions::enum_one_to_six_echo(value);
    assert(result == value);
    assert(static_cast<int>(result) == x);
}

static void test_enum_one_to_six()
{
    test_enum_one_to_six_value(foo::EnumOneToSix::one, 1);
    test_enum_one_to_six_value(foo::EnumOneToSix::two, 2);
    test_enum_one_to_six_value(foo::EnumOneToSix::three, 3);
    test_enum_one_to_six_value(foo::EnumOneToSix::four, 4);
    test_enum_one_to_six_value(foo::EnumOneToSix::five, 5);
    test_enum_one_to_six_value(foo::EnumOneToSix::six, 6);
}


static void test_enum_disjoint_enum_value(foo::EnumDisjoint value, int x)
{    
    const auto result = foo::EnumEchoFunctions::enum_disjoint_echo(value);
    assert(result == value);
    assert(static_cast<int>(result) == x);
}

static void test_enum_disjoint()
{
    test_enum_disjoint_enum_value(foo::EnumDisjoint::five, 5);
    test_enum_disjoint_enum_value(foo::EnumDisjoint::one, 1);
    test_enum_disjoint_enum_value(foo::EnumDisjoint::twenty, 20);
    test_enum_disjoint_enum_value(foo::EnumDisjoint::four, 4);
    test_enum_disjoint_enum_value(foo::EnumDisjoint::seven, 7);
    test_enum_disjoint_enum_value(foo::EnumDisjoint::two, 2);
}

static void test_enum_single()
{
    const auto value = foo::EnumSingle::single;
    const auto result = foo::EnumEchoFunctions::enum_single_echo(value);
    assert(result == value);    
}


static void test_enum_to_string()
{
    assert(strcmp("two", foo::to_string(foo::EnumZeroToFive::two)) == 0);    
    assert(strcmp("five", foo::to_string(foo::EnumDisjoint::five)) == 0);
    assert(strcmp("single", foo::to_string(foo::EnumSingle::single)) == 0);
    try {
        foo::to_string((foo::EnumSingle)foo::EnumZeroToFive::four);
        assert(false);
    }
    catch (const std::invalid_argument& ex)
    {
        assert(strcmp("Undefined value for enum 'EnumSingle'", ex.what()) == 0);
    }
}

void enum_tests()
{    
    test_enum_zero_to_five();
    test_enum_one_to_six();    
    test_enum_disjoint();    
    test_enum_single();    
    test_enum_to_string();
}
