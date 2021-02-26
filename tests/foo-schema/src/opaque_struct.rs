use oo_bindgen::native_function::Type;
use oo_bindgen::native_struct::NativeStructType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let opaque_struct = lib.declare_native_struct("OpaqueStruct")?;

    lib.define_native_struct(&opaque_struct)?
        .with_type(NativeStructType::Opaque)
        .add("id", Type::Uint64, "64-bit id")?
        .doc("Opaque structure")?
        .build()?;

    Ok(())
}
