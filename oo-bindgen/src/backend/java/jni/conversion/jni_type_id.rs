use crate::model::*;

pub(crate) trait JniTypeId {
    /// get the JNI identifier of the type used to find methods and fields
    fn jni_type_id(&self) -> TypeId;
}

/// Identifier which may be a fixed value or generated from type name in the library
pub(crate) enum TypeId {
    Fixed(&'static str),
    LibraryType(Name),
}

impl TypeId {
    pub(crate) fn as_string(&self, lib_path: &str) -> String {
        match self {
            TypeId::Fixed(x) => x.to_string(),
            TypeId::LibraryType(name) => {
                format!("L{}/{};", lib_path, name.camel_case())
            }
        }
    }
}

impl JniTypeId for Primitive {
    fn jni_type_id(&self) -> TypeId {
        match self {
            Self::Bool => TypeId::Fixed("Z"),
            Self::U8 => TypeId::Fixed("Lorg/joou/UByte;"),
            Self::S8 => TypeId::Fixed("B"),
            Self::U16 => TypeId::Fixed("Lorg/joou/UShort;"),
            Self::S16 => TypeId::Fixed("S"),
            Self::U32 => TypeId::Fixed("Lorg/joou/UInteger;"),
            Self::S32 => TypeId::Fixed("I"),
            Self::U64 => TypeId::Fixed("Lorg/joou/ULong;"),
            Self::S64 => TypeId::Fixed("J"),
            Self::Float => TypeId::Fixed("F"),
            Self::Double => TypeId::Fixed("D"),
        }
    }
}

impl JniTypeId for StringType {
    fn jni_type_id(&self) -> TypeId {
        TypeId::Fixed("Ljava/lang/String;")
    }
}

impl JniTypeId for DurationType {
    fn jni_type_id(&self) -> TypeId {
        TypeId::Fixed("Ljava/time/Duration;")
    }
}

impl JniTypeId for EnumHandle {
    fn jni_type_id(&self) -> TypeId {
        TypeId::LibraryType(self.name.clone())
    }
}

impl JniTypeId for BasicType {
    fn jni_type_id(&self) -> TypeId {
        match self {
            BasicType::Primitive(x) => x.jni_type_id(),
            BasicType::Duration(x) => x.jni_type_id(),
            BasicType::Enum(x) => x.jni_type_id(),
        }
    }
}

impl JniTypeId for AbstractIteratorHandle {
    fn jni_type_id(&self) -> TypeId {
        TypeId::Fixed("Ljava/util/List;")
    }
}

impl JniTypeId for ClassDeclarationHandle {
    fn jni_type_id(&self) -> TypeId {
        TypeId::LibraryType(self.name.clone())
    }
}

impl<T> JniTypeId for UniversalOr<T>
where
    T: StructFieldType,
{
    fn jni_type_id(&self) -> TypeId {
        TypeId::LibraryType(self.name().clone())
    }
}

impl JniTypeId for UniversalStructHandle {
    fn jni_type_id(&self) -> TypeId {
        TypeId::LibraryType(self.name().clone())
    }
}

impl JniTypeId for CallbackArgument {
    fn jni_type_id(&self) -> TypeId {
        match self {
            CallbackArgument::Basic(x) => x.jni_type_id(),
            CallbackArgument::String(x) => x.jni_type_id(),
            CallbackArgument::Iterator(x) => x.jni_type_id(),
            CallbackArgument::Class(x) => x.jni_type_id(),
            CallbackArgument::Struct(x) => x.jni_type_id(),
        }
    }
}

impl JniTypeId for CallbackReturnValue {
    fn jni_type_id(&self) -> TypeId {
        match self {
            CallbackReturnValue::Basic(x) => x.jni_type_id(),
            CallbackReturnValue::Struct(x) => x.jni_type_id(),
        }
    }
}

impl JniTypeId for OptionalReturnType<CallbackReturnValue, Validated> {
    fn jni_type_id(&self) -> TypeId {
        match self.get_value() {
            None => TypeId::Fixed("V"),
            Some(x) => x.jni_type_id(),
        }
    }
}

impl JniTypeId for InterfaceHandle {
    fn jni_type_id(&self) -> TypeId {
        TypeId::LibraryType(self.name.clone())
    }
}

impl JniTypeId for FunctionArgStructField {
    fn jni_type_id(&self) -> TypeId {
        match self {
            FunctionArgStructField::Basic(x) => x.jni_type_id(),
            FunctionArgStructField::String(x) => x.jni_type_id(),
            FunctionArgStructField::Interface(x) => x.inner.jni_type_id(),
            FunctionArgStructField::Struct(x) => x.jni_type_id(),
        }
    }
}

impl JniTypeId for FunctionReturnStructField {
    fn jni_type_id(&self) -> TypeId {
        match self {
            FunctionReturnStructField::Basic(x) => x.jni_type_id(),
            FunctionReturnStructField::ClassRef(x) => x.jni_type_id(),
            FunctionReturnStructField::Iterator(x) => x.jni_type_id(),
            FunctionReturnStructField::Struct(x) => x.jni_type_id(),
        }
    }
}

impl JniTypeId for CallbackArgStructField {
    fn jni_type_id(&self) -> TypeId {
        match self {
            CallbackArgStructField::Basic(x) => x.jni_type_id(),
            CallbackArgStructField::Iterator(x) => x.jni_type_id(),
            CallbackArgStructField::Struct(x) => x.jni_type_id(),
        }
    }
}

impl JniTypeId for UniversalStructField {
    fn jni_type_id(&self) -> TypeId {
        match self {
            UniversalStructField::Basic(x) => x.jni_type_id(),
            UniversalStructField::Struct(x) => x.jni_type_id(),
        }
    }
}
