use oo_bindgen::types::{BasicType, DurationType};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare interface
    let interface = lib
        .define_asynchronous_interface("CallbackInterface", "Test interface")
        .begin_callback("on_value", "On value callback")?
        .param("value", BasicType::U32, "Value")?
        .returns(BasicType::U32, "Some value")?
        .end_callback()?
        .begin_callback("on_duration", "On duration callback")?
        .param("value", DurationType::Milliseconds, "Value")?
        .returns(DurationType::Milliseconds, "Some value")?
        .end_callback()?
        .build()?;

    // Declare the class
    let cbsource = lib.declare_class("CallbackSource")?;

    // Declare each native function
    let cbsource_new_func = lib
        .define_function("cbsource_new")
        .returns(cbsource.clone(), "Handle to a CallbackSource")?
        .doc("Create a new CallbackSource")?
        .build()?;

    let cbsource_destroy_func = lib
        .define_function("cbsource_destroy")
        .param("cbsource", cbsource.clone(), "Callback source")?
        .returns_nothing()?
        .doc("Destroy a callback source")?
        .build()?;

    let cbsource_set_interface = lib
        .define_function("cbsource_set_interface")
        .param("cbsource", cbsource.clone(), "Callback source")?
        .param("cb", interface, "Callback to add")?
        .returns_nothing()?
        .doc("Add a callback")?
        .build()?;

    let cbsource_set_value_func = lib
        .define_function("cbsource_set_value")
        .param("cbsource", cbsource.clone(), "Callback source")?
        .param("value", BasicType::U32, "New value")?
        .returns(BasicType::U32, "Value returned by the callback")?
        .doc("Set the value and call all the callbacks")?
        .build()?;

    let cbsource_set_duration_func = lib
        .define_function("cbsource_set_duration")
        .param("cbsource", cbsource.clone(), "Callback source")?
        .param("value", DurationType::Milliseconds, "New duration")?
        .returns(DurationType::Milliseconds, "Some value")?
        .doc("Set the duration and call all the callbacks")?
        .build()?;

    // Define the class
    let _cbsource = lib
        .define_class(&cbsource)?
        .constructor(&cbsource_new_func)?
        .destructor(&cbsource_destroy_func)?
        .method("SetInterface", &cbsource_set_interface)?
        .method("SetValue", &cbsource_set_value_func)?
        .method("SetDuration", &cbsource_set_duration_func)?
        .disposable_destroy()?
        .doc("CallbackSource class")?
        .build()?;

    Ok(())
}
