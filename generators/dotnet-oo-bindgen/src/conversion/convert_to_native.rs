use oo_bindgen::model::*;

/// Conversion from .NET types to native types
pub(crate) trait ConvertToNative {
    /// Convert the .NET type to a native type that may require cleanup
    fn convert_to_native(&self, from: &str) -> Option<String>;
    /// Cleanup the native type
    fn cleanup_native(&self, from: &str) -> Option<String>;
}

impl ConvertToNative for DurationType {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Milliseconds => Some(format!("(ulong){}.TotalMilliseconds", from)),
            Self::Seconds => Some(format!("(ulong){}.TotalSeconds", from)),
        }
    }

    fn cleanup_native(&self, _from: &str) -> Option<String> {
        None
    }
}

impl ConvertToNative for Primitive {
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
            Self::Float => None,
            Self::Double => None,
        }
    }

    fn cleanup_native(&self, _from: &str) -> Option<String> {
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
            Self::Float => None,
            Self::Double => None,
        }
    }
}

impl<D> ConvertToNative for Handle<Enum<D>>
where
    D: DocReference,
{
    fn convert_to_native(&self, _: &str) -> Option<String> {
        None
    }

    fn cleanup_native(&self, _: &str) -> Option<String> {
        None
    }
}

impl ConvertToNative for BasicType {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Primitive(x) => x.convert_to_native(from),
            Self::Duration(x) => x.convert_to_native(from),
            Self::Enum(x) => x.convert_to_native(from),
        }
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Primitive(x) => x.cleanup_native(from),
            Self::Duration(x) => x.cleanup_native(from),
            Self::Enum(x) => x.cleanup_native(from),
        }
    }
}

impl ConvertToNative for StringType {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!("Helpers.RustString.ToNative({})", from))
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        Some(format!("Helpers.RustString.Destroy({});", from))
    }
}

impl<D> ConvertToNative for Handle<Interface<D>>
where
    D: DocReference,
{
    fn convert_to_native(&self, from: &str) -> Option<String> {
        let name = self.name.camel_case();
        let inner_transform = if let Some(cb) = self.get_functional_callback() {
            match self.mode {
                InterfaceCategory::Synchronous | InterfaceCategory::Asynchronous => {
                    if cb.functional_transform.enabled() {
                        format!("functional.{}.create({})", name, from)
                    } else {
                        from.to_string()
                    }
                }
                InterfaceCategory::Future => {
                    // we don't perform functional transforms on future interfaces
                    from.to_string()
                }
            }
        } else {
            from.to_string()
        };
        Some(format!("new I{}NativeAdapter({})", name, inner_transform))
    }

    fn cleanup_native(&self, _from: &str) -> Option<String> {
        None
    }
}

impl ConvertToNative for ClassDeclarationHandle {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!("{}.self", from))
    }

    fn cleanup_native(&self, _: &str) -> Option<String> {
        None
    }
}

impl<D> ConvertToNative for Handle<Collection<D>>
where
    D: DocReference,
{
    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.ToNative({})",
            self.collection_class.name.camel_case(),
            from
        ))
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Helpers.Cleanup({});",
            self.collection_class.name.camel_case(),
            from
        ))
    }
}

impl ConvertToNative for FunctionArgStructField {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgStructField::Basic(x) => x.convert_to_native(from),
            FunctionArgStructField::String(x) => x.convert_to_native(from),
            FunctionArgStructField::Interface(x) => x.inner.convert_to_native(from),
            FunctionArgStructField::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgStructField::Basic(x) => x.cleanup_native(from),
            FunctionArgStructField::String(x) => x.cleanup_native(from),
            FunctionArgStructField::Interface(x) => x.inner.cleanup_native(from),
            FunctionArgStructField::Struct(x) => x.cleanup_native(from),
        }
    }
}

impl ConvertToNative for UniversalStructField {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.convert_to_native(from),
            UniversalStructField::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.cleanup_native(from),
            UniversalStructField::Struct(x) => x.cleanup_native(from),
        }
    }
}

impl ConvertToNative for FunctionArgument {
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

    fn cleanup_native(&self, from: &str) -> Option<String> {
        match self {
            FunctionArgument::Basic(x) => x.cleanup_native(from),
            FunctionArgument::String(x) => x.cleanup_native(from),
            FunctionArgument::Collection(x) => x.cleanup_native(from),
            FunctionArgument::Struct(x) => x.cleanup_native(from),
            FunctionArgument::StructRef(x) => x.inner.cleanup_native(from),
            FunctionArgument::ClassRef(x) => x.cleanup_native(from),
            FunctionArgument::Interface(x) => x.cleanup_native(from),
        }
    }
}

impl ConvertToNative for CallbackReturnValue {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_native(from),
            Self::Struct(x) => x.convert_to_native(from),
        }
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.cleanup_native(from),
            Self::Struct(x) => x.cleanup_native(from),
        }
    }
}

impl ConvertToNative for UniversalOr<FunctionArgStructField> {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.convert_to_native(from),
            UniversalOr::Universal(x) => x.convert_to_native(from),
        }
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.cleanup_native(from),
            UniversalOr::Universal(x) => x.cleanup_native(from),
        }
    }
}

impl ConvertToNative for StructDeclarationHandle {
    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.ToNativeRef({})",
            self.name.camel_case(),
            from
        ))
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.NativeRefCleanup({});",
            self.name.camel_case(),
            from
        ))
    }
}

impl<T, D> ConvertToNative for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn convert_to_native(&self, from: &str) -> Option<String> {
        Some(format!(
            "{}Native.ToNative({})",
            self.name().camel_case(),
            from
        ))
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        Some(format!("{}.Dispose();", from))
    }
}

/*
impl<T, D> ToNative for OptionalReturnType<T, D>
    where
        D: DocReference,
        T: Clone + ToNative,
{
    fn to_native(&self, from: &str) -> Option<String> {
        self.get_value().and_then(|x| x.to_native(from))
    }

    fn cleanup_native(&self, from: &str) -> Option<String> {
        self.get_value().and_then(|x| x.cleanup_native(from))
    }
}
*/
