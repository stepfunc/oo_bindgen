#include <assert.h>

#include "foo.h"

typedef struct Data {
    uint32_t last_value;
    bool destroy_called;
} Data;

static void on_value(uint32_t value, Data* data)
{
    data->last_value = value;
}

static void on_destroy(Data* data)
{
    data->destroy_called = true;
}

static void simple_callback_test()
{
    CallbackSource* cb_source = cbsource_new();

    Data data =
    {
        .last_value = 0,
        .destroy_called = false,
    };

    CallbackInterface interface =
    {
        .on_value = &on_value,
        .on_destroy = &on_destroy,
        .data = &data,
    };

    cbsource_add(cb_source, interface);

    assert(0 == data.last_value);
    cbsource_set_value(cb_source, 24);
    assert(24 == data.last_value);

    assert(!data.destroy_called);
    cbsource_destroy(cb_source);
    assert(data.destroy_called);
}

void callback_tests()
{
    simple_callback_test();
}
