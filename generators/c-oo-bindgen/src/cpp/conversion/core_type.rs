use heck::{CamelCase, SnakeCase};
use oo_bindgen::class::{Class, ClassDeclarationHandle, StaticClass};
use oo_bindgen::collection::Collection;
use oo_bindgen::constants::Constant;
use oo_bindgen::doc::DocReference;
use oo_bindgen::enum_type::{Enum, EnumVariant};
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::interface::{CallbackFunction, Interface};
use oo_bindgen::iterator::IteratorItemType;
use oo_bindgen::structs::*;
use oo_bindgen::types::{Arg, BasicType, StringType};
use oo_bindgen::{Handle, StructType, UniversalOr};

pub(crate) trait CoreCppType {
    fn core_cpp_type(&self) -> String;
}

impl CoreCppType for BasicType {
    fn core_cpp_type(&self) -> String {
        match self {
            BasicType::Bool => "bool".to_string(),
            BasicType::U8 => "uint8_t".to_string(),
            BasicType::S8 => "int8_t".to_string(),
            BasicType::U16 => "uint16_t".to_string(),
            BasicType::S16 => "int16_t".to_string(),
            BasicType::U32 => "uint32_t".to_string(),
            BasicType::S32 => "int32_t".to_string(),
            BasicType::U64 => "uint64_t".to_string(),
            BasicType::S64 => "int64_t".to_string(),
            BasicType::Float32 => "float".to_string(),
            BasicType::Double64 => "double".to_string(),
            BasicType::Duration(_) => "std::chrono::steady_clock::duration".to_string(),
            BasicType::Enum(x) => x.core_cpp_type(),
        }
    }
}

impl CoreCppType for StringType {
    fn core_cpp_type(&self) -> String {
        "std::string".to_string()
    }
}

impl<T, D> CoreCppType for Struct<T, D>
where
    D: DocReference,
    T: StructFieldType,
{
    fn core_cpp_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl<T> CoreCppType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn core_cpp_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CoreCppType for StructDeclaration {
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl<D> CoreCppType for StructType<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.declaration().core_cpp_type()
    }
}

impl<D> CoreCppType for Handle<Enum<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl<D> CoreCppType for EnumVariant<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl<D> CoreCppType for ErrorType<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.exception_name.to_camel_case()
    }
}

impl<D> CoreCppType for Handle<Interface<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl<D> CoreCppType for Handle<oo_bindgen::iterator::Iterator<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.iter_class.name.to_camel_case()
    }
}

impl CoreCppType for IteratorItemType {
    fn core_cpp_type(&self) -> String {
        match self {
            IteratorItemType::Struct(x) => x.core_cpp_type(),
        }
    }
}

impl<T, D> CoreCppType for Arg<T, D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl<D> CoreCppType for CallbackFunction<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl<D> CoreCppType for Constant<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreCppType for ClassDeclarationHandle {
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl<D> CoreCppType for Handle<Class<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl<D> CoreCppType for Handle<StaticClass<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl<D> CoreCppType for Handle<Collection<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        let inner = match &self.item_type {
            FunctionArgument::Basic(x) => x.core_cpp_type(),
            FunctionArgument::String(x) => x.core_cpp_type(),
            FunctionArgument::Collection(x) => x.core_cpp_type(),
            FunctionArgument::Struct(x) => x.core_cpp_type(),
            FunctionArgument::StructRef(_) => unimplemented!(),
            FunctionArgument::ClassRef(_) => unimplemented!(),
            FunctionArgument::Interface(_) => unimplemented!(),
        };
        format!("std::vector<{}>", inner)
    }
}
