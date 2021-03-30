#include <assert.h>
#include <stdlib.h>

#include "foo.h"

static void test_next()
{
    #define SOME_STRING "ABCDE"
    foo_string_iterator_t* it = foo_iterator_create(SOME_STRING);

    assert(foo_iterator_next(it)->value == 65);
    assert(foo_iterator_next(it)->value == 66);
    assert(foo_iterator_next(it)->value == 67);
    assert(foo_iterator_next(it)->value == 68);
    assert(foo_iterator_next(it)->value == 69);
    assert(foo_iterator_next(it) == NULL);

    foo_iterator_destroy(it);
}

void iterator_tests()
{
    test_next();
}
