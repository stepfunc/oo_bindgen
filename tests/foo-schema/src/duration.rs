use oo_bindgen::native_function::*;
use oo_bindgen::types::{BasicType, DurationMapping};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare each echo function
    let duration_ms_echo_func = lib
        .declare_native_function("duration_ms_echo")?
        .param(
            "value",
            BasicType::Duration(DurationMapping::Milliseconds),
            "Duration",
        )?
        .return_type(ReturnType::new(
            BasicType::Duration(DurationMapping::Milliseconds),
            "Duration",
        ))?
        .doc("Echo duration as count of milliseconds")?
        .build()?;

    let duration_s_echo_func = lib
        .declare_native_function("duration_s_echo")?
        .param(
            "value",
            BasicType::Duration(DurationMapping::Seconds),
            "Duration",
        )?
        .return_type(ReturnType::new(
            BasicType::Duration(DurationMapping::Seconds),
            "Duration",
        ))?
        .doc("Echo duration as count of seconds")?
        .build()?;

    // Declare static class
    lib.define_static_class("DurationEchoFunctions")?
        .static_method("MillisecondsEcho", &duration_ms_echo_func)?
        .static_method("SecondsEcho", &duration_s_echo_func)?
        .doc("Duration echos functions")?
        .build()?;

    Ok(())
}
