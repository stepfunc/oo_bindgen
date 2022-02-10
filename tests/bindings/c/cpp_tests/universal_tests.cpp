#include <cassert>

#include "foo.hpp"

using namespace foo;

class Incrementer : public UniversalInterface {
    UniversalOuterStruct on_value(const UniversalOuterStruct& value) override {
        UniversalOuterStruct ret(value);
        ret.inner.value = value.inner.value + 1;
        ret.delay = value.delay + std::chrono::milliseconds(1);
        return ret;
    }
};

// demonstrates that a universal struct can be used in all 4 schema positions
static void test_universal_interface()
{
    foo::UniversalOuterStruct input;
    input.inner.value = 42;
    input.delay = std::chrono::milliseconds(77);
   
    Incrementer incrementer;
    const auto output = UniversalInterfaceTests::invoke(input, incrementer);

    assert(output.inner.value == 43);
    assert(output.delay == std::chrono::milliseconds(78));
}

void universal_tests()
{
    test_universal_interface();
}
