use oo_bindgen::native_function::{ReturnType, Type};
use oo_bindgen::{BindingError, LibraryBuilder};

pub(crate) fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let error_type = lib
        .define_error_type("MyError")?
        .add_error("BadPassword", "Wrong password!")?
        .doc("Errors returned by the various functions")?
        .build()?;

    let _func = lib
        .declare_native_function("get_special_number")?
        .param("password", Type::String, "secret password")?
        .return_type(ReturnType::Type(Type::Uint32, "unlocked value".into()))?
        .fails_with(error_type)?
        .doc("Use a password to retrieve a secret value")?
        .build()?;

    Ok(())
}
