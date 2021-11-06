use crate::doc::{Doc, DocReference, Unvalidated, Validated};
use crate::name::{IntoName, Name};
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
pub struct ErrorType<D>
where
    D: DocReference,
{
    pub exception_name: Name,
    pub exception_type: ExceptionType,
    pub inner: Handle<Enum<D>>,
}

impl ErrorType<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<ErrorType<Validated>> {
        Ok(ErrorType {
            exception_name: self.exception_name.clone(),
            exception_type: self.exception_type,
            inner: self.inner.validate(lib)?,
        })
    }
}

pub struct ErrorTypeBuilder<'a> {
    exception_name: Name,
    exception_type: ExceptionType,
    inner: EnumBuilder<'a>,
}

impl<'a> ErrorTypeBuilder<'a> {
    pub(crate) fn new(
        exception_name: Name,
        exception_type: ExceptionType,
        inner: EnumBuilder<'a>,
    ) -> Self {
        Self {
            exception_name,
            exception_type,
            inner,
        }
    }

    pub fn add_error<T: IntoName, D: Into<Doc<Unvalidated>>>(
        self,
        name: T,
        doc: D,
    ) -> BindResult<Self> {
        Ok(Self {
            inner: self.inner.push(name.into_name()?, doc)?,
            ..self
        })
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(self, doc: D) -> BindResult<Self> {
        Ok(Self {
            inner: self.inner.doc(doc)?,
            ..self
        })
    }

    pub fn build(self) -> BindResult<ErrorType<Unvalidated>> {
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
