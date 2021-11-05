use crate::cpp::conversion::ToNative;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::doc::DocReference;
use oo_bindgen::interface::InterfaceHandle;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{FunctionArgStructField, Struct, StructFieldType, UniversalStructField};
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::{Handle, UniversalOr};

pub(crate) trait ToNativeStructField {
    /// takes a C++ type and converts it to a native type
    fn to_native_struct_field(&self, expr: String) -> String;

    /// does the type require a move operation that modifies the C++ type
    fn requires_move(&self) -> bool;
}

impl ToNativeStructField for StringType {
    fn to_native_struct_field(&self, expr: String) -> String {
        // since the C++ struct will still be valid for the function call there's no need to copy
        // the string. Rust immediately makes a copy of the string inside the FFI layer.
        self.to_native(expr)
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl<T, D> ToNativeStructField for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType + ToNativeStructField,
{
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
        // interfaces in function arguments are passed by unique_ptr, therefore we
        // have to move the unique_ptr into the interface conversion function where
        // it will be released and Rust will manage the lifetime via a Drop impl
        true
    }
}

impl ToNativeStructField for ClassDeclarationHandle {
    fn to_native_struct_field(&self, expr: String) -> String {
        format!("::convert::get({})", expr)
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl ToNativeStructField for IteratorHandle {
    fn to_native_struct_field(&self, expr: String) -> String {
        format!("::convert::to_native({})", expr)
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl ToNativeStructField for BasicType {
    fn to_native_struct_field(&self, expr: String) -> String {
        self.to_native(expr)
    }

    fn requires_move(&self) -> bool {
        false
    }
}

impl ToNativeStructField for FunctionArgStructField {
    fn to_native_struct_field(&self, expr: String) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.to_native_struct_field(expr),
            FunctionArgStructField::String(x) => x.to_native_struct_field(expr),
            FunctionArgStructField::Interface(x) => x.to_native_struct_field(expr),
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
