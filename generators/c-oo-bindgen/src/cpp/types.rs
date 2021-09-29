use oo_bindgen::native_function::ReturnType;

use crate::cpp::names::CppName;
use oo_bindgen::callback::InterfaceHandle;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::native_enum::EnumHandle;
use oo_bindgen::native_struct::AnyStructHandle;
use oo_bindgen::struct_common::NativeStructDeclarationHandle;
use oo_bindgen::types::{AnyType, BasicType, DurationType};

/// These types are always be pass-by-value in C++
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Primitive {
    Bool,
    Uint8,
    Sint8,
    Uint16,
    Sint16,
    Uint32,
    Sint32,
    Uint64,
    Sint64,
    Float,
    Double,
    Enum(EnumHandle),
    Duration(DurationType),
}

impl Primitive {
    fn get_cpp_type(&self) -> String {
        match self {
            Primitive::Bool => "bool".to_owned(),
            Primitive::Uint8 => "uint8_t".to_owned(),
            Primitive::Sint8 => "int8_t".to_owned(),
            Primitive::Uint16 => "uint16_t".to_owned(),
            Primitive::Sint16 => "int16_t".to_owned(),
            Primitive::Uint32 => "uint32_t".to_owned(),
            Primitive::Sint32 => "int32_t".to_owned(),
            Primitive::Uint64 => "uint64_t".to_owned(),
            Primitive::Sint64 => "int64_t".to_owned(),
            Primitive::Float => "float".to_owned(),
            Primitive::Double => "double".to_owned(),
            Primitive::Enum(x) => x.cpp_name(),
            Primitive::Duration(_) => "std::chrono::steady_clock::duration".to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CppType {
    Primitive(Primitive),
    String,
    Struct(AnyStructHandle),
    StructRef(NativeStructDeclarationHandle),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
}

pub(crate) trait CppTypes {
    fn get_cpp_struct_member_type(&self) -> String;
    fn get_cpp_func_argument_type(&self) -> String;
    fn get_cpp_struct_constructor_type(&self) -> String;
}

pub(crate) trait CppReturnType {
    fn get_cpp_return_type(&self) -> String;
}

impl CppReturnType for ReturnType {
    fn get_cpp_return_type(&self) -> String {
        match self {
            ReturnType::Void => "void".to_owned(),
            ReturnType::Type(t, _) => CppType::new(t.clone()).get_cpp_return_type(),
        }
    }
}

impl CppType {
    fn new(x: AnyType) -> Self {
        match x {
            AnyType::Basic(x) => match x {
                BasicType::Bool => Primitive::Bool.into(),
                BasicType::Uint8 => Primitive::Uint8.into(),
                BasicType::Sint8 => Primitive::Sint8.into(),
                BasicType::Uint16 => Primitive::Uint16.into(),
                BasicType::Sint16 => Primitive::Sint16.into(),
                BasicType::Uint32 => Primitive::Uint32.into(),
                BasicType::Sint32 => Primitive::Sint32.into(),
                BasicType::Uint64 => Primitive::Uint64.into(),
                BasicType::Sint64 => Primitive::Sint64.into(),
                BasicType::Float => Primitive::Float.into(),
                BasicType::Double => Primitive::Double.into(),
                BasicType::Duration(x) => Primitive::Duration(x).into(),
                BasicType::Enum(x) => Primitive::Enum(x).into(),
            },
            AnyType::String => Self::String,
            AnyType::Struct(x) => Self::Struct(x),
            AnyType::StructRef(x) => Self::StructRef(x),
            AnyType::ClassRef(x) => Self::ClassRef(x),
            AnyType::Interface(x) => Self::Interface(x),
            AnyType::Iterator(x) => Self::Iterator(x),
            AnyType::Collection(x) => Self::Collection(x),
        }
    }

    fn get_cpp_struct_member_type(&self) -> String {
        match self {
            CppType::Primitive(x) => x.get_cpp_type(),
            CppType::String => "std::string".to_owned(),
            CppType::Struct(x) => x.cpp_name(),
            // these probably shouldn't be allowed in structs at all
            CppType::StructRef(x) => x.cpp_name(),
            CppType::ClassRef(x) => x.cpp_name(),
            CppType::Interface(x) => format!("std::unique_ptr<{}>", x.cpp_name()),
            CppType::Iterator(x) => format!("std::vector<{}>", x.item_type.cpp_name()),
            CppType::Collection(x) => {
                format!("std::vector<{}>", x.item_type.get_cpp_struct_member_type())
            }
        }
    }

    fn get_cpp_return_type(&self) -> String {
        match self {
            CppType::Primitive(x) => x.get_cpp_type(),
            CppType::String => "std::string".to_owned(),
            CppType::Struct(x) => x.cpp_name(),
            // these probably shouldn't be allowed in return types
            CppType::StructRef(x) => x.cpp_name(),
            CppType::ClassRef(x) => x.cpp_name(),
            CppType::Interface(x) => format!("std::unique_ptr<{}>", x.cpp_name()),
            CppType::Iterator(x) => format!("std::vector<{}>", x.item_type.cpp_name()),
            CppType::Collection(x) => {
                format!("std::vector<{}>", x.item_type.get_cpp_struct_member_type())
            }
        }
    }

    fn get_cpp_function_argument_type(&self) -> String {
        match self {
            CppType::Primitive(x) => x.get_cpp_type(),
            CppType::String => "const std::string&".to_owned(),
            CppType::Struct(x) => format!("const {}&", x.cpp_name()),
            // these probably shouldn't be allowed in structs at all
            CppType::StructRef(x) => format!("std::unique_ptr<{}>", x.cpp_name()),
            CppType::ClassRef(x) => format!("{}&", x.cpp_name()),
            CppType::Interface(x) => format!("std::unique_ptr<{}>", x.cpp_name()),
            CppType::Iterator(x) => format!("Iterator<{}>&", x.item_type.cpp_name()),
            CppType::Collection(x) => format!(
                "const std::vector<{}>&",
                x.item_type.get_cpp_struct_member_type()
            ),
        }
    }

    fn get_struct_constructor_argument_type(&self) -> String {
        match self {
            CppType::Primitive(x) => x.get_cpp_type(),
            CppType::String => "const std::string&".to_owned(),
            CppType::Struct(x) => format!("const {}&", x.cpp_name()),
            // these probably shouldn't be allowed in structs at all
            CppType::StructRef(x) => format!("const {}&", x.cpp_name()),
            CppType::ClassRef(x) => format!("{}&", x.cpp_name()),
            CppType::Interface(x) => format!("std::unique_ptr<{}>", x.cpp_name()),
            CppType::Iterator(x) => format!("const std::vector<{}>&", x.item_type.cpp_name()),
            CppType::Collection(x) => format!(
                "const std::vector<{}>&",
                x.item_type.get_cpp_struct_member_type()
            ),
        }
    }
}

impl CppTypes for AnyType {
    fn get_cpp_struct_member_type(&self) -> String {
        CppType::new(self.clone()).get_cpp_struct_member_type()
    }

    fn get_cpp_func_argument_type(&self) -> String {
        CppType::new(self.clone()).get_cpp_function_argument_type()
    }

    fn get_cpp_struct_constructor_type(&self) -> String {
        CppType::new(self.clone()).get_struct_constructor_argument_type()
    }
}

impl From<Primitive> for CppType {
    fn from(x: Primitive) -> Self {
        CppType::Primitive(x)
    }
}
