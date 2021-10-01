use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let opaque_struct = lib.declare_native_struct("OpaqueStruct")?;

    lib.define_native_struct(&opaque_struct)?
        .make_opaque()
        .add("id", BasicType::Uint64, "64-bit id")?
        .doc("Opaque structure")?
        .build()?;

    Ok(())
}
