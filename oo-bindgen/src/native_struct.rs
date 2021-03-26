use crate::collection::CollectionHandle;
use crate::doc::Doc;
use crate::iterator::IteratorHandle;
use crate::*;
use std::collections::HashSet;
use std::time::Duration;

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
pub enum StructElementType {
    Bool(Option<bool>),
    Uint8(Option<u8>),
    Sint8(Option<i8>),
    Uint16(Option<u16>),
    Sint16(Option<i16>),
    Uint32(Option<u32>),
    Sint32(Option<i32>),
    Uint64(Option<u64>),
    Sint64(Option<i64>),
    Float(Option<f32>),
    Double(Option<f64>),
    String(Option<String>),
    Struct(NativeStructHandle),
    StructRef(NativeStructDeclarationHandle),
    Enum(NativeEnumHandle, Option<String>),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
    Duration(DurationMapping, Option<Duration>),
}

impl StructElementType {
    pub fn to_type(&self) -> Type {
        match self {
            Self::Bool(_) => Type::Bool,
            Self::Uint8(_) => Type::Uint8,
            Self::Sint8(_) => Type::Sint8,
            Self::Uint16(_) => Type::Uint16,
            Self::Sint16(_) => Type::Sint16,
            Self::Uint32(_) => Type::Uint32,
            Self::Sint32(_) => Type::Sint32,
            Self::Uint64(_) => Type::Uint64,
            Self::Sint64(_) => Type::Sint64,
            Self::Float(_) => Type::Float,
            Self::Double(_) => Type::Double,
            Self::String(_) => Type::String,
            Self::Struct(handle) => Type::Struct(handle.clone()),
            Self::StructRef(handle) => Type::StructRef(handle.clone()),
            Self::Enum(handle, _) => Type::Enum(handle.clone()),
            Self::ClassRef(handle) => Type::ClassRef(handle.clone()),
            Self::Interface(handle) => Type::Interface(handle.clone()),
            Self::Iterator(handle) => Type::Iterator(handle.clone()),
            Self::Collection(handle) => Type::Collection(handle.clone()),
            Self::Duration(mapping, _) => Type::Duration(*mapping),
        }
    }

    pub fn has_default(&self) -> bool {
        match self {
            Self::Bool(default) => default.is_some(),
            Self::Uint8(default) => default.is_some(),
            Self::Sint8(default) => default.is_some(),
            Self::Uint16(default) => default.is_some(),
            Self::Sint16(default) => default.is_some(),
            Self::Uint32(default) => default.is_some(),
            Self::Sint32(default) => default.is_some(),
            Self::Uint64(default) => default.is_some(),
            Self::Sint64(default) => default.is_some(),
            Self::Float(default) => default.is_some(),
            Self::Double(default) => default.is_some(),
            Self::String(default) => default.is_some(),
            Self::Struct(handle) => handle.is_default_constructed(),
            Self::StructRef(_) => false,
            Self::Enum(_, default) => default.is_some(),
            Self::ClassRef(_) => false,
            Self::Interface(_) => false,
            Self::Iterator(_) => false,
            Self::Collection(_) => false,
            Self::Duration(_, default) => default.is_some(),
        }
    }

    fn validate(&self) -> Result<()> {
        match self {
            Self::Enum(handle, Some(default)) => {
                if handle.find_variant_by_name(default).is_none() {
                    Err(BindingError::NativeEnumDoesNotContainVariant {
                        name: handle.name.to_string(),
                        variant_name: default.to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

impl From<Type> for StructElementType {
    fn from(from: Type) -> Self {
        match from {
            Type::Bool => Self::Bool(None),
            Type::Uint8 => Self::Uint8(None),
            Type::Sint8 => Self::Sint8(None),
            Type::Uint16 => Self::Uint16(None),
            Type::Sint16 => Self::Sint16(None),
            Type::Uint32 => Self::Uint32(None),
            Type::Sint32 => Self::Sint32(None),
            Type::Uint64 => Self::Uint64(None),
            Type::Sint64 => Self::Sint64(None),
            Type::Float => Self::Float(None),
            Type::Double => Self::Double(None),
            Type::String => Self::String(None),
            Type::Struct(handle) => Self::Struct(handle),
            Type::StructRef(handle) => Self::StructRef(handle),
            Type::Enum(handle) => Self::Enum(handle, None),
            Type::ClassRef(handle) => Self::ClassRef(handle),
            Type::Interface(handle) => Self::Interface(handle),
            Type::Iterator(handle) => Self::Iterator(handle),
            Type::Collection(handle) => Self::Collection(handle),
            Type::Duration(mapping) => Self::Duration(mapping, None),
        }
    }
}

/// struct type affects the type of code the backend will generate
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NativeStructType {
    /// struct members are public
    Public,
    /// struct members are private (except C of course), and the struct is just an opaque "token"
    Opaque,
}

#[derive(Debug)]
pub struct NativeStructElement {
    pub name: String,
    pub element_type: StructElementType,
    pub doc: Doc,
}

/// C-style structure definition
#[derive(Debug)]
pub struct NativeStruct {
    pub struct_type: NativeStructType,
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

    pub fn is_default_constructed(&self) -> bool {
        self.elements.iter().all(|el| el.element_type.has_default())
    }

    pub fn elements(&self) -> impl Iterator<Item = &NativeStructElement> {
        self.elements.iter()
    }
}

pub type NativeStructHandle = Handle<NativeStruct>;

pub struct NativeStructBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    struct_type: NativeStructType,
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
            struct_type: NativeStructType::Public, // defaults to a public struct
            declaration,
            elements: Vec::new(),
            element_names_set: HashSet::new(),
            doc: None,
        }
    }

    pub fn make_opaque(mut self) -> Self {
        self.struct_type = NativeStructType::Opaque;
        self
    }

    pub fn add<S: Into<String>, T: Into<StructElementType>, D: Into<Doc>>(
        mut self,
        name: S,
        element_type: T,
        doc: D,
    ) -> Result<Self> {
        let name = name.into();
        let element_type = element_type.into();
        element_type.validate()?;

        self.lib.validate_type(&element_type.to_type())?;
        if self.element_names_set.insert(name.to_string()) {
            self.elements.push(NativeStructElement {
                name,
                element_type,
                doc: doc.into(),
            });
            Ok(self)
        } else {
            Err(
                BindingError::NativeStructAlreadyContainsElementWithSameName {
                    handle: self.declaration,
                    element_name: name,
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
            }),
        }
    }

    pub fn build(self) -> Result<NativeStructHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        let handle = NativeStructHandle::new(NativeStruct {
            struct_type: self.struct_type,
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
        self.definition.elements()
    }

    pub fn doc(&self) -> &Doc {
        &self.definition.doc
    }

    pub fn find_method<T: AsRef<str>>(&self, method_name: T) -> Option<&NativeFunctionHandle> {
        for method in &self.methods {
            if method.name == method_name.as_ref() {
                return Some(&method.native_function);
            }
        }

        for method in &self.static_methods {
            if method.name == method_name.as_ref() {
                return Some(&method.native_function);
            }
        }

        None
    }

    pub fn find_element<T: AsRef<str>>(&self, element_name: T) -> Option<&NativeStructElement> {
        self.elements().find(|el| el.name == element_name.as_ref())
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

    pub fn method<T: Into<String>>(
        mut self,
        name: T,
        native_function: &NativeFunctionHandle,
    ) -> Result<Self> {
        let name = name.into();
        self.lib.validate_native_function(native_function)?;
        self.validate_first_param(native_function)?;

        if self.element_names_set.insert(name.to_string()) {
            self.methods.push(Method {
                name,
                native_function: native_function.clone(),
            });
            Ok(self)
        } else {
            Err(BindingError::StructAlreadyContainsElementWithSameName {
                handle: self.definition.declaration(),
                element_name: name,
            })
        }
    }

    pub fn static_method<T: Into<String>>(
        mut self,
        name: T,
        native_function: &NativeFunctionHandle,
    ) -> Result<Self> {
        let name = name.into();
        self.lib.validate_native_function(native_function)?;

        if self.element_names_set.insert(name.to_string()) {
            self.static_methods.push(Method {
                name,
                native_function: native_function.clone(),
            });
            Ok(self)
        } else {
            Err(BindingError::StructAlreadyContainsElementWithSameName {
                handle: self.definition.declaration(),
                element_name: name,
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
