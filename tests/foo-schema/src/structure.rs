use std::time::Duration;

use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::StructElementType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let other_structure = lib.declare_native_struct("OtherStructure")?;
    let other_structure = lib
        .define_native_struct(&other_structure)?
        .add("test", StructElementType::Uint16(Some(41)), "test")?
        .doc("Structure within a structure")?
        .build()?;

    let structure_enum = lib
        .define_native_enum("StructureEnum")?
        .push("Var1", "Var1")?
        .push("Var2", "Var2")?
        .push("Var3", "Var3")?
        .doc("Enum")?
        .build()?;

    let structure = lib.declare_native_struct("Structure")?;

    let structure_interface = lib
        .define_interface("StructureInterface", "Interface within a structure")?
        .callback("on_value", "Callback on value")?
        .param("value", Type::StructRef(structure.clone()), "New value")?
        .return_type(ReturnType::Void)?
        .build()?
        .destroy_callback("on_destroy")?
        .build()?;

    let structure = lib
        .define_native_struct(&structure)?
        .add(
            "boolean_value",
            StructElementType::Bool(Some(true)),
            "boolean_value",
        )?
        .add(
            "uint8_value",
            StructElementType::Uint8(Some(1)),
            "uint8_value",
        )?
        .add(
            "int8_value",
            StructElementType::Sint8(Some(-1)),
            "int8_value",
        )?
        .add(
            "uint16_value",
            StructElementType::Uint16(Some(2)),
            "uint16_value",
        )?
        .add(
            "int16_value",
            StructElementType::Sint16(Some(-2)),
            "int16_value",
        )?
        .add(
            "uint32_value",
            StructElementType::Uint32(Some(3)),
            "uint32_value",
        )?
        .add(
            "int32_value",
            StructElementType::Sint32(Some(-3)),
            "int32_value",
        )?
        .add(
            "uint64_value",
            StructElementType::Uint64(Some(4)),
            "uint64_value",
        )?
        .add(
            "int64_value",
            StructElementType::Sint64(Some(-4)),
            "int64_value",
        )?
        .add(
            "float_value",
            StructElementType::Float(Some(12.34)),
            "float_value",
        )?
        .add(
            "double_value",
            StructElementType::Double(Some(-56.78)),
            "double_value",
        )?
        .add(
            "string_value",
            StructElementType::String(Some("Hello".to_string())),
            "string_value",
        )?
        .add(
            "structure_value",
            Type::Struct(other_structure),
            "structure_value",
        )?
        .add(
            "enum_value",
            StructElementType::Enum(structure_enum, Some(1)),
            "enum_value",
        )?
        .add(
            "interface_value",
            Type::Interface(structure_interface),
            "interface_value",
        )?
        .add(
            "duration_millis",
            StructElementType::Duration(
                DurationMapping::Milliseconds,
                Some(Duration::from_millis(4200)),
            ),
            "duration_millis",
        )?
        .add(
            "duration_seconds",
            StructElementType::Duration(DurationMapping::Seconds, Some(Duration::from_secs(76))),
            "duration_seconds",
        )?
        .add(
            "duration_seconds_float",
            StructElementType::Duration(
                DurationMapping::SecondsFloat,
                Some(Duration::from_millis(15250)),
            ),
            "duration_seconds_float",
        )?
        .doc("Test structure")?
        .build()?;

    // Declare each echo function
    let struct_by_value_echo_func = lib
        .declare_native_function("struct_by_value_echo")?
        .param(
            "value",
            Type::Struct(structure.clone()),
            "Structure to echo",
        )?
        .return_type(ReturnType::new(
            Type::Struct(structure.clone()),
            "Echoed structure",
        ))?
        .doc("Echo structure by value")?
        .build()?;

    let struct_by_reference_echo_func = lib
        .declare_native_function("struct_by_reference_echo")?
        .param(
            "value",
            Type::StructRef(structure.declaration()),
            "Structure to echo",
        )?
        .return_type(ReturnType::new(
            Type::Struct(structure.clone()),
            "Echoed structure",
        ))?
        .doc("Echo structure by reference")?
        .build()?;

    // Declare structs methods
    lib.define_struct(&structure)?
        .static_method("StructByValueEcho", &struct_by_value_echo_func)?
        .method("StructByReferenceEcho", &struct_by_reference_echo_func)?
        .build();

    Ok(())
}
