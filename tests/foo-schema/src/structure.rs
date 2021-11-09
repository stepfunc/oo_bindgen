use oo_bindgen::name::Name;
use oo_bindgen::structs::*;
use oo_bindgen::types::{BasicType, DurationType, StringType};
use oo_bindgen::*;
use std::time::Duration;

pub fn define_numbers_structure(lib: &mut LibraryBuilder) -> BackTraced<UniversalStructHandle> {
    let uint8_value = Name::create("uint8_value")?;
    let int8_value = Name::create("int8_value")?;
    let uint16_value = Name::create("uint16_value")?;
    let int16_value = Name::create("int16_value")?;
    let uint32_value = Name::create("uint32_value")?;
    let int32_value = Name::create("int32_value")?;
    let uint64_value = Name::create("uint64_value")?;
    let int64_value = Name::create("int64_value")?;
    let float_value = Name::create("float_value")?;
    let double_value = Name::create("double_value")?;

    let numbers = lib.declare_universal_struct("numbers")?;
    let numbers = lib
        .define_universal_struct(numbers)?
        .doc("structure containing all the numeric types")?
        .add(uint8_value.clone(), BasicType::U8, "uint8 value")?
        .add(int8_value.clone(), BasicType::S8, "int8 value")?
        .add(uint16_value.clone(), BasicType::U16, "uint16 value")?
        .add(int16_value.clone(), BasicType::S16, "int16 value")?
        .add(uint32_value.clone(), BasicType::U32, "uint32 value")?
        .add(int32_value.clone(), BasicType::S32, "int32 value")?
        .add(uint64_value.clone(), BasicType::U64, "uint64 value")?
        .add(int64_value.clone(), BasicType::S64, "int64 value")?
        .add(float_value.clone(), BasicType::Float32, "float value")?
        .add(double_value.clone(), BasicType::Double64, "double value")?
        .end_fields()?
        .begin_initializer(
            "init",
            InitializerType::Normal,
            "Initialize {struct:numbers} to default values",
        )?
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
        .end_initializer()?
        .build()?;

    Ok(numbers)
}

pub fn define_inner_structure(lib: &mut LibraryBuilder) -> BackTraced<FunctionArgStructHandle> {
    let empty_interface = lib
        .define_asynchronous_interface("empty_interface", "Interface within a structure")?
        .build()?;

    let numbers = define_numbers_structure(lib)?;

    let interface_field = Name::create("interface_field")?;
    let numbers_field = Name::create("numbers_field")?;
    let inner_structure = lib.declare_function_arg_struct("inner_structure")?;

    let inner_structure = lib
        .define_function_argument_struct(inner_structure)?
        .doc("A structure containing a {interface:empty_interface} and a {struct:numbers}")?
        .add(interface_field, empty_interface, "an empty interface")?
        .add(numbers_field.clone(), numbers, "struct full of numbers")?
        .end_fields()?
        // constructor definition
        .begin_initializer(
            "init",
            InitializerType::Normal,
            "Initialize to default values",
        )?
        .default_struct(&numbers_field)?
        .end_initializer()?
        .build()?;

    Ok(inner_structure)
}

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let structure_enum = lib
        .define_enum("structure_enum")?
        .push("var1", "Var1")?
        .push("var2", "Var2")?
        .push("var3", "Var3")?
        .doc("Enum")?
        .build()?;

    let inner_structure = define_inner_structure(lib)?;

    let structure = lib.declare_function_arg_struct("structure")?;

    let enum_var1_field = Name::create("enum_var1")?;
    let enum_var2_field = Name::create("enum_var2")?;
    let boolean_true_field = Name::create("boolean_true")?;
    let boolean_false_field = Name::create("boolean_false")?;
    let string_hello_field = Name::create("string_hello")?;
    let duration_millis_field = Name::create("duration_millis")?;
    let duration_seconds_field = Name::create("duration_seconds")?;
    let inner_structure_field = Name::create("inner_structure")?;

    lib.define_function_argument_struct(structure)?
        .doc("Test structure")?
        .add(
            enum_var1_field.clone(),
            structure_enum.clone(),
            "enum value defaulting to Var1",
        )?
        .add(
            enum_var2_field.clone(),
            structure_enum,
            "enum value defaulting to Var2",
        )?
        .add(
            boolean_true_field.clone(),
            BasicType::Bool,
            "boolean value defaulting to true",
        )?
        .add(
            boolean_false_field.clone(),
            BasicType::Bool,
            "boolean value defaulting to false",
        )?
        .add(
            string_hello_field.clone(),
            StringType,
            "string value defaulting to 'Hello'",
        )?
        .add(
            duration_millis_field.clone(),
            DurationType::Milliseconds,
            "duration in milliseconds",
        )?
        .add(
            duration_seconds_field.clone(),
            DurationType::Seconds,
            "duration in seconds",
        )?
        .add(inner_structure_field, inner_structure, "inner structure")?
        .end_fields()?
        // construct all values with defaults
        .begin_initializer(
            "init",
            InitializerType::Normal,
            "Initialize {struct:structure} to default values",
        )?
        .default_variant(&enum_var1_field, "var1")?
        .default_variant(&enum_var2_field, "var2")?
        .default(&boolean_true_field, true)?
        .default(&boolean_false_field, false)?
        .default_string(&string_hello_field, "Hello")?
        .default(&duration_millis_field, Duration::from_millis(4200))?
        .default(&duration_seconds_field, Duration::from_secs(76))?
        .end_initializer()?
        .build()?;

    Ok(())
}
