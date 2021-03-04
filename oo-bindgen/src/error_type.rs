use crate::doc::Doc;
use crate::*;

// error types are just special kinds of enums
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ErrorType {
    pub inner: NativeEnumHandle,
}

pub struct ErrorTypeBuilder<'a> {
    inner: NativeEnumBuilder<'a>,
}

impl<'a> ErrorTypeBuilder<'a> {
    pub(crate) fn new(inner: NativeEnumBuilder<'a>) -> Self {
        Self { inner }
    }

    pub fn add_error<T: Into<String>, D: Into<Doc>>(self, name: T, doc: D) -> Result<Self> {
        Ok(Self {
            inner: self.inner.push(name, doc)?,
        })
    }

    pub fn doc<D: Into<Doc>>(self, doc: D) -> Result<Self> {
        Ok(Self {
            inner: self.inner.doc(doc)?,
        })
    }

    pub fn build(self) -> Result<ErrorType> {
        let (inner, lib) = self.inner.build_and_release()?;

        let error_type = ErrorType { inner };

        lib.error_types.insert(error_type.clone());

        Ok(error_type)
    }
}
