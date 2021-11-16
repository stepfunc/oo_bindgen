use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let opaque_struct = lib.declare_universal_struct("opaque_struct")?;

    let get_id_fn = lib
        .define_function("opaque_struct_get_id")?
        .param("value", opaque_struct.clone(), "struct value")?
        .returns(Primitive::U64, "value of id field")?
        .doc("Get the id field of the struct")?
        .build_static("get_id")?;

    let opaque_struct = lib
        .define_opaque_struct(opaque_struct)?
        .add("id", Primitive::U64, "64-bit id")?
        .doc("Opaque structure")?
        .end_fields()?
        .build()?;

    let opaque_struct_magic_init_fn = lib
        .define_function("opaque_struct_magic_init")?
        .returns(opaque_struct, "initialized value")?
        .doc("Create an OpaqueStruct initialized with a magic id")?
        .build_static("create_magic_value")?;

    lib.define_static_class("opaque_struct_helpers")?
        .doc("Helpers for manipulating instances of {struct:opaque_struct}")?
        .static_method(get_id_fn)?
        .static_method(opaque_struct_magic_init_fn)?
        .build()?;

    Ok(())
}
