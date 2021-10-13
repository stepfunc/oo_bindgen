use oo_bindgen::types::{BasicType, DurationType, STRING_TYPE};
use oo_bindgen::*;
use oo_bindgen::structs::common::{FieldName, ConstructorValue, ConstructorName, Struct};
use oo_bindgen::structs::function_struct::{FunctionArgStructHandle, FunctionArgStructFieldType};
use oo_bindgen::enum_type::EnumHandle;

pub fn define_inner_structure(lib: &mut LibraryBuilder, structure_enum: EnumHandle) -> BindResult<FunctionArgStructHandle> {

    let test_field = FieldName::new("test");
    let first_enum_field = FieldName::new("first_enum_value");
    let int1_field = FieldName::new("int1");
    let bool2_field = FieldName::new("bool2");
    let second_enum_field = FieldName::new("second_enum_value");

    let inner_structure = lib.declare_struct("InnerStructure")?;
    let inner_structure = lib
        .define_fstruct(&inner_structure)?
        .doc("Structure used within another structure")?
        .add(test_field.clone(), BasicType::Uint16, "test uint16 field")?
        // The following pattern used to crash in Java because of the way we handled boolean
        .add(
            first_enum_field.clone(),
            structure_enum.clone(),
            "first_enum_value",
        )?
        .add(int1_field.clone(), BasicType::Sint16, "int field")?
        .add(bool2_field.clone(), BasicType::Bool, "boolean field")?
        .add(
            second_enum_field.clone(),
            structure_enum.clone(),
            "second enum value",
        )?
        .end_fields()?

        // constructor definition
        .new_constructor(ConstructorName::Normal("init".to_string()), "Initialize to default values".into())?
        .add(&test_field, ConstructorValue::Uint16(41))?
        .add(&first_enum_field, ConstructorValue::Enum("Var2".to_string()))?
        .add(&int1_field, ConstructorValue::Sint16(1))?
        .add(&bool2_field, ConstructorValue::Bool(false))?
        .add(&second_enum_field, ConstructorValue::Enum("Var2".to_string()))?
        .end_constructor()?
        .build()?;

    Ok(inner_structure)
}

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let structure_enum = lib
        .define_enum("StructureEnum")
        .push("Var1", "Var1")?
        .push("Var2", "Var2")?
        .push("Var3", "Var3")?
        .doc("Enum")?
        .build()?;

    let inner_structure = define_inner_structure(lib, structure_enum.clone())?;


    let structure = lib.declare_struct("Structure")?;

    let empty_interface = lib
        .define_interface("EmptyInterface", "Interface within a structure")
        .build()?;

    lib.define_fstruct(&structure)?
        .add("enum_value", structure_enum.clone(), "enum_value")?
        .add("boolean_value", BasicType::Bool, "boolean_value")?
        .add("boolean_value2", BasicType::Bool, "boolean_value2")?
        .add("enum_value2", structure_enum, "enum_value2")?
        .add("uint8_value", BasicType::Uint8, "uint8_value")?
        .add("int8_value", BasicType::Sint8, "int8_value")?
        .add("uint16_value", BasicType::Uint16, "uint16_value")?
        .add("int16_value", BasicType::Sint16, "int16_value")?
        .add("uint32_value", BasicType::Uint32, "uint32_value")?
        .add("int32_value", BasicType::Sint32, "int32_value")?
        .add("uint64_value", BasicType::Uint64, "uint64_value")?
        .add("int64_value", BasicType::Sint64, "int64_value")?
        .add("float_value", BasicType::Float32, "float_value")?
        .add("double_value", BasicType::Double64, "double_value")?
        .add("string_value", STRING_TYPE, "string_value")?
        .add("structure_value", inner_structure, "structure_value")?
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
