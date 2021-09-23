use std::time::Duration;

use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::{NativeStructHandle, StructElementType};
use oo_bindgen::types::DurationMapping;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<NativeStructHandle, BindingError> {
    let structure_enum = lib
        .define_native_enum("StructureEnum")?
        .push("Var1", "Var1")?
        .push("Var2", "Var2")?
        .push("Var3", "Var3")?
        .doc("Enum")?
        .build()?;

    let other_structure = lib.declare_native_struct("OtherStructure")?;
    let other_structure = lib
        .define_native_struct(&other_structure)?
        .add("test", StructElementType::Uint16(Some(41)), "test")?
        // The following pattern used to crash in Java because of the way we handled boolean
        .add(
            "first_enum_value",
            StructElementType::Enum(structure_enum.clone(), Some("Var2".to_string())),
            "first_enum_value",
        )?
        .add("int1", StructElementType::Sint16(Some(1)), "int1")?
        .add("bool2", StructElementType::Bool(Some(false)), "bool2")?
        .add(
            "second_enum_value",
            StructElementType::Enum(structure_enum.clone(), Some("Var2".to_string())),
            "second_enum_value",
        )?
        .doc("Structure within a structure")?
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
            "enum_value",
            StructElementType::Enum(structure_enum.clone(), Some("Var2".to_string())),
            "enum_value",
        )?
        .add(
            "boolean_value",
            StructElementType::Bool(Some(true)),
            "boolean_value",
        )?
        .add(
            "boolean_value2",
            StructElementType::Bool(Some(true)),
            "boolean_value2",
        )?
        .add(
            "enum_value2",
            StructElementType::Enum(structure_enum, Some("Var2".to_string())),
            "enum_value2",
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
            Type::Struct(other_structure.clone()),
            "structure_value",
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

    Ok(other_structure)
}
