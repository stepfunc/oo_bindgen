use crate::*;

#[derive(Debug)]
pub struct NativeEnum {
    pub name: String,
    pub variants: Vec<String>,
}

pub type NativeEnumHandle = Handle<NativeEnum>;

pub struct NativeEnumBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    variants: Vec<String>,
    variants_set: HashSet<String>,
}

impl<'a> NativeEnumBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String) -> Self {
        Self {
            lib,
            name,
            variants: Vec::new(),
            variants_set: HashSet::new(),
        }
    }

    pub fn variant(mut self, name: &str) -> Result<Self> {
        if self.variants_set.insert(name.to_string()) {
            self.variants.push(name.to_string());
            Ok(self)
        } else {
            Err(BindingError::NativeEnumAlreadyContainsVariantWithSameName{
                name: self.name,
                variant_name: name.to_string(),
            })
        }
    }

    pub fn build(self) -> NativeEnumHandle {
        let handle = NativeEnumHandle::new(NativeEnum {
            name: self.name,
            variants: self.variants,
        });

        self.lib.native_enums.insert(handle.clone());
        self.lib.statements.push(Statement::EnumDefinition(handle.clone()));

        handle
    }
}
