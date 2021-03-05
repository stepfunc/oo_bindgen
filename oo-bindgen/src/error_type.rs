use crate::doc::Doc;
use crate::*;

// error types are just special kinds of enums
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ErrorType {
    pub exception_name: String,
    pub inner: NativeEnumHandle,
}

impl ErrorType {
    pub fn to_enum_type(&self) -> Type {
        Type::Enum(self.inner.clone())
    }
}

pub struct ErrorTypeBuilder<'a> {
    exception_name: String,
    inner: NativeEnumBuilder<'a>,
}

impl<'a> ErrorTypeBuilder<'a> {
    pub(crate) fn new(exception_name: String, inner: NativeEnumBuilder<'a>) -> Self {
        Self {
            exception_name,
            inner,
        }
    }

    pub fn add_error<T: Into<String>, D: Into<Doc>>(self, name: T, doc: D) -> Result<Self> {
        Ok(Self {
            inner: self.inner.push(name, doc)?,
            ..self
        })
    }

    pub fn doc<D: Into<Doc>>(self, doc: D) -> Result<Self> {
        Ok(Self {
            inner: self.inner.doc(doc)?,
            ..self
        })
    }

    pub fn build(self) -> Result<ErrorType> {
        let (inner, lib) = self.inner.build_and_release()?;

        lib.check_unique_symbol(&self.exception_name)?;

        let err = ErrorType {
            exception_name: self.exception_name,
            inner,
        };

        lib.statements.push(Statement::ErrorType(err.clone()));

        Ok(err)
    }
}
