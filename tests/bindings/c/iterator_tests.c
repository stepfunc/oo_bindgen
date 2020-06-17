#include <assert.h>

#include "foo.h"

static void test_next()
{
    #define SOME_STRING "ABCDE"
    StringIterator* it = iterator_create(SOME_STRING);

    assert(iterator_next(it)->value == 65);
    assert(iterator_next(it)->value == 66);
    assert(iterator_next(it)->value == 67);
    assert(iterator_next(it)->value == 68);
    assert(iterator_next(it)->value == 69);
    assert(iterator_next(it) == NULL);

    iterator_destroy(it);
}

void iterator_tests()
{
    test_next();
}
