#include <cassert>
#include <cstring>

#include "foo.hpp"

#define CORRECT_PASSWORD "12345"
#define WRONG_PASSWORD "wrong!"
#define MAGIC_NUMBER 42

static void test_constructor_that_throws()
{
    try {
        foo::ClassWithPassword instance(std::string(WRONG_PASSWORD));
        assert(false);
    }
    catch (const foo::MyException& ex)
    {
        assert(ex.error == foo::MyError::bad_password);
    }

    foo::ClassWithPassword instance(std::string(CORRECT_PASSWORD));

    assert(instance.get_special_value() == MAGIC_NUMBER);
}

static void test_static_method_that_throws()
{
    try {
        foo::ClassWithPassword::get_special_value(std::string(WRONG_PASSWORD));
        assert(false);
    }
    catch (const foo::MyException& ex) {
        assert(ex.error == foo::MyError::bad_password);
    }

    assert(foo::ClassWithPassword::get_special_value(std::string(CORRECT_PASSWORD)) == MAGIC_NUMBER);
}

static void test_defensive_exception_after_move()
{
    foo::ClassWithPassword instance(std::string(CORRECT_PASSWORD));
    foo::ClassWithPassword other(std::move(instance));

    try {
        instance.get_special_value();
    }
    catch (const std::logic_error& ex) {
        assert(strcmp(ex.what(), "class method invoked after move operation") == 0);
    }

    assert(other.get_special_value() == MAGIC_NUMBER);
}

void error_tests()
{
    test_constructor_that_throws();
    test_static_method_that_throws();
    test_defensive_exception_after_move();
}
