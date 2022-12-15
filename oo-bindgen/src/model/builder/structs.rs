use std::collections::HashSet;
use std::rc::Rc;

use crate::model::*;

pub type FunctionReturnStructBuilder<'a> = StructFieldBuilder<'a, FunctionReturnStructField>;
pub type CallbackArgStructBuilder<'a> = StructFieldBuilder<'a, CallbackArgStructField>;
pub type FunctionArgStructBuilder<'a> = StructFieldBuilder<'a, FunctionArgStructField>;
pub type UniversalStructBuilder<'a> = StructFieldBuilder<'a, UniversalStructField>;

pub struct StructFieldBuilder<'a, F>
where
    F: StructFieldType,
{
    lib: &'a mut LibraryBuilder,
    visibility: Visibility,
    declaration: TypedStructDeclaration<F>,
    fields: Vec<StructField<F, Unvalidated>>,
    field_names: HashSet<String>,
    doc: Option<Doc<Unvalidated>>,
}

impl<'a, F> StructFieldBuilder<'a, F>
where
    F: StructFieldType,
{
    pub(crate) fn new(lib: &'a mut LibraryBuilder, declaration: TypedStructDeclaration<F>) -> Self {
        Self::new_impl(lib, declaration, Visibility::Public)
    }

    pub(crate) fn opaque(
        lib: &'a mut LibraryBuilder,
        declaration: TypedStructDeclaration<F>,
    ) -> Self {
        Self::new_impl(lib, declaration, Visibility::Private)
    }

    fn new_impl(
        lib: &'a mut LibraryBuilder,
        declaration: TypedStructDeclaration<F>,
        visibility: Visibility,
    ) -> Self {
        Self {
            lib,
            visibility,
            declaration,
            fields: Vec::new(),
            field_names: HashSet::new(),
            doc: None,
        }
    }

    pub fn add<S: IntoName, V: Into<F>, D: Into<Doc<Unvalidated>>>(
        mut self,
        name: S,
        field_type: V,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        let field_type = field_type.into();

        if self.field_names.insert(name.to_string()) {
            self.fields.push(StructField {
                name,
                field_type,
                doc: doc.into(),
            });
            Ok(self)
        } else {
            Err(BindingErrorVariant::StructFieldDuplicateName {
                handle: self.declaration.inner.clone(),
                field_name: name,
            }
            .into())
        }
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingErrorVariant::DocAlreadyDefined {
                symbol_name: self.declaration.name().clone(),
            }
            .into()),
        }
    }

    pub fn end_fields(self) -> BindResult<StructMethodBuilder<'a, F>> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingErrorVariant::DocNotDefined {
                    symbol_name: self.declaration.name().clone(),
                }
                .into())
            }
        };

        Ok(StructMethodBuilder {
            lib: self.lib,
            visibility: self.visibility,
            declaration: self.declaration,
            fields: self.fields,
            initializers: Vec::new(),
            doc,
        })
    }
}

pub struct StructInitializerBuilder<'a, F>
where
    F: StructFieldType,
{
    name: Name,
    initializer_type: InitializerType,
    builder: StructMethodBuilder<'a, F>,
    fields: Vec<InitializedValue>,
    doc: Doc<Unvalidated>,
}

pub struct StructMethodBuilder<'a, F>
where
    F: StructFieldType,
{
    lib: &'a mut LibraryBuilder,
    visibility: Visibility,
    declaration: TypedStructDeclaration<F>,
    fields: Vec<StructField<F, Unvalidated>>,
    initializers: Vec<Handle<Initializer<Unvalidated>>>,
    doc: Doc<Unvalidated>,
}

impl<'a, F> StructMethodBuilder<'a, F>
where
    F: StructFieldType,
{
    pub fn begin_initializer<D: Into<Doc<Unvalidated>>, S: IntoName>(
        self,
        name: S,
        initializer_type: InitializerType,
        doc: D,
    ) -> BindResult<StructInitializerBuilder<'a, F>> {
        let name = name.into_name()?;

        // check that we don't have any other field or initializer with this name
        if self.fields.iter().any(|field| name == field.name)
            || self.initializers.iter().any(|c| name == c.name)
        {
            return Err(BindingErrorVariant::StructInitializerDuplicateName {
                struct_name: self.declaration.name().clone(),
                initializer_name: name,
            }
            .into());
        }

        Ok(StructInitializerBuilder {
            name,
            initializer_type,
            builder: self,
            fields: Vec::new(),
            doc: doc.into(),
        })
    }

    pub fn add_full_initializer<S: IntoName>(self, name: S) -> BindResult<Self> {
        let name = name.into_name()?;
        let struct_name = self.declaration.name().clone();
        self.begin_initializer(
            name,
            InitializerType::Normal,
            format!("Fully construct {{struct:{struct_name}}} specifying the value of each field"),
        )?
        .end_initializer()
    }

    pub fn build(self) -> BindResult<Handle<Struct<F, Unvalidated>>> {
        let handle = Handle::new(Struct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: self.fields,
            initializers: self.initializers,
            doc: self.doc,
        });

        self.lib
            .add_statement(Statement::StructDefinition(F::create_struct_type(
                handle.clone(),
            )))?;

        Ok(handle)
    }
}

impl<'a, F> StructInitializerBuilder<'a, F>
where
    F: StructFieldType,
{
    pub fn default<D: Into<InitializerDefault>>(
        mut self,
        name: &Name,
        value: D,
    ) -> BindResult<Self> {
        let value = value.into();

        // check that we haven't already defined this field
        if self.fields.iter().any(|f| f.name == *name) {
            return Err(BindingErrorVariant::StructInitializerDuplicateField {
                struct_name: self.builder.declaration.name().clone(),
                field_name: name.clone(),
            }
            .into());
        }

        // find the field and validate it
        let value = match self.builder.fields.iter().find(|f| f.name == *name) {
            Some(x) => x.field_type.validate_default_value(&value)?,
            None => {
                return Err(BindingErrorVariant::StructInitializerUnknownField {
                    struct_name: self.builder.declaration.name().clone(),
                    field_name: name.clone(),
                }
                .into());
            }
        };

        self.fields.push(InitializedValue {
            name: name.clone(),
            value,
        });

        Ok(self)
    }

    pub fn default_struct(self, name: &Name) -> BindResult<Self> {
        self.default(name, InitializerDefault::DefaultStruct)
    }

    pub fn default_variant<S: Into<String>>(self, name: &Name, variant: S) -> BindResult<Self> {
        self.default(name, InitializerDefault::Enum(variant.into()))
    }

    pub fn default_string<S: Into<String>>(self, name: &Name, value: S) -> BindResult<Self> {
        self.default(name, InitializerDefault::String(value.into()))
    }

    pub fn end_initializer(mut self) -> BindResult<StructMethodBuilder<'a, F>> {
        let initializer = Handle::new(Initializer {
            name: self.name,
            initializer_type: self.initializer_type,
            values: Rc::new(self.fields),
            doc: self.doc,
        });

        if let Some(x) = self
            .builder
            .initializers
            .iter()
            .find(|other| initializer.collides_with(other))
        {
            return Err(BindingErrorVariant::StructDuplicateInitializerArgs {
                struct_name: self.builder.declaration.name().clone(),
                this_initializer: initializer.name.clone(),
                other_initializer: x.name.clone(),
            }
            .into());
        }

        self.builder.initializers.push(initializer);
        Ok(self.builder)
    }
}
