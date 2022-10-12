use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let simple_struct = lib.declare_universal_struct("wrapped_number")?;

    let num_field = Name::create("num")?;

    let wrapped_number = lib
        .define_universal_struct(simple_struct)?
        .add(num_field.clone(), Primitive::S32, "the number")?
        .doc("Wrapped around a single i32")?
        .end_fields()?
        .begin_initializer(
            "default_value",
            InitializerType::Static,
            "initialize to default values",
        )?
        .default(&num_field, NumberValue::S32(42))?
        .end_initializer()?
        .build()?;

    let switch_position = lib
        .define_enum("switch_position")?
        .push("on", "Switch is in the ON position")?
        .push("off", "Switch is in the OFF position")?
        .doc("Switches can be ON or OFF!")?
        .build()?;

    let default_switch_pos = switch_position.value("on")?;

    // Declare interface
    let interface = lib
        .define_interface("defaulted_interface", "Test interface with default methods")?
        // bool
        .begin_callback(
            "get_bool_value",
            "Retrieve a bool value from a user interface",
        )?
        .returns_with_default(
            PrimitiveValue::Bool(true),
            "Some value special value from the user or the default",
        )?
        .end_callback()?
        // i32
        .begin_callback(
            "get_i32_value",
            "Retrieve an i32 value from a user interface",
        )?
        .returns_with_default(
            PrimitiveValue::S32(42),
            "Some value special value from the user or the default",
        )?
        .end_callback()?
        // u32
        .begin_callback(
            "get_u32_value",
            "Retrieve an u32 value from a user interface",
        )?
        .returns_with_default(
            PrimitiveValue::U32(42),
            "Some value special value from the user or the default",
        )?
        .end_callback()?
        // duration
        .begin_callback(
            "get_duration_ms",
            "Retrieve a millisecond duration from the user",
        )?
        .returns_with_default(
            DurationValue::Milliseconds(42),
            "Retrieve a duration from a user interface",
        )?
        .end_callback()?
        // enum
        .begin_callback("get_switch_position", "retrieve the position of a switch")?
        .returns_with_default(default_switch_pos, "The current position of the switch")?
        .end_callback()?
        // struct
        .begin_callback(
            "get_wrapped_number",
            "retrieve a structure which is just a wrapped i32",
        )?
        .returns_with_default(
            wrapped_number.zero_parameter_initializer("default_value")?,
            "Wrapped number value",
        )?
        .end_callback()?
        .build_sync()?;

    let get_bool = lib
        .define_function("get_bool_value")?
        .param("cb", interface.clone(), "callback interface")?
        .returns(Primitive::Bool, "value retrieved from interface")?
        .doc("retrieve value from interface")?
        .build_static("get_bool_value")?;

    let get_u32 = lib
        .define_function("get_u32_value")?
        .param("cb", interface.clone(), "callback interface")?
        .returns(Primitive::U32, "value retrieved from interface")?
        .doc("retrieve value from interface")?
        .build_static("get_u32_value")?;

    let get_i32 = lib
        .define_function("get_i32_value")?
        .param("cb", interface.clone(), "callback interface")?
        .returns(Primitive::S32, "value retrieved from interface")?
        .doc("retrieve value from interface")?
        .build_static("get_i32_value")?;

    let get_duration = lib
        .define_function("get_duration_value")?
        .param("cb", interface.clone(), "callback interface")?
        .returns(DurationType::Milliseconds, "value retrieved from interface")?
        .doc("retrieve value from interface")?
        .build_static("get_duration_value")?;

    let get_switch_pos = lib
        .define_function("get_switch_pos")?
        .param("cb", interface.clone(), "callback interface")?
        .returns(switch_position, "current position of the switch")?
        .doc("retrieve the current position of the switch")?
        .build_static("get_switch_pos")?;

    let get_wrapped_number = lib
        .define_function("get_wrapped_number")?
        .param("cb", interface, "callback interface")?
        .returns(
            wrapped_number,
            "wrapped number retrieved from the interface",
        )?
        .doc("retrieve the current wrapped number value")?
        .build_static("get_wrapped_number")?;

    // Define the class
    lib.define_static_class("default_interface_test")?
        .static_method(get_bool)?
        .static_method(get_u32)?
        .static_method(get_i32)?
        .static_method(get_duration)?
        .static_method(get_switch_pos)?
        .static_method(get_wrapped_number)?
        .doc("Class that demonstrate the usage of an async interface")?
        .build()?;

    Ok(())
}
