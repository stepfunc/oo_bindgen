use oo_bindgen::error_type::ExceptionType;
use oo_bindgen::function_struct::FStructHandle;
use oo_bindgen::types::{BasicType, STRING_TYPE};
use oo_bindgen::{BindingError, LibraryBuilder};

pub(crate) fn define(
    lib: &mut LibraryBuilder,
    structure: FStructHandle,
) -> Result<(), BindingError> {
    let error_type = lib
        .define_error_type("MyError", "MyException", ExceptionType::UncheckedException)?
        .add_error("BadPassword", "Wrong password!")?
        .add_error("NullArgument", "Provided argument was NULL")?
        .doc("Errors returned by the various functions")?
        .build()?;

    let my_class = lib.declare_class("ClassWithPassword")?;

    let get_special_number_fb = lib
        .define_function("get_special_number")?
        .param("password", STRING_TYPE, "secret password")?
        .returns(BasicType::Uint32, "unlocked value")?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a secret value")?
        .build()?;

    let get_struct_fn = lib
        .define_function("get_struct")?
        .param("password", STRING_TYPE, "secret password")?
        .returns(structure, "A struct")?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a struct")?
        .build()?;

    let echo_password_fn = lib
        .define_function("echo_password")?
        .param("password", STRING_TYPE, "secret password")?
        .returns(STRING_TYPE, "The password")?
        .fails_with(error_type.clone())?
        .doc("Use a password and echoes it if it's valid")?
        .build()?;

    let constructor_fn = lib
        .define_function("create_class_with_password")?
        .param("password", STRING_TYPE, "secret password")?
        .returns(my_class.clone(), "allocated class")?
        .fails_with(error_type.clone())?
        .doc("Use a password to allocate a class")?
        .build()?;

    let get_special_value_fn = lib
        .define_function("get_special_value_from_class")?
        .param("instance", my_class.clone(), "class instance")?
        .returns(BasicType::Uint32, "special value")?
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
        .method("GetSpecialValueFromInstance", &get_special_value_fn)?
        .static_method("GetSpecialValue", &get_special_number_fb)?
        .static_method("GetStruct", &get_struct_fn)?
        .static_method("EchoPassword", &echo_password_fn)?
        .doc("A very special class")?
        .build()?;

    Ok(())
}
