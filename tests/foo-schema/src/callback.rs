use oo_bindgen::types::{BasicType, DurationType};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // Declare interface
    let interface = lib
        .define_asynchronous_interface("callback_interface", "Test interface")?
        .begin_callback(
            "on_value",
            "On value callback which takes parameter {param:value}",
        )?
        .param("value", BasicType::U32, "Value")?
        .returns(BasicType::U32, "Some value")?
        .end_callback()?
        .begin_callback("on_duration", "On duration callback")?
        .param("value", DurationType::Milliseconds, "Value")?
        .returns(DurationType::Milliseconds, "Some value")?
        .end_callback()?
        .build()?;

    // Declare the class
    let cbsource = lib.declare_class("callback_source")?;

    // Declare each native function
    let cbsource_new_func = lib
        .define_function("cbsource_new")?
        .returns(cbsource.clone(), "Handle to a callback_source")?
        .doc("Create a new CallbackSource")?
        .build()?;

    let callback_source_destructor =
        lib.define_destructor(cbsource.clone(), "Destroy a callback source")?;

    let cbsource_set_interface = lib
        .define_method("set_interface", cbsource.clone())?
        .param("cb", interface, "Callback to add")?
        .returns_nothing()?
        .doc("Add a callback")?
        .build()?;

    let cbsource_set_value_func = lib
        .define_method("set_value", cbsource.clone())?
        .param("value", BasicType::U32, "New value")?
        .returns(BasicType::U32, "Value returned by the callback")?
        .doc("Set the value and call all the callbacks")?
        .build()?;

    let cbsource_set_duration_func = lib
        .define_method("set_duration", cbsource.clone())?
        .param("value", DurationType::Milliseconds, "New duration")?
        .returns(DurationType::Milliseconds, "Some value")?
        .doc("Set the duration and call all the callbacks")?
        .build()?;

    // Define the class
    let _cbsource = lib
        .define_class(&cbsource)?
        .constructor(&cbsource_new_func)?
        .destructor(callback_source_destructor)?
        .method(cbsource_set_interface)?
        .method(cbsource_set_value_func)?
        .method(cbsource_set_duration_func)?
        .disposable_destroy()?
        .doc("Class that demonstrate the usage of an async interface")?
        .build()?;

    Ok(())
}
