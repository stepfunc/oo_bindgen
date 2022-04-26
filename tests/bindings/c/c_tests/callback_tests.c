#include <assert.h>
#include <stddef.h>

#include "foo.h"

typedef struct data {
    uint32_t last_value;
    uint64_t last_duration;
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
    };

    foo_callback_interface_t interface = foo_callback_interface_init(
        &on_value,
        &on_duration,
        &on_destroy,
        &data
    );

    foo_callback_source_set_interface(cb_source, interface);

    assert(0 == data.last_value);
    uint32_t result = foo_callback_source_set_value(cb_source, 24);
    assert(24 == result);
    assert(24 == data.last_value);

    assert(0 == data.last_duration);
    uint64_t duration_result = foo_callback_source_set_duration(cb_source, 76);
    assert(76 == duration_result);
    assert(76 == data.last_duration);

    assert(!data.destroy_called);
    foo_callback_source_destroy(cb_source);
    assert(data.destroy_called);
}

static void optional_callback_test()
{
    foo_callback_source_t* cb_source = foo_callback_source_create();

    foo_callback_interface_t interface = foo_callback_interface_init(NULL, NULL, NULL, NULL);

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
