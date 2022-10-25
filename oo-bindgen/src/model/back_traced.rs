use std::fmt::Formatter;

use backtrace::Backtrace;

use crate::model::{BadName, BindingError, BindingErrorVariant};

pub type BackTraced<T> = Result<T, BackTracedBindingError>;

#[derive(Debug)]
pub struct BackTracedBindingError {
    pub(crate) error: BindingError,
    pub(crate) backtrace: Backtrace,
}

impl From<BindingErrorVariant> for BackTracedBindingError {
    fn from(error: BindingErrorVariant) -> Self {
        BackTracedBindingError {
            error: error.into(),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<BindingError> for BackTracedBindingError {
    fn from(error: BindingError) -> Self {
        BackTracedBindingError {
            error,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<BadName> for BackTracedBindingError {
    fn from(err: BadName) -> Self {
        BindingErrorVariant::BadName { err }.into()
    }
}

impl std::fmt::Display for BackTracedBindingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.error)?;
        writeln!(f, "origin:")?;
        writeln!(f, "{:?}", self.backtrace)
    }
}

impl std::error::Error for BackTracedBindingError {}
