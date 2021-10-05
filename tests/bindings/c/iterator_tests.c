#include <assert.h>
#include <stdlib.h>

#include "foo.h"


void on_values(foo_string_iterator_t* it, void* ctx)
{
    assert(foo_iterator_next(it)->value == 65);
    assert(foo_iterator_next(it)->value == 66);
    assert(foo_iterator_next(it)->value == 67);
    assert(foo_iterator_next(it)->value == 68);
    assert(foo_iterator_next(it)->value == 69);
    assert(foo_iterator_next(it) == NULL);    

    *((int*)ctx) += 1;
}

static void test_callback_with_iterator()
{
    int invoked_count = 0;

    foo_values_receiver_t receiver = foo_values_receiver_init(
        on_values,
        NULL,
        &invoked_count
    );
   
    foo_invoke_callback("ABCDE", receiver);
   
    assert(invoked_count == 1);       
}

void iterator_tests()
{
    test_callback_with_iterator();
}
