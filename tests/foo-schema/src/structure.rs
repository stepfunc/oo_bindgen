use std::time::Duration;

use oo_bindgen::native_struct::{AllStructFieldType, AllStructHandle};
use oo_bindgen::types::{AllTypes, DurationType};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<AllStructHandle, BindingError> {
    let structure_enum = lib
        .define_enum("StructureEnum")?
        .push("Var1", "Var1")?
        .push("Var2", "Var2")?
        .push("Var3", "Var3")?
        .doc("Enum")?
        .build()?;

    let other_structure = lib.declare_native_struct("OtherStructure")?;
    let other_structure = lib
        .define_native_struct(&other_structure)?
        .add("test", AllStructFieldType::Uint16(Some(41)), "test")?
        // The following pattern used to crash in Java because of the way we handled boolean
        .add(
            "first_enum_value",
            AllStructFieldType::Enum(structure_enum.clone(), Some("Var2".to_string())),
            "first_enum_value",
        )?
        .add("int1", AllStructFieldType::Sint16(Some(1)), "int1")?
        .add("bool2", AllStructFieldType::Bool(Some(false)), "bool2")?
        .add(
            "second_enum_value",
            AllStructFieldType::Enum(structure_enum.clone(), Some("Var2".to_string())),
            "second_enum_value",
        )?
        .doc("Structure within a structure")?
        .build()?;

    let structure = lib.declare_native_struct("Structure")?;

    let structure_interface = lib
        .define_interface("StructureInterface", "Interface within a structure")?
        .callback("on_value", "Callback on value")?
        .param("value", structure.clone(), "New value")?
        .returns_nothing()?
        .build()?
        .destroy_callback("on_destroy")?
        .build()?;

    let structure = lib
        .define_native_struct(&structure)?
        .add(
            "enum_value",
            AllStructFieldType::Enum(structure_enum.clone(), Some("Var2".to_string())),
            "enum_value",
        )?
        .add(
            "boolean_value",
            AllStructFieldType::Bool(Some(true)),
            "boolean_value",
        )?
        .add(
            "boolean_value2",
            AllStructFieldType::Bool(Some(true)),
            "boolean_value2",
        )?
        .add(
            "enum_value2",
            AllStructFieldType::Enum(structure_enum, Some("Var2".to_string())),
            "enum_value2",
        )?
        .add(
            "uint8_value",
            AllStructFieldType::Uint8(Some(1)),
            "uint8_value",
        )?
        .add(
            "int8_value",
            AllStructFieldType::Sint8(Some(-1)),
            "int8_value",
        )?
        .add(
            "uint16_value",
            AllStructFieldType::Uint16(Some(2)),
            "uint16_value",
        )?
        .add(
            "int16_value",
            AllStructFieldType::Sint16(Some(-2)),
            "int16_value",
        )?
        .add(
            "uint32_value",
            AllStructFieldType::Uint32(Some(3)),
            "uint32_value",
        )?
        .add(
            "int32_value",
            AllStructFieldType::Sint32(Some(-3)),
            "int32_value",
        )?
        .add(
            "uint64_value",
            AllStructFieldType::Uint64(Some(4)),
            "uint64_value",
        )?
        .add(
            "int64_value",
            AllStructFieldType::Sint64(Some(-4)),
            "int64_value",
        )?
        .add(
            "float_value",
            AllStructFieldType::Float(Some(12.34)),
            "float_value",
        )?
        .add(
            "double_value",
            AllStructFieldType::Double(Some(-56.78)),
            "double_value",
        )?
        .add(
            "string_value",
            AllStructFieldType::String(Some("Hello".to_string())),
            "string_value",
        )?
        .add(
            "structure_value",
            other_structure.clone(),
            "structure_value",
        )?
        .add("interface_value", structure_interface, "interface_value")?
        .add(
            "duration_millis",
            AllStructFieldType::Duration(
                DurationType::Milliseconds,
                Some(Duration::from_millis(4200)),
            ),
            "duration_millis",
        )?
        .add(
            "duration_seconds",
            AllStructFieldType::Duration(DurationType::Seconds, Some(Duration::from_secs(76))),
            "duration_seconds",
        )?
        .doc("Test structure")?
        .build()?;

    // Declare each echo function
    let struct_by_value_echo_func = lib
        .declare_native_function("struct_by_value_echo")?
        .param(
            "value",
            AllTypes::Struct(structure.clone()),
            "Structure to echo",
        )?
        .returns(structure.clone(), "Echoed structure")?
        .doc("Echo structure by value")?
        .build()?;

    let struct_by_reference_echo_func = lib
        .declare_native_function("struct_by_reference_echo")?
        .param(
            "value",
            AllTypes::StructRef(structure.declaration()),
            "Structure to echo",
        )?
        .returns(structure.clone(), "Echoed structure")?
        .doc("Echo structure by reference")?
        .build()?;

    // Declare structs methods
    lib.define_struct(&structure)?
        .static_method("StructByValueEcho", &struct_by_value_echo_func)?
        .method("StructByReferenceEcho", &struct_by_reference_echo_func)?
        .build();

    Ok(other_structure)
}
