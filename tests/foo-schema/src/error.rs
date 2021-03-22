use oo_bindgen::error_type::ExceptionType;
use oo_bindgen::native_function::{ReturnType, Type};
use oo_bindgen::native_struct::NativeStructHandle;
use oo_bindgen::{BindingError, LibraryBuilder};

pub(crate) fn define(
    lib: &mut LibraryBuilder,
    structure: NativeStructHandle,
) -> Result<(), BindingError> {
    let error_type = lib
        .define_error_type("MyError", "MyException", ExceptionType::UncheckedException)?
        .add_error("BadPassword", "Wrong password!")?
        .add_error("NullArgument", "Provided argument was NULL")?
        .doc("Errors returned by the various functions")?
        .build()?;

    let my_class = lib.declare_class("ClassWithPassword")?;

    let get_special_number_fb = lib
        .declare_native_function("get_special_number")?
        .param("password", Type::String, "secret password")?
        .return_type(ReturnType::new(Type::Uint32, "unlocked value"))?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a secret value")?
        .build()?;

    let get_struct_fn = lib
        .declare_native_function("get_struct")?
        .param("password", Type::String, "secret password")?
        .return_type(ReturnType::new(Type::Struct(structure), "A struct"))?
        .fails_with(error_type.clone())?
        .doc("Use a password to retrieve a struct")?
        .build()?;

    let echo_password_fn = lib
        .declare_native_function("echo_password")?
        .param("password", Type::String, "secret password")?
        .return_type(ReturnType::new(Type::String, "The password"))?
        .fails_with(error_type.clone())?
        .doc("Use a password and echoes it if it's valid")?
        .build()?;

    let constructor_fn = lib
        .declare_native_function("create_class_with_password")?
        .param("password", Type::String, "secret password")?
        .return_type(ReturnType::Type(
            Type::ClassRef(my_class.clone()),
            "allocated class".into(),
        ))?
        .fails_with(error_type.clone())?
        .doc("Use a password to allocate a class")?
        .build()?;

    let get_special_value_fn = lib
        .declare_native_function("get_special_value_from_class")?
        .param(
            "instance",
            Type::ClassRef(my_class.clone()),
            "class instance",
        )?
        .return_type(ReturnType::Type(Type::Uint32, "special value".into()))?
        .fails_with(error_type)?
        .doc("extract a special value from the class instance")?
        .build()?;

    let destructor_fn = lib
        .declare_native_function("destroy_class_with_password")?
        .param(
            "instance",
            Type::ClassRef(my_class.clone()),
            "class to destroy",
        )?
        .return_nothing()?
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
