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
    assert(foo_get_special_value(WRONG_PASSWORD, &number) == FOO_MY_ERROR_BAD_PASSWORD);
    assert(number == 0);
    assert(foo_get_special_value(CORRECT_PASSWORD, &number) == FOO_MY_ERROR_OK);
    assert(number == MAGIC_NUMBER);
}

static void test_allocation_via_out_parameter()
{
    foo_class_with_password_t* instance = NULL;
    assert(foo_class_with_password_create(WRONG_PASSWORD, &instance) == FOO_MY_ERROR_BAD_PASSWORD);
    assert(!instance);
    assert(foo_class_with_password_create(CORRECT_PASSWORD, &instance) == FOO_MY_ERROR_OK);
    assert(instance);

    uint32_t number = 0;
    assert(foo_class_with_password_get_special_value(instance, &number) == FOO_MY_ERROR_OK);
    assert(number == MAGIC_NUMBER);

    foo_class_with_password_destroy(instance);
}

void error_tests()
{
    test_integer_out_parameter();
    test_allocation_via_out_parameter();
}
