use oo_bindgen::model::*;

pub(crate) trait CoreCppType {
    fn core_cpp_type(&self) -> String;
}

impl CoreCppType for Primitive {
    fn core_cpp_type(&self) -> String {
        match self {
            Primitive::Bool => "bool".to_string(),
            Primitive::U8 => "uint8_t".to_string(),
            Primitive::S8 => "int8_t".to_string(),
            Primitive::U16 => "uint16_t".to_string(),
            Primitive::S16 => "int16_t".to_string(),
            Primitive::U32 => "uint32_t".to_string(),
            Primitive::S32 => "int32_t".to_string(),
            Primitive::U64 => "uint64_t".to_string(),
            Primitive::S64 => "int64_t".to_string(),
            Primitive::Float => "float".to_string(),
            Primitive::Double => "double".to_string(),
        }
    }
}

impl CoreCppType for BasicType {
    fn core_cpp_type(&self) -> String {
        match self {
            BasicType::Primitive(x) => x.core_cpp_type(),
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
        self.name().camel_case()
    }
}

impl<T> CoreCppType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn core_cpp_type(&self) -> String {
        self.name().camel_case()
    }
}

impl CoreCppType for StructDeclaration {
    fn core_cpp_type(&self) -> String {
        self.name.camel_case()
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
        self.name.camel_case()
    }
}

impl<D> CoreCppType for EnumVariant<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_string()
    }
}

impl<D> CoreCppType for ErrorType<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.exception_name.camel_case()
    }
}

impl<D> CoreCppType for Handle<Interface<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.camel_case()
    }
}

impl<D> CoreCppType for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.iter_class.name.camel_case()
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
    T: Clone,
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_string()
    }
}

impl<D> CoreCppType for CallbackFunction<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_string()
    }
}

impl<D> CoreCppType for Constant<D>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.to_string()
    }
}

impl CoreCppType for ClassDeclarationHandle {
    fn core_cpp_type(&self) -> String {
        self.name.camel_case()
    }
}

impl<D> CoreCppType for Handle<Class<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name().camel_case()
    }
}

impl<D> CoreCppType for Handle<StaticClass<D>>
where
    D: DocReference,
{
    fn core_cpp_type(&self) -> String {
        self.name.camel_case()
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
            FunctionArgument::StructRef(x) => x.inner.core_cpp_type(),
            FunctionArgument::ClassRef(x) => x.core_cpp_type(),
            FunctionArgument::Interface(x) => x.core_cpp_type(),
        };
        format!("std::vector<{}>", inner)
    }
}
