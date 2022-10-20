use std::collections::HashSet;

use crate::model::*;

pub struct EnumBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    variants: Vec<EnumVariant<Unvalidated>>,
    variant_names: HashSet<String>,
    variant_values: HashSet<i32>,
    next_value: i32,
    doc: OptionalDoc,
}

impl<'a> EnumBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: Name) -> Self {
        Self {
            lib,
            name: name.clone(),
            variants: Vec::new(),
            variant_names: HashSet::new(),
            variant_values: HashSet::new(),
            next_value: 0,
            doc: OptionalDoc::new(name),
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
            Err(BindingError::DuplicateEnumVariantName {
                name: self.name,
                variant_name: name.to_string(),
            })
        } else {
            Err(BindingError::DuplicateEnumVariantValue {
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
        self.doc.set(doc.into())?;
        Ok(self)
    }

    pub(crate) fn build_and_release(
        self,
    ) -> BindResult<(Handle<Enum<Unvalidated>>, &'a mut LibraryBuilder)> {
        let handle = Handle::new(Enum {
            name: self.name,
            settings: self.lib.settings.clone(),
            variants: self.variants,
            doc: self.doc.extract()?,
        });

        self.lib
            .add_statement(Statement::EnumDefinition(handle.clone()))?;

        Ok((handle, self.lib))
    }

    pub fn build(self) -> BindResult<Handle<Enum<Unvalidated>>> {
        let (ret, _) = self.build_and_release()?;
        Ok(ret)
    }
}
