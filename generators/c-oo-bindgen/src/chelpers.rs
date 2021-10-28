use crate::ctype::CType;
use oo_bindgen::interface::CallbackFunction;
use oo_bindgen::Library;

pub(crate) fn callback_parameters(lib: &Library, func: &CallbackFunction) -> String {
    func.arguments
        .iter()
        .map(|arg| arg.arg_type.to_c_type(&lib.c_ffi_prefix))
        .chain(std::iter::once("void*".to_string()))
        .collect::<Vec<String>>()
        .join(", ")
}
