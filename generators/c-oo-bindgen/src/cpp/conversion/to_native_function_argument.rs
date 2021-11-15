use crate::cpp::conversion::{CoreCppType, ToNative};
use crate::cpp::formatting::FriendClass;
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::interface::InterfaceMode;

pub(crate) trait ToNativeFunctionArgument {
    fn to_native_function_argument(&self, expr: String) -> String;

    // some function arguments cannot be converted at the call site
    // and require a shadow parameter. The shadow parameter itself
    // map require some mapping at the call site.
    fn shadow_parameter_mapping(&self) -> Option<Box<dyn Fn(String) -> String>>;

    fn requires_shadow_parameter(&self) -> bool {
        self.shadow_parameter_mapping().is_some()
    }
}

impl ToNativeFunctionArgument for FunctionArgument {
    fn to_native_function_argument(&self, expr: String) -> String {
        match self {
            FunctionArgument::Basic(x) => x.to_native(expr),
            FunctionArgument::String(x) => x.to_native(expr),
            FunctionArgument::Collection(x) => {
                format!("{}({})", x.collection_class.core_cpp_type(), expr)
            }
            FunctionArgument::Struct(_) => {
                format!("::convert::to_native({})", expr)
            }
            FunctionArgument::StructRef(_) => {
                format!("::convert::to_native({})", expr)
            }
            FunctionArgument::ClassRef(x) => {
                format!("{}::get({})", x.friend_class(), expr)
            }
            FunctionArgument::Interface(x) => match x.mode {
                InterfaceMode::Synchronous => {
                    format!("::convert::to_native({})", expr)
                }
                InterfaceMode::Asynchronous => {
                    format!("::convert::to_native(std::move({}))", expr)
                }
                InterfaceMode::Future => {
                    format!("::convert::to_native(std::move({}))", expr)
                }
            },
        }
    }

    fn shadow_parameter_mapping(&self) -> Option<Box<dyn Fn(String) -> String>> {
        match self {
            FunctionArgument::Basic(_) => None,
            FunctionArgument::String(_) => None,
            FunctionArgument::Collection(x) => {
                let friend_class = x.collection_class.friend_class();
                Some(Box::new(move |e| format!("{}::get({})", friend_class, e)))
            }
            FunctionArgument::Struct(_) => None,
            FunctionArgument::StructRef(_) => Some(Box::new(|e| format!("&{}", e))),
            FunctionArgument::ClassRef(_) => None,
            FunctionArgument::Interface(_) => None,
        }
    }
}
