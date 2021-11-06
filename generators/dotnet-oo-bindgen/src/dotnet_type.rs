use heck::{CamelCase, MixedCase};
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::Collection;
use oo_bindgen::doc::DocReference;
use oo_bindgen::enum_type::Enum;
use oo_bindgen::function::*;
use oo_bindgen::interface::*;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::*;
use oo_bindgen::types::{BasicType, DurationType, StringType};
use oo_bindgen::{Handle, UniversalOr};

const INT_PTR_STRING: &str = "IntPtr";

pub(crate) trait DotnetType {
    /// Returns the .NET natural type
    fn as_dotnet_type(&self) -> String;
    /// Return the .NET representation of the native C type
    fn as_native_type(&self) -> String;
    fn convert_to_native(&self, from: &str) -> Option<String>;
    fn cleanup(&self, from: &str) -> Option<String>;
    fn convert_from_native(&self, from: &str) -> Option<String>;
}

impl DotnetType for DurationType {
    fn as_dotnet_type(&self) -> String {
        "TimeSpan".to_string()
    }

    fn as_native_type(&self) -> String {
        "ulong".to_string()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Milliseconds => Some(format!("(ulong){}.TotalMilliseconds", from)),
            Self::Seconds => Some(format!("(ulong){}.TotalSeconds", from)),
        }
    }

    fn cleanup(&self, _from: &str) -> Option<String> {
        None
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Milliseconds => Some(format!("TimeSpan.FromMilliseconds({})", from)),
            Self::Seconds => Some(format!("TimeSpan.FromSeconds({})", from)),
        }
    }
}

impl DotnetType for BasicType {
    fn as_dotnet_type(&self) -> String {
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
            Self::Float32 => "float".to_string(),
            Self::Double64 => "double".to_string(),
            Self::Duration(x) => x.as_dotnet_type(),
            Self::Enum(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
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
            Self::Float32 => "float".to_string(),
            Self::Double64 => "double".to_string(),
            Self::Duration(x) => x.as_native_type(),
            Self::Enum(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("Convert.ToByte({})", from)),
            Self::U8 => None,
            Self::S8 => None,
            Self::U16 => None,
            Self::S16 => None,
            Self::U32 => None,
            Self::S32 => None,
            Self::U64 => None,
            Self::S64 => None,
            Self::Float32 => None,
            Self::Double64 => None,
            Self::Duration(x) => x.convert_to_native(from),
            Self::Enum(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => None,
            Self::U8 => None,
            Self::S8 => None,
            Self::U16 => None,
            Self::S16 => None,
            Self::U32 => None,
            Self::S32 => None,
            Self::U64 => None,
            Self::S64 => None,
            Self::Float32 => None,
            Self::Double64 => None,
            Self::Duration(x) => x.cleanup(from),
            Self::Enum(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
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
            Self::Float32 => None,
            Self::Double64 => None,
            Self::Duration(x) => x.convert_from_native(from),
            Self::Enum(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for StringType {
    fn as_dotnet_type(&self) -> String {
        "string".to_string()
    }

    fn as_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!("Helpers.RustString.ToNative({})", from))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!("Helpers.RustString.Destroy({});", from))
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!("Helpers.RustString.FromNative({})", from))
    }
}

impl<D> DotnetType for Handle<Interface<D>>
where
    D: DocReference,
{
    fn as_dotnet_type(&self) -> String {
        format!("I{}", self.name.to_camel_case())
    }

    fn as_native_type(&self) -> String {
        format!("I{}NativeAdapter", self.name.to_camel_case())
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "new I{}NativeAdapter({})",
            self.name.to_camel_case(),
            from
        ))
    }

    fn cleanup(&self, _from: &str) -> Option<String> {
        None
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "I{}NativeAdapter.FromNative({}.{})",
            self.name.to_camel_case(),
            from,
            CTX_VARIABLE_NAME.to_mixed_case()
        ))
    }
}

impl DotnetType for ClassDeclarationHandle {
    fn as_dotnet_type(&self) -> String {
        self.name.to_camel_case()
    }

    fn as_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!("{}.self", from))
    }

    fn cleanup(&self, _: &str) -> Option<String> {
        None
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}.FromNative({})",
            self.name.to_camel_case(),
            from
        ))
    }
}

impl<D> DotnetType for Handle<Collection<D>>
where
    D: DocReference,
{
    fn as_dotnet_type(&self) -> String {
        format!(
            "System.Collections.Generic.ICollection<{}>",
            self.item_type.as_dotnet_type()
        )
    }

    fn as_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.ToNative({})",
            self.collection_class.name.to_camel_case(),
            from
        ))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.Cleanup({});",
            self.collection_class.name.to_camel_case(),
            from
        ))
    }

    fn convert_from_native(&self, _from: &str) -> Option<String> {
        Some(format!(
            "System.Collections.Immutable.ImmutableArray<{}>.Empty",
            self.item_type.as_dotnet_type()
        ))
    }
}

impl<D> DotnetType for Handle<oo_bindgen::iterator::Iterator<D>>
where
    D: DocReference,
{
    fn as_dotnet_type(&self) -> String {
        format!(
            "System.Collections.Generic.ICollection<{}>",
            self.item_type.name().to_camel_case()
        )
    }

    fn as_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }

    fn convert_to_native(&self, _: &str) -> Option<String> {
        Some("IntPtr.Zero".to_string())
    }

    fn cleanup(&self, _: &str) -> Option<String> {
        None
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.FromNative({})",
            self.iter_class.name.to_camel_case(),
            from
        ))
    }
}

impl<T> DotnetType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn as_dotnet_type(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_dotnet_type(),
            UniversalOr::Universal(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_native_type(),
            UniversalOr::Universal(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.convert_to_native(from),
            UniversalOr::Universal(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.cleanup(from),
            UniversalOr::Universal(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.convert_from_native(from),
            UniversalOr::Universal(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for FunctionArgStructField {
    fn as_dotnet_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_dotnet_type(),
            FunctionArgStructField::String(x) => x.as_dotnet_type(),
            FunctionArgStructField::Interface(x) => x.as_dotnet_type(),
            FunctionArgStructField::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_native_type(),
            FunctionArgStructField::String(x) => x.as_native_type(),
            FunctionArgStructField::Interface(x) => x.as_native_type(),
            FunctionArgStructField::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgStructField::Basic(x) => x.convert_to_native(from),
            FunctionArgStructField::String(x) => x.convert_to_native(from),
            FunctionArgStructField::Interface(x) => x.convert_to_native(from),
            FunctionArgStructField::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgStructField::Basic(x) => x.cleanup(from),
            FunctionArgStructField::String(x) => x.cleanup(from),
            FunctionArgStructField::Interface(x) => x.cleanup(from),
            FunctionArgStructField::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgStructField::Basic(x) => x.convert_from_native(from),
            FunctionArgStructField::String(x) => x.convert_from_native(from),
            FunctionArgStructField::Interface(x) => x.convert_from_native(from),
            FunctionArgStructField::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for FunctionReturnStructField {
    fn as_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_dotnet_type(),
            Self::ClassRef(x) => x.as_dotnet_type(),
            Self::Struct(x) => x.as_dotnet_type(),
            Self::Iterator(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_native_type(),
            Self::ClassRef(x) => x.as_native_type(),
            Self::Struct(x) => x.as_native_type(),
            Self::Iterator(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_native(from),
            Self::ClassRef(x) => x.convert_to_native(from),
            Self::Struct(x) => x.convert_to_native(from),
            Self::Iterator(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.cleanup(from),
            Self::ClassRef(x) => x.cleanup(from),
            Self::Struct(x) => x.cleanup(from),
            Self::Iterator(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_from_native(from),
            Self::ClassRef(x) => x.convert_from_native(from),
            Self::Struct(x) => x.convert_from_native(from),
            Self::Iterator(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for CallbackArgStructField {
    fn as_dotnet_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_dotnet_type(),
            CallbackArgStructField::Iterator(x) => x.as_dotnet_type(),
            CallbackArgStructField::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_native_type(),
            CallbackArgStructField::Iterator(x) => x.as_native_type(),
            CallbackArgStructField::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            CallbackArgStructField::Basic(x) => x.convert_to_native(from),
            CallbackArgStructField::Iterator(x) => x.convert_to_native(from),
            CallbackArgStructField::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            CallbackArgStructField::Basic(x) => x.cleanup(from),
            CallbackArgStructField::Iterator(x) => x.cleanup(from),
            CallbackArgStructField::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            CallbackArgStructField::Basic(x) => x.convert_from_native(from),
            CallbackArgStructField::Iterator(x) => x.convert_from_native(from),
            CallbackArgStructField::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for UniversalStructField {
    fn as_dotnet_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_dotnet_type(),
            UniversalStructField::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_native_type(),
            UniversalStructField::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.convert_to_native(from),
            UniversalStructField::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.cleanup(from),
            UniversalStructField::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.convert_from_native(from),
            UniversalStructField::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl<D> DotnetType for Handle<Enum<D>>
where
    D: DocReference,
{
    fn as_dotnet_type(&self) -> String {
        self.name.to_camel_case()
    }

    fn as_native_type(&self) -> String {
        self.name.to_camel_case()
    }

    fn convert_to_native(&self, _: &str) -> Option<String> {
        None
    }

    fn cleanup(&self, _: &str) -> Option<String> {
        None
    }

    fn convert_from_native(&self, _: &str) -> Option<String> {
        None
    }
}

impl DotnetType for FunctionArgument {
    fn as_dotnet_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.as_dotnet_type(),
            FunctionArgument::String(x) => x.as_dotnet_type(),
            FunctionArgument::Collection(x) => x.as_dotnet_type(),
            FunctionArgument::Struct(x) => x.as_dotnet_type(),
            FunctionArgument::StructRef(x) => x.inner.as_dotnet_type(),
            FunctionArgument::ClassRef(x) => x.as_dotnet_type(),
            FunctionArgument::Interface(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.as_native_type(),
            FunctionArgument::String(x) => x.as_native_type(),
            FunctionArgument::Collection(x) => x.as_native_type(),
            FunctionArgument::Struct(x) => x.as_native_type(),
            FunctionArgument::StructRef(x) => x.inner.as_native_type(),
            FunctionArgument::ClassRef(x) => x.as_native_type(),
            FunctionArgument::Interface(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgument::Basic(x) => x.convert_to_native(from),
            FunctionArgument::String(x) => x.convert_to_native(from),
            FunctionArgument::Collection(x) => x.convert_to_native(from),
            FunctionArgument::Struct(x) => x.convert_to_native(from),
            FunctionArgument::StructRef(x) => x.inner.convert_to_native(from),
            FunctionArgument::ClassRef(x) => x.convert_to_native(from),
            FunctionArgument::Interface(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgument::Basic(x) => x.cleanup(from),
            FunctionArgument::String(x) => x.cleanup(from),
            FunctionArgument::Collection(x) => x.cleanup(from),
            FunctionArgument::Struct(x) => x.cleanup(from),
            FunctionArgument::StructRef(x) => x.inner.cleanup(from),
            FunctionArgument::ClassRef(x) => x.cleanup(from),
            FunctionArgument::Interface(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgument::Basic(x) => x.convert_from_native(from),
            FunctionArgument::String(x) => x.convert_from_native(from),
            FunctionArgument::Collection(x) => x.convert_from_native(from),
            FunctionArgument::Struct(x) => x.convert_from_native(from),
            FunctionArgument::StructRef(x) => x.inner.convert_from_native(from),
            FunctionArgument::ClassRef(x) => x.convert_from_native(from),
            FunctionArgument::Interface(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for CallbackArgument {
    fn as_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_dotnet_type(),
            Self::String(x) => x.as_dotnet_type(),
            Self::Iterator(x) => x.as_dotnet_type(),
            Self::Struct(x) => x.as_dotnet_type(),
            Self::Class(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_native_type(),
            Self::String(x) => x.as_native_type(),
            Self::Iterator(x) => x.as_native_type(),
            Self::Struct(x) => x.as_native_type(),
            Self::Class(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_native(from),
            Self::String(x) => x.convert_to_native(from),
            Self::Iterator(x) => x.convert_to_native(from),
            Self::Struct(x) => x.convert_to_native(from),
            Self::Class(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.cleanup(from),
            Self::String(x) => x.cleanup(from),
            Self::Iterator(x) => x.cleanup(from),
            Self::Struct(x) => x.cleanup(from),
            Self::Class(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_from_native(from),
            Self::String(x) => x.convert_from_native(from),
            Self::Iterator(x) => x.convert_from_native(from),
            Self::Struct(x) => x.convert_from_native(from),
            Self::Class(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for CallbackReturnValue {
    fn as_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_dotnet_type(),
            Self::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_native_type(),
            Self::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_native(from),
            Self::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.cleanup(from),
            Self::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_from_native(from),
            Self::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for FunctionReturnValue {
    fn as_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_dotnet_type(),
            Self::String(x) => x.as_dotnet_type(),
            Self::ClassRef(x) => x.as_dotnet_type(),
            Self::Struct(x) => x.as_dotnet_type(),
            Self::StructRef(x) => x.untyped().as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_native_type(),
            Self::String(x) => x.as_native_type(),
            Self::ClassRef(x) => x.as_native_type(),
            Self::Struct(x) => x.as_native_type(),
            Self::StructRef(x) => x.untyped().as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_native(from),
            Self::String(x) => x.convert_to_native(from),
            Self::ClassRef(x) => x.convert_to_native(from),
            Self::Struct(x) => x.convert_to_native(from),
            Self::StructRef(x) => x.untyped().convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.cleanup(from),
            Self::String(x) => x.cleanup(from),
            Self::ClassRef(x) => x.cleanup(from),
            Self::Struct(x) => x.cleanup(from),
            Self::StructRef(x) => x.untyped().cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_from_native(from),
            Self::String(x) => x.convert_from_native(from),
            Self::ClassRef(x) => x.convert_from_native(from),
            Self::Struct(x) => x.convert_from_native(from),
            Self::StructRef(x) => x.untyped().convert_from_native(from),
        }
    }
}

impl DotnetType for StructDeclarationHandle {
    fn as_dotnet_type(&self) -> String {
        self.name.to_camel_case()
    }

    fn as_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.ToNativeRef({})",
            self.name.to_camel_case(),
            from
        ))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.NativeRefCleanup({});",
            self.name.to_camel_case(),
            from
        ))
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.FromNativeRef({})",
            self.name.to_camel_case(),
            from
        ))
    }
}

impl<T, D> DotnetType for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn as_dotnet_type(&self) -> String {
        self.name().to_camel_case()
    }

    fn as_native_type(&self) -> String {
        format!("{}Native", self.name().to_camel_case())
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.ToNative({})",
            self.name().to_camel_case(),
            from
        ))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!("{}.Dispose();", from))
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.FromNative({})",
            self.name().to_camel_case(),
            from
        ))
    }
}

impl<T, D> DotnetType for ReturnType<T, D>
where
    D: DocReference,
    T: Clone + DotnetType,
{
    fn as_dotnet_type(&self) -> String {
        match self {
            Self::Void => "void".to_string(),
            Self::Type(return_type, _) => return_type.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Void => "void".to_string(),
            Self::Type(return_type, _) => return_type.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Void => None,
            Self::Type(return_type, _) => return_type.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Void => None,
            Self::Type(return_type, _) => return_type.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Void => None,
            Self::Type(return_type, _) => return_type.convert_from_native(from),
        }
    }
}
