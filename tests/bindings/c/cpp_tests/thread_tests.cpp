#include <cassert>

#include <iostream>

#include "foo.hpp"
#include <future>


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
                
        auto promise = std::make_shared<std::promise<uint32_t>>();        
        auto future = promise->get_future();        
        tc.add(4, [promise](uint32_t result) {
            promise->set_value(result);
        });        
        assert(future.get() == 46);

        tc.update(43);
    }
    // destructor shuts down the Rust thread which makes it safe to check the changes
    assert(changes->size() == 2);
    assert((*changes)[0] == 46);
    assert((*changes)[1] == 43);    
}

void thread_tests()
{    
    test_async_callbacks();
}
