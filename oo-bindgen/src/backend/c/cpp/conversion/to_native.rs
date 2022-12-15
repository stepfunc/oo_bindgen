use crate::model::*;

/// Some types have a C++ -> C conversion that is context independent
pub(crate) trait ToNative {
    fn to_native(&self, expr: String) -> String;
}

impl ToNative for DurationType {
    fn to_native(&self, expr: String) -> String {
        match self {
            DurationType::Milliseconds => format!("::convert::to_milli_sec_u64({expr})"),
            DurationType::Seconds => format!("::convert::to_sec_u64({expr})"),
        }
    }
}

impl ToNative for Handle<Enum<Unvalidated>> {
    fn to_native(&self, expr: String) -> String {
        format!("::convert::to_native({expr})")
    }
}

impl ToNative for Primitive {
    fn to_native(&self, expr: String) -> String {
        match self {
            Self::Bool => expr,
            Self::U8 => expr,
            Self::S8 => expr,
            Self::U16 => expr,
            Self::S16 => expr,
            Self::U32 => expr,
            Self::S32 => expr,
            Self::U64 => expr,
            Self::S64 => expr,
            Self::Float => expr,
            Self::Double => expr,
        }
    }
}

impl ToNative for BasicType {
    fn to_native(&self, expr: String) -> String {
        match self {
            Self::Primitive(x) => x.to_native(expr),
            Self::Duration(t) => t.to_native(expr),
            Self::Enum(t) => t.to_native(expr),
        }
    }
}

impl ToNative for StringType {
    fn to_native(&self, expr: String) -> String {
        format!("{expr}.c_str()")
    }
}
