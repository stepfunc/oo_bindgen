use oo_bindgen::*;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare interface
    let interface = lib.define_interface("CallbackInterface")?
        .callback("on_value")?
            .param("value", Type::Uint32)?
            .arg("data")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("on_duration")?
            .param("value", Type::Duration(DurationMapping::Milliseconds))?
            .arg("data")?
            .return_type(ReturnType::Void)?
            .build()?
        .destroy_callback("on_destroy")?
        .arg("data")?
        .build()?;

    // Declare the class
    let cbsource = lib.declare_class("CallbackSource")?;

    // Declare each native function
    let cbsource_new_func = lib.declare_native_function("cbsource_new")?
        .return_type(ReturnType::Type(Type::ClassRef(cbsource.clone())))?
        .build()?;

    let cbsource_destroy_func = lib.declare_native_function("cbsource_destroy")?
        .param("cbsource", Type::ClassRef(cbsource.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let cbsource_add_func = lib.declare_native_function("cbsource_add")?
        .param("cbsource", Type::ClassRef(cbsource.clone()))?
        .param("cb", Type::Interface(interface))?
        .return_type(ReturnType::Void)?
        .build()?;

    let cbsource_set_value_func = lib.declare_native_function("cbsource_set_value")?
        .param("cbsource", Type::ClassRef(cbsource.clone()))?
        .param("value", Type::Uint32)?
        .return_type(ReturnType::Void)?
        .build()?;

    let cbsource_set_duration_func = lib.declare_native_function("cbsource_set_duration")?
        .param("cbsource", Type::ClassRef(cbsource.clone()))?
        .param("value", Type::Duration(DurationMapping::Milliseconds))?
        .return_type(ReturnType::Void)?
        .build()?;

    // Define the class
    let _cbsource = lib.define_class(&cbsource)?
        .constructor(&cbsource_new_func)?
        .destructor(&cbsource_destroy_func)?
        .method("AddFunc", &cbsource_add_func)?
        .method("SetValue", &cbsource_set_value_func)?
        .method("SetDuration", &cbsource_set_duration_func)?
        .build();

    Ok(())
}
