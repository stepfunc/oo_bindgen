#include <cassert>
#include <sstream>

#include "foo.hpp"


/*
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

void check_chunk(foo_chunk_iterator_t* it, char char1, char char2, char char3)
{
    foo_chunk_t* chunk = foo_next_chunk(it);
    foo_byte_value_t* byte = foo_next_byte_value(chunk->iter);
    assert((char)byte->value == char1);
    byte = foo_next_byte_value(chunk->iter);
    assert((char)byte->value == char2);
    byte = foo_next_byte_value(chunk->iter);
    assert((char)byte->value == char3);
    assert(foo_next_byte_value(chunk->iter) == NULL);
}

void on_chunks(foo_chunk_iterator_t* it, void* ctx)
{
    *((int*)ctx) += 1;

    // first chunk
    check_chunk(it, 'h', 'e', 'l');
    check_chunk(it, 'l', 'o', ' ');
    check_chunk(it, 'w', 'o', 'r');
    check_chunk(it, 'l', 'd', '!');

    // end iteration
    assert(foo_next_chunk(it) == NULL);
}
*/

class ValuesReceiver : public foo::ValuesReceiver {
public:
    std::stringstream ss;
    size_t invoked_count = 0;

    void on_characters(foo::StringIterator& values) override {
        ++invoked_count;
        while (values.next())
        {
            const auto item = values.get();
            this->ss.put(static_cast<char>(item.value));
        }
    }
};

void test_callback_with_iterator()
{
    int invoked_count = 0;

    ValuesReceiver receiver;

    foo::IteratorTestHelper::invoke_callback("ABCDE", receiver);

    assert(receiver.invoked_count == 1);
    assert(receiver.ss.str() == "ABCDE");
}

/*
void test_double_iterator_with_lifetime()
{
    int invoked_count = 0;
    foo_chunk_receiver_t receiver = foo_chunk_receiver_init(
        on_chunks,
        NULL,
        &invoked_count
    );

    foo_iterate_string_by_chunks("hello world!", 3, receiver);

    assert(invoked_count == 1);
}
*/

void iterator_tests()
{
    test_callback_with_iterator();
    //test_double_iterator_with_lifetime();
}
