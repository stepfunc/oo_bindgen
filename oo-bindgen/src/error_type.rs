use crate::doc::Doc;
use crate::*;

/// Type of exception to generate (only used in Java atm)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExceptionType {
    /// Statically checked exceptions
    CheckedException,
    /// Runtime checked exceptions
    UncheckedException,
}

// error types are just special kinds of enums
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ErrorType {
    pub exception_name: String,
    pub exception_type: ExceptionType,
    pub inner: NativeEnumHandle,
}

impl ErrorType {
    pub fn to_enum_type(&self) -> Type {
        Type::Enum(self.inner.clone())
    }
}

pub struct ErrorTypeBuilder<'a> {
    exception_name: String,
    exception_type: ExceptionType,
    inner: NativeEnumBuilder<'a>,
}

impl<'a> ErrorTypeBuilder<'a> {
    pub(crate) fn new(
        exception_name: String,
        exception_type: ExceptionType,
        inner: NativeEnumBuilder<'a>,
    ) -> Self {
        Self {
            exception_name,
            exception_type,
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
            exception_type: self.exception_type,
            inner,
        };

        lib.statements.push(Statement::ErrorType(err.clone()));

        Ok(err)
    }
}
