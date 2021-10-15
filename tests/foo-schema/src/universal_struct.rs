use oo_bindgen::{BindResult, LibraryBuilder};
use oo_bindgen::structs::univeral_struct::UniversalStructHandle;
use oo_bindgen::structs::common::FieldName;
use oo_bindgen::types::{BasicType, DurationType};

fn define_inner_struct(lib: &mut LibraryBuilder) -> BindResult<UniversalStructHandle> {
    let inner = lib.declare_struct("UniversalInnerStruct")?;

    let value = FieldName::new("value");
    lib.define_ustruct(&inner)?
        .doc("Simple universal struct")?
        .add(value, BasicType::Sint32, "integer value")?
        .end_fields()?
        .build()
}

pub fn define(lib: &mut LibraryBuilder) -> BindResult<()> {
    let inner_struct = define_inner_struct(lib)?;

    let delay = FieldName::new("delay");
    let outer_struct = lib.declare_struct("UniversalOuterStruct")?;
    let outer_struct = lib
        .define_ustruct(&outer_struct)?
        .doc("Simple universal struct with an inner structure")?
        .add("inner", inner_struct, "An inner structure")?
        .add(delay, DurationType::Seconds, "A duration value")?
        .end_fields()?
        .build()?;

    let _modify_fn = lib.define_function("modify_universal_struct")
        .doc("modifies a universal structure and returns the modified value")?
        .param("value", outer_struct.clone(), "value to return modified")?
        .returns(outer_struct, "Modified value")?
        .build()?;

    // TODO - a static helper class for OO langs

    Ok(())
}