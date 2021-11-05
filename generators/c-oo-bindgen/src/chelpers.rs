use crate::ctype::CType;
use oo_bindgen::doc::Validated;
use oo_bindgen::interface::CallbackFunction;

pub(crate) fn callback_parameters(func: &CallbackFunction<Validated>) -> String {
    func.arguments
        .iter()
        .map(|arg| arg.arg_type.to_c_type())
        .chain(std::iter::once("void*".to_string()))
        .collect::<Vec<String>>()
        .join(", ")
}
