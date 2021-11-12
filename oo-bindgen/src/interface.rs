use std::collections::HashSet;

use crate::doc::{Doc, DocReference, DocString, Unvalidated, Validated};
use crate::iterator::IteratorHandle;
use crate::name::{IntoName, Name};
use crate::return_type::{OptionalReturnType, ReturnType};
use crate::structs::{
    CallbackArgStructField, CallbackArgStructHandle, UniversalOr, UniversalStructHandle,
};
use crate::types::{Arg, DurationType, StringType};
use crate::*;
use std::rc::Rc;

/// Types that can be used as callback function arguments
#[derive(Debug, Clone, PartialEq)]
pub enum CallbackArgument {
    Basic(BasicType),
    String(StringType),
    Iterator(IteratorHandle),
    Class(ClassDeclarationHandle),
    Struct(UniversalOr<CallbackArgStructField>),
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

/// A flag to the backend that tells it whether or not
/// to optimize callbacks into Functors in the public API
/// This flag is only inspected for functional interfaces
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionalTransform {
    /// If the interface is functional, it should be optimized into
    /// functors if the language supports them
    Yes,
    /// If the interface is functional, it will NOT be transformed
    No,
}

impl FunctionalTransform {
    pub fn enabled(&self) -> bool {
        match self {
            FunctionalTransform::Yes => true,
            FunctionalTransform::No => false,
        }
    }
}

#[derive(Debug)]
pub struct CallbackFunction<D>
where
    D: DocReference,
{
    pub name: Name,
    pub functional_transform: FunctionalTransform,
    pub return_type: OptionalReturnType<CallbackReturnValue, D>,
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
            functional_transform: self.functional_transform,
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
    /// An asynchronous interface which has particular properties
    Future,
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
    /// Return a reference to a CallbackFunction if and only if the interface has a single callback.
    ///
    /// This type of interface can be converted to a Functor-type in many backend languages
    pub fn get_functional_callback(&self) -> Option<&CallbackFunction<D>> {
        match self.callbacks.len() {
            1 => self.callbacks.get(0),
            _ => None,
        }
    }

    pub fn is_functional(&self) -> bool {
        self.get_functional_callback().is_some()
    }

    pub fn find_callback<S: AsRef<str>>(&self, name: S) -> Option<&CallbackFunction<D>> {
        self.callbacks
            .iter()
            .find(|callback| callback.name.as_ref() == name.as_ref())
    }
}

pub type InterfaceHandle = Handle<Interface<Unvalidated>>;

#[derive(Debug, Clone)]
pub struct FutureInterface<D>
where
    D: DocReference,
{
    pub value_type: CallbackArgument,
    pub value_type_doc: DocString<D>,
    pub interface: Handle<Interface<D>>,
}

impl FutureInterface<Unvalidated> {
    pub(crate) fn new(
        value_type: CallbackArgument,
        interface: Handle<Interface<Unvalidated>>,
        value_type_doc: DocString<Unvalidated>,
    ) -> Self {
        Self {
            value_type,
            value_type_doc,
            interface,
        }
    }

    pub(crate) fn validate(
        &self,
        lib: &UnvalidatedFields,
    ) -> BindResult<FutureInterface<Validated>> {
        Ok(FutureInterface {
            value_type: self.value_type.clone(),
            value_type_doc: self.value_type_doc.validate(&self.interface.name, lib)?,
            interface: self.interface.validate(lib)?,
        })
    }
}

pub struct InterfaceBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    callbacks: Vec<CallbackFunction<Unvalidated>>,
    callback_names: HashSet<String>,
    doc: Doc<Unvalidated>,
}

impl<'a> InterfaceBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: Name, doc: Doc<Unvalidated>) -> Self {
        Self {
            lib,
            name,
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

    /// Build the interface and mark it as only used in a synchronous context.
    ///
    /// A synchronous interface is one that is invoked only during a function call which
    /// takes it as an argument. The Rust backend will NOT generate `Send` and `Sync`
    /// implementations so that it be cannot be transferred across thread boundaries.
    pub fn build_sync(self) -> BindResult<InterfaceHandle> {
        self.build(InterfaceType::Synchronous)
    }

    /// Build the interface and mark it as used in an asynchronous context.
    ///
    /// An asynchronous interface is one that is invoked some time after it is
    /// passed as a function argument. The Rust backend will mark the C representation
    /// of this interface as `Send` and `Sync` so that it be transferred across thread
    /// boundaries.
    pub fn build_async(self) -> BindResult<InterfaceHandle> {
        self.build(InterfaceType::Asynchronous)
    }

    pub(crate) fn build(self, interface_type: InterfaceType) -> BindResult<InterfaceHandle> {
        let handle = InterfaceHandle::new(Interface {
            name: self.name,
            interface_type,
            callbacks: self.callbacks,
            doc: self.doc,
            settings: self.lib.settings.clone(),
        });

        self.lib
            .add_statement(Statement::InterfaceDefinition(handle.clone()))?;
        Ok(handle)
    }

    fn check_unique_callback_name(&mut self, name: &Name) -> BindResult<()> {
        if name == &self.lib.settings.interface.destroy_func_name {
            return Err(BindingError::InterfaceMethodWithReservedName {
                name: self.lib.settings.interface.destroy_func_name.clone(),
            });
        }

        if name == &self.lib.settings.interface.context_variable_name.clone() {
            return Err(BindingError::InterfaceMethodWithReservedName {
                name: self.lib.settings.interface.context_variable_name.clone(),
            });
        }

        if self.callback_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::InterfaceDuplicateCallbackName {
                interface_name: self.name.clone(),
                callback_name: name.clone(),
            })
        }
    }
}

pub struct CallbackFunctionBuilder<'a> {
    builder: InterfaceBuilder<'a>,
    name: Name,
    functional_transform: FunctionalTransform,
    return_type: OptionalReturnType<CallbackReturnValue, Unvalidated>,
    arguments: Vec<Arg<CallbackArgument, Unvalidated>>,
    doc: Doc<Unvalidated>,
}

impl<'a> CallbackFunctionBuilder<'a> {
    pub(crate) fn new(builder: InterfaceBuilder<'a>, name: Name, doc: Doc<Unvalidated>) -> Self {
        Self {
            builder,
            name,
            functional_transform: FunctionalTransform::No,
            return_type: OptionalReturnType::new(),
            arguments: Vec::new(),
            doc,
        }
    }

    pub fn enable_functional_transform(mut self) -> Self {
        self.functional_transform = FunctionalTransform::Yes;
        self
    }

    pub fn param<S: IntoName, D: Into<DocString<Unvalidated>>, P: Into<CallbackArgument>>(
        mut self,
        name: S,
        arg_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let arg_type = arg_type.into();
        let name = name.into_name()?;

        if name == self.builder.lib.settings.interface.context_variable_name {
            return Err(BindingError::CallbackMethodArgumentWithReservedName {
                name: self
                    .builder
                    .lib
                    .settings
                    .interface
                    .context_variable_name
                    .clone(),
            });
        }

        self.arguments.push(Arg::new(arg_type, name, doc.into()));
        Ok(self)
    }

    pub fn returns<T: Into<CallbackReturnValue>, D: Into<DocString<Unvalidated>>>(
        mut self,
        t: T,
        d: D,
    ) -> BindResult<Self> {
        self.return_type.set(&self.name, t.into(), d.into())?;
        Ok(self)
    }

    pub fn end_callback(mut self) -> BindResult<InterfaceBuilder<'a>> {
        let cb = CallbackFunction {
            name: self.name,
            functional_transform: self.functional_transform,
            return_type: self.return_type,
            arguments: self.arguments,
            doc: self.doc,
        };

        self.builder.callbacks.push(cb);
        Ok(self.builder)
    }
}
