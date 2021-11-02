use oo_bindgen::error_type::ExceptionType;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::{BindingError, LibraryBuilder};

pub(crate) fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let error_type = lib
        .define_error_type(
            "my_error",
            "my_exception",
            ExceptionType::UncheckedException,
        )?
        .add_error("bad_password", "Wrong password!")?
        .add_error("null_argument", "Provided argument was NULL")?
        .doc("Errors returned by the various functions")?
        .build()?;

    let my_class = lib.declare_class("class_with_password")?;

    let get_special_number_fb = lib
        .define_function("get_special_number")?
        .param("password", StringType, "secret password")?
        .returns(BasicType::U32, "unlocked value")?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a secret value")?
        .build()?;

    let get_struct_fn = lib
        .define_function("validate_password")?
        .param("password", StringType, "secret password")?
        .returns_nothing()?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a struct")?
        .build()?;

    let echo_password_fn = lib
        .define_function("echo_password")?
        .param("password", StringType, "secret password")?
        .returns(StringType, "The password")?
        .fails_with(error_type.clone())?
        .doc("Use a password and echoes it if it's valid")?
        .build()?;

    let constructor_fn = lib
        .define_function("create_class_with_password")?
        .param("password", StringType, "secret password")?
        .returns(my_class.clone(), "allocated class")?
        .fails_with(error_type.clone())?
        .doc("Use a password to allocate a class")?
        .build()?;

    let get_special_value_fn = lib
        .define_function("get_special_value_from_class")?
        .param("instance", my_class.clone(), "class instance")?
        .returns(BasicType::U32, "special value")?
        .fails_with(error_type)?
        .doc("extract a special value from the class instance")?
        .build()?;

    let destructor_fn = lib
        .define_function("destroy_class_with_password")?
        .param("instance", my_class.clone(), "class to destroy")?
        .returns_nothing()?
        .doc("Destroy an instance")?
        .build()?;

    lib.define_class(&my_class)?
        .constructor(&constructor_fn)?
        .destructor(&destructor_fn)?
        .method("get_special_value_from_instance", &get_special_value_fn)?
        .static_method("get_special_value", &get_special_number_fb)?
        .static_method("validate_password", &get_struct_fn)?
        .static_method("echo_password", &echo_password_fn)?
        .doc("A very special class")?
        .build()?;

    Ok(())
}
