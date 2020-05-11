use crate::*;

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub value: i32,
}

#[derive(Debug)]
pub struct NativeEnum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

pub type NativeEnumHandle = Handle<NativeEnum>;

pub struct NativeEnumBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    variants: Vec<EnumVariant>,
    variant_names: HashSet<String>,
    variant_values: HashSet<i32>,
    next_value: i32,
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
        }
    }

    pub fn variant(mut self, name: &str, value: i32) -> Result<Self> {
        let unique_name = self.variant_names.insert(name.to_string());
        let unique_value = self.variant_values.insert(value);
        if unique_name && unique_value {
            self.variants.push(EnumVariant {
                name: name.to_string(),
                value,
            });
            self.next_value = value + 1;
            Ok(self)
        } else if !unique_name {
            Err(BindingError::NativeEnumAlreadyContainsVariantWithSameName{
                name: self.name,
                variant_name: name.to_string(),
            })
        } else {
            Err(BindingError::NativeEnumAlreadyContainsVariantWithSameValue{
                name: self.name,
                variant_value: value,
            })
        }
    }

    pub fn push(self, name: &str) -> Result<Self> {
        let value = self.next_value;
        self.variant(name, value)
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
