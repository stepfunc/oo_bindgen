use crate::*;
use std::collections::HashSet;

/// C-style structure forward declaration
#[derive(Debug)]
pub struct NativeStructDeclaration {
    pub name: String,
}

impl NativeStructDeclaration {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

pub type NativeStructDeclarationHandle = Handle<NativeStructDeclaration>;

#[derive(Debug)]
pub struct NativeStructElement {
    pub name: String,
    pub element_type: Type,
}

/// C-style structure definition
#[derive(Debug)]
pub struct NativeStruct {
    pub declaration: NativeStructDeclarationHandle,
    pub elements: Vec<NativeStructElement>,
}

impl NativeStruct {
    pub fn name(&self) -> &str {
        &self.declaration.name
    }

    pub fn declaration(&self) -> NativeStructDeclarationHandle {
        self.declaration.clone()
    }
}

pub type NativeStructHandle = Handle<NativeStruct>;

pub struct NativeStructBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    declaration: NativeStructDeclarationHandle,
    elements: Vec<NativeStructElement>,
    element_names_set: HashSet<String>,
}

impl<'a> NativeStructBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, declaration: NativeStructDeclarationHandle) -> Self {
        Self {
            lib,
            declaration,
            elements: Vec::new(),
            element_names_set: HashSet::new(),
        }
    }

    pub fn add(mut self, name: &str, element_type: Type) -> Result<Self> {
        self.lib.validate_type(&element_type)?;
        if self.element_names_set.insert(name.to_string()) {
            self.elements.push(NativeStructElement{
                name: name.to_string(),
                element_type,
            });
            Ok(self)
        } else {
            Err(BindingError::NativeStructAlreadyContainsElementWithSameName{
                handle: self.declaration,
                element_name: name.to_string(),
            })
        }
    }

    pub fn build(self) -> NativeStructHandle {
        let handle = NativeStructHandle::new(NativeStruct {
            declaration: self.declaration.clone(),
            elements: self.elements,
        });

        self.lib.native_structs.insert(handle.declaration.clone(), handle.clone());
        self.lib.statements.push(Statement::StructDefinition(handle.clone()));

        handle
    }
}
