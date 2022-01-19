use oo_bindgen::model::*;

const INT_PTR_STRING: &str = "IntPtr";

/// provides information about a type
pub(crate) trait TypeInfo {
    /// Returns the .NET natural type
    fn get_dotnet_type(&self) -> String;
    /// Return the .NET representation of the native C type
    fn get_native_type(&self) -> String;
}

impl TypeInfo for DurationType {
    fn get_dotnet_type(&self) -> String {
        "TimeSpan".to_string()
    }

    fn get_native_type(&self) -> String {
        "ulong".to_string()
    }
}

impl TypeInfo for Primitive {
    fn get_dotnet_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::U8 => "byte".to_string(),
            Self::S8 => "sbyte".to_string(),
            Self::U16 => "ushort".to_string(),
            Self::S16 => "short".to_string(),
            Self::U32 => "uint".to_string(),
            Self::S32 => "int".to_string(),
            Self::U64 => "ulong".to_string(),
            Self::S64 => "long".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            Self::Bool => "byte".to_string(),
            Self::U8 => "byte".to_string(),
            Self::S8 => "sbyte".to_string(),
            Self::U16 => "ushort".to_string(),
            Self::S16 => "short".to_string(),
            Self::U32 => "uint".to_string(),
            Self::S32 => "int".to_string(),
            Self::U64 => "ulong".to_string(),
            Self::S64 => "long".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
        }
    }
}

impl<D> TypeInfo for Handle<Enum<D>>
where
    D: DocReference,
{
    fn get_dotnet_type(&self) -> String {
        self.name.camel_case()
    }

    fn get_native_type(&self) -> String {
        self.name.camel_case()
    }
}

impl TypeInfo for BasicType {
    fn get_dotnet_type(&self) -> String {
        match self {
            Self::Primitive(x) => x.get_dotnet_type(),
            Self::Duration(x) => x.get_dotnet_type(),
            Self::Enum(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            Self::Primitive(x) => x.get_native_type(),
            Self::Duration(x) => x.get_native_type(),
            Self::Enum(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for StringType {
    fn get_dotnet_type(&self) -> String {
        "string".to_string()
    }

    fn get_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }
}

pub(crate) fn base_functor_type<D>(cb: &CallbackFunction<D>) -> &'static str
where
    D: DocReference,
{
    if cb.return_type.is_none() {
        "Action"
    } else {
        "Func"
    }
}

pub(crate) fn full_functor_type<D>(cb: &CallbackFunction<D>) -> String
where
    D: DocReference,
{
    fn arg_types<D>(args: &[Arg<CallbackArgument, D>]) -> String
    where
        D: DocReference,
    {
        args.iter()
            .map(|x| x.arg_type.get_dotnet_type())
            .collect::<Vec<String>>()
            .join(", ")
    }

    match (&cb.return_type.get_value(), cb.arguments.as_slice()) {
        (None, []) => "Action".to_string(),
        (None, args) => {
            format!("Action<{}>", arg_types(args))
        }
        (Some(t), []) => {
            format!("Func<{}>", t.get_dotnet_type())
        }
        (Some(t), args) => {
            format!("Func<{}, {}>", arg_types(args), t.get_dotnet_type())
        }
    }
}

impl<D> TypeInfo for Handle<Interface<D>>
where
    D: DocReference,
{
    fn get_dotnet_type(&self) -> String {
        if let Some(cb) = self.get_functional_callback() {
            if cb.functional_transform.enabled() {
                return full_functor_type(cb);
            }
        }

        format!("I{}", self.name.camel_case())
    }

    fn get_native_type(&self) -> String {
        format!("I{}NativeAdapter", self.name.camel_case())
    }
}

impl TypeInfo for ClassDeclarationHandle {
    fn get_dotnet_type(&self) -> String {
        self.name.camel_case()
    }

    fn get_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }
}

impl<D> TypeInfo for Handle<Collection<D>>
where
    D: DocReference,
{
    fn get_dotnet_type(&self) -> String {
        format!(
            "System.Collections.Generic.ICollection<{}>",
            self.item_type.get_dotnet_type()
        )
    }

    fn get_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }
}

impl<D> TypeInfo for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn get_dotnet_type(&self) -> String {
        match &self.item_type {
            IteratorItemType::Primitive(x) => {
                format!(
                    "System.Collections.Generic.ICollection<{}>",
                    x.get_dotnet_type()
                )
            }
            IteratorItemType::Struct(x) => {
                format!(
                    "System.Collections.Generic.ICollection<{}>",
                    x.name().camel_case()
                )
            }
        }
    }

    fn get_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }
}

impl<T> TypeInfo for UniversalOr<T>
where
    T: StructFieldType,
{
    fn get_dotnet_type(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.get_dotnet_type(),
            UniversalOr::Universal(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.get_native_type(),
            UniversalOr::Universal(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for FunctionArgStructField {
    fn get_dotnet_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.get_dotnet_type(),
            FunctionArgStructField::String(x) => x.get_dotnet_type(),
            FunctionArgStructField::Interface(x) => x.inner.get_dotnet_type(),
            FunctionArgStructField::Struct(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.get_native_type(),
            FunctionArgStructField::String(x) => x.get_native_type(),
            FunctionArgStructField::Interface(x) => x.inner.get_native_type(),
            FunctionArgStructField::Struct(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for FunctionReturnStructField {
    fn get_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_dotnet_type(),
            Self::ClassRef(x) => x.get_dotnet_type(),
            Self::Struct(x) => x.get_dotnet_type(),
            Self::Iterator(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_native_type(),
            Self::ClassRef(x) => x.get_native_type(),
            Self::Struct(x) => x.get_native_type(),
            Self::Iterator(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for CallbackArgStructField {
    fn get_dotnet_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.get_dotnet_type(),
            CallbackArgStructField::Iterator(x) => x.get_dotnet_type(),
            CallbackArgStructField::Struct(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.get_native_type(),
            CallbackArgStructField::Iterator(x) => x.get_native_type(),
            CallbackArgStructField::Struct(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for UniversalStructField {
    fn get_dotnet_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.get_dotnet_type(),
            UniversalStructField::Struct(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.get_native_type(),
            UniversalStructField::Struct(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for FunctionArgument {
    fn get_dotnet_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.get_dotnet_type(),
            FunctionArgument::String(x) => x.get_dotnet_type(),
            FunctionArgument::Collection(x) => x.get_dotnet_type(),
            FunctionArgument::Struct(x) => x.get_dotnet_type(),
            FunctionArgument::StructRef(x) => x.inner.get_dotnet_type(),
            FunctionArgument::ClassRef(x) => x.get_dotnet_type(),
            FunctionArgument::Interface(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.get_native_type(),
            FunctionArgument::String(x) => x.get_native_type(),
            FunctionArgument::Collection(x) => x.get_native_type(),
            FunctionArgument::Struct(x) => x.get_native_type(),
            FunctionArgument::StructRef(x) => x.inner.get_native_type(),
            FunctionArgument::ClassRef(x) => x.get_native_type(),
            FunctionArgument::Interface(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for CallbackArgument {
    fn get_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_dotnet_type(),
            Self::String(x) => x.get_dotnet_type(),
            Self::Iterator(x) => x.get_dotnet_type(),
            Self::Struct(x) => x.get_dotnet_type(),
            Self::Class(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_native_type(),
            Self::String(x) => x.get_native_type(),
            Self::Iterator(x) => x.get_native_type(),
            Self::Struct(x) => x.get_native_type(),
            Self::Class(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for CallbackReturnValue {
    fn get_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_dotnet_type(),
            Self::Struct(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_native_type(),
            Self::Struct(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for PrimitiveRef {
    fn get_dotnet_type(&self) -> String {
        self.inner.get_dotnet_type()
    }

    fn get_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }
}

impl TypeInfo for FunctionReturnValue {
    fn get_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_dotnet_type(),
            Self::String(x) => x.get_dotnet_type(),
            Self::ClassRef(x) => x.get_dotnet_type(),
            Self::Struct(x) => x.get_dotnet_type(),
            Self::StructRef(x) => x.untyped().get_dotnet_type(),
            Self::PrimitiveRef(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_native_type(),
            Self::String(x) => x.get_native_type(),
            Self::ClassRef(x) => x.get_native_type(),
            Self::Struct(x) => x.get_native_type(),
            Self::StructRef(x) => x.untyped().get_native_type(),
            Self::PrimitiveRef(x) => x.get_native_type(),
        }
    }
}

impl TypeInfo for StructDeclarationHandle {
    fn get_dotnet_type(&self) -> String {
        self.name.camel_case()
    }

    fn get_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }
}

impl<T, D> TypeInfo for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn get_dotnet_type(&self) -> String {
        self.name().camel_case()
    }

    fn get_native_type(&self) -> String {
        format!("{}Native", self.name().camel_case())
    }
}

const VOID: &str = "void";

impl<T, D> TypeInfo for OptionalReturnType<T, D>
where
    D: DocReference,
    T: Clone + TypeInfo,
{
    fn get_dotnet_type(&self) -> String {
        match self.get_value() {
            None => VOID.to_string(),
            Some(x) => x.get_dotnet_type(),
        }
    }

    fn get_native_type(&self) -> String {
        match self.get_value() {
            None => VOID.to_string(),
            Some(x) => x.get_native_type(),
        }
    }
}
