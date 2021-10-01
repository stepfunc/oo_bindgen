use oo_bindgen::types::DurationType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare each echo function
    let duration_ms_echo_func = lib
        .define_function("duration_ms_echo")?
        .param("value", DurationType::Milliseconds, "Duration")?
        .returns(DurationType::Milliseconds, "Duration")?
        .doc("Echo duration as count of milliseconds")?
        .build()?;

    let duration_s_echo_func = lib
        .define_function("duration_s_echo")?
        .param("value", DurationType::Seconds, "Duration")?
        .returns(DurationType::Seconds, "Duration")?
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
