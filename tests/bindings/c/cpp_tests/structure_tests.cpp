#include <cassert>
#include <cmath>

#include "foo.hpp"

#define ENGLISH_STRING_1 "I like to be home with my monkey and my dog"

static void check_numbers_struct(const foo::Numbers& x)
{
    assert(x.uint8_value == 1);
    assert(x.int8_value == -1);
    assert(x.uint16_value == 2);
    assert(x.int16_value == -2);
    assert(x.uint32_value == 3);
    assert(x.int32_value == -3);
    assert(x.uint64_value == 4);
    assert(x.int64_value == -4);
    assert(fabs(x.float_value - 12.34f) < 0.001f);
    assert(fabs(x.double_value + 56.78) < 0.001);
}

static void check_inner_struct(const foo::InnerStructure& x)
{    
    check_numbers_struct(x.numbers_field);
}

static void check_struct(const foo::Structure& x)
{    
    assert(x.boolean_true == true);
    assert(x.boolean_false == false);
    assert(x.duration_millis == std::chrono::milliseconds(4200));
    assert(x.duration_seconds == std::chrono::seconds(76));
    assert(x.enum_var1 == foo::StructureEnum::var1);
    assert(x.enum_var2 == foo::StructureEnum::var2);
    assert(x.string_hello == "Hello");
    check_inner_struct(x.inner_structure);           
}

class EmptyInterface : public foo::EmptyInterface {};

static void test_struct_init()
{
    foo::Structure test(foo::InnerStructure(std::move(std::make_unique<EmptyInterface>())));
    check_struct(test);
}


void structure_tests()
{    
    test_struct_init();
}
