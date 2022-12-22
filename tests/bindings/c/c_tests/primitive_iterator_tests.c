#include <assert.h>
#include <stdlib.h>

#include "foo.h"

void receive_range(foo_range_iterator_t* it, void* ctx)
{
    uint32_t* value = NULL;
    for (uint32_t i = 1; i <= 3; ++i)
    {
        value = foo_range_iterator_next(it);
        assert(value);
        assert(*value == i);
    }
    
    assert(!foo_range_iterator_next(it));
}

void primitive_iterator_tests()
{
    foo_range_receiver_t receiver = {
        .on_range = &receive_range
    };

    foo_invoke_range_callback(1, 3, receiver);
}
