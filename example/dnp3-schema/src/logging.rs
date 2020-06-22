use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<NativeEnumHandle, BindingError> {
    let log_level_enum = lib
        .define_native_enum("LogLevel")?
        .push("Error")?
        .push("Warn")?
        .push("Info")?
        .push("Debug")?
        .push("Trace")?
        .build();

    let log_callback_interface = lib
        .define_interface("Logger")?
        .callback("on_message")?
        .param("level", Type::Enum(log_level_enum.clone()))?
        .param("message", Type::String)?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    let set_callback_fn = lib
        .declare_native_function("logging_set_callback")?
        .param("handler", Type::Interface(log_callback_interface))?
        .return_type(ReturnType::Void)?
        .build()?;

    let set_log_level_fn = lib
        .declare_native_function("logging_set_log_level")?
        .param("level", Type::Enum(log_level_enum))?
        .return_type(ReturnType::Void)?
        .build()?;

    let logging_class = lib.declare_class("Logging")?;
    let _logging_class = lib
        .define_class(&logging_class)?
        .static_method("SetHandler", &set_callback_fn)?
        .static_method("SetLogLevel", &set_log_level_fn)?
        .build();

    let decode_log_level_enum = lib
        .define_native_enum("DecodeLogLevel")?
        .push("Nothing")?
        .push("Header")?
        .push("ObjectHeaders")?
        .push("ObjectValues")?
        .build();

    Ok(decode_log_level_enum)
}
