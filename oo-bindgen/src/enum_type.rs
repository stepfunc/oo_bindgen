use std::collections::HashSet;

use crate::doc::{Doc, DocReference, Unvalidated, Validated};
use crate::name::{IntoName, Name};
use crate::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct EnumVariant<T>
where
    T: DocReference,
{
    pub name: Name,
    pub value: i32,
    pub doc: Doc<T>,
}

impl EnumVariant<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<EnumVariant<Validated>> {
        Ok(EnumVariant {
            name: self.name.clone(),
            value: self.value,
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

#[derive(Debug)]
pub struct Enum<T>
where
    T: DocReference,
{
    pub name: Name,
    pub settings: Rc<LibrarySettings>,
    pub variants: Vec<EnumVariant<T>>,
    pub doc: Doc<T>,
}

impl Enum<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Enum<Validated>> {
        let variants: BindResult<Vec<EnumVariant<Validated>>> =
            self.variants.iter().map(|x| x.validate(lib)).collect();

        Ok(Enum {
            name: self.name.clone(),
            settings: self.settings.clone(),
            variants: variants?,
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

impl<T> Enum<T>
where
    T: DocReference,
{
    pub fn find_variant_by_name<S: AsRef<str>>(&self, variant_name: S) -> Option<&EnumVariant<T>> {
        self.variants
            .iter()
            .find(|variant| variant.name.as_ref() == variant_name.as_ref())
    }

    pub fn validate_contains_variant_name(&self, variant_name: &str) -> BindResult<()> {
        if self.find_variant_by_name(variant_name).is_none() {
            Err(BindingError::EnumDoesNotContainVariant {
                name: self.name.clone(),
                variant_name: variant_name.to_string(),
            })
        } else {
            Ok(())
        }
    }

    pub fn find_variant_by_value(&self, value: i32) -> Option<&EnumVariant<T>> {
        self.variants.iter().find(|variant| variant.value == value)
    }
}

pub type EnumHandle = Handle<Enum<Unvalidated>>;

pub struct EnumBuilder<'a> {
    pub(crate) lib: &'a mut LibraryBuilder,
    name: Name,
    variants: Vec<EnumVariant<Unvalidated>>,
    variant_names: HashSet<String>,
    variant_values: HashSet<i32>,
    next_value: i32,
    doc: Option<Doc<Unvalidated>>,
}

impl<'a> EnumBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: Name) -> Self {
        Self {
            lib,
            name,
            variants: Vec::new(),
            variant_names: HashSet::new(),
            variant_values: HashSet::new(),
            next_value: 0,
            doc: None,
        }
    }

    pub fn variant<T: IntoName, D: Into<Doc<Unvalidated>>>(
        mut self,
        name: T,
        value: i32,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        let unique_name = self.variant_names.insert(name.to_string());
        let unique_value = self.variant_values.insert(value);
        if unique_name && unique_value {
            self.variants.push(EnumVariant {
                name,
                value,
                doc: doc.into(),
            });
            self.next_value = value + 1;
            Ok(self)
        } else if !unique_name {
            Err(BindingError::EnumAlreadyContainsVariantWithSameName {
                name: self.name,
                variant_name: name.to_string(),
            })
        } else {
            Err(BindingError::EnumAlreadyContainsVariantWithSameValue {
                name: self.name,
                variant_value: value,
            })
        }
    }

    pub fn push<T: IntoName, D: Into<Doc<Unvalidated>>>(self, name: T, doc: D) -> BindResult<Self> {
        let value = self.next_value;
        self.variant(name.into_name()?, value, doc)
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

    pub(crate) fn build_and_release(self) -> BindResult<(EnumHandle, &'a mut LibraryBuilder)> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.name,
                })
            }
        };

        let handle = EnumHandle::new(Enum {
            name: self.name,
            settings: self.lib.settings.clone(),
            variants: self.variants,
            doc,
        });

        self.lib
            .add_statement(Statement::EnumDefinition(handle.clone()))?;

        Ok((handle, self.lib))
    }

    pub fn build(self) -> BindResult<EnumHandle> {
        let (ret, _) = self.build_and_release()?;
        Ok(ret)
    }
}
