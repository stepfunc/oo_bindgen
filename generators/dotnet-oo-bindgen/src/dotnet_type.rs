use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::Collection;
use oo_bindgen::doc::DocReference;
use oo_bindgen::enum_type::Enum;
use oo_bindgen::function::*;
use oo_bindgen::interface::*;
use oo_bindgen::return_type::OptionalReturnType;
use oo_bindgen::structs::*;
use oo_bindgen::types::{Arg, BasicType, DurationType, StringType};
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
            .map(|x| x.arg_type.as_dotnet_type())
            .collect::<Vec<String>>()
            .join(", ")
    }

    match (&cb.return_type.get_value(), cb.arguments.as_slice()) {
        (None, []) => "Action".to_string(),
        (None, args) => {
            format!("Action<{}>", arg_types(args))
        }
        (Some(t), []) => {
            format!("Func<{}>", t.as_dotnet_type())
        }
        (Some(t), args) => {
            format!("Func<{}, {}>", arg_types(args), t.as_dotnet_type())
        }
    }
}

impl<D> DotnetType for Handle<Interface<D>>
where
    D: DocReference,
{
    fn as_dotnet_type(&self) -> String {
        if let Some(cb) = self.get_functional_callback() {
            if cb.functional_transform.enabled() {
                return full_functor_type(cb);
            }
        }

        format!("I{}", self.name.camel_case())
    }

    fn as_native_type(&self) -> String {
        format!("I{}NativeAdapter", self.name.camel_case())
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        let name = self.name.camel_case();
        let inner_transform = if let Some(cb) = self.get_functional_callback() {
            if cb.functional_transform.enabled() {
                format!("functional.{}.create({})", name, from)
            } else {
                from.to_string()
            }
        } else {
            from.to_string()
        };
        Some(format!("new I{}NativeAdapter({})", name, inner_transform))
    }

    fn cleanup(&self, _from: &str) -> Option<String> {
        None
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "I{}NativeAdapter.FromNative({}.{})",
            self.name.camel_case(),
            from,
            self.settings.interface.context_variable_name.mixed_case()
        ))
    }
}

impl DotnetType for ClassDeclarationHandle {
    fn as_dotnet_type(&self) -> String {
        self.name.camel_case()
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
        Some(format!("{}.FromNative({})", self.name.camel_case(), from))
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
            self.collection_class.name.camel_case(),
            from
        ))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.Cleanup({});",
            self.collection_class.name.camel_case(),
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
            self.item_type.name().camel_case()
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
            self.iter_class.name.camel_case(),
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
        self.name.camel_case()
    }

    fn as_native_type(&self) -> String {
        self.name.camel_case()
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
        self.name.camel_case()
    }

    fn as_native_type(&self) -> String {
        INT_PTR_STRING.to_string()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.ToNativeRef({})",
            self.name.camel_case(),
            from
        ))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.NativeRefCleanup({});",
            self.name.camel_case(),
            from
        ))
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.FromNativeRef({})",
            self.name.camel_case(),
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
        self.name().camel_case()
    }

    fn as_native_type(&self) -> String {
        format!("{}Native", self.name().camel_case())
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.ToNative({})",
            self.name().camel_case(),
            from
        ))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        Some(format!("{}.Dispose();", from))
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.FromNative({})",
            self.name().camel_case(),
            from
        ))
    }
}

const VOID: &str = "void";

impl<T, D> DotnetType for OptionalReturnType<T, D>
where
    D: DocReference,
    T: Clone + DotnetType,
{
    fn as_dotnet_type(&self) -> String {
        match self.get_value() {
            None => VOID.to_string(),
            Some(x) => x.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self.get_value() {
            None => VOID.to_string(),
            Some(x) => x.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        self.get_value().and_then(|x| x.convert_to_native(from))
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        self.get_value().and_then(|x| x.cleanup(from))
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        self.get_value().and_then(|x| x.convert_from_native(from))
    }
}
