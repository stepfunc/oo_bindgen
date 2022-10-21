use std::rc::Rc;

use crate::model::*;

/// How to render a numeric constant
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Representation {
    Hex,
}

/// Types of constants available
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ConstantValue {
    U8(u8, Representation),
}

/// Constant belonging to a set of constants
#[derive(Debug)]
pub struct Constant<T>
where
    T: DocReference,
{
    pub name: Name,
    pub value: ConstantValue,
    pub doc: Doc<T>,
}

impl Constant<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Constant<Validated>> {
        Ok(Constant {
            name: self.name.clone(),
            value: self.value,
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

/// Set of constants
#[derive(Debug)]
pub struct ConstantSet<T>
where
    T: DocReference,
{
    /// name of the set
    pub name: Name,
    /// common library settings
    pub settings: Rc<LibrarySettings>,
    /// values
    pub values: Vec<Constant<T>>,
    /// documentation
    pub doc: Doc<T>,
}

impl ConstantSet<Unvalidated> {
    pub(crate) fn validate(
        &self,
        lib: &LibraryFields,
    ) -> BindResult<Handle<ConstantSet<Validated>>> {
        let values: BindResult<Vec<Constant<Validated>>> =
            self.values.iter().map(|x| x.validate(lib)).collect();

        Ok(Handle::new(ConstantSet {
            name: self.name.clone(),
            settings: self.settings.clone(),
            values: values?,
            doc: self.doc.validate(&self.name, lib)?,
        }))
    }
}

pub type ConstantSetHandle = Handle<ConstantSet<Unvalidated>>;
