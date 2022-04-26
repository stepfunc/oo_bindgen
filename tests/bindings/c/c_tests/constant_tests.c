#include <assert.h>
#include <stddef.h>

#include "foo.h"

void constant_tests()
{
    assert(FOO_SPECIAL_VALUES_ONE == 1);
    assert(FOO_SPECIAL_VALUES_TWO == 2);
}
