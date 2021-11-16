use crate::model::*;

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
