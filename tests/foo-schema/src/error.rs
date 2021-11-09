use oo_bindgen::error_type::ExceptionType;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::{BackTraced, LibraryBuilder};

pub(crate) fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
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
        .define_function("get_special_value")?
        .param("password", StringType, "secret password")?
        .returns(BasicType::U32, "unlocked value")?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a secret value")?
        .build_static_with_same_name()?;

    let get_struct_fn = lib
        .define_function("validate_password")?
        .param("password", StringType, "secret password")?
        .returns_nothing()?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a struct")?
        .build_static_with_same_name()?;

    let echo_password_fn = lib
        .define_function("echo_password")?
        .param("password", StringType, "secret password")?
        .returns(StringType, "The password")?
        .fails_with(error_type.clone())?
        .doc("Use a password and echoes it if it's valid")?
        .build_static_with_same_name()?;

    let constructor = lib
        .define_constructor(my_class.clone())?
        .param("password", StringType, "secret password")?
        .fails_with(error_type.clone())?
        .doc("Use a password to allocate a class")?
        .build()?;

    let get_special_value = lib
        .define_method("get_special_value", my_class.clone())?
        .returns(BasicType::U32, "special value")?
        .fails_with(error_type)?
        .doc("extract a special value from the class instance")?
        .build()?;

    let destructor = lib.define_destructor(my_class.clone(), "Destroy an instance")?;

    lib.define_class(&my_class)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(get_special_value)?
        .static_method(get_special_number_fb)?
        .static_method(get_struct_fn)?
        .static_method(echo_password_fn)?
        .doc("A very special class")?
        .build()?;

    Ok(())
}
