use crate::cpp::conversion::{ToNative, ToNativeStructField};
use oo_bindgen::interface::CallbackReturnValue;

pub(crate) trait ToNativeCallbackReturnValue {
    fn to_native_callback_return_value(&self, expr: String) -> String;
}

impl ToNativeCallbackReturnValue for CallbackReturnValue {
    fn to_native_callback_return_value(&self, expr: String) -> String {
        match self {
            CallbackReturnValue::Basic(x) => x.to_native(expr),
            CallbackReturnValue::Struct(x) => x.to_native_struct_field(expr),
        }
    }
}
