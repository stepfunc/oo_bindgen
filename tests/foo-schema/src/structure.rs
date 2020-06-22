use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let other_structure = lib.declare_native_struct("OtherStructure")?;
    let other_structure = lib
        .define_native_struct(&other_structure)?
        .add("test", Type::Uint16)?
        .build();

    let structure_enum = lib
        .define_native_enum("StructureEnum")?
        .push("Var1")?
        .push("Var2")?
        .push("Var3")?
        .build();

    let structure = lib.declare_native_struct("Structure")?;

    let structure_interface = lib
        .define_interface("StructureInterface")?
        .callback("on_value")?
        .param("value", Type::StructRef(structure.clone()))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    let structure = lib
        .define_native_struct(&structure)?
        .add("boolean_value", Type::Bool)?
        .add("uint8_value", Type::Uint8)?
        .add("int8_value", Type::Sint8)?
        .add("uint16_value", Type::Uint16)?
        .add("int16_value", Type::Sint16)?
        .add("uint32_value", Type::Uint32)?
        .add("int32_value", Type::Sint32)?
        .add("uint64_value", Type::Uint64)?
        .add("int64_value", Type::Sint64)?
        .add("float_value", Type::Float)?
        .add("double_value", Type::Double)?
        .add("string_value", Type::String)?
        .add("structure_value", Type::Struct(other_structure))?
        .add("enum_value", Type::Enum(structure_enum))?
        .add("interface_value", Type::Interface(structure_interface))?
        .add(
            "duration_millis",
            Type::Duration(DurationMapping::Milliseconds),
        )?
        .add("duration_seconds", Type::Duration(DurationMapping::Seconds))?
        .add(
            "duration_seconds_float",
            Type::Duration(DurationMapping::SecondsFloat),
        )?
        .build();

    // Declare each echo function
    let struct_by_value_echo_func = lib
        .declare_native_function("struct_by_value_echo")?
        .param("value", Type::Struct(structure.clone()))?
        .return_type(ReturnType::Type(Type::Struct(structure.clone())))?
        .build()?;

    let struct_by_reference_echo_func = lib
        .declare_native_function("struct_by_reference_echo")?
        .param("value", Type::StructRef(structure.declaration()))?
        .return_type(ReturnType::Type(Type::Struct(structure.clone())))?
        .build()?;

    // Declare structs methods
    lib.define_struct(&structure)?
        .static_method("StructByValueEcho", &struct_by_value_echo_func)?
        .method("StructByReferenceEcho", &struct_by_reference_echo_func)?
        .build();

    Ok(())
}
