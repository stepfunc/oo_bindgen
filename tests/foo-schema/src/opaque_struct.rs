use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let opaque_struct = lib.declare_struct("OpaqueStruct")?;

    let opaque_struct = lib
        .define_function_return_struct(&opaque_struct)?
        .make_opaque()
        .add("id", BasicType::Uint64, "64-bit id")?
        .doc("Opaque structure")?
        .end_fields()?
        .build()?;

    let get_id_fn = lib
        .define_function("opaque_struct_get_id")
        .param("value", opaque_struct.declaration.clone(), "struct value")?
        .returns(BasicType::Uint64, "value of id field")?
        .doc("Get the id field of the struct")?
        .build()?;

    let opaque_struct_magic_init_fn = lib
        .define_function("opaque_struct_magic_init")
        .returns(opaque_struct, "initialized value")?
        .doc("Create an OpaqueStruct initialized with a magic id")?
        .build()?;

    lib.define_static_class("OpaqueStructHelpers")
        .doc("Helpers for manipulating instances of {struct:OpaqueStruct}")?
        .static_method("get_id", &get_id_fn)?
        .static_method("create_magic_value", &opaque_struct_magic_init_fn)?
        .build()?;

    Ok(())
}
