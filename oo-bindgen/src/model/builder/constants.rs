use std::collections::HashSet;

use crate::model::*;

pub struct ConstantSetBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    names: HashSet<String>,
    values: Vec<Constant<Unvalidated>>,
    doc: OptionalDoc,
}

impl<'a> ConstantSetBuilder<'a> {
    pub fn new(lib: &'a mut LibraryBuilder, name: Name) -> Self {
        Self {
            lib,
            name: name.clone(),
            names: HashSet::new(),
            values: Vec::new(),
            doc: OptionalDoc::new(name),
        }
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        self.doc.set(doc.into())?;
        Ok(self)
    }

    pub fn add<T: IntoName, D: Into<Doc<Unvalidated>>>(
        mut self,
        name: T,
        value: ConstantValue,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        if self.names.contains(name.as_ref()) {
            return Err(BindingErrorVariant::ConstantNameAlreadyUsed {
                set_name: self.name,
                constant_name: name,
            }
            .into());
        }
        self.values.push(Constant {
            name,
            value,
            doc: doc.into(),
        });
        Ok(self)
    }

    pub fn build(self) -> BindResult<()> {
        let handle = Handle::new(ConstantSet {
            name: self.name,
            settings: self.lib.clone_settings(),
            values: self.values,
            doc: self.doc.extract()?,
        });

        self.lib.add_statement(Statement::Constants(handle))?;

        Ok(())
    }
}
