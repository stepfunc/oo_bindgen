use oo_bindgen::*;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<ClassDeclarationHandle, BindingError> {
    // Forward declare the class
    let runtime_class = lib.declare_class("Runtime")?;

    // Declare the C-style structs
    let config_struct = lib.declare_native_struct("RuntimeConfig")?;
    let config_struct = lib.define_native_struct(&config_struct)?
        .add("num_core_threads", Type::Uint16)?
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

    let decode_log_level_enum = lib.define_native_enum("DecodeLogLevel")?
        .push("Nothing")?
        .push("Header")?
        .push("ObjectHeaders")?
        .push("ObjectValues")?
        .build();

    let reconnect_strategy = lib.declare_native_struct("ReconnectStrategy")?;
    let reconnect_strategy = lib.define_native_struct(&reconnect_strategy)?
        .add("min_delay", Type::Duration(DurationMapping::Milliseconds))?
        .add("max_delay", Type::Duration(DurationMapping::Milliseconds))?
        .build();

    let client_state_enum = lib.define_native_enum("ClientState")?
        .push("Connecting")?
        .push("Connected")?
        .push("WaitAfterFailedConnect")?
        .push("WaitAfterDisconnect")?
        .push("Shutdown")?
        .build();

    let client_state_listener = lib.define_interface("ClientStateListener")?
        .callback("on_change")?
            .param("state", Type::Enum(client_state_enum))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    let master_class = lib.declare_class("Master")?;

    let add_master_tcp_fn = lib.declare_native_function("runtime_add_master_tcp")?
        .param("runtime", Type::ClassRef(runtime_class.clone()))?
        .param("address", Type::Uint16)?
        .param("level", Type::Enum(decode_log_level_enum))?
        .param("strategy", Type::Struct(reconnect_strategy))?
        .param("response_timeout", Type::Duration(DurationMapping::Milliseconds))?
        .param("endpoint", Type::String)?
        .param("listener", Type::Interface(client_state_listener))?
        .return_type(ReturnType::Type(Type::ClassRef(master_class.clone())))?
        .build()?;

    // Declare the object-oriented class
    let _runtime_class = lib.define_class(&runtime_class)?
        .constructor(&new_fn)?
        .destructor(&destroy_fn)?
        .method("add_master_tcp", &add_master_tcp_fn)?
        .build();

    Ok(master_class)
}
