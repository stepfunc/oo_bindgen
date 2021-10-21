use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::structs::*;
use oo_bindgen::types::{BasicType, DurationType, StringType};
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
        .define_function_argument_struct(&inner_structure)?
        .doc("Structure used within another structure")?
        .add(test_field.clone(), BasicType::U16, "test uint16 field")?
        // The following pattern used to crash in Java because of the way we handled boolean
        .add(
            first_enum_field.clone(),
            structure_enum.clone(),
            "first_enum_value",
        )?
        .add(int1_field.clone(), BasicType::S16, "int field")?
        .add(bool2_field.clone(), BasicType::Bool, "boolean field")?
        .add(
            second_enum_field.clone(),
            structure_enum,
            "second enum value",
        )?
        .end_fields()?
        // constructor definition
        .begin_constructor(
            "init",
            ConstructorType::Normal,
            "Initialize to default values",
        )?
        .default(&test_field, Number::U16(41))?
        .default(
            &first_enum_field,
            ConstructorDefault::Enum("Var2".to_string()),
        )?
        .default(&int1_field, Number::S16(1))?
        .default(&bool2_field, false)?
        .default(
            &second_enum_field,
            ConstructorDefault::Enum("Var2".to_string()),
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

    lib.define_function_argument_struct(&structure)?
        .add(enum_value.clone(), structure_enum.clone(), "enum_value")?
        .add(boolean_value.clone(), BasicType::Bool, "boolean_value")?
        .add(boolean_value2.clone(), BasicType::Bool, "boolean_value2")?
        .add(enum_value2.clone(), structure_enum, "enum_value2")?
        .add(uint8_value.clone(), BasicType::U8, "uint8_value")?
        .add(int8_value.clone(), BasicType::S8, "int8_value")?
        .add(uint16_value.clone(), BasicType::U16, "uint16_value")?
        .add(int16_value.clone(), BasicType::S16, "int16_value")?
        .add(uint32_value.clone(), BasicType::U32, "uint32_value")?
        .add(int32_value.clone(), BasicType::S32, "int32_value")?
        .add(uint64_value.clone(), BasicType::U64, "uint64_value")?
        .add(int64_value.clone(), BasicType::S64, "int64_value")?
        .add(float_value.clone(), BasicType::Float32, "float_value")?
        .add(double_value.clone(), BasicType::Double64, "double_value")?
        .add(string_value.clone(), StringType, "string_value")?
        .add(structure_value.clone(), inner_structure, "structure_value")?
        .add(
            empty_interface,
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
        .begin_constructor(
            "init",
            ConstructorType::Normal,
            "Initialize {struct:Structure} to default values",
        )?
        .default(&enum_value, ConstructorDefault::Enum("Var2".into()))?
        .default(&boolean_value, true)?
        .default(&boolean_value2, true)?
        .default(&enum_value2, ConstructorDefault::Enum("Var2".into()))?
        .default(&uint8_value, Number::U8(1))?
        .default(&int8_value, Number::S8(-1))?
        .default(&uint16_value, Number::U16(2))?
        .default(&int16_value, Number::S16(-2))?
        .default(&uint32_value, Number::U32(3))?
        .default(&int32_value, Number::S32(-3))?
        .default(&uint64_value, Number::U64(4))?
        .default(&int64_value, Number::S64(-4))?
        .default(&float_value, Number::Float(12.34))?
        .default(&double_value, Number::Double(-56.78))?
        .default(&string_value, ConstructorDefault::String("Hello".into()))?
        .default(&structure_value, ConstructorDefault::DefaultStruct)?
        .default(
            &duration_millis,
            ConstructorDefault::Duration(Duration::from_millis(4200)),
        )?
        .default(
            &duration_seconds,
            ConstructorDefault::Duration(Duration::from_secs(76)),
        )?
        .end_constructor()?
        .build()?;

    Ok(())
}
