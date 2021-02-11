#include <assert.h>
#include <stddef.h>

#include "foo.h"

void constant_tests()
{
    assert(SPECIAL_VALUES_ONE == 1);
    assert(SPECIAL_VALUES_TWO == 2);
}
