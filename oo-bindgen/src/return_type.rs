use crate::doc::DocString;
use crate::types::AnyType;

#[derive(Debug)]
pub enum ReturnType<T>
where
    T: Into<AnyType>,
{
    Void,
    Type(T, DocString),
}

impl<T> ReturnType<T>
where
    T: Into<AnyType>,
{
    pub fn void() -> Self {
        ReturnType::Void
    }

    pub fn new<D: Into<DocString>, U: Into<T>>(return_type: U, doc: D) -> Self {
        ReturnType::Type(return_type.into(), doc.into())
    }

    pub fn is_void(&self) -> bool {
        if let Self::Void = self {
            return true;
        }
        false
    }
}
