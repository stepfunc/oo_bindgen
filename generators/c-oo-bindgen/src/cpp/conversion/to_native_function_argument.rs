use crate::cpp::conversion::ToNative;
use crate::cpp::formatting::FriendClass;
use oo_bindgen::function::FunctionArgument;

pub(crate) trait ToNativeFunctionArgument {
    fn to_native_function_argument(&self, expr: String) -> String;

    // some function arguments cannot be converted at the call site
    // and require a shadow parameter. The shadow parameter itself
    // map require some mapping at the call site.
    fn shadow_parameter_mapping(&self) -> Option<fn(String) -> String>;

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
                format!("// Collection<{}>", x.collection_type.name)
            }
            FunctionArgument::Struct(x) => {
                format!("// Struct<{}>", x.name())
            }
            FunctionArgument::StructRef(_) => {
                format!("::convert::to_native({})", expr)
            }
            FunctionArgument::ClassRef(x) => {
                format!("{}::get({})", x.friend_class(), expr)
            }
            FunctionArgument::Interface(_) => {
                format!("::convert::to_native({})", expr)
            }
        }
    }

    fn shadow_parameter_mapping(&self) -> Option<fn(String) -> String> {
        match self {
            FunctionArgument::Basic(_) => None,
            FunctionArgument::String(_) => None,
            FunctionArgument::Collection(_) => Some(|x| x),
            FunctionArgument::Struct(_) => None,
            FunctionArgument::StructRef(_) => Some(|x| format!("&{}", x)),
            FunctionArgument::ClassRef(_) => None,
            FunctionArgument::Interface(_) => None,
        }
    }
}
