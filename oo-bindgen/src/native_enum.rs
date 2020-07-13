use crate::*;
use crate::doc::Doc;

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub value: i32,
    pub doc: Doc,
}

#[derive(Debug)]
pub struct NativeEnum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub doc: Doc,
}

pub type NativeEnumHandle = Handle<NativeEnum>;

pub struct NativeEnumBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    variants: Vec<EnumVariant>,
    variant_names: HashSet<String>,
    variant_values: HashSet<i32>,
    next_value: i32,
    doc: Option<Doc>,
}

impl<'a> NativeEnumBuilder<'a> {
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

    pub fn variant<D: Into<Doc>>(mut self, name: &str, value: i32, doc: D) -> Result<Self> {
        let unique_name = self.variant_names.insert(name.to_string());
        let unique_value = self.variant_values.insert(value);
        if unique_name && unique_value {
            self.variants.push(EnumVariant {
                name: name.to_string(),
                value,
                doc: doc.into(),
            });
            self.next_value = value + 1;
            Ok(self)
        } else if !unique_name {
            Err(BindingError::NativeEnumAlreadyContainsVariantWithSameName {
                name: self.name,
                variant_name: name.to_string(),
            })
        } else {
            Err(
                BindingError::NativeEnumAlreadyContainsVariantWithSameValue {
                    name: self.name,
                    variant_value: value,
                },
            )
        }
    }

    pub fn push<D: Into<Doc>>(self, name: &str, doc: D) -> Result<Self> {
        let value = self.next_value;
        self.variant(name, value, doc)
    }

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> Result<Self> {
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

    pub fn build(self) -> Result<NativeEnumHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => return Err(BindingError::DocNotDefined {
                symbol_name: self.name
            })
        };

        let handle = NativeEnumHandle::new(NativeEnum {
            name: self.name,
            variants: self.variants,
            doc,
        });

        self.lib.native_enums.insert(handle.clone());
        self.lib
            .statements
            .push(Statement::EnumDefinition(handle.clone()));

        Ok(handle)
    }
}
