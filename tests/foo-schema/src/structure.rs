use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let other_structure = lib.declare_native_struct("OtherStructure")?;
    let other_structure = lib
        .define_native_struct(&other_structure)?
        .add("test", Type::Uint16, "test")?
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
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    let structure = lib
        .define_native_struct(&structure)?
        .add("boolean_value", Type::Bool, "boolean_value")?
        .add("uint8_value", Type::Uint8, "uint8_value")?
        .add("int8_value", Type::Sint8, "int8_value")?
        .add("uint16_value", Type::Uint16, "uint16_value")?
        .add("int16_value", Type::Sint16, "int16_value")?
        .add("uint32_value", Type::Uint32, "uint32_value")?
        .add("int32_value", Type::Sint32, "int32_value")?
        .add("uint64_value", Type::Uint64, "uint64_value")?
        .add("int64_value", Type::Sint64, "int64_value")?
        .add("float_value", Type::Float, "float_value")?
        .add("double_value", Type::Double, "double_value")?
        .add("string_value", Type::String, "string_value")?
        .add(
            "structure_value",
            Type::Struct(other_structure),
            "structure_value",
        )?
        .add("enum_value", Type::Enum(structure_enum), "enum_value")?
        .add(
            "interface_value",
            Type::Interface(structure_interface),
            "interface_value",
        )?
        .add(
            "duration_millis",
            Type::Duration(DurationMapping::Milliseconds),
            "duration_millis",
        )?
        .add(
            "duration_seconds",
            Type::Duration(DurationMapping::Seconds),
            "duration_seconds",
        )?
        .add(
            "duration_seconds_float",
            Type::Duration(DurationMapping::SecondsFloat),
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
