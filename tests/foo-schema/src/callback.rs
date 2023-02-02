use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let names = lib.declare_universal_struct("names")?;

    let names = lib
        .define_universal_struct(names)?
        .doc("struct with strings!")?
        .add("first_name", StringType, "somebody's first name")?
        .add("last_name", StringType, "somebody's last name")?
        .end_fields()?
        .add_full_initializer("init")?
        .build()?;

    let names_iterator = lib.define_iterator("names_iter", names.clone())?;

    // Declare interface
    let interface = lib
        .define_interface("callback_interface", "Test interface")?
        .begin_callback(
            "on_value",
            "On value callback which takes parameter {param:value}",
        )?
        .param("value", Primitive::U32, "Value")?
        .returns(Primitive::U32, "Some value")?
        .end_callback()?
        .begin_callback("on_duration", "On duration callback")?
        .param("value", DurationType::Milliseconds, "Value")?
        .returns(DurationType::Milliseconds, "Some value")?
        .end_callback()?
        .begin_callback("on_names", "Callback with a struct of names")?
        .param("names", names.clone(), "Some names")?
        .end_callback()?
        .begin_callback("on_several_names", "Callback over an iterator of names")?
        .param("names", names_iterator, "{iterator} of {struct:names}")?
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
        .param("value", Primitive::U32, "New value")?
        .returns(Primitive::U32, "Value returned by the callback")?
        .doc("Set the value and call all the callbacks")?
        .build()?;

    let set_duration = lib
        .define_method("set_duration", callback_source.clone())?
        .param("value", DurationType::Milliseconds, "New duration")?
        .returns(DurationType::Milliseconds, "Some value")?
        .doc("Set the duration and call all the callbacks")?
        .build()?;

    let invoke_on_names = lib
        .define_method("invoke_on_names", callback_source.clone())?
        .param("names", names, "First and last name")?
        .doc("Invoke the on_names callback")?
        .build()?;

    let invoke_on_several_names = lib
        .define_method("invoke_on_several_names", callback_source.clone())?
        .doc("Invoke the on_name_several_names callback")?
        .build()?;

    // Define the class
    lib.define_class(&callback_source)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(set_interface)?
        .method(set_value)?
        .method(set_duration)?
        .method(invoke_on_names)?
        .method(invoke_on_several_names)?
        .disposable_destroy()?
        .doc("Class that demonstrate the usage of an async interface")?
        .build()?;

    Ok(())
}
