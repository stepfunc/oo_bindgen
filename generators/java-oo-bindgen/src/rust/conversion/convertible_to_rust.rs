use oo_bindgen::model::*;

/// Trait used for converting types from JNI to Rust
///
/// Conversion happens in two phases:
///
/// 1) `to_rust` is called to do a primary conversion from the JNI type to the Rust type. If a
/// conversion is required, it will frequently be used to shadow the variable.
///
/// 2) `call_site` is called to do a secondary conversion to extract the final type passed to
/// the native function. This is generally used to get an inner type from some RAII type, e.g.
/// JavaString.
///
/// Conversions assume that there are two variables in scope:
///
/// * _env - JNIEnv
/// * _cache - Pre-allocated JniCache
///
pub(crate) trait ConvertibleToRust {
    /// Optionally, convert an expression (variable) to a primary type
    /// This usually takes the form of a shadowed variable
    ///
    /// **note:** This should only be used in contexts where primitives are unboxed.
    fn to_rust(&self, expr: &str) -> Option<String>;

    /// Same as `to_rust` but used in contexts where primitives are always boxed, e.g.
    /// when converting values in a Java list to a native collection type
    fn to_rust_from_object(&self, expr: &str) -> Option<String> {
        self.to_rust(expr)
    }

    /// Optional, convert an expression to another type at the function call site
    fn call_site(&self, expr: &str) -> Option<String>;
}

impl ConvertibleToRust for StringType {
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(format!("_env.get_string({}.into()).unwrap()", expr))
    }

    fn call_site(&self, expr: &str) -> Option<String> {
        Some(format!("{}.as_ptr()", expr))
    }
}

impl ConvertibleToRust for Primitive {
    fn to_rust(&self, expr: &str) -> Option<String> {
        match self {
            Primitive::Bool => Some(format!("_cache.primitives.boolean_value(&_env, {})", expr)),
            Primitive::U8 => Some(format!("_cache.joou.ubyte_to_rust(&_env, {})", expr)),
            Primitive::S8 => {
                None
                //Some(format!("_cache.primitives.byte_value(&_env, {})", expr))
            }
            Primitive::U16 => Some(format!("_cache.joou.ushort_to_rust(&_env, {})", expr)),
            Primitive::S16 => {
                None
                //Some(format!("_cache.primitives.short_value(&_env, {})", expr))
            }
            Primitive::U32 => Some(format!("_cache.joou.uinteger_to_rust(&_env, {})", expr)),
            Primitive::S32 => {
                None
                //Some(format!("_cache.primitives.integer_value(&_env, {})", expr))
            }
            Primitive::U64 => Some(format!("_cache.joou.ulong_to_rust(&_env, {})", expr)),
            Primitive::S64 => {
                None
                //Some(format!("_cache.primitives.long_value(&_env, {})", expr))
            }
            Primitive::Float => Some(format!("_cache.primitives.float_value(&_env, {})", expr)),
            Primitive::Double => Some(format!("_cache.primitives.double_value(&_env, {})", expr)),
        }
    }

    fn to_rust_from_object(&self, expr: &str) -> Option<String> {
        match self {
            Primitive::Bool => self.to_rust(expr),
            Primitive::U8 => self.to_rust(expr),
            Primitive::S8 => Some(format!("_cache.primitives.byte_value(&_env, {})", expr)),
            Primitive::U16 => self.to_rust(expr),
            Primitive::S16 => Some(format!("_cache.primitives.short_value(&_env, {})", expr)),
            Primitive::U32 => self.to_rust(expr),
            Primitive::S32 => Some(format!("_cache.primitives.integer_value(&_env, {})", expr)),
            Primitive::U64 => self.to_rust(expr),
            Primitive::S64 => Some(format!("_cache.primitives.long_value(&_env, {})", expr)),
            Primitive::Float => self.to_rust(expr),
            Primitive::Double => self.to_rust(expr),
        }
    }

    fn call_site(&self, _expr: &str) -> Option<String> {
        match self {
            Primitive::Bool => None,
            Primitive::U8 => None,
            Primitive::S8 => None,
            Primitive::U16 => None,
            Primitive::S16 => None,
            Primitive::U32 => None,
            Primitive::S32 => None,
            Primitive::U64 => None,
            Primitive::S64 => None,
            Primitive::Float => None,
            Primitive::Double => None,
        }
    }
}

impl ConvertibleToRust for DurationType {
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(match self {
            DurationType::Milliseconds => {
                format!("_cache.duration.duration_to_millis(&_env, {})", expr)
            }
            DurationType::Seconds => {
                format!("_cache.duration.duration_to_seconds(&_env, {})", expr)
            }
        })
    }

    fn call_site(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl ConvertibleToRust for Handle<Enum<Unvalidated>> {
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(format!(
            "_cache.enums.enum_{}.enum_to_rust(&_env, {})",
            self.name, expr
        ))
    }

    fn call_site(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl ConvertibleToRust for BasicType {
    fn to_rust(&self, expr: &str) -> Option<String> {
        match self {
            BasicType::Primitive(x) => x.to_rust(expr),
            BasicType::Duration(x) => x.to_rust(expr),
            BasicType::Enum(x) => x.to_rust(expr),
        }
    }

    fn call_site(&self, expr: &str) -> Option<String> {
        match self {
            BasicType::Primitive(x) => x.call_site(expr),
            BasicType::Duration(x) => x.call_site(expr),
            BasicType::Enum(x) => x.call_site(expr),
        }
    }
}

impl ConvertibleToRust for CollectionHandle {
    fn to_rust(&self, expr: &str) -> Option<String> {
        // create the helper guard object that allocates and fills the native collection from the list
        Some(format!(
            "helpers::{}::new(_env, {}).unwrap()",
            self.collection_class.name.camel_case(),
            expr
        ))
    }

    fn call_site(&self, expr: &str) -> Option<String> {
        // use the inner native collection type for the function call
        Some(format!("*{}", expr))
    }
}

impl ConvertibleToRust for InterfaceHandle {
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(format!(
            "_cache.interfaces.interface_{}.interface_to_rust(&_env, {})",
            self.name, expr
        ))
    }

    fn call_site(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl ConvertibleToRust for FunctionArgStructDeclaration {
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(format!(
            "_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {})",
            self.inner.name, expr
        ))
    }

    fn call_site(&self, expr: &str) -> Option<String> {
        // mutably borrow the converted struct, there is an implicit conversion to *mut
        Some(format!("&{}", expr))
    }
}

impl<T> ConvertibleToRust for Handle<Struct<T, Unvalidated>>
where
    T: StructFieldType,
{
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(format!(
            "_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {})",
            self.name(),
            expr
        ))
    }

    fn call_site(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl ConvertibleToRust for UniversalOr<FunctionArgStructField> {
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(format!(
            "_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {})",
            self.name(),
            expr
        ))
    }

    fn call_site(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl ConvertibleToRust for ClassDeclarationHandle {
    fn to_rust(&self, expr: &str) -> Option<String> {
        Some(format!(
            "_cache.classes.{}_to_rust(&_env, {})",
            self.name, expr
        ))
    }

    fn call_site(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl ConvertibleToRust for FunctionArgument {
    fn to_rust(&self, expr: &str) -> Option<String> {
        match self {
            FunctionArgument::Basic(x) => x.to_rust(expr),
            FunctionArgument::String(x) => x.to_rust(expr),
            FunctionArgument::Collection(x) => x.to_rust(expr),
            FunctionArgument::Struct(x) => x.to_rust(expr),
            FunctionArgument::StructRef(x) => x.to_rust(expr),
            FunctionArgument::ClassRef(x) => x.to_rust(expr),
            FunctionArgument::Interface(x) => x.to_rust(expr),
        }
    }

    fn call_site(&self, expr: &str) -> Option<String> {
        match self {
            FunctionArgument::Basic(x) => x.call_site(expr),
            FunctionArgument::String(x) => x.call_site(expr),
            FunctionArgument::Collection(x) => x.call_site(expr),
            FunctionArgument::Struct(x) => x.call_site(expr),
            FunctionArgument::StructRef(x) => x.call_site(expr),
            FunctionArgument::ClassRef(x) => x.call_site(expr),
            FunctionArgument::Interface(x) => x.call_site(expr),
        }
    }
}

impl ConvertibleToRust for CallbackReturnValue {
    fn to_rust(&self, expr: &str) -> Option<String> {
        match self {
            CallbackReturnValue::Basic(x) => x.to_rust(expr),
            CallbackReturnValue::Struct(x) => x.to_rust(expr),
        }
    }

    fn call_site(&self, expr: &str) -> Option<String> {
        match self {
            CallbackReturnValue::Basic(x) => x.call_site(expr),
            CallbackReturnValue::Struct(x) => x.call_site(expr),
        }
    }
}
