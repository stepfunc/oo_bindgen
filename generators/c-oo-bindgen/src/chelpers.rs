use oo_bindgen::callback::{CallbackFunction, CallbackParameter};
use crate::CFormatting;
use oo_bindgen::Library;
use heck::SnakeCase;

pub(crate) fn callback_parameters(lib: &Library, func: &CallbackFunction) -> String {
    func.parameters
        .iter()
        .map(|param| match param {
            CallbackParameter::Arg(_) => "void*".to_string(),
            CallbackParameter::Parameter(param) => {
                param.param_type.to_c_type(&lib.c_ffi_prefix)
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) fn callback_parameters_with_var_names(lib: &Library, func: &CallbackFunction) -> String {
    func.parameters
        .iter()
        .map(|param| match param {
            CallbackParameter::Arg(name) => format!("void* {}", name),
            CallbackParameter::Parameter(param) => {
                format!("{} {}", param.param_type.to_c_type(&lib.c_ffi_prefix), param.name.to_snake_case())
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}