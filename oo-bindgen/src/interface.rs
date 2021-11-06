use std::collections::HashSet;

use crate::doc::{Doc, DocReference, DocString, Unvalidated, Validated};
use crate::iterator::IteratorHandle;
use crate::name::{IntoName, Name};
use crate::return_type::ReturnType;
use crate::structs::{CallbackArgStructField, CallbackArgStructHandle, UniversalStructHandle};
use crate::types::{Arg, DurationType, StringType, TypeValidator, ValidatedType};
use crate::*;
use std::rc::Rc;

pub const CTX_VARIABLE_NAME: &str = "ctx";
pub const DESTROY_FUNC_NAME: &str = "on_destroy";

/// Types that can be used as callback function arguments
#[derive(Debug, Clone, PartialEq)]
pub enum CallbackArgument {
    Basic(BasicType),
    String(StringType),
    Iterator(IteratorHandle),
    Class(ClassDeclarationHandle),
    Struct(UniversalOr<CallbackArgStructField>),
}

impl TypeValidator for CallbackArgument {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            CallbackArgument::Basic(x) => x.get_validated_type(),
            CallbackArgument::String(x) => x.get_validated_type(),
            CallbackArgument::Iterator(x) => x.get_validated_type(),
            CallbackArgument::Struct(x) => x.get_validated_type(),
            CallbackArgument::Class(x) => x.get_validated_type(),
        }
    }
}

impl From<BasicType> for CallbackArgument {
    fn from(x: BasicType) -> Self {
        Self::Basic(x)
    }
}

impl From<EnumHandle> for CallbackArgument {
    fn from(x: EnumHandle) -> Self {
        Self::Basic(BasicType::Enum(x))
    }
}

impl From<DurationType> for CallbackArgument {
    fn from(x: DurationType) -> Self {
        CallbackArgument::Basic(BasicType::Duration(x))
    }
}

impl From<IteratorHandle> for CallbackArgument {
    fn from(x: IteratorHandle) -> Self {
        Self::Iterator(x)
    }
}

impl From<UniversalStructHandle> for CallbackArgument {
    fn from(x: UniversalStructHandle) -> Self {
        Self::Struct(UniversalOr::Universal(x))
    }
}

impl From<CallbackArgStructHandle> for CallbackArgument {
    fn from(x: CallbackArgStructHandle) -> Self {
        Self::Struct(x.into())
    }
}

impl From<StringType> for CallbackArgument {
    fn from(x: StringType) -> Self {
        Self::String(x)
    }
}

impl From<ClassDeclarationHandle> for CallbackArgument {
    fn from(x: ClassDeclarationHandle) -> Self {
        Self::Class(x)
    }
}

/// types that can be returned from callback functions
#[derive(Debug, Clone, PartialEq)]
pub enum CallbackReturnValue {
    Basic(BasicType),
    Struct(UniversalStructHandle),
}

impl From<BasicType> for CallbackReturnValue {
    fn from(x: BasicType) -> Self {
        CallbackReturnValue::Basic(x)
    }
}

impl From<UniversalStructHandle> for CallbackReturnValue {
    fn from(x: UniversalStructHandle) -> Self {
        CallbackReturnValue::Struct(x)
    }
}

impl From<DurationType> for CallbackReturnValue {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

impl From<EnumHandle> for CallbackReturnValue {
    fn from(x: EnumHandle) -> Self {
        Self::Basic(BasicType::Enum(x))
    }
}

pub type CallbackReturnType<T> = ReturnType<CallbackReturnValue, T>;

#[derive(Debug)]
pub struct CallbackFunction<D>
where
    D: DocReference,
{
    pub name: Name,
    pub return_type: ReturnType<CallbackReturnValue, D>,
    pub arguments: Vec<Arg<CallbackArgument, D>>,
    pub doc: Doc<D>,
}

impl CallbackFunction<Unvalidated> {
    pub(crate) fn validate(
        &self,
        lib: &UnvalidatedFields,
    ) -> BindResult<CallbackFunction<Validated>> {
        let arguments: BindResult<Vec<Arg<CallbackArgument, Validated>>> =
            self.arguments.iter().map(|x| x.validate(lib)).collect();

        let argument_names: Vec<Name> = self.arguments.iter().map(|x| x.name.clone()).collect();

        Ok(CallbackFunction {
            name: self.name.clone(),
            return_type: self.return_type.validate(&self.name, lib)?,
            arguments: arguments?,
            doc: self
                .doc
                .validate_with_args(&self.name, lib, Some(&argument_names))?,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    /// The interface will only be used in a synchronous context and the Rust
    /// backend will not generate Sync / Send implementations so it cannot be sent
    /// to other threads.
    Synchronous,
    /// The interface is used in asynchronous contexts where it will be invoked after
    /// a function call using it. The Rust backend will generate unsafe Sync / Send
    /// implementations allowing it to be based to other threads
    Asynchronous,
}

#[derive(Debug)]
pub struct Interface<D>
where
    D: DocReference,
{
    pub name: Name,
    pub interface_type: InterfaceType,
    pub callbacks: Vec<CallbackFunction<D>>,
    pub doc: Doc<D>,
    pub settings: Rc<LibrarySettings>,
}

impl Interface<Unvalidated> {
    pub(crate) fn validate(
        &self,
        lib: &UnvalidatedFields,
    ) -> BindResult<Handle<Interface<Validated>>> {
        let callbacks: BindResult<Vec<CallbackFunction<Validated>>> =
            self.callbacks.iter().map(|x| x.validate(lib)).collect();

        Ok(Handle::new(Interface {
            name: self.name.clone(),
            interface_type: self.interface_type,
            callbacks: callbacks?,
            doc: self.doc.validate(&self.name, lib)?,
            settings: self.settings.clone(),
        }))
    }
}

impl<D> Interface<D>
where
    D: DocReference,
{
    pub fn find_callback<S: AsRef<str>>(&self, name: S) -> Option<&CallbackFunction<D>> {
        self.callbacks
            .iter()
            .find(|callback| callback.name.as_ref() == name.as_ref())
    }

    pub fn is_functional(&self) -> bool {
        self.callbacks.len() == 1
    }
}

pub type InterfaceHandle = Handle<Interface<Unvalidated>>;

pub struct InterfaceBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    interface_type: InterfaceType,
    callbacks: Vec<CallbackFunction<Unvalidated>>,
    callback_names: HashSet<String>,
    doc: Doc<Unvalidated>,
}

impl<'a> InterfaceBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        name: Name,
        interface_type: InterfaceType,
        doc: Doc<Unvalidated>,
    ) -> Self {
        Self {
            lib,
            name,
            interface_type,
            callbacks: Vec::new(),
            callback_names: Default::default(),
            doc,
        }
    }

    pub fn begin_callback<T: IntoName, D: Into<Doc<Unvalidated>>>(
        mut self,
        name: T,
        doc: D,
    ) -> BindResult<CallbackFunctionBuilder<'a>> {
        let name = name.into_name()?;
        self.check_unique_callback_name(&name)?;
        Ok(CallbackFunctionBuilder::new(self, name, doc.into()))
    }

    pub fn build(self) -> BindResult<InterfaceHandle> {
        let handle = InterfaceHandle::new(Interface {
            name: self.name,
            interface_type: self.interface_type,
            callbacks: self.callbacks,
            doc: self.doc,
            settings: self.lib.settings.clone(),
        });

        self.lib
            .add_statement(Statement::InterfaceDefinition(handle.clone()))?;
        Ok(handle)
    }

    fn check_unique_callback_name(&mut self, name: &Name) -> BindResult<()> {
        if name == DESTROY_FUNC_NAME {
            return Err(BindingError::InterfaceMethodWithReservedName {
                name: DESTROY_FUNC_NAME,
            });
        }

        if name == CTX_VARIABLE_NAME {
            return Err(BindingError::InterfaceMethodWithReservedName {
                name: CTX_VARIABLE_NAME,
            });
        }

        if self.callback_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::InterfaceHasElementWithSameName {
                interface_name: self.name.to_string(),
                element_name: name.to_string(),
            })
        }
    }
}

pub struct CallbackFunctionBuilder<'a> {
    builder: InterfaceBuilder<'a>,
    name: Name,
    return_type: Option<CallbackReturnType<Unvalidated>>,
    arguments: Vec<Arg<CallbackArgument, Unvalidated>>,
    doc: Doc<Unvalidated>,
}

impl<'a> CallbackFunctionBuilder<'a> {
    pub(crate) fn new(builder: InterfaceBuilder<'a>, name: Name, doc: Doc<Unvalidated>) -> Self {
        Self {
            builder,
            name,
            return_type: None,
            arguments: Vec::new(),
            doc,
        }
    }

    pub fn param<S: IntoName, D: Into<DocString<Unvalidated>>, P: Into<CallbackArgument>>(
        mut self,
        name: S,
        arg_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let arg_type = arg_type.into();
        let name = name.into_name()?;

        if name == CTX_VARIABLE_NAME {
            return Err(BindingError::CallbackMethodArgumentWithReservedName {
                name: CTX_VARIABLE_NAME,
            });
        }

        self.builder.lib.validate_type(&arg_type)?;
        self.arguments.push(Arg::new(arg_type, name, doc.into()));
        Ok(self)
    }

    pub fn returns<T: Into<CallbackReturnValue>, D: Into<DocString<Unvalidated>>>(
        self,
        t: T,
        d: D,
    ) -> BindResult<Self> {
        self.return_type(CallbackReturnType::new(t, d))
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        self.return_type(CallbackReturnType::Void)
    }

    fn return_type(mut self, return_type: CallbackReturnType<Unvalidated>) -> BindResult<Self> {
        match self.return_type {
            None => {
                self.return_type = Some(return_type);
                Ok(self)
            }
            Some(_) => Err(BindingError::ReturnTypeAlreadyDefined {
                func_name: self.name,
            }),
        }
    }

    pub fn end_callback(mut self) -> BindResult<InterfaceBuilder<'a>> {
        let return_type = self.return_type.ok_or(BindingError::ReturnTypeNotDefined {
            func_name: self.name.clone(),
        })?;

        let cb = CallbackFunction {
            name: self.name,
            return_type,
            arguments: self.arguments,
            doc: self.doc,
        };

        self.builder.callbacks.push(cb);
        Ok(self.builder)
    }
}
