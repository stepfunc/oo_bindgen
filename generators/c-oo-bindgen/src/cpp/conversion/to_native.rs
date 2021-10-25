use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::interface::InterfaceHandle;
use oo_bindgen::structs::{
    FunctionArgStructField, FunctionArgStructHandle, UniversalStructField, UniversalStructHandle,
};
use oo_bindgen::types::{BasicType, DurationType, StringType};
use oo_bindgen::UniversalOr;

pub(crate) trait ToNativeStructField {
    /// takes a C++ type and converts it to a native value of the same type
    fn to_native_struct_field(&self, expr: String) -> String;

    /// does the type require a move operation that modifies the C++ type
    fn requires_move(&self) -> bool;
}

impl ToNativeStructField for DurationType {
    fn to_native_struct_field(&self, expr: String) -> String {
        match self {
            DurationType::Milliseconds => format!("to_sec_u64({})", expr),
            DurationType::Seconds => format!("to_milli_sec_u64({})", expr),
        }
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl ToNativeStructField for EnumHandle {
    fn to_native_struct_field(&self, expr: String) -> String {
        format!("enum_to_native({})", expr)
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl ToNativeStructField for BasicType {
    fn to_native_struct_field(&self, expr: String) -> String {
        match self {
            BasicType::Bool => expr,
            BasicType::U8 => expr,
            BasicType::S8 => expr,
            BasicType::U16 => expr,
            BasicType::S16 => expr,
            BasicType::U32 => expr,
            BasicType::S32 => expr,
            BasicType::U64 => expr,
            BasicType::S64 => expr,
            BasicType::Float32 => expr,
            BasicType::Double64 => expr,
            BasicType::Duration(t) => t.to_native_struct_field(expr),
            BasicType::Enum(t) => t.to_native_struct_field(expr),
        }
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl ToNativeStructField for StringType {
    fn to_native_struct_field(&self, expr: String) -> String {
        format!("{}.c_str()", expr)
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl ToNativeStructField for UniversalStructHandle {
    fn to_native_struct_field(&self, expr: String) -> String {
        format!("to_native({})", expr)
    }

    fn requires_move(&self) -> bool {
        self.fields.iter().any(|x| x.field_type.requires_move())
    }
}

impl ToNativeStructField for FunctionArgStructHandle {
    fn to_native_struct_field(&self, expr: String) -> String {
        format!("to_native({})", expr)
    }

    fn requires_move(&self) -> bool {
        self.fields.iter().any(|x| x.field_type.requires_move())
    }
}

impl ToNativeStructField for InterfaceHandle {
    fn to_native_struct_field(&self, expr: String) -> String {
        format!("to_native(std::move({}))", expr)
    }

    fn requires_move(&self) -> bool {
        true
    }
}

impl ToNativeStructField for FunctionArgStructField {
    fn to_native_struct_field(&self, expr: String) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.to_native_struct_field(expr),
            FunctionArgStructField::String(x) => x.to_native_struct_field(expr),
            FunctionArgStructField::Interface(x) => x.to_native_struct_field(expr),
            FunctionArgStructField::Collection(_) => {
                unimplemented!()
            }
            FunctionArgStructField::Struct(x) => match x {
                UniversalOr::Specific(x) => x.to_native_struct_field(expr),
                UniversalOr::Universal(x) => x.to_native_struct_field(expr),
            },
        }
    }

    fn requires_move(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(x) => x.requires_move(),
            FunctionArgStructField::String(x) => x.requires_move(),
            FunctionArgStructField::Interface(x) => x.requires_move(),
            FunctionArgStructField::Collection(_x) => unimplemented!(),
            FunctionArgStructField::Struct(x) => match x {
                UniversalOr::Specific(x) => x.requires_move(),
                UniversalOr::Universal(x) => x.requires_move(),
            },
        }
    }
}

impl ToNativeStructField for UniversalStructField {
    fn to_native_struct_field(&self, expr: String) -> String {
        match self {
            UniversalStructField::Basic(x) => x.to_native_struct_field(expr),
            UniversalStructField::Struct(x) => x.to_native_struct_field(expr),
        }
    }

    fn requires_move(&self) -> bool {
        match self {
            UniversalStructField::Basic(x) => x.requires_move(),
            UniversalStructField::Struct(x) => x.requires_move(),
        }
    }
}
