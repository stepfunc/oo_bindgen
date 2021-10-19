#include <assert.h>
#include <math.h>
#include <string.h>

#include "foo.h"

// demonstrate that the structure can be used in function argument and return contexts
static void test_universal_struct_conversions()
{    
    foo_universal_outer_struct_t input = foo_universal_outer_struct_init();
    input.inner.value = 42;
    input.delay = 77;
    foo_universal_outer_struct_t output = foo_increment_universal_struct(input);
    assert(output.inner.value == 43);
    assert(output.delay == 78);
}

void universal_tests()
{    
    test_universal_struct_conversions();
}
