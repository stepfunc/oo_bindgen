use crate::collection::CollectionHandle;
use crate::doc::Doc;
use crate::iterator::IteratorHandle;
use crate::struct_common::{NativeStructDeclarationHandle, Visibility};
use crate::types::{AllTypes, BasicType, DurationType};
use crate::*;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug)]
pub enum AllStructFieldType {
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
    Struct(AllStructHandle),
    StructRef(NativeStructDeclarationHandle),
    Enum(EnumHandle, Option<String>),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
    Duration(DurationType, Option<Duration>),
}

impl AllStructFieldType {
    pub fn to_all_types(&self) -> AllTypes {
        match self {
            Self::Bool(_) => BasicType::Bool.into(),
            Self::Uint8(_) => BasicType::Uint8.into(),
            Self::Sint8(_) => BasicType::Sint8.into(),
            Self::Uint16(_) => BasicType::Uint16.into(),
            Self::Sint16(_) => BasicType::Sint16.into(),
            Self::Uint32(_) => BasicType::Uint32.into(),
            Self::Sint32(_) => BasicType::Sint32.into(),
            Self::Uint64(_) => BasicType::Uint64.into(),
            Self::Sint64(_) => BasicType::Sint64.into(),
            Self::Float(_) => BasicType::Float.into(),
            Self::Double(_) => BasicType::Double.into(),
            Self::String(_) => AllTypes::String,
            Self::Struct(handle) => AllTypes::Struct(handle.clone()),
            Self::StructRef(handle) => AllTypes::StructRef(handle.clone()),
            Self::Enum(handle, _) => BasicType::Enum(handle.clone()).into(),
            Self::ClassRef(handle) => AllTypes::ClassRef(handle.clone()),
            Self::Interface(handle) => AllTypes::Interface(handle.clone()),
            Self::Iterator(handle) => AllTypes::Iterator(handle.clone()),
            Self::Collection(handle) => AllTypes::Collection(handle.clone()),
            Self::Duration(mapping, _) => AllTypes::Basic(BasicType::Duration(*mapping)),
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
            Self::Struct(handle) => handle.all_fields_have_defaults(),
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
                return handle.validate_contains_variant_name(default)
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct AllStructField {
    pub name: String,
    pub field_type: AllStructFieldType,
    pub doc: Doc,
}

/// C-style structure definition
#[derive(Debug)]
pub struct AllStruct {
    pub visibility: Visibility,
    pub declaration: NativeStructDeclarationHandle,
    pub fields: Vec<AllStructField>,
    pub doc: Doc,
}

impl AllStruct {
    pub fn name(&self) -> &str {
        &self.declaration.name
    }

    pub fn declaration(&self) -> NativeStructDeclarationHandle {
        self.declaration.clone()
    }

    /// returns true if all struct fields have a default value
    pub fn all_fields_have_defaults(&self) -> bool {
        self.fields.iter().all(|el| el.field_type.has_default())
    }

    /// returns true if none of the struct fields have a default value
    pub fn no_fields_have_defaults(&self) -> bool {
        self.fields.iter().all(|el| !el.field_type.has_default())
    }

    pub fn fields(&self) -> impl Iterator<Item = &AllStructField> {
        self.fields.iter()
    }
}

pub type AllStructHandle = Handle<AllStruct>;

impl From<AllStructHandle> for AllTypes {
    fn from(x: AllStructHandle) -> Self {
        Self::Struct(x)
    }
}

impl From<AllStructHandle> for AllStructFieldType {
    fn from(x: AllStructHandle) -> Self {
        AllStructFieldType::Struct(x)
    }
}

pub struct AllStructBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    struct_type: Visibility,
    declaration: NativeStructDeclarationHandle,
    elements: Vec<AllStructField>,
    element_names_set: HashSet<String>,
    doc: Option<Doc>,
}

impl<'a> AllStructBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        declaration: NativeStructDeclarationHandle,
    ) -> Self {
        Self {
            lib,
            struct_type: Visibility::Public, // defaults to a public struct
            declaration,
            elements: Vec::new(),
            element_names_set: HashSet::new(),
            doc: None,
        }
    }

    pub fn make_opaque(mut self) -> Self {
        self.struct_type = Visibility::Private;
        self
    }

    pub fn add<S: Into<String>, T: Into<AllStructFieldType>, D: Into<Doc>>(
        mut self,
        name: S,
        element_type: T,
        doc: D,
    ) -> Result<Self> {
        let name = name.into();
        let element_type = element_type.into();
        element_type.validate()?;

        self.lib.validate_type(&element_type.to_all_types())?;
        if self.element_names_set.insert(name.to_string()) {
            self.elements.push(AllStructField {
                name,
                field_type: element_type,
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

    pub fn build(self) -> Result<AllStructHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        let handle = AllStructHandle::new(AllStruct {
            visibility: self.struct_type,
            declaration: self.declaration.clone(),
            fields: self.elements,
            doc,
        });

        self.lib.native_structs.insert(
            handle.declaration.clone(),
            NativeStructType::All(handle.clone()),
        );
        self.lib
            .statements
            .push(Statement::NativeStructDefinition(handle.clone()));

        Ok(handle)
    }
}

/// Associated method for structures
#[derive(Debug)]
pub struct Struct {
    pub definition: NativeStructType,
    pub methods: Vec<Method>,
    pub static_methods: Vec<Method>,
}

impl Struct {
    pub(crate) fn new(definition: NativeStructType) -> Self {
        Self {
            definition,
            methods: Vec::new(),
            static_methods: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        self.definition.name()
    }

    pub fn declaration(&self) -> NativeStructDeclarationHandle {
        self.definition.declaration()
    }

    pub fn definition(&self) -> AllStructHandle {
        match &self.definition {
            NativeStructType::All(x) => x.clone(),
            NativeStructType::FStruct(_, x) => x.clone(),
        }
    }

    pub fn elements(&self) -> impl Iterator<Item = &AllStructField> {
        match &self.definition {
            NativeStructType::All(x) => x.fields(),
            NativeStructType::FStruct(_, x) => x.fields(),
        }
    }

    pub fn doc(&self) -> &Doc {
        match &self.definition {
            NativeStructType::All(x) => &x.doc,
            NativeStructType::FStruct(_, x) => &x.doc,
        }
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

    pub fn find_element<T: AsRef<str>>(&self, element_name: T) -> Option<&AllStructField> {
        self.elements().find(|el| el.name == element_name.as_ref())
    }
}

pub type StructHandle = Handle<Struct>;

pub struct StructBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    definition: AllStructHandle,
    element_names_set: HashSet<String>,
    methods: Vec<Method>,
    static_methods: Vec<Method>,
}

impl<'a> StructBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, definition: AllStructHandle) -> Self {
        let mut element_names_set = HashSet::new();
        for el in &definition.fields {
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
            definition: NativeStructType::All(self.definition.clone()),
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
            if let AllTypes::StructRef(first_param_type) = &first_param.param_type {
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
