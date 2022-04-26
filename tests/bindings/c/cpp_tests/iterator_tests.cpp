#include <cassert>
#include <sstream>

#include "foo.hpp"

void test_callback_with_iterator()
{
    std::stringstream ss;
    auto receiver = foo::functional::values_receiver(
        [&](foo::StringIterator& values) {
            while (values.next()) {
                ss.put(static_cast<char>(values.get().value));
            }
        }
    );
    foo::IteratorTestHelper::invoke_callback("ABCDE", receiver);

    assert(ss.str() == "ABCDE");
}

void test_double_iterator_with_lifetime()
{    
    std::vector<std::string> items;
    auto receiver = foo::functional::chunk_receiver([&](foo::ChunkIterator& chunks) {
        while (chunks.next())
        {
            std::stringstream ss;
            auto chunk = chunks.get();
            while (chunk.iter.next())
            {
                const auto value = chunk.iter.get();
                ss.put(static_cast<char>(value.value));
            }
            items.push_back(ss.str());
        }
    });

    foo::DoubleIteratorTestHelper::iterate_string_by_chunks("hello world!", 3, receiver);

    assert(items.size() == 4);
    assert(items[0] == "hel");
    assert(items[1] == "lo ");
    assert(items[2] == "wor");
    assert(items[3] == "ld!");
}

void iterator_tests()
{
    test_callback_with_iterator();
    test_double_iterator_with_lifetime();
}
