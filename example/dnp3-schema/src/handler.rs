use oo_bindgen::*;
use oo_bindgen::native_function::*;
use oo_bindgen::interface::InterfaceHandle;

pub fn define(lib: &mut LibraryBuilder) -> Result<InterfaceHandle, BindingError> {
    let control = lib.declare_native_struct("Control")?;
    let control = lib.define_native_struct(&control)?
        .add("fir", Type::Bool)?
        .add("fin", Type::Bool)?
        .add("con", Type::Bool)?
        .add("uns", Type::Bool)?
        .add("seq", Type::Uint8)?
        .build();

    let response_function = lib.define_native_enum("ResponseFunction")?
        .push("Response")?
        .push("UnsolicitedResponse")?
        .build();

    // TODO: add struct methods to isolate bits
    let iin1 = lib.declare_native_struct("IIN1")?;
    let iin1 = lib.define_native_struct(&iin1)?
        .add("value", Type::Uint8)?
        .build();

    let iin2 = lib.declare_native_struct("IIN2")?;
    let iin2 = lib.define_native_struct(&iin2)?
        .add("value", Type::Uint8)?
        .build();

    let iin = lib.declare_native_struct("IIN")?;
    let iin = lib.define_native_struct(&iin)?
        .add("iin1", Type::Struct(iin1.clone()))?
        .add("iin2", Type::Struct(iin2.clone()))?
        .build();

    let response_header = lib.declare_native_struct("ResponseHeader")?;
    let response_header = lib.define_native_struct(&response_header)?
        .add("control", Type::Struct(control.clone()))?
        .add("func", Type::Enum(response_function.clone()))?
        .add("iin", Type::Struct(iin.clone()))?
        .build();

    let read_handler_interface = lib.define_interface("ReadHandler")?
        .callback("begin_fragment")?
            .param("header", Type::Struct(response_header.clone()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("end_fragment")?
            .param("header", Type::Struct(response_header.clone()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    Ok(read_handler_interface)
}
