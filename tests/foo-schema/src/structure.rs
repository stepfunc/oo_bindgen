use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::structs::common::{ConstructorName, ConstructorValue, FieldName};
use oo_bindgen::structs::function_struct::FunctionArgStructHandle;
use oo_bindgen::types::{BasicType, DurationType, STRING_TYPE};
use oo_bindgen::*;
use std::time::Duration;

pub fn define_inner_structure(
    lib: &mut LibraryBuilder,
    structure_enum: EnumHandle,
) -> BindResult<FunctionArgStructHandle> {
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
        .new_constructor(
            ConstructorName::Normal("init".to_string()),
            "Initialize to default values".into(),
        )?
        .add(&test_field, ConstructorValue::Uint16(41))?
        .add(
            &first_enum_field,
            ConstructorValue::Enum("Var2".to_string()),
        )?
        .add(&int1_field, ConstructorValue::Sint16(1))?
        .add(&bool2_field, ConstructorValue::Bool(false))?
        .add(
            &second_enum_field,
            ConstructorValue::Enum("Var2".to_string()),
        )?
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

    let an_empty_interface = lib
        .define_interface("EmptyInterface", "Interface within a structure")
        .build()?;

    let enum_value = FieldName::new("enum_value");
    let boolean_value = FieldName::new("boolean_value");
    let boolean_value2 = FieldName::new("boolean_value2");
    let enum_value2 = FieldName::new("enum_value2");
    let uint8_value = FieldName::new("uint8_value");
    let int8_value = FieldName::new("int8_value");
    let uint16_value = FieldName::new("uint16_value");
    let int16_value = FieldName::new("int16_value");
    let uint32_value = FieldName::new("uint32_value");
    let int32_value = FieldName::new("int32_value");
    let uint64_value = FieldName::new("uint64_value");
    let int64_value = FieldName::new("int64_value");
    let float_value = FieldName::new("float_value");
    let double_value = FieldName::new("double_value");
    let string_value = FieldName::new("string_value");
    let structure_value = FieldName::new("structure_value");
    let empty_interface = FieldName::new("empty_interface");
    let duration_millis = FieldName::new("duration_millis");
    let duration_seconds = FieldName::new("duration_seconds");

    lib.define_fstruct(&structure)?
        .add(enum_value.clone(), structure_enum.clone(), "enum_value")?
        .add(boolean_value.clone(), BasicType::Bool, "boolean_value")?
        .add(boolean_value2.clone(), BasicType::Bool, "boolean_value2")?
        .add(enum_value2.clone(), structure_enum, "enum_value2")?
        .add(uint8_value.clone(), BasicType::Uint8, "uint8_value")?
        .add(int8_value.clone(), BasicType::Sint8, "int8_value")?
        .add(uint16_value.clone(), BasicType::Uint16, "uint16_value")?
        .add(int16_value.clone(), BasicType::Sint16, "int16_value")?
        .add(uint32_value.clone(), BasicType::Uint32, "uint32_value")?
        .add(int32_value.clone(), BasicType::Sint32, "int32_value")?
        .add(uint64_value.clone(), BasicType::Uint64, "uint64_value")?
        .add(int64_value.clone(), BasicType::Sint64, "int64_value")?
        .add(float_value.clone(), BasicType::Float32, "float_value")?
        .add(double_value.clone(), BasicType::Double64, "double_value")?
        .add(string_value.clone(), STRING_TYPE, "string_value")?
        .add(structure_value.clone(), inner_structure, "structure_value")?
        .add(
            empty_interface.clone(),
            an_empty_interface,
            "interface that does nothing",
        )?
        .add(
            duration_millis.clone(),
            DurationType::Milliseconds,
            "duration_millis",
        )?
        .add(
            duration_seconds.clone(),
            DurationType::Seconds,
            "duration_seconds",
        )?
        .doc("Test structure")?
        .end_fields()?

        // construct all values with defaults
        .new_constructor(ConstructorName::Normal("init".to_string()), "Initialize {struct:Structure} to default values".into())?
        .add(&enum_value, ConstructorValue::Enum("Var2".into()))?
        .add(&boolean_value, ConstructorValue::Bool(true))?
        .add(&boolean_value2, ConstructorValue::Bool(true))?
        .add(&enum_value2, ConstructorValue::Enum("Var2".into()))?
        .add(&uint8_value, ConstructorValue::Uint8(1))?
        .add(&int8_value, ConstructorValue::Sint8(-1))?
        .add(&uint16_value, ConstructorValue::Uint16(2))?
        .add(&int16_value, ConstructorValue::Sint16(-2))?
        .add(&uint32_value, ConstructorValue::Uint32(3))?
        .add(&int32_value, ConstructorValue::Sint32(-3))?
        .add(&uint64_value, ConstructorValue::Uint64(4))?
        .add(&int64_value, ConstructorValue::Sint64(-4))?
        .add(&float_value, ConstructorValue::Float(12.34))?
        .add(&double_value, ConstructorValue::Double(-56.78))?
        .add(&string_value, ConstructorValue::String("Hello".into()))?
        .add(&structure_value, ConstructorValue::DefaultStruct)?
        .add(&duration_millis, ConstructorValue::Duration(DurationType::Milliseconds, Duration::from_millis(4200)))?
        .add(&duration_seconds, ConstructorValue::Duration(DurationType::Seconds, Duration::from_secs(76)))?
        .end_constructor()?

        .build()?;

    Ok(())
}
