use crate::any_struct::{AnyStruct, AnyStructField, AnyStructFieldType};
use crate::doc::Doc;
use crate::enum_type::EnumHandle;
use crate::types::AnyType;
use crate::{BindResult, BindingError, Handle, LibraryBuilder, Statement, StructType};
use std::collections::HashSet;

/// An enum which might contain a validated default value
#[derive(Clone, Debug)]
pub struct EnumField {
    pub handle: EnumHandle,
    pub default_variant: Option<String>,
}

impl EnumField {
    pub(crate) fn new(handle: EnumHandle) -> Self {
        Self {
            handle,
            default_variant: None,
        }
    }

    pub fn try_default(handle: EnumHandle, default_variant: &str) -> BindResult<Self> {
        handle.validate_contains_variant_name(default_variant)?;
        Ok(Self {
            handle,
            default_variant: Some(default_variant.to_string()),
        })
    }
}

/// struct type affects the type of code the backend will generate
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visibility {
    /// struct members are public
    Public,
    /// struct members are private (except C of course), and the struct is just an opaque "token"
    Private,
}

/// C-style structure forward declaration
#[derive(Debug)]
pub struct StructDeclaration {
    pub name: String,
}

impl StructDeclaration {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

pub type StructDeclarationHandle = Handle<StructDeclaration>;

impl From<StructDeclarationHandle> for AnyType {
    fn from(x: StructDeclarationHandle) -> Self {
        Self::StructRef(x)
    }
}

pub trait StructFieldType: Clone + Sized {
    /// indicates if the field has a default value specified
    fn has_default(&self) -> bool;

    /// convert a structure to a StructType
    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType;

    /// TODO - this will go away
    fn to_any_struct_field_type(self) -> AnyStructFieldType;

    /// TODO - this will go away
    fn to_any_type(&self) -> AnyType;
}

#[derive(Debug)]
pub struct StructField<F>
where
    F: StructFieldType,
{
    pub name: String,
    pub field_type: F,
    pub doc: Doc,
}

impl<F> StructField<F>
where
    F: StructFieldType,
{
    pub(crate) fn to_any_struct_field(&self) -> AnyStructField {
        AnyStructField {
            name: self.name.clone(),
            field_type: self.field_type.clone().to_any_struct_field_type(),
            doc: self.doc.clone(),
        }
    }
}

/// C-style structure definition
#[derive(Debug)]
pub struct Struct<F>
where
    F: StructFieldType,
{
    pub visibility: Visibility,
    pub declaration: StructDeclarationHandle,
    pub fields: Vec<StructField<F>>,
    pub doc: Doc,
}

impl<F> Struct<F>
where
    F: StructFieldType,
{
    pub fn to_any_struct(&self) -> Handle<AnyStruct> {
        Handle::new(AnyStruct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: self
                .fields
                .iter()
                .map(|f| f.to_any_struct_field())
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
        self.fields
            .iter()
            .all(|field| field.field_type.has_default())
    }

    /// returns true if none of the struct fields have a default value
    pub fn no_fields_have_defaults(&self) -> bool {
        self.fields
            .iter()
            .all(|field| !field.field_type.has_default())
    }

    pub fn fields(&self) -> impl Iterator<Item = &StructField<F>> {
        self.fields.iter()
    }
}

pub struct StructBuilder<'a, F>
where
    F: StructFieldType,
{
    lib: &'a mut LibraryBuilder,
    visibility: Visibility,
    declaration: StructDeclarationHandle,
    fields: Vec<StructField<F>>,
    field_names: HashSet<String>,
    doc: Option<Doc>,
}

impl<'a, F> StructBuilder<'a, F>
where
    F: StructFieldType,
{
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

    pub fn add<S: Into<String>, V: Into<F>, D: Into<Doc>>(
        mut self,
        name: S,
        field_type: V,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into();
        let field_type = field_type.into();

        self.lib.validate_type(&field_type.to_any_type())?;
        if self.field_names.insert(name.to_string()) {
            self.fields.push(StructField {
                name,
                field_type,
                doc: doc.into(),
            });
            Ok(self)
        } else {
            Err(BindingError::StructAlreadyContainsFieldWithSameName {
                handle: self.declaration,
                field_name: name,
            })
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

    pub fn build(self) -> BindResult<Handle<Struct<F>>> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        let handle = Handle::new(Struct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: self.fields,
            doc,
        });

        self.lib
            .add_statement(Statement::StructDefinition(F::create_struct_type(
                handle.clone(),
            )))?;

        Ok(handle)
    }
}
