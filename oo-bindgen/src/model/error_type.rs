use crate::model::*;

/// Type of exception to generate (only used in Java atm)
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExceptionType {
    /// Statically checked exceptions
    CheckedException,
    /// Runtime checked exceptions
    UncheckedException,
}

/// A type that wraps an inner enum and provides
/// information on how it maps to exceptions in
/// languages that support them
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ErrorType<D>
where
    D: DocReference,
{
    pub(crate) exception_name: Name,
    pub(crate) exception_type: ExceptionType,
    pub(crate) inner: Handle<Enum<D>>,
}

impl ErrorType<Unvalidated> {
    /// Clone the underlying Enum that represents the error
    pub fn clone_enum(&self) -> EnumHandle {
        self.inner.clone()
    }

    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<ErrorType<Validated>> {
        Ok(ErrorType {
            exception_name: self.exception_name.clone(),
            exception_type: self.exception_type,
            inner: self.inner.validate(lib)?,
        })
    }
}

pub type ErrorTypeHandle = ErrorType<Unvalidated>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct OptionalErrorType<D>
where
    D: DocReference,
{
    inner: Option<ErrorType<D>>,
}

impl<D> OptionalErrorType<D>
where
    D: DocReference,
{
    pub(crate) fn new() -> Self {
        Self { inner: None }
    }

    pub(crate) fn get(&self) -> Option<&ErrorType<D>> {
        self.inner.as_ref()
    }

    pub(crate) fn is_some(&self) -> bool {
        self.inner.is_some()
    }
}

impl OptionalErrorType<Unvalidated> {
    pub(crate) fn set(&mut self, parent: &Name, err: &ErrorType<Unvalidated>) -> BindResult<()> {
        if self.inner.is_some() {
            return Err(BindingErrorVariant::ErrorTypeAlreadyDefined {
                function: parent.clone(),
                error_type: err.inner.name.clone(),
            }
            .into());
        }

        self.inner = Some(err.clone());
        Ok(())
    }

    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<OptionalErrorType<Validated>> {
        match &self.inner {
            None => Ok(OptionalErrorType::new()),
            Some(x) => Ok(OptionalErrorType {
                inner: Some(x.validate(lib)?),
            }),
        }
    }
}
