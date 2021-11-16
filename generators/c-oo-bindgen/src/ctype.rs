use oo_bindgen::model::*;

pub(crate) trait CType {
    fn to_c_type(&self) -> String;
}

struct Pointer<'a, T>
where
    T: CType,
{
    inner: &'a T,
}

fn pointer<T>(inner: &T) -> Pointer<T>
where
    T: CType,
{
    Pointer { inner }
}

impl<'a, T> CType for Pointer<'a, T>
where
    T: CType,
{
    fn to_c_type(&self) -> String {
        format!("{}*", self.inner.to_c_type())
    }
}

impl CType for StringType {
    fn to_c_type(&self) -> String {
        "const char*".to_string()
    }
}

impl<D> CType for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn to_c_type(&self) -> String {
        format!("{}*", self.iter_class.to_c_type())
    }
}

impl CType for CallbackArgument {
    fn to_c_type(&self) -> String {
        match self {
            CallbackArgument::Basic(x) => x.to_c_type(),
            CallbackArgument::String(x) => x.to_c_type(),
            CallbackArgument::Iterator(x) => x.to_c_type(),
            CallbackArgument::Struct(x) => x.to_c_type(),
            CallbackArgument::Class(x) => pointer(x).to_c_type(),
        }
    }
}

impl CType for FunctionReturnValue {
    fn to_c_type(&self) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.to_c_type(),
            FunctionReturnValue::String(x) => x.to_c_type(),
            FunctionReturnValue::ClassRef(x) => pointer(x).to_c_type(),
            FunctionReturnValue::Struct(x) => x.to_c_type(),
            FunctionReturnValue::StructRef(x) => pointer(x.untyped()).to_c_type(),
        }
    }
}

impl CType for CallbackReturnValue {
    fn to_c_type(&self) -> String {
        match self {
            CallbackReturnValue::Basic(x) => x.to_c_type(),
            CallbackReturnValue::Struct(x) => x.to_c_type(),
        }
    }
}

impl CType for StructDeclarationHandle {
    fn to_c_type(&self) -> String {
        format!("{}_{}_t", self.settings.c_ffi_prefix, self.name)
    }
}

impl<D> CType for StructType<D>
where
    D: DocReference,
{
    fn to_c_type(&self) -> String {
        self.declaration().to_c_type()
    }
}

impl<T, D> CType for Struct<T, D>
where
    D: DocReference,
    T: StructFieldType,
{
    fn to_c_type(&self) -> String {
        format!(
            "{}_{}_t",
            self.declaration.inner.settings.c_ffi_prefix,
            self.name()
        )
    }
}

impl<D> CType for Handle<Enum<D>>
where
    D: DocReference,
{
    fn to_c_type(&self) -> String {
        format!("{}_{}_t", self.settings.c_ffi_prefix, self.name)
    }
}

impl CType for ClassDeclarationHandle {
    fn to_c_type(&self) -> String {
        format!("{}_{}_t", self.settings.c_ffi_prefix, self.name)
    }
}

impl<D> CType for Handle<Interface<D>>
where
    D: DocReference,
{
    fn to_c_type(&self) -> String {
        format!("{}_{}_t", self.settings.c_ffi_prefix, self.name)
    }
}

impl<D> CType for Handle<Collection<D>>
where
    D: DocReference,
{
    fn to_c_type(&self) -> String {
        self.collection_class.to_c_type()
    }
}

impl CType for IteratorItemType {
    fn to_c_type(&self) -> String {
        match self {
            IteratorItemType::Struct(x) => x.to_c_type(),
        }
    }
}

impl<D> CType for Handle<Function<D>>
where
    D: DocReference,
{
    fn to_c_type(&self) -> String {
        format!("{}_{}", self.settings.c_ffi_prefix, self.name)
    }
}

impl CType for Primitive {
    fn to_c_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::U8 => "uint8_t".to_string(),
            Self::S8 => "int8_t".to_string(),
            Self::U16 => "uint16_t".to_string(),
            Self::S16 => "int16_t".to_string(),
            Self::U32 => "uint32_t".to_string(),
            Self::S32 => "int32_t".to_string(),
            Self::U64 => "uint64_t".to_string(),
            Self::S64 => "int64_t".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
        }
    }
}

impl CType for BasicType {
    fn to_c_type(&self) -> String {
        match self {
            Self::Primitive(x) => x.to_c_type(),
            Self::Duration(_) => "uint64_t".to_string(),
            Self::Enum(handle) => handle.to_c_type(),
        }
    }
}

impl<T> CType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn to_c_type(&self) -> String {
        self.to_struct_type().to_c_type()
    }
}

impl CType for FunctionArgStructField {
    fn to_c_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.to_c_type(),
            FunctionArgStructField::String(x) => x.to_c_type(),
            FunctionArgStructField::Interface(x) => x.inner.to_c_type(),
            FunctionArgStructField::Struct(x) => x.to_c_type(),
        }
    }
}

impl CType for FunctionReturnStructField {
    fn to_c_type(&self) -> String {
        match self {
            Self::Basic(x) => x.to_c_type(),
            Self::ClassRef(x) => pointer(x).to_c_type(),
            Self::Struct(x) => x.to_c_type(),
            Self::Iterator(x) => x.to_c_type(),
        }
    }
}

impl CType for CallbackArgStructField {
    fn to_c_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.to_c_type(),
            CallbackArgStructField::Iterator(x) => pointer(x).to_c_type(),
            CallbackArgStructField::Struct(x) => x.to_c_type(),
        }
    }
}

impl CType for UniversalStructField {
    fn to_c_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.to_c_type(),
            UniversalStructField::Struct(x) => x.to_c_type(),
        }
    }
}

impl CType for FunctionArgument {
    fn to_c_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.to_c_type(),
            FunctionArgument::String(x) => x.to_c_type(),
            FunctionArgument::Collection(x) => pointer(x).to_c_type(),
            FunctionArgument::Struct(x) => x.to_c_type(),
            FunctionArgument::StructRef(x) => pointer(&x.inner).to_c_type(),
            FunctionArgument::ClassRef(x) => pointer(x).to_c_type(),
            FunctionArgument::Interface(x) => x.to_c_type(),
        }
    }
}

impl<T> CType for OptionalReturnType<T, Validated>
where
    T: Clone + CType,
{
    fn to_c_type(&self) -> String {
        match self.get_value() {
            None => "void".to_string(),
            Some(t) => t.to_c_type(),
        }
    }
}
