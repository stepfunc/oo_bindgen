use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare interface
    let interface = lib
        .define_interface("CallbackInterface", "Test interface")?
        .callback("on_value", "On value callback")?
        .param("value", Type::Uint32, "Value")?
        .return_type(ReturnType::Type(Type::Uint32, "Some value".into()))?
        .build()?
        .callback("on_duration", "On duration callback")?
        .param(
            "value",
            Type::Duration(DurationMapping::Milliseconds),
            "Value",
        )?
        .return_type(ReturnType::Type(
            Type::Duration(DurationMapping::Milliseconds),
            "Some value".into(),
        ))?
        .build()?
        .destroy_callback("on_destroy")?
        .build()?;

    let one_time_callback = lib
        .define_one_time_callback("OneTimeCallbackInterface", "Test one-time-interface")?
        .callback("on_value", "On value callback")?
        .param("value", Type::Uint32, "Value")?
        .return_type(ReturnType::Type(Type::Uint32, "Some value".into()))?
        .build()?
        .build()?;

    // Declare the class
    let cbsource = lib.declare_class("CallbackSource")?;

    // Declare each native function
    let cbsource_new_func = lib
        .declare_native_function("cbsource_new")?
        .return_type(ReturnType::new(
            Type::ClassRef(cbsource.clone()),
            "Handle to a CallbackSource",
        ))?
        .doc("Create a new CallbackSource")?
        .build()?;

    let cbsource_destroy_func = lib
        .declare_native_function("cbsource_destroy")?
        .param(
            "cbsource",
            Type::ClassRef(cbsource.clone()),
            "Callback source",
        )?
        .return_type(ReturnType::void())?
        .doc("Destroy a callback source")?
        .build()?;

    let cbsource_set_interface = lib
        .declare_native_function("cbsource_set_interface")?
        .param(
            "cbsource",
            Type::ClassRef(cbsource.clone()),
            "Callback source",
        )?
        .param("cb", Type::Interface(interface), "Callback to add")?
        .return_type(ReturnType::void())?
        .doc("Add a callback")?
        .build()?;

    let cbsource_call_one_time_func = lib
        .declare_native_function("cbsource_call_one_time")?
        .param(
            "cbsource",
            Type::ClassRef(cbsource.clone()),
            "Callback source",
        )?
        .param(
            "cb",
            Type::OneTimeCallback(one_time_callback),
            "Callback to add",
        )?
        .return_type(ReturnType::Type(
            Type::Uint32,
            "Value returned by the callback".into(),
        ))?
        .doc("Add a one-time callback")?
        .build()?;

    let cbsource_set_value_func = lib
        .declare_native_function("cbsource_set_value")?
        .param(
            "cbsource",
            Type::ClassRef(cbsource.clone()),
            "Callback source",
        )?
        .param("value", Type::Uint32, "New value")?
        .return_type(ReturnType::Type(
            Type::Uint32,
            "Value returned by the callback".into(),
        ))?
        .doc("Set the value and call all the callbacks")?
        .build()?;

    let cbsource_set_duration_func = lib
        .declare_native_function("cbsource_set_duration")?
        .param(
            "cbsource",
            Type::ClassRef(cbsource.clone()),
            "Callback source",
        )?
        .param(
            "value",
            Type::Duration(DurationMapping::Milliseconds),
            "New duration",
        )?
        .return_type(ReturnType::Type(
            Type::Duration(DurationMapping::Milliseconds),
            "Some value".into(),
        ))?
        .doc("Set the duration and call all the callbacks")?
        .build()?;

    // Define the class
    let _cbsource = lib
        .define_class(&cbsource)?
        .constructor(&cbsource_new_func)?
        .destructor(&cbsource_destroy_func)?
        .method("SetInterface", &cbsource_set_interface)?
        .method("CallOneTime", &cbsource_call_one_time_func)?
        .method("SetValue", &cbsource_set_value_func)?
        .method("SetDuration", &cbsource_set_duration_func)?
        .doc("CallbackSource class")?
        .build()?;

    Ok(())
}
