use crate::enum_type::EnumHandle;

use crate::doc::DocString;
use std::time::Duration;
use crate::StructType;
use crate::structs::common::StructDeclarationHandle;
use crate::interface::InterfaceHandle;
use crate::class::ClassDeclarationHandle;
use crate::iterator::IteratorHandle;
use crate::collection::CollectionHandle;
use crate::structs::function_struct::FStructFieldType;
use crate::structs::function_return_struct::RStructFieldType;
use crate::structs::callback_struct::CStructFieldType;
use crate::structs::univeral_struct::UStructFieldType;

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

#[derive(Debug, Clone)]
pub struct Arg<T> {
    pub arg_type: T,
    pub name: String,
    pub doc: DocString,
}

impl<T> Arg<T>
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
    Float32,
    Double64,
    Duration(DurationType),
    Enum(EnumHandle),
}

impl BasicType {
    /// get the string representation of the type used in the Rust for the C FFI
    pub fn get_c_rust_type(&self) -> &str {
        match self {
            Self::Bool => "bool",
            Self::Uint8 => "u8",
            Self::Sint8 => "i8",
            Self::Uint16 => "u16",
            Self::Sint16 => "i16",
            Self::Uint32 => "u32",
            Self::Sint32 => "i32",
            Self::Uint64 => "u64",
            Self::Sint64 => "i64",
            Self::Float32 => "f32",
            Self::Double64 => "f64",
            Self::Duration(_) => "u64",
            Self::Enum(_) => "std::os::raw::c_int",
        }
    }
}

pub trait TypeExtractor {
    fn get_basic_type(&self) -> Option<&BasicType>;

    fn get_duration_type(&self) -> Option<DurationType> {
        match self.get_basic_type() {
            Some(x) => {
                match x {
                    BasicType::Duration(x) => Some(*x),
                    _ => None,
                }
            }
            None => None,
        }
    }
}

impl TypeExtractor for FStructFieldType {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}

impl TypeExtractor for RStructFieldType {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}

impl TypeExtractor for CStructFieldType {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}

impl TypeExtractor for UStructFieldType {
    fn get_basic_type(&self) -> Option<&BasicType> {
        match self {
            Self::Basic(x) => Some(x),
            _ => None,
        }
    }
}

/// types that require validation in the library
pub enum ValidatedType {
    Enum(EnumHandle),
    StructRef(StructDeclarationHandle),
    Struct(StructType),
    Interface(InterfaceHandle),
    ClassRef(ClassDeclarationHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
}

pub trait TypeValidator {
    fn get_validated_type(&self) -> Option<ValidatedType>;
}

impl TypeValidator for BasicType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            BasicType::Bool => None,
            BasicType::Uint8 => None,
            BasicType::Sint8 => None,
            BasicType::Uint16 => None,
            BasicType::Sint16 => None,
            BasicType::Uint32 => None,
            BasicType::Sint32 => None,
            BasicType::Uint64 => None,
            BasicType::Sint64 => None,
            BasicType::Float32 => None,
            BasicType::Double64 => None,
            BasicType::Duration(_) => None,
            BasicType::Enum(x) => Some(ValidatedType::Enum(x.clone()))
        }
    }
}

impl TypeValidator for StructDeclarationHandle {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        Some(ValidatedType::StructRef(self.clone()))
    }
}

impl TypeValidator for StructType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        Some(ValidatedType::Struct(self.clone()))
    }
}

impl TypeValidator for InterfaceHandle {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        Some(ValidatedType::Interface(self.clone()))
    }
}

impl TypeValidator for IteratorHandle {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        Some(ValidatedType::Iterator(self.clone()))
    }
}

impl TypeValidator for ClassDeclarationHandle {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        Some(ValidatedType::ClassRef(self.clone()))
    }
}

impl TypeValidator for CollectionHandle {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        Some(ValidatedType::Collection(self.clone()))
    }
}