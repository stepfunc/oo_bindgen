#include <assert.h>
#include <math.h>
#include <string.h>

#include "foo.h"

#define ENGLISH_STRING_1 "I like to be home with my monkey and my dog"

static foo_structure_t create_struct()
{
    foo_structure_t result =
    {
        .boolean_value = true,
        .uint8_value = 1,
        .int8_value = -1,
        .uint16_value = 2,
        .int16_value = -2,
        .uint32_value = 3,
        .int32_value = -3,
        .uint64_value = 4,
        .int64_value = -4,
        .float_value = 12.34f,
        .double_value = -56.78,
        .string_value = ENGLISH_STRING_1,

        .structure_value =
        {
            .test = 41
        },
        .enum_value = FOO_STRUCTURE_ENUM_VAR2,

        .duration_millis = 4200,
        .duration_seconds = 76,        
    };

    return result;
}

static void check_struct(foo_structure_t* structure)
{
    assert(structure->boolean_value == true);
    assert(structure->uint8_value == 1);
    assert(structure->int8_value == -1);
    assert(structure->uint16_value == 2);
    assert(structure->int16_value == -2);
    assert(structure->uint32_value == 3);
    assert(structure->int32_value == -3);
    assert(structure->uint64_value == 4);
    assert(structure->int64_value == -4);
    assert(strcmp(ENGLISH_STRING_1, structure->string_value) == 0);
    assert(fabs(structure->float_value - 12.34f) < 0.001f);
    assert(fabs(structure->double_value + 56.78) < 0.001);

    assert(structure->structure_value.test == 41);
    assert(structure->enum_value == FOO_STRUCTURE_ENUM_VAR2);

    assert(structure->duration_millis == 4200);
    assert(structure->duration_seconds == 76);    
}

static void test_struct_init()
{
    foo_structure_t test = foo_structure_init(foo_empty_interface_init(NULL, NULL));
    assert(strcmp("Hello", test.string_value) == 0);
    test.string_value = ENGLISH_STRING_1;
    check_struct(&test);
}

void structure_tests()
{    
    test_struct_init();
}
