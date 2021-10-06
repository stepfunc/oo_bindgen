use oo_bindgen::types::{DurationType, BasicType, STRING_TYPE};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let structure_enum = lib
        .define_enum("StructureEnum")
        .push("Var1", "Var1")?
        .push("Var2", "Var2")?
        .push("Var3", "Var3")?
        .doc("Enum")?
        .build()?;

    let other_structure = lib.declare_struct("InnerStructure")?;
    let other_structure = lib
        .define_fstruct(&other_structure)?
        .doc("Structure used within another structure")?
        .add("test", BasicType::Uint16, "test")?
        // The following pattern used to crash in Java because of the way we handled boolean
        .add(
            "first_enum_value",
            structure_enum.clone(),
            "first_enum_value",
        )?
        .add("int1", BasicType::Sint16, "int1")?
        .add("bool2", BasicType::Bool, "bool2")?
        .add(
            "second_enum_value",
            structure_enum.clone(),
            "second_enum_value",
        )?
        .end_fields()?
        .build()?;

    let structure = lib.declare_struct("Structure")?;

    let empty_interface = lib
        .define_interface("EmptyInterface", "Interface within a structure")
        .build()?;

    lib.define_fstruct(&structure)?
        .add(
            "enum_value",
            structure_enum.clone(),
            "enum_value",
        )?
        .add(
            "boolean_value",
            BasicType::Bool,
            "boolean_value",
        )?
        .add(
            "boolean_value2",
            BasicType::Bool,
            "boolean_value2",
        )?
        .add(
            "enum_value2",
            structure_enum.clone(),
            "enum_value2",
        )?
        .add(
            "uint8_value",
            BasicType::Uint8,
            "uint8_value",
        )?
        .add(
            "int8_value",
            BasicType::Sint8,
            "int8_value",
        )?
        .add(
            "uint16_value",
            BasicType::Uint16,
            "uint16_value",
        )?
        .add(
            "int16_value",
            BasicType::Sint16,
            "int16_value",
        )?
        .add(
            "uint32_value",
            BasicType::Uint32,
            "uint32_value",
        )?
        .add(
            "int32_value",
            BasicType::Sint32,
            "int32_value",
        )?
        .add(
            "uint64_value",
            BasicType::Uint64,
            "uint64_value",
        )?
        .add(
            "int64_value",
            BasicType::Sint64,
            "int64_value",
        )?
        .add(
            "float_value",
            BasicType::Float32,
            "float_value",
        )?
        .add(
            "double_value",
            BasicType::Double64,
            "double_value",
        )?
        .add(
            "string_value",
            STRING_TYPE,
            "string_value",
        )?
        .add("structure_value", other_structure, "structure_value")?
        .add(
            "empty_interface",
            empty_interface,
            "interface that does nothing",
        )?
        .add(
            "duration_millis",
                DurationType::Milliseconds,
            "duration_millis",
        )?
        .add(
            "duration_seconds",
            DurationType::Seconds,
            "duration_seconds",
        )?
        .doc("Test structure")?
        .end_fields()?
        .build()?;

    Ok(())
}
