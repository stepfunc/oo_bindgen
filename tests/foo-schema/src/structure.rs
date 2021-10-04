use std::time::Duration;

use oo_bindgen::function_struct::{FStructFieldType, FStructHandle};
use oo_bindgen::struct_common::EnumField;
use oo_bindgen::types::DurationType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<FStructHandle, BindingError> {
    let structure_enum = lib
        .define_enum("StructureEnum")
        .push("Var1", "Var1")?
        .push("Var2", "Var2")?
        .push("Var3", "Var3")?
        .doc("Enum")?
        .build()?;

    let other_structure = lib.declare_struct("OtherStructure")?;
    let other_structure = lib
        .define_fstruct(&other_structure)?
        .add("test", FStructFieldType::Uint16(Some(41)), "test")?
        // The following pattern used to crash in Java because of the way we handled boolean
        .add(
            "first_enum_value",
            EnumField::try_default(structure_enum.clone(), "Var2")?,
            "first_enum_value",
        )?
        .add("int1", FStructFieldType::Sint16(Some(1)), "int1")?
        .add("bool2", FStructFieldType::Bool(Some(false)), "bool2")?
        .add(
            "second_enum_value",
            EnumField::try_default(structure_enum.clone(), "Var2")?,
            "second_enum_value",
        )?
        .doc("Structure within a structure")?
        .build()?;

    let structure = lib.declare_struct("Structure")?;

    let empty_interface = lib
        .define_interface("EmptyInterface", "Interface within a structure")
        .destroy_callback("on_destroy")?
        .build()?;

    lib.define_fstruct(&structure)?
        .add(
            "enum_value",
            EnumField::try_default(structure_enum.clone(), "Var2")?,
            "enum_value",
        )?
        .add(
            "boolean_value",
            FStructFieldType::Bool(Some(true)),
            "boolean_value",
        )?
        .add(
            "boolean_value2",
            FStructFieldType::Bool(Some(true)),
            "boolean_value2",
        )?
        .add(
            "enum_value2",
            EnumField::try_default(structure_enum, "Var2")?,
            "enum_value2",
        )?
        .add(
            "uint8_value",
            FStructFieldType::Uint8(Some(1)),
            "uint8_value",
        )?
        .add(
            "int8_value",
            FStructFieldType::Sint8(Some(-1)),
            "int8_value",
        )?
        .add(
            "uint16_value",
            FStructFieldType::Uint16(Some(2)),
            "uint16_value",
        )?
        .add(
            "int16_value",
            FStructFieldType::Sint16(Some(-2)),
            "int16_value",
        )?
        .add(
            "uint32_value",
            FStructFieldType::Uint32(Some(3)),
            "uint32_value",
        )?
        .add(
            "int32_value",
            FStructFieldType::Sint32(Some(-3)),
            "int32_value",
        )?
        .add(
            "uint64_value",
            FStructFieldType::Uint64(Some(4)),
            "uint64_value",
        )?
        .add(
            "int64_value",
            FStructFieldType::Sint64(Some(-4)),
            "int64_value",
        )?
        .add(
            "float_value",
            FStructFieldType::Float(Some(12.34)),
            "float_value",
        )?
        .add(
            "double_value",
            FStructFieldType::Double(Some(-56.78)),
            "double_value",
        )?
        .add(
            "string_value",
            FStructFieldType::String(Some("Hello".to_string())),
            "string_value",
        )?
        .add(
            "structure_value",
            other_structure.clone(),
            "structure_value",
        )?
        .add(
            "empty_interface",
            empty_interface,
            "interface that does nothing",
        )?
        .add(
            "duration_millis",
            FStructFieldType::Duration(
                DurationType::Milliseconds,
                Some(Duration::from_millis(4200)),
            ),
            "duration_millis",
        )?
        .add(
            "duration_seconds",
            FStructFieldType::Duration(DurationType::Seconds, Some(Duration::from_secs(76))),
            "duration_seconds",
        )?
        .doc("Test structure")?
        .build()?;

    Ok(other_structure)
}
