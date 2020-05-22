use oo_bindgen::*;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare each echo function
    let duration_ms_echo_func = lib.declare_native_function("duration_ms_echo")?
        .param("value", Type::Duration(DurationMapping::Milliseconds))?
        .return_type(ReturnType::Type(Type::Duration(DurationMapping::Milliseconds)))?
        .build()?;

    let duration_s_echo_func = lib.declare_native_function("duration_s_echo")?
        .param("value", Type::Duration(DurationMapping::Seconds))?
        .return_type(ReturnType::Type(Type::Duration(DurationMapping::Seconds)))?
        .build()?;

    let duration_s_float_echo_func = lib.declare_native_function("duration_s_float_echo")?
        .param("value", Type::Duration(DurationMapping::SecondsFloat))?
        .return_type(ReturnType::Type(Type::Duration(DurationMapping::SecondsFloat)))?
        .build()?;

    // Declare static class
    let class = lib.declare_class("DurationEchoFunctions")?;
    lib.define_class(&class)?
        .static_method("MillisecondsEcho", &duration_ms_echo_func)?
        .static_method("SecondsEcho", &duration_s_echo_func)?
        .static_method("SecondsFloatEcho", &duration_s_float_echo_func)?
        .build();

    Ok(())
}
