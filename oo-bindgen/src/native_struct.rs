use crate::*;
use crate::doc::Doc;
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
    pub doc: Doc,
}

/// C-style structure definition
#[derive(Debug)]
pub struct NativeStruct {
    pub declaration: NativeStructDeclarationHandle,
    pub elements: Vec<NativeStructElement>,
    pub doc: Doc,
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
    doc: Option<Doc>,
}

impl<'a> NativeStructBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        declaration: NativeStructDeclarationHandle,
    ) -> Self {
        Self {
            lib,
            declaration,
            elements: Vec::new(),
            element_names_set: HashSet::new(),
            doc: None,
        }
    }

    pub fn add<D: Into<Doc>>(mut self, name: &str, element_type: Type, doc: D) -> Result<Self> {
        self.lib.validate_type(&element_type)?;
        if self.element_names_set.insert(name.to_string()) {
            self.elements.push(NativeStructElement {
                name: name.to_string(),
                element_type,
                doc: doc.into(),
            });
            Ok(self)
        } else {
            Err(
                BindingError::NativeStructAlreadyContainsElementWithSameName {
                    handle: self.declaration,
                    element_name: name.to_string(),
                },
            )
        }
    }

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> Result<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingError::DocAlreadyDefined {
                symbol_name: self.declaration.name.clone(),
            })
        }
    }

    pub fn build(self) -> Result<NativeStructHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => return Err(BindingError::DocNotDefined {
                symbol_name: self.declaration.name.clone(),
            })
        };

        let handle = NativeStructHandle::new(NativeStruct {
            declaration: self.declaration.clone(),
            elements: self.elements,
            doc,
        });

        self.lib
            .native_structs
            .insert(handle.declaration.clone(), handle.clone());
        self.lib
            .statements
            .push(Statement::NativeStructDefinition(handle.clone()));

        Ok(handle)
    }
}

/// Associated method for structures
#[derive(Debug)]
pub struct Struct {
    pub definition: NativeStructHandle,
    pub methods: Vec<Method>,
    pub static_methods: Vec<Method>,
}

impl Struct {
    pub(crate) fn new(definition: NativeStructHandle) -> Self {
        Self {
            definition,
            methods: Vec::new(),
            static_methods: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.definition.name()
    }

    pub fn declaration(&self) -> NativeStructDeclarationHandle {
        self.definition.declaration()
    }

    pub fn definition(&self) -> NativeStructHandle {
        self.definition.clone()
    }

    pub fn elements(&self) -> impl Iterator<Item = &NativeStructElement> {
        self.definition.elements.iter()
    }

    pub fn doc(&self) -> &Doc {
        &self.definition.doc
    }
}

pub type StructHandle = Handle<Struct>;

pub struct StructBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    definition: NativeStructHandle,
    element_names_set: HashSet<String>,
    methods: Vec<Method>,
    static_methods: Vec<Method>,
}

impl<'a> StructBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, definition: NativeStructHandle) -> Self {
        let mut element_names_set = HashSet::new();
        for el in &definition.elements {
            element_names_set.insert(el.name.clone());
        }

        Self {
            lib,
            definition,
            element_names_set,
            methods: Vec::new(),
            static_methods: Vec::new(),
        }
    }

    pub fn method(mut self, name: &str, native_function: &NativeFunctionHandle) -> Result<Self> {
        self.lib.validate_native_function(native_function)?;
        self.validate_first_param(native_function)?;

        if self.element_names_set.insert(name.to_string()) {
            self.methods.push(Method {
                name: name.to_string(),
                native_function: native_function.clone(),
            });
            Ok(self)
        } else {
            Err(BindingError::StructAlreadyContainsElementWithSameName {
                handle: self.definition.declaration(),
                element_name: name.to_string(),
            })
        }
    }

    pub fn static_method(
        mut self,
        name: &str,
        native_function: &NativeFunctionHandle,
    ) -> Result<Self> {
        self.lib.validate_native_function(native_function)?;

        if self.element_names_set.insert(name.to_string()) {
            self.static_methods.push(Method {
                name: name.to_string(),
                native_function: native_function.clone(),
            });
            Ok(self)
        } else {
            Err(BindingError::StructAlreadyContainsElementWithSameName {
                handle: self.definition.declaration(),
                element_name: name.to_string(),
            })
        }
    }

    pub fn build(self) -> StructHandle {
        let handle = StructHandle::new(Struct {
            definition: self.definition.clone(),
            methods: self.methods,
            static_methods: self.static_methods,
        });

        self.lib
            .defined_structs
            .insert(handle.definition.clone(), handle.clone());
        self.lib
            .statements
            .push(Statement::StructDefinition(handle.clone()));

        handle
    }

    fn validate_first_param(&self, native_function: &NativeFunctionHandle) -> Result<()> {
        if let Some(first_param) = native_function.parameters.first() {
            if let Type::StructRef(first_param_type) = &first_param.param_type {
                if first_param_type == &self.definition.declaration() {
                    return Ok(());
                }
            }
        }

        Err(BindingError::FirstMethodParameterIsNotStructType {
            handle: self.definition.declaration.clone(),
            native_func: native_function.clone(),
        })
    }
}
