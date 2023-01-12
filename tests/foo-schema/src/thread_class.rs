use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // Declare the class
    let thread_class = lib.declare_class("thread_class")?;

    let error_type = lib
        .define_error_type(
            "math_is_broken",
            "broken_math_exception",
            ExceptionType::CheckedException,
        )?
        .add_error("math_is_broke", "hey, sometime is happens")?
        .add_error("dropped", "callback was dropped")?
        .doc("sometime math just doesn't work")?
        .build()?;

    let value_change_listener = lib
        .define_interface(
            "value_change_listener",
            "modifies a value on a remote thread",
        )?
        .begin_callback("on_value_change", "called when a value is modified")?
        .param("value", Primitive::U32, "updated value")?
        .enable_functional_transform()
        .end_callback()?
        .build_async()?;

    let operation = lib
        .define_interface(
            "operation",
            "interface for performing an operation on a value",
        )?
        .begin_callback("execute", "Take a value and return a modified value")?
        .param("value", Primitive::U32, "input value")?
        .returns(Primitive::U32, "modified value")?
        .enable_functional_transform()
        .end_callback()?
        .build_async()?;

    let execute = lib
        .define_method("execute", thread_class.clone())?
        .param(
            "operation",
            operation,
            "operation to perform on the value owned by the thread",
        )?
        .doc("Execute an operation on the internal value and trigger a callback")?
        .build()?;

    // Declare each native function
    let constructor = lib
        .define_constructor(thread_class.clone())?
        .param(
            "value",
            Primitive::U32,
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
        .param("value", Primitive::U32, "value to update")?
        .doc("Update the internal value and trigger callbacks to the {interface:value_change_listener}")?
        .build()?;

    let queue_error = lib
        .define_method("queue_error", thread_class.clone())?
        .param(
            "next_error",
            error_type.clone_enum(),
            "error to return next time {class:thread_class.add()} is called",
        )?
        .doc("Next time {class:thread_class.add()} is called, fail it with this error")?
        .build()?;

    let drop_next_add = lib
        .define_method("drop_next_add", thread_class.clone())?
        .doc("Next time {class:thread_class.add()} is called, the callback promise will just get dropped")?
        .build()?;

    let add_handler = lib.define_future_interface(
        "add_handler",
        "receives a single value from an add operation",
        Primitive::U32,
        "result of the add operation",
        error_type,
        "dropped",
    )?;

    let add_async = lib
        .define_future_method("add", thread_class.clone(), add_handler)?
        .param(
            "value",
            Primitive::U32,
            "Value to add to the internal value",
        )?
        .doc("adds a supplied value to an internal value")?
        .build()?;

    // Define the class
    lib.define_class(&thread_class)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(update)?
        .method(execute)?
        .method(queue_error)?
        .method(drop_next_add)?
        .async_method(add_async)?
        .custom_destroy("shutdown")?
        .doc("A class that manipulations integers on a Rust thread")?
        .build()?;

    Ok(())
}
