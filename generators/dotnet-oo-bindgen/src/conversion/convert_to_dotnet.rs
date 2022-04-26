use crate::conversion::TypeInfo;
use oo_bindgen::model::*;

pub(crate) trait ConvertToDotNet {
    /// optional conversion from native type to .NET type
    fn convert_to_dotnet(&self, from: &str) -> Option<String>;
}

impl ConvertToDotNet for DurationType {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            Self::Milliseconds => Some(format!("TimeSpan.FromMilliseconds({})", from)),
            Self::Seconds => Some(format!("TimeSpan.FromSeconds({})", from)),
        }
    }
}

impl ConvertToDotNet for Primitive {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("Convert.ToBoolean({})", from)),
            Self::U8 => None,
            Self::S8 => None,
            Self::U16 => None,
            Self::S16 => None,
            Self::U32 => None,
            Self::S32 => None,
            Self::U64 => None,
            Self::S64 => None,
            Self::Float => None,
            Self::Double => None,
        }
    }
}

impl<D> ConvertToDotNet for Handle<Enum<D>>
where
    D: DocReference,
{
    fn convert_to_dotnet(&self, _: &str) -> Option<String> {
        None
    }
}

impl ConvertToDotNet for BasicType {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            Self::Primitive(x) => x.convert_to_dotnet(from),
            Self::Duration(x) => x.convert_to_dotnet(from),
            Self::Enum(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for StringType {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        Some(format!("Helpers.RustString.FromNative({})", from))
    }
}

impl<D> ConvertToDotNet for Handle<Interface<D>>
where
    D: DocReference,
{
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        Some(format!(
            "I{}NativeAdapter.FromNative({}.{})",
            self.name.camel_case(),
            from,
            self.settings.interface.context_variable_name.mixed_case()
        ))
    }
}

impl ConvertToDotNet for ClassDeclarationHandle {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        Some(format!("{}.FromNative({})", self.name.camel_case(), from))
    }
}

impl<D> ConvertToDotNet for Handle<Collection<D>>
where
    D: DocReference + TypeInfo,
{
    fn convert_to_dotnet(&self, _from: &str) -> Option<String> {
        Some(format!(
            "System.Collections.Immutable.ImmutableArray<{}>.Empty",
            self.item_type.get_dotnet_type()
        ))
    }
}

impl<D> ConvertToDotNet for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.FromNative({})",
            self.iter_class.name.camel_case(),
            from
        ))
    }
}

impl ConvertToDotNet for UniversalOr<FunctionReturnStructField> {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.convert_to_dotnet(from),
            UniversalOr::Universal(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for UniversalOr<CallbackArgStructField> {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.convert_to_dotnet(from),
            UniversalOr::Universal(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for FunctionReturnStructField {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_dotnet(from),
            Self::ClassRef(x) => x.convert_to_dotnet(from),
            Self::Struct(x) => x.convert_to_dotnet(from),
            Self::Iterator(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for CallbackArgStructField {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            CallbackArgStructField::Basic(x) => x.convert_to_dotnet(from),
            CallbackArgStructField::Iterator(x) => x.convert_to_dotnet(from),
            CallbackArgStructField::Struct(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for UniversalStructField {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.convert_to_dotnet(from),
            UniversalStructField::Struct(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for CallbackArgument {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_dotnet(from),
            Self::String(x) => x.convert_to_dotnet(from),
            Self::Iterator(x) => x.convert_to_dotnet(from),
            Self::Struct(x) => x.convert_to_dotnet(from),
            Self::Class(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for PrimitiveRef {
    fn convert_to_dotnet(&self, expr: &str) -> Option<String> {
        match self.inner {
            Primitive::Bool => Some(format!("Helpers.PrimitivePointer.ReadBool({})", expr)),
            Primitive::U8 => Some(format!(
                "Helpers.PrimitivePointer.Unsigned.ReadByte({})",
                expr
            )),
            Primitive::S8 => Some(format!(
                "Helpers.PrimitivePointer.Signed.ReadByte({})",
                expr
            )),
            Primitive::U16 => Some(format!(
                "Helpers.PrimitivePointer.Unsigned.ReadShort({})",
                expr
            )),
            Primitive::S16 => Some(format!(
                "Helpers.PrimitivePointer.Signed.ReadShort({})",
                expr
            )),
            Primitive::U32 => Some(format!(
                "Helpers.PrimitivePointer.Unsigned.ReadInt({})",
                expr
            )),
            Primitive::S32 => Some(format!("Helpers.PrimitivePointer.Signed.ReadInt({})", expr)),
            Primitive::U64 => Some(format!(
                "Helpers.PrimitivePointer.Unsigned.ReadLong({})",
                expr
            )),
            Primitive::S64 => Some(format!(
                "Helpers.PrimitivePointer.Signed.ReadLong({})",
                expr
            )),
            Primitive::Float => Some(format!("Helpers.PrimitivePointer.ReadFloat({})", expr)),
            Primitive::Double => Some(format!("Helpers.PrimitivePointer.ReadDouble({})", expr)),
        }
    }
}

impl ConvertToDotNet for FunctionReturnValue {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_dotnet(from),
            Self::String(x) => x.convert_to_dotnet(from),
            Self::ClassRef(x) => x.convert_to_dotnet(from),
            Self::Struct(x) => x.convert_to_dotnet(from),
            Self::StructRef(x) => x.untyped().convert_to_dotnet(from),
            Self::PrimitiveRef(x) => x.convert_to_dotnet(from),
        }
    }
}

impl ConvertToDotNet for StructDeclarationHandle {
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.FromNativeRef({})",
            self.name.camel_case(),
            from
        ))
    }
}

impl<T, D> ConvertToDotNet for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn convert_to_dotnet(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.FromNative({})",
            self.name().camel_case(),
            from
        ))
    }
}

/*
impl<T, D> ToDotNet for OptionalReturnType<T, D>
    where
        D: DocReference,
        T: Clone + ToDotNet,
{
    fn to_dotnet(&self, from: &str) -> Option<String> {
        self.get_value().and_then(|x| x.to_dotnet(from))
    }
}
*/
