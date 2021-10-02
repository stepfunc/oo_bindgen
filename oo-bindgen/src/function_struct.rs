use crate::collection::CollectionHandle;
use crate::doc::Doc;
use crate::struct_common::{StructDeclarationHandle, Visibility};
use crate::types::{AnyType, DurationType};
use crate::*;
use std::collections::HashSet;
use std::time::Duration;

/// Types that can be used in a function struct, some of which might have a default value
#[derive(Debug)]
pub enum FStructFieldType {
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
    Enum(EnumHandle, Option<String>),
    Interface(InterfaceHandle),
    Collection(CollectionHandle),
    Duration(DurationType, Option<Duration>),
    Struct(FStructHandle),
}

impl From<FStructHandle> for FStructFieldType {
    fn from(x: FStructHandle) -> Self {
        FStructFieldType::Struct(x)
    }
}

impl From<InterfaceHandle> for FStructFieldType {
    fn from(x: InterfaceHandle) -> Self {
        FStructFieldType::Interface(x)
    }
}

impl FStructFieldType {
    pub fn to_any_type(&self) -> AnyType {
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
            Self::Enum(handle, _) => BasicType::Enum(handle.clone()).into(),
            Self::Interface(handle) => AnyType::Interface(handle.clone()),
            Self::Collection(handle) => AnyType::Collection(handle.clone()),
            Self::Duration(t, _) => AnyType::Basic(BasicType::Duration(*t)),
            Self::Struct(x) => AnyType::Struct(x.to_any_struct()),
        }
    }

    pub fn to_any_struct_field_type(&self) -> AnyStructFieldType {
        match self {
            Self::Bool(d) => AnyStructFieldType::Bool(*d),
            Self::Uint8(d) => AnyStructFieldType::Uint8(*d),
            Self::Sint8(d) => AnyStructFieldType::Sint8(*d),
            Self::Uint16(d) => AnyStructFieldType::Uint16(*d),
            Self::Sint16(d) => AnyStructFieldType::Sint16(*d),
            Self::Uint32(d) => AnyStructFieldType::Uint32(*d),
            Self::Sint32(d) => AnyStructFieldType::Sint32(*d),
            Self::Uint64(d) => AnyStructFieldType::Uint64(*d),
            Self::Sint64(d) => AnyStructFieldType::Sint64(*d),
            Self::Float(d) => AnyStructFieldType::Float(*d),
            Self::Double(d) => AnyStructFieldType::Double(*d),
            Self::String(d) => AnyStructFieldType::String(d.clone()),
            Self::Enum(h, d) => AnyStructFieldType::Enum(h.clone(), d.clone()),
            Self::Interface(h) => AnyStructFieldType::Interface(h.clone()),
            Self::Collection(h) => AnyStructFieldType::Collection(h.clone()),
            Self::Duration(t, d) => AnyStructFieldType::Duration(*t, *d),
            Self::Struct(x) => AnyStructFieldType::Struct(x.to_any_struct()),
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
            Self::Enum(_, default) => default.is_some(),
            Self::Interface(_) => false,
            Self::Collection(_) => false,
            Self::Duration(_, default) => default.is_some(),
            Self::Struct(x) => x.all_fields_have_defaults(),
        }
    }

    fn validate(&self) -> BindResult<()> {
        match self {
            Self::Enum(handle, Some(default)) => handle.validate_contains_variant_name(default),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct FStructField {
    pub name: String,
    pub field_type: FStructFieldType,
    pub doc: Doc,
}

impl FStructField {
    pub(crate) fn to_any_struct_field(&self) -> AnyStructField {
        AnyStructField {
            name: self.name.clone(),
            field_type: self.field_type.to_any_struct_field_type(),
            doc: self.doc.clone(),
        }
    }
}

/// Function structs can be used as native function parameters
#[derive(Debug)]
pub struct FStruct {
    pub visibility: Visibility,
    pub declaration: StructDeclarationHandle,
    pub fields: Vec<FStructField>,
    pub doc: Doc,
}

impl FStruct {
    pub(crate) fn to_any_struct(&self) -> Handle<AnyStruct> {
        Handle::new(AnyStruct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: self
                .fields
                .iter()
                .map(|x| x.to_any_struct_field())
                .collect(),
            doc: self.doc.clone(),
        })
    }

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
        self.fields.iter().all(|f| !f.field_type.has_default())
    }

    pub fn fields(&self) -> impl Iterator<Item = &FStructField> {
        self.fields.iter()
    }
}

pub type FStructHandle = Handle<FStruct>;

pub struct FStructBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    visibility: Visibility,
    declaration: StructDeclarationHandle,
    fields: Vec<FStructField>,
    field_names: HashSet<String>,
    doc: Option<Doc>,
}

impl<'a> FStructBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, declaration: StructDeclarationHandle) -> Self {
        Self {
            lib,
            visibility: Visibility::Public, // defaults to a public struct
            declaration,
            fields: Vec::new(),
            field_names: HashSet::new(),
            doc: None,
        }
    }

    pub fn make_opaque(mut self) -> Self {
        self.visibility = Visibility::Private;
        self
    }

    pub fn add<S: Into<String>, T: Into<FStructFieldType>, D: Into<Doc>>(
        mut self,
        name: S,
        field_type: T,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into();
        let field_type = field_type.into();
        field_type.validate()?;

        self.lib.validate_type(&field_type.to_any_type())?;
        if self.field_names.insert(name.to_string()) {
            self.fields.push(FStructField {
                name,
                field_type,
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

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> BindResult<Self> {
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

    pub fn build(self) -> BindResult<FStructHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        let handle = Handle::new(FStruct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: self.fields,
            doc,
        });

        let struct_type = StructType::FStruct(handle.clone(), handle.to_any_struct());

        self.lib
            .add_statement(Statement::StructDefinition(struct_type))?;

        Ok(handle)
    }
}
