use crate::doc::{DocReference, DocString};

#[derive(Debug)]
pub enum ReturnType<T, D>
where
    D: DocReference,
{
    Void,
    Type(T, DocString<D>),
}

impl<T, D> ReturnType<T, D>
where
    D: DocReference,
{
    pub fn void() -> Self {
        ReturnType::Void
    }

    pub fn new<C: Into<DocString<D>>, U: Into<T>>(return_type: U, doc: C) -> Self {
        ReturnType::Type(return_type.into(), doc.into())
    }

    pub fn is_void(&self) -> bool {
        if let Self::Void = self {
            return true;
        }
        false
    }
}
