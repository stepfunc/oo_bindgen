use oo_bindgen::native_function::{BasicType, ReturnType, Type};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let opaque_struct = lib.declare_native_struct("OpaqueStruct")?;

    let opaque_struct = lib
        .define_native_struct(&opaque_struct)?
        .make_opaque()
        .add("id", Type::Basic(BasicType::Uint64), "64-bit id")?
        .doc("Opaque structure")?
        .build()?;

    let get_id_fn = lib
        .declare_native_function("opaque_struct_get_id")?
        .param(
            "value",
            Type::StructRef(opaque_struct.declaration.clone()),
            "struct value",
        )?
        .return_type(ReturnType::Type(
            BasicType::Uint64.into(),
            "value of id field".into(),
        ))?
        .doc("Get the id field of the struct")?
        .build()?;

    let opaque_struct_magic_init_fn = lib
        .declare_native_function("opaque_struct_magic_init")?
        .return_type(ReturnType::Type(
            Type::Struct(opaque_struct.clone()),
            "initialized value".into(),
        ))?
        .doc("Create an OpaqueStruct initialized with a magic id")?
        .build()?;

    lib.define_struct(&opaque_struct)?
        .method("GetId", &get_id_fn)?
        .static_method("CreateMagicValue", &opaque_struct_magic_init_fn)?
        .build();

    Ok(())
}
