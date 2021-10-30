#include <cassert>

#include "foo.hpp"

using namespace foo;

void version_tests()
{
    assert(foo_version_major == 1);
    assert(foo_version_minor == 2);
    assert(foo_version_patch == 3);
    assert(strcmp(foo_version_string, "1.2.3") == 0);
}
