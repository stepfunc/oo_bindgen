use oo_bindgen::structs::{ConstructorDefault, ConstructorName, FieldName, UniversalStructHandle};
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
            ConstructorName::normal("init"),
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
        .new_constructor(
            ConstructorName::Normal("init".to_string()),
            "Construct a {struct:UniversalOuterStruct} initialized to default values",
        )?
        .add(&inner_field, ConstructorDefault::DefaultStruct)?
        .add(
            &delay_field,
            ConstructorDefault::Duration(Duration::from_secs(5)),
        )?
        .end_constructor()?
        .build()?;

    Ok(outer_struct)
}

pub fn define_increment_function(
    lib: &mut LibraryBuilder,
    handle: &UniversalStructHandle,
) -> BindResult<()> {
    let increment_fn = lib
        .define_function("increment_universal_struct")
        .doc("increments values in a universal structure and returns the modified value")?
        .param("value", handle.clone(), "value to increment")?
        .returns(handle.clone(), "Incremented value")?
        .build()?;

    lib.define_static_class("UniversalHelperClass")
        .doc("Provides static methods for OO languages to test universal structs")?
        .static_method("Modify", &increment_fn)?
        .build()?;

    Ok(())
}

pub fn define(lib: &mut LibraryBuilder) -> BindResult<()> {
    let outer_struct = define_outer_struct(lib)?;

    define_increment_function(lib, &outer_struct)?;

    Ok(())
}
