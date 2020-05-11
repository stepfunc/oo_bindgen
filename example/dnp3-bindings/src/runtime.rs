use oo_bindgen::*;
use oo_bindgen::class::ClassHandle;
use oo_bindgen::native_function::*;

pub fn define_runtime(lib: &mut LibraryBuilder) -> Result<ClassHandle, BindingError> {
    // Forward declare the class
    let runtime_class = lib.declare_class("Runtime")?;

    // Declare the C-style structs
    let config_struct = lib.declare_native_struct("RuntimeConfig")?;
    let config_struct = lib.define_native_struct(&config_struct)?
        .add("num_core_threads", Type::Uint16)?
        .build();

    // Define the C-style enums
    let _decode_log_level_enum = lib.define_native_enum("DecodeLogLevel")?
        .push("Nothing")?
        .push("Header")?
        .push("ObjectHeaders")?
        .push("ObjectValues")?
        .build();

    // Declare the native functions
    let new_fn = lib.declare_native_function("runtime_new")?
        .param("config", Type::StructRef(config_struct.declaration()))?
        .return_type(ReturnType::Type(Type::ClassRef(runtime_class.clone())))?
        .build()?;

    let destroy_fn = lib.declare_native_function("runtime_destroy")?
        .param("runtime", Type::ClassRef(runtime_class.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    // Declare the object-oriented class
    let runtime_class = lib.define_class(&runtime_class)?
        .constructor(&new_fn)?
        .destructor(&destroy_fn)?
        .build();

    Ok(runtime_class)
}
