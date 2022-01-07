#include <assert.h>
#include <stdlib.h>

#include "foo.hpp"

class RangeReceiver : public foo::RangeReceiver {
public:
    size_t count = 0;

    void on_range(foo::RangeIterator& values) override {
        auto value = 1;
        while (values.next()) {
            assert(values.get() == value);
            ++value;
            ++count;
        }        
    }
};

void primitive_iterator_tests()
{
    RangeReceiver receiver;
    foo::RangeIteratorTestHelper::invoke_range_callback(1, 3, receiver);
    assert(receiver.count == 3);
}
