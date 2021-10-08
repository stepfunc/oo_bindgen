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

/* TODO
pub(crate) fn callback_parameters_with_var_names(lib: &Library, func: &CallbackFunction) -> String {
    func.arguments
        .iter()
        .map(|arg| {
            format!(
                "{} {}",
                arg.arg_type.to_c_type(&lib.c_ffi_prefix),
                arg.name.to_snake_case()
            )
        })
        .chain(std::iter::once(format!("void* {}", CTX_VARIABLE_NAME)))
        .collect::<Vec<String>>()
        .join(", ")
}
*/
