use std::collections::HashSet;

use crate::doc::Doc;
use crate::*;

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub value: i32,
    pub doc: Doc,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub doc: Doc,
}

impl Enum {
    pub fn find_variant_by_name<T: AsRef<str>>(&self, variant_name: T) -> Option<&EnumVariant> {
        self.variants
            .iter()
            .find(|variant| variant.name == variant_name.as_ref())
    }

    pub fn validate_contains_variant_name(&self, variant_name: &str) -> BindResult<()> {
        if self.find_variant_by_name(variant_name).is_none() {
            Err(BindingError::EnumDoesNotContainVariant {
                name: self.name.to_string(),
                variant_name: variant_name.to_string(),
            })
        } else {
            Ok(())
        }
    }

    pub fn find_variant_by_value(&self, value: i32) -> Option<&EnumVariant> {
        self.variants.iter().find(|variant| variant.value == value)
    }
}

pub type EnumHandle = Handle<Enum>;

pub struct EnumBuilder<'a> {
    pub(crate) lib: &'a mut LibraryBuilder,
    name: String,
    variants: Vec<EnumVariant>,
    variant_names: HashSet<String>,
    variant_values: HashSet<i32>,
    next_value: i32,
    doc: Option<Doc>,
}

impl<'a> EnumBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String) -> Self {
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

    pub fn variant<T: Into<String>, D: Into<Doc>>(
        mut self,
        name: T,
        value: i32,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into();
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
                variant_name: name,
            })
        } else {
            Err(BindingError::EnumAlreadyContainsVariantWithSameValue {
                name: self.name,
                variant_value: value,
            })
        }
    }

    pub fn push<T: Into<String>, D: Into<Doc>>(self, name: T, doc: D) -> BindResult<Self> {
        let value = self.next_value;
        self.variant(name, value, doc)
    }

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> BindResult<Self> {
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
