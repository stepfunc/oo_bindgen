use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare the class
    let testclass = lib.declare_class("test_class")?;

    // Declare each native function
    let testclass_new_func = lib
        .define_function("testclass_new")?
        .param("value", BasicType::U32, "Value")?
        .returns(testclass.clone(), "New TestClass")?
        .doc(doc("Create a new {class:test_class}")
            .details("Here are some details about {class:test_class}. You can call {class:test_class.get_value()} method.")
            .details("Here is a reference to a constructor {class:test_class.[constructor]} and to a destructor {class:test_class.[destructor]}.")
            .details("Here are some details about the struct {struct:structure}. It has the {struct:structure.boolean_true} element" )
            .details("Here are some details about {enum:enum_zero_to_five}. It has the {enum:enum_zero_to_five.two} variant.")
            .details("Here are some details about {interface:callback_interface}. It has the {interface:callback_interface.on_value()} callback.")
            .details("Here's a {null}. Here's the {iterator}.")
            .warning("And here's a dangerous warning! Do not use {class:test_class.get_value()}"),
        )?
        .build()?;

    let testclass_destroy_func = lib
        .define_function("testclass_destroy")?
        .param("testclass", testclass.clone(), "Class handle")?
        .returns_nothing()?
        .doc("Destroy a test class")?
        .build()?;

    let testclass_get_value_func = lib
        .define_function("testclass_get_value")?
        .param("testclass", testclass.clone(), "TestClass handle")?
        .returns(BasicType::U32, "Current value")?
        .doc("Get value (don't forget the {param:testclass}!)")?
        .build()?;

    let testclass_increment_value_func = lib
        .define_function("testclass_increment_value")?
        .param("testclass", testclass.clone(), "TestClass handle")?
        .returns_nothing()?
        .doc("Increment value")?
        .build()?;

    let get_value_cb = lib
        .define_synchronous_interface("get_value_callback", "GetValue callback handler")?
        .begin_callback("on_value", "On value callback")?
        .param("value", BasicType::U32, "Value")?
        .returns_nothing()?
        .end_callback()?
        .build()?;

    let testclass_get_value_async_func = lib
        .define_function("testclass_get_value_async")?
        .param("testclass", testclass.clone(), "TestClass handle")?
        .param("cb", get_value_cb, "Callback to call with the value")?
        .returns_nothing()?
        .doc("Get value through a callback")?
        .build()?;

    let testclass_construction_counter = lib
        .define_function("testclass_construction_counter")?
        .returns(BasicType::U32, "Number of calls to the constructor")?
        .doc("Get number of calls to the constructor")?
        .build()?;

    // Define the class
    let _testclass = lib
        .define_class(&testclass)?
        .constructor(&testclass_new_func)?
        .destructor(&testclass_destroy_func)?
        .method("get_value", &testclass_get_value_func)?
        .method("increment_value", &testclass_increment_value_func)?
        .async_method("get_value_async", &testclass_get_value_async_func)?
        .static_method("construction_counter", &testclass_construction_counter)?
        .custom_destroy("delete")?
        .doc("A test class")?
        .build()?;

    Ok(())
}
