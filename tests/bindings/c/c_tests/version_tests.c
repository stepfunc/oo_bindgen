#include <assert.h>
#include <string.h>

#include "foo.h"

static void test_version_individual_numbers()
{
    assert(FOO_VERSION_MAJOR == 1);
    assert(FOO_VERSION_MINOR == 2);
    assert(FOO_VERSION_PATCH == 3);
}

static void test_version_string()
{
    assert(strcmp(FOO_VERSION_STRING, "1.2.3") == 0);
}

void version_tests()
{
    test_version_individual_numbers();
    test_version_string();
}
