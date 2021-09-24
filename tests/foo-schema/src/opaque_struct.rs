use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let opaque_struct = lib.declare_native_struct("OpaqueStruct")?;

    let opaque_struct = lib
        .define_native_struct(&opaque_struct)?
        .make_opaque()
        .add("id", BasicType::Uint64, "64-bit id")?
        .doc("Opaque structure")?
        .build()?;

    let get_id_fn = lib
        .declare_native_function("opaque_struct_get_id")?
        .param("value", opaque_struct.declaration.clone(), "struct value")?
        .returns(BasicType::Uint64, "value of id field")?
        .doc("Get the id field of the struct")?
        .build()?;

    let opaque_struct_magic_init_fn = lib
        .declare_native_function("opaque_struct_magic_init")?
        .returns(opaque_struct.clone(), "initialized value")?
        .doc("Create an OpaqueStruct initialized with a magic id")?
        .build()?;

    lib.define_struct(&opaque_struct)?
        .method("GetId", &get_id_fn)?
        .static_method("CreateMagicValue", &opaque_struct_magic_init_fn)?
        .build();

    Ok(())
}
