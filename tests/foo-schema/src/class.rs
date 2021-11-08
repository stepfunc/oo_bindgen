use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // Declare the class
    let test_class = lib.declare_class("test_class")?;

    // Declare each native function
    let constructor = lib
        .define_constructor(test_class.clone())?
        .param("value", BasicType::U32, "Value")?
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

    let destructor = lib.define_destructor(test_class.clone(), "Destroy a {class:test_class}")?;

    let get_value = lib
        .define_method("get_value", test_class.clone())?
        .returns(BasicType::U32, "Current value")?
        .doc("Get the value")?
        .build()?;

    let increment_value = lib
        .define_method("increment_value", test_class.clone())?
        .returns_nothing()?
        .doc("Increment value")?
        .build()?;

    let get_value_callback = lib
        .define_synchronous_interface("get_value_callback", "GetValue callback handler")?
        .begin_callback("on_value", "On value callback")?
        .param("value", BasicType::U32, "Value")?
        .returns_nothing()?
        .end_callback()?
        .build()?;

    let test_class_get_value_async_func = lib
        .define_function("get_value_async")?
        .param("testclass", test_class.clone(), "TestClass handle")?
        .param("cb", get_value_callback, "Callback to call with the value")?
        .returns_nothing()?
        .doc("Get value through a callback")?
        .build()?;

    let test_class_construction_counter = lib
        .define_function("construction_counter")?
        .returns(BasicType::U32, "Number of calls to the constructor")?
        .doc("Get number of calls to the constructor")?
        .build()?;

    // Define the class
    let _test_class = lib
        .define_class(&test_class)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(get_value)?
        .method(increment_value)?
        .async_method("get_value_async", &test_class_get_value_async_func)?
        .static_method("construction_counter", &test_class_construction_counter)?
        .custom_destroy("delete")?
        .doc("A test class")?
        .build()?;

    Ok(())
}
