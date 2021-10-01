use crate::native_enum::EnumHandle;
use crate::any_struct::{AnyStructFieldType, AnyStructHandle};

use crate::callback::InterfaceHandle;
use crate::class::ClassDeclarationHandle;
use crate::collection::CollectionHandle;
use crate::doc::DocString;
use crate::function_struct::FStructHandle;
use crate::iterator::IteratorHandle;
use crate::struct_common::StructDeclarationHandle;
use std::time::Duration;

/// Marker class used to denote the String type with conversions to more specialized types
#[derive(Copy, Clone, Debug)]
pub struct StringType;

pub const STRING_TYPE: StringType = StringType {};

/// Durations may be represented in multiple ways in the underlying C API
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DurationType {
    /// Duration is represented as a count of milliseconds in a u64 value
    Milliseconds,
    /// Duration is represented as a count of seconds in a u64 value
    Seconds,
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

impl From<DurationType> for AnyType {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

impl From<StringType> for AnyType {
    fn from(_: StringType) -> Self {
        AnyType::String
    }
}

impl From<FStructHandle> for AnyType {
    fn from(x: FStructHandle) -> Self {
        AnyType::Struct(x.to_any_struct())
    }
}

impl From<DurationType> for AnyStructFieldType {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

#[derive(Debug, Clone)]
pub struct Arg<T>
where
    T: Into<AnyType>,
{
    pub arg_type: T,
    pub name: String,
    pub doc: DocString,
}

impl<T> Arg<T>
where
    T: Into<AnyType> + Clone,
{
    pub fn new(arg_type: T, name: String, doc: DocString) -> Self {
        Self {
            arg_type,
            name,
            doc,
        }
    }
}

/// Basic types are trivially copyable. They can be used
/// in almost any context within the API model
#[derive(Debug, Clone, PartialEq)]
pub enum BasicType {
    Bool,
    Uint8,
    Sint8,
    Uint16,
    Sint16,
    Uint32,
    Sint32,
    Uint64,
    Sint64,
    Float,
    Double,
    Duration(DurationType),
    Enum(EnumHandle),
}

/// This is just sticking around until we refactor existing backends
#[derive(Debug, Clone, PartialEq)]
pub enum AnyType {
    Basic(BasicType),
    String,

    // Complex types
    Struct(AnyStructHandle),
    StructRef(StructDeclarationHandle),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
}

impl From<BasicType> for AnyType {
    fn from(t: BasicType) -> Self {
        AnyType::Basic(t)
    }
}

impl From<BasicType> for AnyStructFieldType {
    fn from(t: BasicType) -> Self {
        match t {
            BasicType::Bool => AnyStructFieldType::Bool(None),
            BasicType::Uint8 => AnyStructFieldType::Uint8(None),
            BasicType::Sint8 => AnyStructFieldType::Sint8(None),
            BasicType::Uint16 => AnyStructFieldType::Uint16(None),
            BasicType::Sint16 => AnyStructFieldType::Sint16(None),
            BasicType::Uint32 => AnyStructFieldType::Uint32(None),
            BasicType::Sint32 => AnyStructFieldType::Sint32(None),
            BasicType::Uint64 => AnyStructFieldType::Uint64(None),
            BasicType::Sint64 => AnyStructFieldType::Sint64(None),
            BasicType::Float => AnyStructFieldType::Float(None),
            BasicType::Double => AnyStructFieldType::Double(None),
            BasicType::Duration(mapping) => AnyStructFieldType::Duration(mapping, None),
            BasicType::Enum(handle) => AnyStructFieldType::Enum(handle, None),
        }
    }
}
