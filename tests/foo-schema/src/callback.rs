use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // Declare interface
    let interface = lib
        .define_interface("callback_interface", "Test interface")?
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
        .build_async()?;

    // Declare the class
    let callback_source = lib.declare_class("callback_source")?;

    // Declare each native function
    let constructor = lib
        .define_constructor(callback_source.clone())?
        .doc("Create a new CallbackSource")?
        .build()?;

    let destructor = lib.define_destructor(callback_source.clone(), "Destroy a callback source")?;

    let set_interface = lib
        .define_method("set_interface", callback_source.clone())?
        .param("cb", interface, "Callback to add")?
        .doc("Add a callback")?
        .build()?;

    let set_value = lib
        .define_method("set_value", callback_source.clone())?
        .param("value", BasicType::U32, "New value")?
        .returns(BasicType::U32, "Value returned by the callback")?
        .doc("Set the value and call all the callbacks")?
        .build()?;

    let set_duration = lib
        .define_method("set_duration", callback_source.clone())?
        .param("value", DurationType::Milliseconds, "New duration")?
        .returns(DurationType::Milliseconds, "Some value")?
        .doc("Set the duration and call all the callbacks")?
        .build()?;

    // Define the class
    lib.define_class(&callback_source)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(set_interface)?
        .method(set_value)?
        .method(set_duration)?
        .disposable_destroy()?
        .doc("Class that demonstrate the usage of an async interface")?
        .build()?;

    Ok(())
}
