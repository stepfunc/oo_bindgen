use std::collections::HashSet;

use crate::doc::{Doc, DocReference, Unvalidated, Validated};
use crate::name::{IntoName, Name};
use crate::*;

/// How to render a numeric constant
#[derive(Copy, Clone, Debug)]
pub enum Representation {
    Hex,
}

/// Types of constants available
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
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Constant<Validated>> {
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
    /// Name of the set
    pub name: Name,
    /// values
    pub values: Vec<Constant<T>>,
    /// documentation
    pub doc: Doc<T>,
}

impl ConstantSet<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<ConstantSet<Validated>> {
        let values: BindResult<Vec<Constant<Validated>>> =
            self.values.iter().map(|x| x.validate(lib)).collect();

        Ok(ConstantSet {
            name: self.name.clone(),
            values: values?,
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

pub type ConstantSetHandle = Handle<ConstantSet<Unvalidated>>;

pub struct ConstantSetBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    names: HashSet<String>,
    values: Vec<Constant<Unvalidated>>,
    doc: Option<Doc<Unvalidated>>,
}

impl<'a> ConstantSetBuilder<'a> {
    pub fn new(lib: &'a mut LibraryBuilder, name: Name) -> Self {
        Self {
            lib,
            name,
            names: HashSet::new(),
            values: Vec::new(),
            doc: None,
        }
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingError::DocAlreadyDefined {
                symbol_name: self.name,
            }),
        }
    }

    pub fn add<T: IntoName, D: Into<Doc<Unvalidated>>>(
        mut self,
        name: T,
        value: ConstantValue,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        if self.names.contains(name.as_ref()) {
            return Err(BindingError::ConstantNameAlreadyUsed {
                set_name: self.name,
                constant_name: name,
            });
        }
        self.values.push(Constant {
            name,
            value,
            doc: doc.into(),
        });
        Ok(self)
    }

    pub fn build(self) -> BindResult<()> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.name,
                })
            }
        };

        let handle = ConstantSetHandle::new(ConstantSet {
            name: self.name,
            values: self.values,
            doc,
        });

        self.lib.add_statement(Statement::Constants(handle))?;

        Ok(())
    }
}
