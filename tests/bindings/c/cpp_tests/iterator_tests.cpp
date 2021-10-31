#include <cassert>
#include <sstream>

#include "foo.hpp"


class ValuesReceiver : public foo::ValuesReceiver {
public:
    std::stringstream ss;    

    void on_characters(foo::StringIterator& values) override {        
        while (values.next())
        {
            const auto item = values.get();
            this->ss.put(static_cast<char>(item.value));
        }
    }
};

class ChunkReceiver : public foo::ChunkReceiver {
public:
    std::vector<std::string> items;    

    void on_chunk(foo::ChunkIterator& chunks) override {        
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
    }
};

void test_callback_with_iterator()
{    
    ValuesReceiver receiver;
    foo::IteratorTestHelper::invoke_callback("ABCDE", receiver);

    assert(receiver.ss.str() == "ABCDE");
}

void test_double_iterator_with_lifetime()
{    
    ChunkReceiver receiver;
    foo::DoubleIteratorTestHelper::iterate_string_by_chunks("hello world!", 3, receiver);

    assert(receiver.items.size() == 4);
    assert(receiver.items[0] == "hel");
    assert(receiver.items[1] == "lo ");
    assert(receiver.items[2] == "wor");
    assert(receiver.items[3] == "ld!");
}

void iterator_tests()
{
    test_callback_with_iterator();
    test_double_iterator_with_lifetime();
}
