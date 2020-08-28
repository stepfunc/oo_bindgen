#include <assert.h>
#include <stddef.h>

#include "foo.h"

typedef struct data {
    uint32_t last_value;
    uint64_t last_duration;
    bool destroy_called;
} data_t;

static void on_value(uint32_t value, void* context)
{
    data_t* data = (data_t*)context;
    data->last_value = value;
}

static void on_duration(uint64_t value, void* context)
{
    data_t* data = (data_t*)context;
    data->last_duration = value;
}

static void on_destroy(void* context)
{
    data_t* data = (data_t*)context;
    data->destroy_called = true;
}

static void simple_callback_test()
{
    callback_source_t* cb_source = cbsource_new();

    data_t data =
    {
        .last_value = 0,
        .destroy_called = false,
    };

    callback_interface_t interface =
    {
        .on_value = &on_value,
        .on_duration = &on_duration,
        .on_destroy = &on_destroy,
        .ctx = &data,
    };

    cbsource_add(cb_source, interface);

    assert(0 == data.last_value);
    cbsource_set_value(cb_source, 24);
    assert(24 == data.last_value);

    assert(0 == data.last_duration);
    cbsource_set_duration(cb_source, 76);
    assert(76 == data.last_duration);

    assert(!data.destroy_called);
    cbsource_destroy(cb_source);
    assert(data.destroy_called);
}

static void optional_callback_test()
{
    callback_source_t* cb_source = cbsource_new();

    callback_interface_t interface =
    {
        .on_value = NULL,
        .on_duration = NULL,
        .on_destroy = NULL,
        .ctx = NULL,
    };

    cbsource_add(cb_source, interface);

    cbsource_set_value(cb_source, 24);
    cbsource_set_duration(cb_source, 76);

    cbsource_destroy(cb_source);
}

void callback_tests()
{
    simple_callback_test();
    optional_callback_test();
}
