use std::time::Duration;

use crate::model::*;

/// Marker class used to denote the String type with conversions to more specialized types
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StringType;

/// Durations may be represented in multiple ways in the underlying C API
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum DurationType {
    /// Duration is represented as a count of milliseconds in a u64 value
    Milliseconds,
    /// Duration is represented as a count of seconds in a u64 value
    Seconds,
}

/// Same as DurationType but with an associated value
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum DurationValue {
    /// Duration is represented as a count of milliseconds in a u64 value
    Milliseconds(u64),
    /// Duration is represented as a count of seconds in a u64 value
    Seconds(u64),
}

impl From<DurationValue> for DurationType {
    fn from(x: DurationValue) -> Self {
        match x {
            DurationValue::Milliseconds(_) => DurationType::Milliseconds,
            DurationValue::Seconds(_) => DurationType::Seconds,
        }
    }
}

impl DurationType {
    pub fn unit(&self) -> &'static str {
        match self {
            DurationType::Milliseconds => "milliseconds",
            DurationType::Seconds => "seconds",
        }
    }

    pub fn get_value_string(&self, duration: Duration) -> String {
        match self {
            DurationType::Milliseconds => {
                format!("{}", duration.as_millis())
            }
            DurationType::Seconds => {
                format!("{}", duration.as_secs())
            }
        }
    }
}

impl From<DurationType> for BasicType {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x)
    }
}

#[derive(Debug, Clone)]
pub struct Arg<T, D>
where
    T: Clone,
    D: DocReference,
{
    pub(crate) arg_type: T,
    pub(crate) name: Name,
    pub(crate) doc: DocString<D>,
}

impl<T> Arg<T, Unvalidated>
where
    T: Clone,
{
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Arg<T, Validated>> {
        Ok(Arg {
            arg_type: self.arg_type.clone(),
            name: self.name.clone(),
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

impl<T, D> Arg<T, D>
where
    T: Clone,
    D: DocReference,
{
    pub(crate) fn new(arg_type: T, name: Name, doc: DocString<D>) -> Self {
        Self {
            arg_type,
            name,
            doc,
        }
    }
}

/// primitive types in most languages
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Primitive {
    Bool,
    U8,
    S8,
    U16,
    S16,
    U32,
    S32,
    U64,
    S64,
    Float,
    Double,
}

/// same as primitive, but with a value
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum PrimitiveValue {
    Bool(bool),
    U8(u8),
    S8(i8),
    U16(u16),
    S16(i16),
    U32(u32),
    S32(i32),
    U64(u64),
    S64(i64),
    Float(f32),
    Double(f64),
}

impl From<PrimitiveValue> for Primitive {
    fn from(x: PrimitiveValue) -> Self {
        match x {
            PrimitiveValue::Bool(_) => Primitive::Bool,
            PrimitiveValue::U8(_) => Primitive::U8,
            PrimitiveValue::S8(_) => Primitive::S8,
            PrimitiveValue::U16(_) => Primitive::U16,
            PrimitiveValue::S16(_) => Primitive::S16,
            PrimitiveValue::U32(_) => Primitive::U32,
            PrimitiveValue::S32(_) => Primitive::S32,
            PrimitiveValue::U64(_) => Primitive::U64,
            PrimitiveValue::S64(_) => Primitive::S64,
            PrimitiveValue::Float(_) => Primitive::Float,
            PrimitiveValue::Double(_) => Primitive::Double,
        }
    }
}

/// Basic types are trivially copyable. They can be used
/// in almost any context within the API model
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicType {
    Primitive(Primitive),
    Duration(DurationType),
    Enum(Handle<Enum<Unvalidated>>),
}

impl From<Primitive> for BasicType {
    fn from(x: Primitive) -> Self {
        BasicType::Primitive(x)
    }
}

impl InitializerValidator for Primitive {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match self {
            Self::U8 => match value {
                InitializerDefault::Numeric(NumberValue::U8(x)) => Ok(NumberValue::U8(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "u8".to_string(),
                    value: value.clone(),
                }),
            },
            Self::S8 => match value {
                InitializerDefault::Numeric(NumberValue::S8(x)) => Ok(NumberValue::S8(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "i8".to_string(),
                    value: value.clone(),
                }),
            },
            Self::U16 => match value {
                InitializerDefault::Numeric(NumberValue::U16(x)) => Ok(NumberValue::U16(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "u16".to_string(),
                    value: value.clone(),
                }),
            },
            Self::S16 => match value {
                InitializerDefault::Numeric(NumberValue::S16(x)) => Ok(NumberValue::S16(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "i16".to_string(),
                    value: value.clone(),
                }),
            },
            Self::U32 => match value {
                InitializerDefault::Numeric(NumberValue::U32(x)) => Ok(NumberValue::U32(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "u32".to_string(),
                    value: value.clone(),
                }),
            },
            Self::S32 => match value {
                InitializerDefault::Numeric(NumberValue::S32(x)) => Ok(NumberValue::S32(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "i32".to_string(),
                    value: value.clone(),
                }),
            },
            Self::U64 => match value {
                InitializerDefault::Numeric(NumberValue::U64(x)) => Ok(NumberValue::U64(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "u64".to_string(),
                    value: value.clone(),
                }),
            },
            Self::S64 => match value {
                InitializerDefault::Numeric(NumberValue::S64(x)) => Ok(NumberValue::S64(*x).into()),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "i64".to_string(),
                    value: value.clone(),
                }),
            },
            Self::Float => match value {
                InitializerDefault::Numeric(NumberValue::Float(x)) => {
                    Ok(NumberValue::Float(*x).into())
                }
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "f32".to_string(),
                    value: value.clone(),
                }),
            },
            Self::Double => match value {
                InitializerDefault::Numeric(NumberValue::Double(x)) => {
                    Ok(NumberValue::Double(*x).into())
                }
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "f64".to_string(),
                    value: value.clone(),
                }),
            },
            Primitive::Bool => match value {
                InitializerDefault::Bool(x) => Ok(ValidatedDefaultValue::Bool(*x)),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "bool".to_string(),
                    value: value.clone(),
                }),
            },
        }
    }
}

impl InitializerValidator for BasicType {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match self {
            BasicType::Primitive(x) => x.validate_default_value(value),
            BasicType::Duration(dt) => match value {
                InitializerDefault::Duration(x) => Ok(ValidatedDefaultValue::Duration(*dt, *x)),
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "Duration".to_string(),
                    value: value.clone(),
                }),
            },
            BasicType::Enum(handle) => match value {
                InitializerDefault::Enum(value) => {
                    handle.validate_contains_variant_name(value)?;
                    Ok(ValidatedDefaultValue::Enum(
                        handle.clone(),
                        Name::create(value)?,
                    ))
                }
                _ => Err(BindingError::StructInitializerBadValueForType {
                    field_type: "Enum".to_string(),
                    value: value.clone(),
                }),
            },
        }
    }
}

impl Primitive {
    /// get the string representation of the type used in the Rust for the C FFI
    pub(crate) fn get_c_rust_type(&self) -> &str {
        match self {
            Self::Bool => "bool",
            Self::U8 => "u8",
            Self::S8 => "i8",
            Self::U16 => "u16",
            Self::S16 => "i16",
            Self::U32 => "u32",
            Self::S32 => "i32",
            Self::U64 => "u64",
            Self::S64 => "i64",
            Self::Float => "f32",
            Self::Double => "f64",
        }
    }
}

impl BasicType {
    /// get the string representation of the type used in the Rust for the C FFI
    pub(crate) fn get_c_rust_type(&self) -> &str {
        match self {
            Self::Primitive(x) => x.get_c_rust_type(),
            Self::Duration(_) => "u64",
            Self::Enum(_) => "std::os::raw::c_int",
        }
    }
}

pub(crate) trait TypeExtractor {
    fn get_basic_type(&self) -> Option<&BasicType>;

    fn get_duration_type(&self) -> Option<DurationType> {
        match self.get_basic_type() {
            Some(BasicType::Duration(x)) => Some(*x),
            _ => None,
        }
    }

    fn get_enum_type(&self) -> Option<Handle<Enum<Unvalidated>>> {
        match self.get_basic_type() {
            Some(BasicType::Enum(x)) => Some(x.clone()),
            _ => None,
        }
    }
}

impl TypeExtractor for FunctionArgStructField {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}

impl TypeExtractor for FunctionReturnStructField {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}

impl TypeExtractor for CallbackArgStructField {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}

impl TypeExtractor for UniversalStructField {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}
