use crate::doc::Doc;
use crate::enum_type::EnumHandle;
use crate::structs::any_struct::{AnyStruct, AnyStructField, AnyStructFieldType};
use crate::types::AnyType;
use crate::{BindResult, BindingError, Handle, LibraryBuilder, Statement, StructType};
use std::collections::HashSet;
use std::fmt::Formatter;

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

    pub fn with_default(handle: EnumHandle, default_variant: &str) -> BindResult<Self> {
        handle.validate_contains_variant_name(default_variant)?;
        Ok(Self {
            handle,
            default_variant: Some(default_variant.to_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldName {
    name: Handle<String>,
}

impl FieldName {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: Handle::new(name.into()),
        }
    }
}

impl std::ops::Deref for FieldName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.name)
    }
}

impl From<&str> for FieldName {
    fn from(x: &str) -> Self {
        FieldName::new(x)
    }
}

/// Values used to define constructors
#[derive(Debug, Clone)]
pub enum ConstructorValue {
    Bool(bool),
    Uint8(u8),
    Sint8(i8),
    Uint16(u16),
    Sint16(i16),
    Uint32(u32),
    Sint32(i32),
    Uint64(u64),
    Sint64(i64),
    Float(f32),
    Double(f64),
    Duration(std::time::Duration),
    Enum(String),
    String(String),
    /// requires that the struct have a default constructor
    DefaultStruct(FieldName),
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
    /*
       /// Check that the default value is valid for the type
       fn validate(&self, name: &FieldName, x: &ConstructorValue) -> BindResult<()>;
    */
}

#[derive(Debug)]
pub struct StructField<F>
where
    F: StructFieldType,
{
    pub name: FieldName,
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
    pub constructors: Vec<Constructor>,
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
            constructors: self.constructors.clone(),
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

pub struct StructFieldBuilder<'a, F>
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

impl<'a, F> StructFieldBuilder<'a, F>
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

    pub fn add<S: Into<FieldName>, V: Into<F>, D: Into<Doc>>(
        mut self,
        name: S,
        field_type: V,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into();
        let field_type = field_type.into();

        self.lib.validate_type(&field_type.to_any_type())?;
        if self.field_names.insert((*name).clone()) {
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

    pub fn end_fields(self) -> BindResult<StructConstructorBuilder<'a, F>> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        Ok(StructConstructorBuilder {
            lib: self.lib,
            visibility: self.visibility,
            declaration: self.declaration,
            fields: self.fields,
            constructors: Vec::new(),
            doc,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InitializedValue {
    pub name: FieldName,
    pub value: ConstructorValue,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    values: Vec<InitializedValue>,
}

impl Constructor {
    pub fn values(&self) -> &[InitializedValue] {
        &self.values
    }
}

pub struct StructConstructorBuilder<'a, F>
where
    F: StructFieldType,
{
    lib: &'a mut LibraryBuilder,
    pub visibility: Visibility,
    pub declaration: StructDeclarationHandle,
    pub fields: Vec<StructField<F>>,
    pub constructors: Vec<Constructor>,
    pub doc: Doc,
}

impl<'a, F> StructConstructorBuilder<'a, F>
where
    F: StructFieldType,
{
    pub fn build(self) -> BindResult<Handle<Struct<F>>> {
        let handle = Handle::new(Struct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: self.fields,
            constructors: self.constructors,
            doc: self.doc,
        });

        self.lib
            .add_statement(Statement::StructDefinition(F::create_struct_type(
                handle.clone(),
            )))?;

        Ok(handle)
    }
}
