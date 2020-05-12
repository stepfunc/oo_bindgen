use oo_bindgen::*;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let other_structure = lib.declare_native_struct("OtherStructure")?;
    let other_structure = lib.define_native_struct(&other_structure)?
        .add("test", Type::Uint16)?
        .build();

    let structure_enum = lib.define_native_enum("StructureEnum")?
        .push("Var1")?
        .push("Var2")?
        .push("Var3")?
        .build();

    let structure = lib.declare_native_struct("Structure")?;
    let structure = lib.define_native_struct(&structure)?
        .add("booleanValue", Type::Bool)?
        .add("uint8Value", Type::Uint8)?
        .add("int8Value", Type::Sint8)?
        .add("uint16Value", Type::Uint16)?
        .add("int16Value", Type::Sint16)?
        .add("uint32Value", Type::Uint32)?
        .add("int32Value", Type::Sint32)?
        .add("uint64Value", Type::Uint64)?
        .add("int64Value", Type::Sint64)?
        .add("structureValue", Type::Struct(other_structure.clone()))?
        .add("enumValue", Type::Enum(structure_enum.clone()))?
        .add("durationMillis", Type::Duration(DurationMapping::Milliseconds))?
        .add("durationSeconds", Type::Duration(DurationMapping::Seconds))?
        .add("durationSecondsFloat", Type::Duration(DurationMapping::SecondsFloat))?
        .build();

    // Declare each echo function
    let struct_by_value_echo_func = lib.declare_native_function("struct_by_value_echo")?
        .param("value", Type::Struct(structure.clone()))?
        .return_type(ReturnType::Type(Type::Struct(structure.clone())))?
        .build()?;

    let struct_by_reference_echo_func = lib.declare_native_function("struct_by_reference_echo")?
        .param("value", Type::StructRef(structure.declaration()))?
        .return_type(ReturnType::Type(Type::Struct(structure.clone())))?
        .build()?;

    // Declare static class
    let class = lib.declare_class("StructEchoFunctions")?;
    lib.define_class(&class)?
        .static_method("StructByValueEcho", &struct_by_value_echo_func)?
        .static_method("StructByReferenceEcho", &struct_by_reference_echo_func)?
        .build();

    Ok(())
}
