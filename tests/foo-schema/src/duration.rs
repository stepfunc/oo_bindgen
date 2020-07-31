use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare each echo function
    let duration_ms_echo_func = lib
        .declare_native_function("duration_ms_echo")?
        .param(
            "value",
            Type::Duration(DurationMapping::Milliseconds),
            "Duration (in milliseconds)",
        )?
        .return_type(ReturnType::new(
            Type::Duration(DurationMapping::Milliseconds),
            "Duration (in milliseconds)",
        ))?
        .doc("Echo duration through count of milliseconds")?
        .build()?;

    let duration_s_echo_func = lib
        .declare_native_function("duration_s_echo")?
        .param(
            "value",
            Type::Duration(DurationMapping::Seconds),
            "Duration (in seconds)",
        )?
        .return_type(ReturnType::new(
            Type::Duration(DurationMapping::Seconds),
            "Duration (in seconds)",
        ))?
        .doc("Echo duration through count of seconds")?
        .build()?;

    let duration_s_float_echo_func = lib
        .declare_native_function("duration_s_float_echo")?
        .param(
            "value",
            Type::Duration(DurationMapping::SecondsFloat),
            "Duration (in seconds)",
        )?
        .return_type(ReturnType::new(
            Type::Duration(DurationMapping::SecondsFloat),
            "Duration (in seconds)",
        ))?
        .doc("Echo duration through floating point (in seconds)")?
        .build()?;

    // Declare static class
    let class = lib.declare_class("DurationEchoFunctions")?;
    lib.define_class(&class)?
        .static_method("MillisecondsEcho", &duration_ms_echo_func)?
        .static_method("SecondsEcho", &duration_s_echo_func)?
        .static_method("SecondsFloatEcho", &duration_s_float_echo_func)?
        .doc("Duration echos functions")?
        .build()?;

    Ok(())
}
