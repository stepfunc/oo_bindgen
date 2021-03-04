use oo_bindgen::native_function::{ReturnType, Type};
use oo_bindgen::{BindingError, LibraryBuilder};

pub(crate) fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let error_type = lib
        .define_error_type("MyError")?
        .add_error("BadPassword", "Wrong password!")?
        .add_error("NullArgument", "Provided argument was NULL")?
        .doc("Errors returned by the various functions")?
        .build()?;

    let my_class = lib.declare_class("ClassWithPassword")?;

    lib.declare_native_function("get_special_number")?
        .param("password", Type::String, "secret password")?
        .return_type(ReturnType::Type(Type::Uint32, "unlocked value".into()))?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a secret value")?
        .build()?;

    lib.declare_native_function("create_class_with_password")?
        .param("password", Type::String, "secret password")?
        .return_type(ReturnType::Type(
            Type::ClassRef(my_class.clone()),
            "allocated class".into(),
        ))?
        .fails_with(error_type.clone())?
        .doc("Use a password to allocate a class")?
        .build()?;

    lib.declare_native_function("get_special_value_from_class")?
        .param(
            "instance",
            Type::ClassRef(my_class.clone()),
            "class instance",
        )?
        .return_type(ReturnType::Type(Type::Uint32, "special value".into()))?
        .fails_with(error_type.clone())?
        .doc("extract a special value from the class instance")?
        .build()?;

    lib.declare_native_function("destroy_class_with_password")?
        .param("instance", Type::ClassRef(my_class), "class to destroy")?
        .return_nothing()?
        .fails_with(error_type)?
        .doc("Destroy an instance")?
        .build()?;

    Ok(())
}
