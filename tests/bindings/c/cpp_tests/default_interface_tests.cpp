#include <cassert>

#include "foo.hpp"

class DefaultedInterface final : public foo::DefaultedInterface {};

void defaulted_interface_tests()
{
    auto instance = DefaultedInterface();

    assert(foo::DefaultInterfaceTest::get_duration_value(instance) == std::chrono::milliseconds(42));
    assert(foo::DefaultInterfaceTest::get_u32_value(instance) == 42);
    assert(foo::DefaultInterfaceTest::get_switch_pos(instance) == foo::SwitchPosition::on);
    assert(foo::DefaultInterfaceTest::get_bool_value(instance));
    assert(foo::DefaultInterfaceTest::get_wrapped_number(instance).num == 42);
}
