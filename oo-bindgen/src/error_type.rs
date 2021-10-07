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
    pub inner: EnumHandle,
}

pub struct ErrorTypeBuilder<'a> {
    exception_name: String,
    exception_type: ExceptionType,
    inner: EnumBuilder<'a>,
}

impl<'a> ErrorTypeBuilder<'a> {
    pub(crate) fn new(
        exception_name: String,
        exception_type: ExceptionType,
        inner: EnumBuilder<'a>,
    ) -> Self {
        Self {
            exception_name,
            exception_type,
            inner,
        }
    }

    pub fn add_error<T: Into<String>, D: Into<Doc>>(self, name: T, doc: D) -> BindResult<Self> {
        Ok(Self {
            inner: self.inner.push(name, doc)?,
            ..self
        })
    }

    pub fn doc<D: Into<Doc>>(self, doc: D) -> BindResult<Self> {
        Ok(Self {
            inner: self.inner.doc(doc)?,
            ..self
        })
    }

    pub fn build(self) -> BindResult<ErrorType> {
        let (inner, lib) = self.inner.build_and_release()?;

        let err = ErrorType {
            exception_name: self.exception_name,
            exception_type: self.exception_type,
            inner,
        };

        lib.add_statement(Statement::ErrorType(err.clone()))?;

        Ok(err)
    }
}
