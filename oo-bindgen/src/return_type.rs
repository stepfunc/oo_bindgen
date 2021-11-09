use crate::doc::{DocReference, DocString, Unvalidated, Validated};
use crate::name::Name;
use crate::{BindResult, UnvalidatedFields};

#[derive(Clone, Debug)]
pub enum ReturnType<T, D>
where
    T: Clone,
    D: DocReference,
{
    Void,
    Type(T, DocString<D>),
}

impl<T> ReturnType<T, Unvalidated>
where
    T: Clone,
{
    pub(crate) fn validate(
        &self,
        name: &Name,
        lib: &UnvalidatedFields,
    ) -> BindResult<ReturnType<T, Validated>> {
        match self {
            ReturnType::Void => Ok(ReturnType::Void),
            ReturnType::Type(t, d) => Ok(ReturnType::Type(t.clone(), d.validate(name, lib)?)),
        }
    }

    pub fn get(&self) -> Option<&T> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(t, _) => Some(t),
        }
    }
}

impl<T, D> ReturnType<T, D>
where
    T: Clone,
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
