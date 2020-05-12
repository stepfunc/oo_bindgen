#include <assert.h>
#include <math.h>

#include "foo.h"

static Structure create_struct()
{
    Structure result =
    {
        .booleanValue = true,
        .uint8Value = 1,
        .int8Value = -1,
        .uint16Value = 2,
        .int16Value = -2,
        .uint32Value = 3,
        .int32Value = -3,
        .uint64Value = 4,
        .int64Value = -4,

        .structureValue =
        {
            .test = 41
        },
        .enumValue = StructureEnum_Var2,

        .durationMillis = 4200,
        .durationSeconds = 76,
        .durationSecondsFloat = 15.25f,
    };

    return result;
}

static void check_struct(Structure* structure)
{
    assert(structure->booleanValue == true);
    assert(structure->uint8Value == 1);
    assert(structure->int8Value == -1);
    assert(structure->uint16Value == 2);
    assert(structure->int16Value == -2);
    assert(structure->uint32Value == 3);
    assert(structure->int32Value == -3);
    assert(structure->uint64Value == 4);
    assert(structure->int64Value == -4);

    assert(structure->structureValue.test == 41);
    assert(structure->enumValue == StructureEnum_Var2);

    assert(structure->durationMillis == 4200);
    assert(structure->durationSeconds == 76);
    assert(fabs(structure->durationSecondsFloat - 15.25f) < 0.001f);
}

static void test_struct_by_value()
{
    Structure test = create_struct();
    Structure result = struct_by_value_echo(test);
    check_struct(&result);
}

static void test_struct_by_reference()
{
    Structure test = create_struct();
    Structure result = struct_by_reference_echo(&test);
    check_struct(&result);
}

void structure_tests()
{
    test_struct_by_value();
    test_struct_by_reference();
}
