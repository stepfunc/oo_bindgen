#include <cassert>

#include "foo.hpp"

#include <iostream>

void collection_tests()
{
    std::vector<std::string> values = { "hello", "big", "world" };
    assert(foo::StringCollectionTestMethods::get_size(values) == 3);
    assert(foo::StringCollectionTestMethods::get_size_with_reserve(values) == 3);
    assert(foo::StringCollectionTestMethods::get_value(values, 1) == "big");
    assert(foo::StringCollectionTestMethods::get_value_with_reserve(values, 1) == "big");
}
