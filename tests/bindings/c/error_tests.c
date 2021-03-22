#include <assert.h>
#include <math.h>
#include <stddef.h>

#include "foo.h"

#define CORRECT_PASSWORD "12345"
#define WRONG_PASSWORD "wrong!"
#define MAGIC_NUMBER 42

static void test_integer_out_parameter()
{
    uint32_t number = 0;
    assert(get_special_number(WRONG_PASSWORD, &number) == MyError_BadPassword);
    assert(number == 0);
    assert(get_special_number(CORRECT_PASSWORD, &number) == MyError_Ok);
    assert(number == MAGIC_NUMBER);
}

static void test_allocation_via_out_parameter()
{
    class_with_password_t* instance = NULL;
    assert(create_class_with_password(WRONG_PASSWORD, &instance) == MyError_BadPassword);
    assert(!instance);
    assert(create_class_with_password(CORRECT_PASSWORD, &instance) == MyError_Ok);
    assert(instance);

    uint32_t number = 0;
    assert(get_special_value_from_class(instance, &number) == MyError_Ok);
    assert(number == MAGIC_NUMBER);

    destroy_class_with_password(instance);
}

void error_tests()
{
    test_integer_out_parameter();
    test_allocation_via_out_parameter();
}
