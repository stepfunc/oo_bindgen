#include <assert.h>
#include <stddef.h>
#include <string.h>

#include "foo.h"

typedef struct data {
    uint32_t last_value;
    uint64_t last_duration;
    bool on_names_called;
    bool destroy_called;
} data_t;

static uint32_t on_value(uint32_t value, void* context)
{
    data_t* data = (data_t*)context;
    data->last_value = value;
    return value;
}

static uint64_t on_duration(uint64_t value, void* context)
{
    data_t* data = (data_t*)context;
    data->last_duration = value;
    return value;
}

static void on_names(foo_names_t names, void* context)
{
    data_t* data = (data_t*)context;
    assert(strcmp(names.first_name, "john") == 0);
    assert(strcmp(names.last_name, "smith") == 0);
    data->on_names_called = true;
}

static void on_destroy(void* context)
{
    data_t* data = (data_t*)context;
    data->destroy_called = true;
}

static void simple_callback_test()
{
    foo_callback_source_t* cb_source = foo_callback_source_create();

    data_t data =
    {
        .last_value = 0,
        .destroy_called = false,
        .on_names_called = false,
    };

    foo_callback_interface_t interface = {
        .on_value = &on_value,
        .on_destroy = &on_destroy,
        .on_duration = &on_duration,
        .on_names = &on_names,
        .ctx = &data,
    };

    foo_callback_source_set_interface(cb_source, interface);

    assert(0 == data.last_value);
    uint32_t result = foo_callback_source_set_value(cb_source, 24);
    assert(24 == result);
    assert(24 == data.last_value);

    assert(0 == data.last_duration);
    uint64_t duration_result = foo_callback_source_set_duration(cb_source, 76);
    assert(76 == duration_result);
    assert(76 == data.last_duration);

    assert(!data.on_names_called);
    foo_names_t names = {
        .first_name = "john",
        .last_name = "smith",
    };
    foo_callback_source_invoke_on_names(cb_source, names);
    assert(data.on_names_called);

    assert(!data.destroy_called);
    foo_callback_source_destroy(cb_source);
    assert(data.destroy_called);
}

static void optional_callback_test()
{
    foo_callback_source_t* cb_source = foo_callback_source_create();

    foo_callback_interface_t interface = { NULL };

    foo_callback_source_set_interface(cb_source, interface);

    foo_callback_source_set_value(cb_source, 24);
    foo_callback_source_set_duration(cb_source, 76);

    foo_callback_source_destroy(cb_source);
}

void callback_tests()
{
    simple_callback_test();
    optional_callback_test();
}
