use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::java::NATIVE_FUNCTIONS_CLASSNAME;

pub(crate) trait JavaType {
    fn as_java_primitive(&self) -> String;
    fn as_java_object(&self) -> String;
}

impl JavaType for Primitive {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Bool => "boolean".to_string(),
            Self::U8 => "UByte".to_string(),
            Self::S8 => "byte".to_string(),
            Self::U16 => "UShort".to_string(),
            Self::S16 => "short".to_string(),
            Self::U32 => "UInteger".to_string(),
            Self::S32 => "int".to_string(),
            Self::U64 => "ULong".to_string(),
            Self::S64 => "long".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Bool => "Boolean".to_string(),
            Self::U8 => "UByte".to_string(),
            Self::S8 => "Byte".to_string(),
            Self::U16 => "UShort".to_string(),
            Self::S16 => "Short".to_string(),
            Self::U32 => "UInteger".to_string(),
            Self::S32 => "Integer".to_string(),
            Self::U64 => "ULong".to_string(),
            Self::S64 => "Long".to_string(),
            Self::Float => "Float".to_string(),
            Self::Double => "Double".to_string(),
        }
    }
}

impl JavaType for BasicType {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Primitive(x) => x.as_java_primitive(),
            Self::Duration(_) => "java.time.Duration".to_string(),
            Self::Enum(handle) => handle.name.camel_case(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Primitive(x) => x.as_java_object(),
            Self::Duration(_) => "java.time.Duration".to_string(),
            Self::Enum(handle) => handle.name.camel_case(),
        }
    }
}

impl JavaType for StringType {
    fn as_java_primitive(&self) -> String {
        "String".to_string()
    }

    fn as_java_object(&self) -> String {
        "String".to_string()
    }
}

impl<D> JavaType for Handle<Interface<D>>
where
    D: DocReference,
{
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name.camel_case()
    }
}

impl<D> JavaType for Handle<Collection<D>>
where
    D: DocReference,
{
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        format!("java.util.List<{}>", self.item_type.as_java_object())
    }
}

impl JavaType for ClassDeclarationHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name.camel_case()
    }
}

impl JavaType for StructDeclarationHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name.camel_case()
    }
}

impl JavaType for IteratorItemType {
    fn as_java_primitive(&self) -> String {
        match self {
            IteratorItemType::Struct(x) => x.as_java_primitive(),
            IteratorItemType::Primitive(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            IteratorItemType::Struct(x) => x.as_java_object(),
            IteratorItemType::Primitive(x) => x.as_java_object(),
        }
    }
}

impl<D> JavaType for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        format!("java.util.List<{}>", self.item_type.as_java_object())
    }
}

impl<T, D> JavaType for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name().camel_case()
    }
}

impl<T> JavaType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn as_java_primitive(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_java_primitive(),
            UniversalOr::Universal(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_java_object(),
            UniversalOr::Universal(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FunctionArgStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_java_primitive(),
            FunctionArgStructField::String(x) => x.as_java_primitive(),
            FunctionArgStructField::Interface(x) => x.inner.as_java_primitive(),
            FunctionArgStructField::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_java_object(),
            FunctionArgStructField::String(x) => x.as_java_object(),
            FunctionArgStructField::Interface(x) => x.inner.as_java_object(),
            FunctionArgStructField::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FunctionReturnStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.as_java_primitive(),
            FunctionReturnStructField::ClassRef(x) => x.as_java_primitive(),
            FunctionReturnStructField::Struct(x) => x.as_java_primitive(),
            FunctionReturnStructField::Iterator(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.as_java_object(),
            FunctionReturnStructField::ClassRef(x) => x.as_java_object(),
            FunctionReturnStructField::Struct(x) => x.as_java_object(),
            FunctionReturnStructField::Iterator(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CallbackArgStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_java_primitive(),
            CallbackArgStructField::Iterator(x) => x.as_java_primitive(),
            CallbackArgStructField::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_java_object(),
            CallbackArgStructField::Iterator(x) => x.as_java_object(),
            CallbackArgStructField::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for UniversalStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_java_primitive(),
            UniversalStructField::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_java_object(),
            UniversalStructField::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FunctionArgument {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::String(x) => x.as_java_primitive(),
            Self::Collection(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
            Self::StructRef(x) => x.inner.as_java_primitive(),
            Self::ClassRef(x) => x.as_java_primitive(),
            Self::Interface(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::String(x) => x.as_java_object(),
            Self::Collection(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
            Self::StructRef(x) => x.inner.as_java_object(),
            Self::ClassRef(x) => x.as_java_object(),
            Self::Interface(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CallbackArgument {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::String(x) => x.as_java_primitive(),
            Self::Iterator(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
            Self::Class(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::String(x) => x.as_java_object(),
            Self::Iterator(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
            Self::Class(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CallbackReturnValue {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
        }
    }
}

/// we always use the Boxed form since it's an optional (reference) of a primitive
impl JavaType for PrimitiveRef {
    fn as_java_primitive(&self) -> String {
        self.inner.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.inner.as_java_object()
    }
}

impl JavaType for FunctionReturnValue {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::String(x) => x.as_java_primitive(),
            Self::ClassRef(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
            Self::StructRef(x) => x.untyped().as_java_primitive(),
            Self::PrimitiveRef(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::String(x) => x.as_java_object(),
            Self::ClassRef(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
            Self::StructRef(x) => x.untyped().as_java_object(),
            Self::PrimitiveRef(x) => x.as_java_object(),
        }
    }
}

const VOID: &str = "void";

impl<T> JavaType for OptionalReturnType<T, Validated>
where
    T: Clone + JavaType,
{
    fn as_java_primitive(&self) -> String {
        match self.get_value() {
            None => VOID.to_string(),
            Some(v) => v.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self.get_value() {
            None => VOID.to_string(),
            Some(v) => v.as_java_object(),
        }
    }
}

pub(crate) fn call_native_function(
    f: &mut dyn Printer,
    method: &Function<Validated>,
    return_destination: &str,
    first_param_is_this: bool,
) -> FormattingResult<()> {
    let params = method
        .arguments
        .iter()
        .enumerate()
        .map(|(idx, param)| {
            if first_param_is_this && idx == 0 {
                "this".to_string()
            } else {
                param.name.mixed_case()
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.newline()?;
    if !method.return_type.is_none() {
        f.write(return_destination)?;
    }

    f.write(&format!(
        "{}.Wrapped.{}({});",
        NATIVE_FUNCTIONS_CLASSNAME, method.name, params
    ))?;

    Ok(())
}
