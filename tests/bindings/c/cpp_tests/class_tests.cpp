#include <cassert>

#include "foo.hpp"

static void async_method_test()
{
    auto shared = std::make_shared<uint32_t>(0);
    foo::TestClass test(41);


    test.add_async(1, [shared](uint32_t sum) { *shared = sum;  });
    assert(*shared == 42);
}

void class_tests()
{
    async_method_test();
}
