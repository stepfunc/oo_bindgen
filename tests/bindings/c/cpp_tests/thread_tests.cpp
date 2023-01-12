#include <cassert>

#include <iostream>

#include "foo.hpp"
#include <future>


struct AddResult {
    bool is_error = false;
    foo::MathIsBroken error = foo::MathIsBroken::ok;
    uint32_t value = 0;
};

class AddHandler : public foo::AddHandler {
    std::shared_ptr<std::promise<AddResult>> result;

public:

    AddHandler(std::shared_ptr<std::promise<AddResult>> result) : result(result) {}

    void on_complete(uint32_t value) override
    {
        AddResult result = {
            false,
            foo::MathIsBroken::ok,
            value
        };

        this->result->set_value(result);
    }
    
    void on_failure(foo::MathIsBroken error) override
    {
        AddResult result = {
            true,
            error,
            0
        };

        this->result->set_value(result);
    }
};

static void test_async_callbacks()
{
    auto changes = std::make_shared <std::vector<uint32_t>>();
    {        
        foo::ThreadClass tc(
            42, 
            foo::functional::value_change_listener(
                [changes](uint32_t value) {
                    changes->push_back(value); 
                }
            )
        );

        {
            tc.update(43);
            auto promise = std::make_shared<std::promise<AddResult>>();
            auto future = promise->get_future();
            tc.add(4, std::make_unique<AddHandler>(promise));
            auto result = future.get();
            assert(!result.is_error);
            assert(result.value == 47);
        }

        {
            tc.queue_error(foo::MathIsBroken::math_is_broke);
            auto promise = std::make_shared<std::promise<AddResult>>();
            auto future = promise->get_future();
            tc.add(3, std::make_unique<AddHandler>(promise));   
            auto result = future.get();
            assert(result.is_error);
            assert(result.error == foo::MathIsBroken::math_is_broke);
        }

        {
            tc.queue_error(foo::MathIsBroken::math_is_broke);
            auto promise = std::make_shared<std::promise<AddResult>>();
            auto future = promise->get_future();
            tc.add(3, std::make_unique<AddHandler>(promise));
            auto result = future.get();
            assert(result.is_error);
            assert(result.error == foo::MathIsBroken::math_is_broke);
        }

        {
            tc.drop_next_add();
            auto promise = std::make_shared<std::promise<AddResult>>();
            auto future = promise->get_future();
            tc.add(3, std::make_unique<AddHandler>(promise));
            auto result = future.get();
            assert(result.is_error);
            assert(result.error == foo::MathIsBroken::dropped);
        }
        
        tc.execute(foo::functional::operation([](uint32_t value) { return 2 * value; }));
    }
    // destructor shuts down the Rust thread which makes it safe to check the changes
    assert(changes->size() == 3);
    assert((*changes)[0] == 43);
    assert((*changes)[1] == 47);
    assert((*changes)[2] == 94);
}

void thread_tests()
{
    test_async_callbacks();
}
