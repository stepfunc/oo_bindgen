use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare the class
    let testclass = lib.declare_class("TestClass")?;

    // Declare each native function
    let testclass_new_func = lib
        .define_function("testclass_new")
        .param("value", BasicType::Uint32, "Value")?
        .returns(testclass.clone(), "New TestClass")?
        .doc(doc("Create a new {class:TestClass}")
            .details("Here are some details about {class:TestClass}. You can call {class:TestClass.GetValue()} method.")
            .details("Here is a reference to a constructor {class:TestClass.[constructor]} and to a destructor {class:TestClass.[destructor]}.")
            // TODO - restore this doc ref
            // .details("Here are some details about the struct {struct:Structure}. It has the {struct:Structure.boolean_value} element and the {struct:Structure.StructByValueEcho()} method." )
            .details("Here are some details about the struct {struct:Structure}. It has the {struct:Structure.boolean_value} element and the struct:Structure.StructByValueEcho() method." )
            .details("Here are some details about {enum:EnumZeroToFive}. It has the {enum:EnumZeroToFive.Two} variant.")
            .details("Here are some details about {interface:CallbackInterface}. It has the {interface:CallbackInterface.on_value()} callback.")
            .details("Here's a {null}. Here's the {iterator}.")
            .warning("And here's a dangerous warning! Do not use {class:TestClass.GetValue()}"),
        )?
        .build()?;

    let testclass_destroy_func = lib
        .define_function("testclass_destroy")
        .param("testclass", testclass.clone(), "Class handle")?
        .returns_nothing()?
        .doc("Destroy a test class")?
        .build()?;

    let testclass_get_value_func = lib
        .define_function("testclass_get_value")
        .param("testclass", testclass.clone(), "TestClass handle")?
        .returns(BasicType::Uint32, "Current value")?
        .doc("Get value (don't forget the {param:testclass}!)")?
        .build()?;

    let testclass_increment_value_func = lib
        .define_function("testclass_increment_value")
        .param("testclass", testclass.clone(), "TestClass handle")?
        .returns_nothing()?
        .doc("Increment value")?
        .build()?;

    let get_value_cb = lib
        .define_interface("GetValueCallback", "GetValue callback handler")
        .callback("on_value", "On value callback")?
        .param("value", BasicType::Uint32, "Value")?
        .returns_nothing()?
        .build()?
        .destroy_callback("on_destroy")?
        .build()?;

    let testclass_get_value_async_func = lib
        .define_function("testclass_get_value_async")
        .param("testclass", testclass.clone(), "TestClass handle")?
        .param("cb", get_value_cb, "Callback to call with the value")?
        .returns_nothing()?
        .doc("Get value through a callback")?
        .build()?;

    let testclass_construction_counter = lib
        .define_function("testclass_construction_counter")
        .returns(BasicType::Uint32, "Number of calls to the constructor")?
        .doc("Get number of calls to the constructor")?
        .build()?;

    // Define the class
    let _testclass = lib
        .define_class(&testclass)?
        .constructor(&testclass_new_func)?
        .destructor(&testclass_destroy_func)?
        .method("GetValue", &testclass_get_value_func)?
        .method("IncrementValue", &testclass_increment_value_func)?
        .async_method("GetValueAsync", &testclass_get_value_async_func)?
        .static_method("ConstructionCounter", &testclass_construction_counter)?
        .custom_destroy("Delete")?
        .doc("TestClass")?
        .build()?;

    Ok(())
}
