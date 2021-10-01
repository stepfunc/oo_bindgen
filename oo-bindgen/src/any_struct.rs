use crate::collection::CollectionHandle;
use crate::doc::Doc;
use crate::iterator::IteratorHandle;
use crate::struct_common::{StructDeclarationHandle, Visibility};
use crate::types::{AnyType, BasicType, DurationType};
use crate::*;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug)]
pub enum AnyStructFieldType {
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
    Struct(AnyStructHandle),
    StructRef(StructDeclarationHandle),
    Enum(EnumHandle, Option<String>),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
    Duration(DurationType, Option<Duration>),
}

impl AnyStructFieldType {
    pub fn to_all_types(&self) -> AnyType {
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
            Self::String(_) => AnyType::String,
            Self::Struct(handle) => AnyType::Struct(handle.clone()),
            Self::StructRef(handle) => AnyType::StructRef(handle.clone()),
            Self::Enum(handle, _) => BasicType::Enum(handle.clone()).into(),
            Self::ClassRef(handle) => AnyType::ClassRef(handle.clone()),
            Self::Interface(handle) => AnyType::Interface(handle.clone()),
            Self::Iterator(handle) => AnyType::Iterator(handle.clone()),
            Self::Collection(handle) => AnyType::Collection(handle.clone()),
            Self::Duration(mapping, _) => AnyType::Basic(BasicType::Duration(*mapping)),
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
            Self::Enum(handle, Some(default)) => handle.validate_contains_variant_name(default),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct AnyStructField {
    pub name: String,
    pub field_type: AnyStructFieldType,
    pub doc: Doc,
}

/// C-style structure definition
#[derive(Debug)]
pub struct AnyStruct {
    pub visibility: Visibility,
    pub declaration: StructDeclarationHandle,
    pub fields: Vec<AnyStructField>,
    pub doc: Doc,
}

impl AnyStruct {
    pub fn name(&self) -> &str {
        &self.declaration.name
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
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

    pub fn fields(&self) -> impl Iterator<Item = &AnyStructField> {
        self.fields.iter()
    }
}

pub type AnyStructHandle = Handle<AnyStruct>;

impl From<AnyStructHandle> for AnyType {
    fn from(x: AnyStructHandle) -> Self {
        Self::Struct(x)
    }
}

impl From<AnyStructHandle> for AnyStructFieldType {
    fn from(x: AnyStructHandle) -> Self {
        AnyStructFieldType::Struct(x)
    }
}

impl From<FStructHandle> for AnyStructHandle {
    fn from(x: FStructHandle) -> Self {
        x.to_any_struct()
    }
}

pub struct AnyStructBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    struct_type: Visibility,
    declaration: StructDeclarationHandle,
    elements: Vec<AnyStructField>,
    element_names_set: HashSet<String>,
    doc: Option<Doc>,
}

impl<'a> AnyStructBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, declaration: StructDeclarationHandle) -> Self {
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

    pub fn add<S: Into<String>, T: Into<AnyStructFieldType>, D: Into<Doc>>(
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
            self.elements.push(AnyStructField {
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

    pub fn build(self) -> Result<AnyStructHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        let handle = AnyStructHandle::new(AnyStruct {
            visibility: self.struct_type,
            declaration: self.declaration.clone(),
            fields: self.elements,
            doc,
        });

        let struct_type = StructType::Any(handle.clone());

        self.lib
            .structs
            .insert(handle.declaration.clone(), struct_type.clone());
        self.lib
            .statements
            .push(Statement::StructDefinition(struct_type));

        Ok(handle)
    }
}
