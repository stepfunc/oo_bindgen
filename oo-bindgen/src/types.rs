use crate::native_function::Type;
use crate::native_struct::StructElementType;
use crate::native_enum::NativeEnumHandle;

use std::time::Duration;

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

impl From<DurationType> for Type {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
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
    Enum(NativeEnumHandle)
}

impl From<BasicType> for Type {
    fn from(t: BasicType) -> Self {
        Type::Basic(t)
    }
}

impl From<BasicType> for StructElementType {
    fn from(t: BasicType) -> Self {
        match t {
            BasicType::Bool => StructElementType::Bool(None),
            BasicType::Uint8 => StructElementType::Uint8(None),
            BasicType::Sint8 => StructElementType::Sint8(None),
            BasicType::Uint16 => StructElementType::Uint16(None),
            BasicType::Sint16 => StructElementType::Sint16(None),
            BasicType::Uint32 => StructElementType::Uint32(None),
            BasicType::Sint32 => StructElementType::Sint32(None),
            BasicType::Uint64 => StructElementType::Uint64(None),
            BasicType::Sint64 => StructElementType::Sint64(None),
            BasicType::Float => StructElementType::Float(None),
            BasicType::Double => StructElementType::Double(None),
            BasicType::Duration(mapping) => StructElementType::Duration(mapping, None),
            BasicType::Enum(handle) => StructElementType::Enum(handle, None),
        }
    }
}
