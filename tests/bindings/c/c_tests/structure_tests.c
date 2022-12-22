#include <assert.h>
#include <math.h>
#include <string.h>

#include "foo.h"

#define ENGLISH_STRING_1 "I like to be home with my monkey and my dog"

static void check_numbers_struct(foo_numbers_t* x)
{
    assert(x->uint8_value == 1);
    assert(x->int8_value == -1);
    assert(x->uint16_value == 2);
    assert(x->int16_value == -2);
    assert(x->uint32_value == 3);
    assert(x->int32_value == -3);
    assert(x->uint64_value == 4);
    assert(x->int64_value == -4);
    assert(fabs(x->float_value - 12.34f) < 0.001f);
    assert(fabs(x->double_value + 56.78) < 0.001);
}

static void check_inner_struct(foo_inner_structure_t* x)
{
    assert(x->interface_field.ctx == NULL);
    assert(x->interface_field.on_destroy == NULL);
    check_numbers_struct(&x->numbers_field);
}

static void check_struct(foo_structure_t* x)
{
    assert(x->boolean_true == true);
    assert(x->boolean_false == false);
    assert(x->duration_millis == 4200);
    assert(x->duration_seconds == 76);
    assert(x->enum_var1 == FOO_STRUCTURE_ENUM_VAR1);
    assert(x->enum_var2 == FOO_STRUCTURE_ENUM_VAR2);
    assert(strcmp("Hello", x->string_hello) == 0);
    check_inner_struct(&x->inner_structure);
}

static void test_struct_init()
{
    foo_empty_interface_t empty = { NULL };
    foo_structure_t test = foo_structure_init(foo_inner_structure_init(empty));
    check_struct(&test);
}

void structure_tests()
{
    test_struct_init();
}
