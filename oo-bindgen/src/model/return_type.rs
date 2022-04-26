use crate::model::*;

#[derive(Clone, Debug)]
pub struct ReturnType<T, D>
where
    T: Clone,
    D: DocReference,
{
    pub value: T,
    pub doc: DocString<D>,
}

impl<T> ReturnType<T, Unvalidated>
where
    T: Clone,
{
    pub(crate) fn validate(
        &self,
        parent: &Name,
        lib: &LibraryFields,
    ) -> BindResult<ReturnType<T, Validated>> {
        Ok(ReturnType::new(
            self.value.clone(),
            self.doc.validate(parent, lib)?,
        ))
    }
}

impl<T, D> ReturnType<T, D>
where
    T: Clone,
    D: DocReference,
{
    pub(crate) fn new(value: T, doc: DocString<D>) -> Self {
        Self { value, doc }
    }
}

#[derive(Clone, Debug)]
pub struct OptionalReturnType<T, D>
where
    T: Clone,
    D: DocReference,
{
    value: Option<ReturnType<T, D>>,
}

impl<T, D> OptionalReturnType<T, D>
where
    T: Clone,
    D: DocReference,
{
    pub fn get(&self) -> Option<&ReturnType<T, D>> {
        self.value.as_ref()
    }

    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    pub fn get_value(&self) -> Option<&T> {
        match &self.value {
            None => None,
            Some(x) => Some(&x.value),
        }
    }

    pub fn get_doc(&self) -> Option<&DocString<D>> {
        match &self.value {
            None => None,
            Some(x) => Some(&x.doc),
        }
    }
}

impl<T> OptionalReturnType<T, Unvalidated>
where
    T: Clone,
{
    pub(crate) fn new() -> Self {
        Self { value: None }
    }

    pub(crate) fn set(
        &mut self,
        parent: &Name,
        value: T,
        doc: DocString<Unvalidated>,
    ) -> BindResult<()> {
        match self.value {
            None => {
                self.value = Some(ReturnType::new(value, doc));
                Ok(())
            }
            Some(_) => Err(BindingError::ReturnTypeAlreadyDefined {
                func_name: parent.clone(),
            }),
        }
    }

    pub(crate) fn validate(
        &self,
        parent: &Name,
        lib: &LibraryFields,
    ) -> BindResult<OptionalReturnType<T, Validated>> {
        match &self.value {
            None => Ok(OptionalReturnType { value: None }),
            Some(x) => Ok(OptionalReturnType {
                value: Some(x.validate(parent, lib)?),
            }),
        }
    }
}
