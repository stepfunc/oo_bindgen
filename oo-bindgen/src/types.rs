use crate::native_function::Type;
use crate::native_struct::StructElementType;

/// Basic (primitive) types are easily convertible without any kind of allocation
/// or cleanup issues in the target language
#[derive(Debug, Copy, Clone, PartialEq)]
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
}

impl BasicType {
    pub fn is_unsigned_integer(&self) -> bool {
        match self {
            Self::Bool => false,
            Self::Uint8 => true,
            Self::Sint8 => false,
            Self::Uint16 => true,
            Self::Sint16 => false,
            Self::Uint32 => true,
            Self::Sint32 => false,
            Self::Uint64 => true,
            Self::Sint64 => false,
            Self::Float => false,
            Self::Double => false,
        }
    }
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
        }
    }
}
