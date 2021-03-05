use oo_bindgen::native_function::{ReturnType, Type};
use oo_bindgen::{BindingError, LibraryBuilder};

pub(crate) fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let error_type = lib
        .define_error_type("MyError", "MyException")?
        .add_error("BadPassword", "Wrong password!")?
        .add_error("NullArgument", "Provided argument was NULL")?
        .doc("Errors returned by the various functions")?
        .build()?;

    let my_class = lib.declare_class("ClassWithPassword")?;

    let get_special_number_fb = lib.declare_native_function("get_special_number")?
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
        .param("instance", Type::ClassRef(my_class.clone()), "class to destroy")?
        .return_nothing()?
        .doc("Destroy an instance")?
        .build()?;

    lib
        .define_class(&my_class)?
        .doc("A very special class")?
        .static_method("GetSpecialValue", &get_special_number_fb)?
        .build()?;

    Ok(())
}
