#include <cassert>

#include "foo.hpp"

struct Data {
    uint32_t value = 0;
    std::chrono::steady_clock::duration duration = std::chrono::steady_clock::duration::zero();
    size_t destructor_count = 0;
    std::string first_name;
    std::string last_name;
};


class CallbackInterface final : public foo::CallbackInterface {
    std::shared_ptr<Data> data;

public:
    CallbackInterface(std::shared_ptr<Data> data) : data(data) {}

    ~CallbackInterface()
    {
        ++(data->destructor_count);
    }

    uint32_t on_value(uint32_t value) override {
        data->value = value;
        return value;
    }
    std::chrono::steady_clock::duration on_duration(std::chrono::steady_clock::duration value) override
    {
        data->duration = value;
        return value;
    }

    void on_names(const foo::Names& names) override {
        data->first_name = names.first_name;
        data->last_name = names.last_name;
    }
};


static void simple_callback_test()
{
    auto data = std::make_shared<Data>();

    {
        foo::CallbackSource cb_source;    
        cb_source.set_interface(std::make_unique<CallbackInterface>(data));

        {
            assert(data->value == 0);
            auto result = cb_source.set_value(24);
            assert(result == 24);
            assert(data->value == 24);
        }
        
        {
            assert(data->duration == std::chrono::steady_clock::duration::zero());
            const auto value = std::chrono::milliseconds(76);
            const auto result = cb_source.set_duration(value);
            assert(result == value);
            assert(data->duration == value);
        }    

        {
            assert(data->first_name == "");
            assert(data->last_name == "");            
            cb_source.invoke_on_names("john", "smith");
            assert(data->first_name == "john");
            assert(data->last_name == "smith");
        }

        assert(data->destructor_count == 0);
    }

    assert(data->destructor_count == 1);
}

void callback_tests()
{
    simple_callback_test();
}
