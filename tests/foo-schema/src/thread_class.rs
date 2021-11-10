use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // Declare the class
    let thread_class = lib.declare_class("thread_class")?;

    let value_change_listener = lib
        .define_asynchronous_interface(
            "value_change_listener",
            "modifies a value on a remote thread",
        )?
        .begin_callback("on_value_change", "called when a value is modified")?
        .param("value", BasicType::U32, "updated value")?
        .returns_nothing()?
        .end_callback()?
        .build()?;

    // Declare each native function
    let constructor = lib
        .define_constructor(thread_class.clone())?
        .param(
            "value",
            BasicType::U32,
            "Initial value that will be manipulated",
        )?
        .param(
            "receiver",
            value_change_listener,
            "callback interface that receives value changes",
        )?
        .doc("Construct an instance of {class:thread_class}")?
        .build()?;

    let destructor =
        lib.define_destructor(thread_class.clone(), "Destroy a {class:thread_class}")?;

    let update = lib
        .define_method("update", thread_class.clone())?
        .param("value", BasicType::U32, "value to update")?
        .returns_nothing()?
        .doc("Update the internal value and trigger callbacks to the {interface:value_change_listener}")?
        .build()?;

    let add_handler = lib.define_future_interface(
        "add_handler",
        "receives a single value from an add operation",
        BasicType::U32,
        "result of the add operation",
    )?;
    let add_async = lib
        .define_future_method("add", thread_class.clone(), add_handler)?
        .param(
            "value",
            BasicType::U32,
            "Value to add to the internal value",
        )?
        .doc("adds a supplied value to an internal value")?
        .build()?;

    // Define the class
    lib.define_class(&thread_class)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(update)?
        .async_method(add_async)?
        .custom_destroy("shutdown")?
        .doc("A class that manipulations integers on a Rust thread")?
        .build()?;

    Ok(())
}
