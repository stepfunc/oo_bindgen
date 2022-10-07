use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // Declare interface
    let interface = lib
        .define_interface("defaulted_interface", "Test interface with default methods")?
        // i32
        .begin_callback(
            "get_u32_value",
            "Retrieve an u32 value from a user interface",
        )?
        .returns_with_default(
            PrimitiveValue::U32(42),
            "Some value special value from the user or the default",
        )?
        .end_callback()?
        .begin_callback(
            "get_duration_ms",
            "Retrieve a millisecond duration from the user",
        )?
        .returns_with_default(
            DurationValue::Milliseconds(42),
            "Retrieve a duration from a user interface",
        )?
        .end_callback()?
        .build_sync()?;

    let get_u32 = lib
        .define_function("get_u32_value")?
        .param("cb", interface.clone(), "callback interface")?
        .returns(Primitive::U32, "value retrieved from interface")?
        .doc("retrieve value from interface")?
        .build_static("get_u32_value")?;

    let get_duration = lib
        .define_function("get_duration_value")?
        .param("cb", interface, "callback interface")?
        .returns(DurationType::Milliseconds, "value retrieved from interface")?
        .doc("retrieve value from interface")?
        .build_static("get_duration_value")?;

    // Define the class
    lib.define_static_class("default_interface_test")?
        .static_method(get_u32)?
        .static_method(get_duration)?
        .doc("Class that demonstrate the usage of an async interface")?
        .build()?;

    Ok(())
}
