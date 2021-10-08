use heck::{CamelCase, MixedCase};
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::function::*;
use oo_bindgen::interface::*;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::callback_struct::CStructFieldType;
use oo_bindgen::structs::common::{Struct, StructDeclarationHandle, StructFieldType};
use oo_bindgen::structs::function_return_struct::RStructFieldType;
use oo_bindgen::structs::function_struct::FStructFieldType;
use oo_bindgen::structs::univeral_struct::UStructFieldType;
use oo_bindgen::types::{BasicType, DurationType, StringType};
use oo_bindgen::Handle;

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
            Self::Uint8 => "byte".to_string(),
            Self::Sint8 => "sbyte".to_string(),
            Self::Uint16 => "ushort".to_string(),
            Self::Sint16 => "short".to_string(),
            Self::Uint32 => "uint".to_string(),
            Self::Sint32 => "int".to_string(),
            Self::Uint64 => "ulong".to_string(),
            Self::Sint64 => "long".to_string(),
            Self::Float32 => "float".to_string(),
            Self::Double64 => "double".to_string(),
            Self::Duration(x) => x.as_dotnet_type(),
            Self::Enum(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Bool => "byte".to_string(),
            Self::Uint8 => "byte".to_string(),
            Self::Sint8 => "sbyte".to_string(),
            Self::Uint16 => "ushort".to_string(),
            Self::Sint16 => "short".to_string(),
            Self::Uint32 => "uint".to_string(),
            Self::Sint32 => "int".to_string(),
            Self::Uint64 => "ulong".to_string(),
            Self::Sint64 => "long".to_string(),
            Self::Float32 => "float".to_string(),
            Self::Double64 => "double".to_string(),
            Self::Duration(x) => x.as_native_type(),
            Self::Enum(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("Convert.ToByte({})", from)),
            Self::Uint8 => None,
            Self::Sint8 => None,
            Self::Uint16 => None,
            Self::Sint16 => None,
            Self::Uint32 => None,
            Self::Sint32 => None,
            Self::Uint64 => None,
            Self::Sint64 => None,
            Self::Float32 => None,
            Self::Double64 => None,
            Self::Duration(x) => x.convert_to_native(from),
            Self::Enum(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => None,
            Self::Uint8 => None,
            Self::Sint8 => None,
            Self::Uint16 => None,
            Self::Sint16 => None,
            Self::Uint32 => None,
            Self::Sint32 => None,
            Self::Uint64 => None,
            Self::Sint64 => None,
            Self::Float32 => None,
            Self::Double64 => None,
            Self::Duration(x) => x.cleanup(from),
            Self::Enum(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("Convert.ToBoolean({})", from)),
            Self::Uint8 => None,
            Self::Sint8 => None,
            Self::Uint16 => None,
            Self::Sint16 => None,
            Self::Uint32 => None,
            Self::Sint32 => None,
            Self::Uint64 => None,
            Self::Sint64 => None,
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

impl DotnetType for InterfaceHandle {
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

impl DotnetType for CollectionHandle {
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
            self.collection_type.name.to_camel_case(),
            from
        ))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.Cleanup({});",
            self.collection_type.name.to_camel_case(),
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

impl DotnetType for IteratorHandle {
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
            self.iter_type.name.to_camel_case(),
            from
        ))
    }
}

impl DotnetType for FStructFieldType {
    fn as_dotnet_type(&self) -> String {
        match self {
            FStructFieldType::Basic(x) => x.as_dotnet_type(),
            FStructFieldType::String(x) => x.as_dotnet_type(),
            FStructFieldType::Interface(x) => x.as_dotnet_type(),
            FStructFieldType::Collection(x) => x.as_dotnet_type(),
            FStructFieldType::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            FStructFieldType::Basic(x) => x.as_native_type(),
            FStructFieldType::String(x) => x.as_native_type(),
            FStructFieldType::Interface(x) => x.as_native_type(),
            FStructFieldType::Collection(x) => x.as_native_type(),
            FStructFieldType::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            FStructFieldType::Basic(x) => x.convert_to_native(from),
            FStructFieldType::String(x) => x.convert_to_native(from),
            FStructFieldType::Interface(x) => x.convert_to_native(from),
            FStructFieldType::Collection(x) => x.convert_to_native(from),
            FStructFieldType::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            FStructFieldType::Basic(x) => x.cleanup(from),
            FStructFieldType::String(x) => x.cleanup(from),
            FStructFieldType::Interface(x) => x.cleanup(from),
            FStructFieldType::Collection(x) => x.cleanup(from),
            FStructFieldType::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            FStructFieldType::Basic(x) => x.convert_from_native(from),
            FStructFieldType::String(x) => x.convert_from_native(from),
            FStructFieldType::Interface(x) => x.convert_from_native(from),
            FStructFieldType::Collection(x) => x.convert_from_native(from),
            FStructFieldType::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for RStructFieldType {
    fn as_dotnet_type(&self) -> String {
        match self {
            RStructFieldType::Basic(x) => x.as_dotnet_type(),
            RStructFieldType::ClassRef(x) => x.as_dotnet_type(),
            RStructFieldType::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            RStructFieldType::Basic(x) => x.as_native_type(),
            RStructFieldType::ClassRef(x) => x.as_native_type(),
            RStructFieldType::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            RStructFieldType::Basic(x) => x.convert_to_native(from),
            RStructFieldType::ClassRef(x) => x.convert_to_native(from),
            RStructFieldType::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            RStructFieldType::Basic(x) => x.cleanup(from),
            RStructFieldType::ClassRef(x) => x.cleanup(from),
            RStructFieldType::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            RStructFieldType::Basic(x) => x.convert_from_native(from),
            RStructFieldType::ClassRef(x) => x.convert_from_native(from),
            RStructFieldType::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for CStructFieldType {
    fn as_dotnet_type(&self) -> String {
        match self {
            CStructFieldType::Basic(x) => x.as_dotnet_type(),
            CStructFieldType::Iterator(x) => x.as_dotnet_type(),
            CStructFieldType::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            CStructFieldType::Basic(x) => x.as_native_type(),
            CStructFieldType::Iterator(x) => x.as_native_type(),
            CStructFieldType::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            CStructFieldType::Basic(x) => x.convert_to_native(from),
            CStructFieldType::Iterator(x) => x.convert_to_native(from),
            CStructFieldType::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            CStructFieldType::Basic(x) => x.cleanup(from),
            CStructFieldType::Iterator(x) => x.cleanup(from),
            CStructFieldType::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            CStructFieldType::Basic(x) => x.convert_from_native(from),
            CStructFieldType::Iterator(x) => x.convert_from_native(from),
            CStructFieldType::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for UStructFieldType {
    fn as_dotnet_type(&self) -> String {
        match self {
            UStructFieldType::Basic(x) => x.as_dotnet_type(),
            UStructFieldType::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            UStructFieldType::Basic(x) => x.as_native_type(),
            UStructFieldType::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            UStructFieldType::Basic(x) => x.convert_to_native(from),
            UStructFieldType::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            UStructFieldType::Basic(x) => x.cleanup(from),
            UStructFieldType::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            UStructFieldType::Basic(x) => x.convert_from_native(from),
            UStructFieldType::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for EnumHandle {
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

impl DotnetType for FArgument {
    fn as_dotnet_type(&self) -> String {
        match self {
            FArgument::Basic(x) => x.as_dotnet_type(),
            FArgument::String(x) => x.as_dotnet_type(),
            FArgument::Collection(x) => x.as_dotnet_type(),
            FArgument::Struct(x) => x.as_dotnet_type(),
            FArgument::StructRef(x) => x.as_dotnet_type(),
            FArgument::ClassRef(x) => x.as_dotnet_type(),
            FArgument::Interface(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            FArgument::Basic(x) => x.as_native_type(),
            FArgument::String(x) => x.as_native_type(),
            FArgument::Collection(x) => x.as_native_type(),
            FArgument::Struct(x) => x.as_native_type(),
            FArgument::StructRef(x) => x.as_native_type(),
            FArgument::ClassRef(x) => x.as_native_type(),
            FArgument::Interface(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            FArgument::Basic(x) => x.convert_to_native(from),
            FArgument::String(x) => x.convert_to_native(from),
            FArgument::Collection(x) => x.convert_to_native(from),
            FArgument::Struct(x) => x.convert_to_native(from),
            FArgument::StructRef(x) => x.convert_to_native(from),
            FArgument::ClassRef(x) => x.convert_to_native(from),
            FArgument::Interface(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            FArgument::Basic(x) => x.cleanup(from),
            FArgument::String(x) => x.cleanup(from),
            FArgument::Collection(x) => x.cleanup(from),
            FArgument::Struct(x) => x.cleanup(from),
            FArgument::StructRef(x) => x.cleanup(from),
            FArgument::ClassRef(x) => x.cleanup(from),
            FArgument::Interface(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            FArgument::Basic(x) => x.convert_from_native(from),
            FArgument::String(x) => x.convert_from_native(from),
            FArgument::Collection(x) => x.convert_from_native(from),
            FArgument::Struct(x) => x.convert_from_native(from),
            FArgument::StructRef(x) => x.convert_from_native(from),
            FArgument::ClassRef(x) => x.convert_from_native(from),
            FArgument::Interface(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for CArgument {
    fn as_dotnet_type(&self) -> String {
        match self {
            CArgument::Basic(x) => x.as_dotnet_type(),
            CArgument::String(x) => x.as_dotnet_type(),
            CArgument::Iterator(x) => x.as_dotnet_type(),
            CArgument::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            CArgument::Basic(x) => x.as_native_type(),
            CArgument::String(x) => x.as_native_type(),
            CArgument::Iterator(x) => x.as_native_type(),
            CArgument::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            CArgument::Basic(x) => x.convert_to_native(from),
            CArgument::String(x) => x.convert_to_native(from),
            CArgument::Iterator(x) => x.convert_to_native(from),
            CArgument::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            CArgument::Basic(x) => x.cleanup(from),
            CArgument::String(x) => x.cleanup(from),
            CArgument::Iterator(x) => x.cleanup(from),
            CArgument::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            CArgument::Basic(x) => x.convert_from_native(from),
            CArgument::String(x) => x.convert_from_native(from),
            CArgument::Iterator(x) => x.convert_from_native(from),
            CArgument::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for CReturnValue {
    fn as_dotnet_type(&self) -> String {
        match self {
            CReturnValue::Basic(x) => x.as_dotnet_type(),
            CReturnValue::Struct(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            CReturnValue::Basic(x) => x.as_native_type(),
            CReturnValue::Struct(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            CReturnValue::Basic(x) => x.convert_to_native(from),
            CReturnValue::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            CReturnValue::Basic(x) => x.cleanup(from),
            CReturnValue::Struct(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            CReturnValue::Basic(x) => x.convert_from_native(from),
            CReturnValue::Struct(x) => x.convert_from_native(from),
        }
    }
}

impl DotnetType for FReturnValue {
    fn as_dotnet_type(&self) -> String {
        match self {
            FReturnValue::Basic(x) => x.as_dotnet_type(),
            FReturnValue::String(x) => x.as_dotnet_type(),
            FReturnValue::ClassRef(x) => x.as_dotnet_type(),
            FReturnValue::Struct(x) => x.as_dotnet_type(),
            FReturnValue::StructRef(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            FReturnValue::Basic(x) => x.as_native_type(),
            FReturnValue::String(x) => x.as_native_type(),
            FReturnValue::ClassRef(x) => x.as_native_type(),
            FReturnValue::Struct(x) => x.as_native_type(),
            FReturnValue::StructRef(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            FReturnValue::Basic(x) => x.convert_to_native(from),
            FReturnValue::String(x) => x.convert_to_native(from),
            FReturnValue::ClassRef(x) => x.convert_to_native(from),
            FReturnValue::Struct(x) => x.convert_to_native(from),
            FReturnValue::StructRef(x) => x.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            FReturnValue::Basic(x) => x.cleanup(from),
            FReturnValue::String(x) => x.cleanup(from),
            FReturnValue::ClassRef(x) => x.cleanup(from),
            FReturnValue::Struct(x) => x.cleanup(from),
            FReturnValue::StructRef(x) => x.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            FReturnValue::Basic(x) => x.convert_from_native(from),
            FReturnValue::String(x) => x.convert_from_native(from),
            FReturnValue::ClassRef(x) => x.convert_from_native(from),
            FReturnValue::Struct(x) => x.convert_from_native(from),
            FReturnValue::StructRef(x) => x.convert_from_native(from),
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

impl<T> DotnetType for Handle<Struct<T>>
where
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

impl<T> DotnetType for ReturnType<T>
where
    T: DotnetType,
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
