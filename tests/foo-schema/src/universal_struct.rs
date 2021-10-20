use oo_bindgen::structs::{ConstructorDefault, ConstructorType, FieldName, UniversalStructHandle};
use oo_bindgen::types::{BasicType, DurationType};
use oo_bindgen::{BindResult, LibraryBuilder};
use std::time::Duration;

fn define_inner_struct(lib: &mut LibraryBuilder) -> BindResult<UniversalStructHandle> {
    let inner = lib.declare_struct("UniversalInnerStruct")?;

    let value_field = FieldName::new("value");
    lib.define_universal_struct(&inner)?
        .doc("Simple universal struct")?
        .add(value_field.clone(), BasicType::Sint32, "integer value")?
        .end_fields()?
        .new_constructor(
            "init",
            ConstructorType::Normal,
            "initializes {struct:UniversalInnerStruct} to default values",
        )?
        .add(&value_field, ConstructorDefault::Sint32(-42))?
        .end_constructor()?
        .build()
}

fn define_outer_struct(lib: &mut LibraryBuilder) -> BindResult<UniversalStructHandle> {
    let inner_struct = define_inner_struct(lib)?;

    let inner_field = FieldName::new("inner");
    let delay_field = FieldName::new("delay");

    let outer_struct = lib.declare_struct("UniversalOuterStruct")?;
    let outer_struct = lib
        .define_universal_struct(&outer_struct)?
        .doc("Simple universal struct with an inner structure")?
        .add(inner_field.clone(), inner_struct, "An inner structure")?
        .add(
            delay_field.clone(),
            DurationType::Seconds,
            "A duration value",
        )?
        .end_fields()?
        // -- constructor --
        .new_constructor(
            "init",
            ConstructorType::Normal,
            "Construct a {struct:UniversalOuterStruct} initialized to default values",
        )?
        .add(&inner_field, ConstructorDefault::DefaultStruct)?
        .add(
            &delay_field,
            ConstructorDefault::Duration(Duration::from_secs(5)),
        )?
        .end_constructor()?
        // -- end constructor --
        // -- constructor --
        .new_constructor(
            "create_default_with_time",
            ConstructorType::Static,
            "Construct a {struct:UniversalOuterStruct} with a default inner value and the specified time",
        )?
        .add(&inner_field, ConstructorDefault::DefaultStruct)?
        .end_constructor()?
        // -- end constructor --
        .build()?;

    Ok(outer_struct)
}

pub fn define(lib: &mut LibraryBuilder) -> BindResult<()> {
    let handle = define_outer_struct(lib)?;

    let interface = lib
        .define_interface("UniversalInterface", "Interface that uses universal types")
        .begin_callback(
            "on_value",
            "callback that receives and returns a universal struct",
        )?
        .param("value", handle.clone(), "Universal struct to modify")?
        .returns(handle.clone(), "Universal struct to return")?
        .end_callback()?
        .build()?;

    lib.define_function("invoke_universal_interface")
        .doc("invokes a universal interface")?
        .param("value", handle.clone(), "value to apply to the interface")?
        .param(
            "callback",
            interface,
            "interface on which to apply the value",
        )?
        .returns(handle, "return value")?
        .build()?;

    Ok(())
}
