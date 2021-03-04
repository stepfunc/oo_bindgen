#include <assert.h>
#include <math.h>

#include "foo.h"

static void test_password()
{
    // my_error_t get_special_number(const char* password, uint32_t* out);
    uint32_t number = 0;
    assert(get_special_number("badpassword", &number) == MyError_BadPassword);
    assert(number == 0);
    assert(get_special_number("solarwinds123", &number) == MyError_Ok);
    assert(number == 42);
}

void error_tests()
{
    test_password();
}
