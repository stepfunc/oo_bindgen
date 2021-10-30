#include <assert.h>
#include <math.h>
#include <string.h>

#include "foo.h"


static foo_universal_outer_struct_t increment_fields(foo_universal_outer_struct_t value, void* ctx)
{
    value.inner.value += 1;
    value.delay += 1;
    return value;
}

// demonstrates that a universal struct can be used in all 4 schema positions
static void test_universal_interface()
{
    foo_universal_outer_struct_t input = foo_universal_outer_struct_init();
    input.inner.value = 42;
    input.delay = 77;

    foo_universal_interface_t increment = foo_universal_interface_init(increment_fields, NULL, NULL);
    foo_universal_outer_struct_t output = foo_invoke_universal_interface(input, increment);
    
    assert(output.inner.value == 43);
    assert(output.delay == 78);
}

void universal_tests()
{        
    test_universal_interface();
}
