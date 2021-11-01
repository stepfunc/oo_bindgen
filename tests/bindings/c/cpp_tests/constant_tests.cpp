#include <cassert>

#include "foo.hpp"

void constant_tests()
{    
    assert(foo::special_values::one == 1);
    assert(foo::special_values::two == 2);
}
