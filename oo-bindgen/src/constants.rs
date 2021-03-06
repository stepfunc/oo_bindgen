use crate::doc::Doc;
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
pub struct Constant {
    pub name: String,
    pub value: ConstantValue,
    pub doc: Doc,
}

/// Set of constants
#[derive(Debug)]
pub struct ConstantSet {
    /// Name of the set
    pub name: String,
    /// values
    pub values: Vec<Constant>,
    /// documentation
    pub doc: Doc,
}

pub type ConstantSetHandle = Handle<ConstantSet>;

pub struct ConstantSetBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    names: HashSet<String>,
    values: Vec<Constant>,
    doc: Option<Doc>,
}

impl<'a> ConstantSetBuilder<'a> {
    pub fn new(lib: &'a mut LibraryBuilder, name: String) -> Self {
        Self {
            lib,
            name,
            names: HashSet::new(),
            values: Vec::new(),
            doc: None,
        }
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

    pub fn add<T: Into<String>, D: Into<Doc>>(
        mut self,
        name: T,
        value: ConstantValue,
        doc: D,
    ) -> Result<Self> {
        let name = name.into();
        if self.names.contains(&name) {
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

    pub fn build(self) -> Result<()> {
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

        self.lib.statements.push(Statement::Constants(handle));

        Ok(())
    }
}
