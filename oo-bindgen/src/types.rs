use crate::enum_type::EnumHandle;
use crate::structs::any_struct::AnyStructHandle;

use crate::class::ClassDeclarationHandle;
use crate::collection::CollectionHandle;
use crate::doc::DocString;
use crate::interface::InterfaceHandle;
use crate::iterator::IteratorHandle;
use crate::structs::common::StructDeclarationHandle;
use crate::structs::function_struct::FStructHandle;
use std::time::Duration;
use crate::function::FArgument;

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
    Float32,
    Double64,
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
    fn from(x: BasicType) -> Self {
        AnyType::Basic(x)
    }
}

impl From<FArgument> for AnyType {
    fn from(x: FArgument) -> Self {
        match x {
            FArgument::Basic(x) => Self::Basic(x),
            FArgument::String => Self::String,
            FArgument::Collection(x) => Self::Collection(x),
            FArgument::Struct(x) => Self::Struct(x.to_any_struct()),
            FArgument::StructRef(x) => Self::StructRef(x),
            FArgument::ClassRef(x) => Self::ClassRef(x),
            FArgument::Interface(x) => Self::Interface(x),
        }
    }
}
