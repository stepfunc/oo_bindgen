use oo_bindgen::model::*;

/// Some types have a C -> C++ conversion that is context independent
pub(crate) trait ToCpp {
    fn to_cpp(&self, expr: String) -> String;
}

impl ToCpp for DurationType {
    fn to_cpp(&self, expr: String) -> String {
        match self {
            DurationType::Milliseconds => format!("::convert::from_milli_sec_u64({})", expr),
            DurationType::Seconds => format!("::convert::from_sec_u64({})", expr),
        }
    }
}

impl<D> ToCpp for Handle<Enum<D>>
where
    D: DocReference,
{
    fn to_cpp(&self, expr: String) -> String {
        format!("::convert::to_cpp({})", expr)
    }
}

impl ToCpp for BasicType {
    fn to_cpp(&self, expr: String) -> String {
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
            Self::Float32 => expr,
            Self::Double64 => expr,
            Self::Duration(x) => x.to_cpp(expr),
            Self::Enum(x) => x.to_cpp(expr),
        }
    }
}

impl ToCpp for StringType {
    fn to_cpp(&self, expr: String) -> String {
        format!("std::string({})", expr)
    }
}

impl ToCpp for ClassDeclarationHandle {
    fn to_cpp(&self, expr: String) -> String {
        format!("::convert::to_cpp({})", expr)
    }
}
