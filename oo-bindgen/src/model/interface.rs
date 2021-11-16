use std::rc::Rc;

use crate::model::*;

/// Types allowed in callback function arguments
#[derive(Debug, Clone, PartialEq)]
pub enum CallbackArgument {
    Basic(BasicType),
    String(StringType),
    Iterator(AbstractIteratorHandle),
    Class(ClassDeclarationHandle),
    Struct(UniversalOr<CallbackArgStructField>),
}

impl From<BasicType> for CallbackArgument {
    fn from(x: BasicType) -> Self {
        Self::Basic(x)
    }
}

impl From<Handle<Enum<Unvalidated>>> for CallbackArgument {
    fn from(x: Handle<Enum<Unvalidated>>) -> Self {
        Self::Basic(BasicType::Enum(x))
    }
}

impl From<DurationType> for CallbackArgument {
    fn from(x: DurationType) -> Self {
        CallbackArgument::Basic(BasicType::Duration(x))
    }
}

impl From<AbstractIteratorHandle> for CallbackArgument {
    fn from(x: AbstractIteratorHandle) -> Self {
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

impl From<Handle<Enum<Unvalidated>>> for CallbackReturnValue {
    fn from(x: Handle<Enum<Unvalidated>>) -> Self {
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
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<CallbackFunction<Validated>> {
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
pub enum InterfaceCategory {
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

#[derive(Debug, Clone)]
pub enum InterfaceType<D>
where
    D: DocReference,
{
    Synchronous(Handle<Interface<D>>),
    Asynchronous(Handle<Interface<D>>),
    Future(FutureInterface<D>),
}

impl InterfaceType<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<InterfaceType<Validated>> {
        match self {
            InterfaceType::Synchronous(x) => Ok(InterfaceType::Synchronous(x.validate(lib)?)),
            InterfaceType::Asynchronous(x) => Ok(InterfaceType::Asynchronous(x.validate(lib)?)),
            InterfaceType::Future(x) => Ok(InterfaceType::Future(x.validate(lib)?)),
        }
    }
}

impl<D> InterfaceType<D>
where
    D: DocReference,
{
    pub fn name(&self) -> &Name {
        match self {
            InterfaceType::Synchronous(x) => &x.name,
            InterfaceType::Asynchronous(x) => &x.name,
            InterfaceType::Future(x) => &x.interface.name,
        }
    }

    pub fn mode(&self) -> InterfaceCategory {
        match self {
            InterfaceType::Synchronous(_) => InterfaceCategory::Synchronous,
            InterfaceType::Asynchronous(_) => InterfaceCategory::Asynchronous,
            InterfaceType::Future(_) => InterfaceCategory::Future,
        }
    }

    pub fn doc(&self) -> &Doc<D> {
        match self {
            InterfaceType::Synchronous(x) => &x.doc,
            InterfaceType::Asynchronous(x) => &x.doc,
            InterfaceType::Future(x) => &x.interface.doc,
        }
    }

    pub fn untyped(&self) -> &Handle<Interface<D>> {
        match self {
            InterfaceType::Synchronous(x) => x,
            InterfaceType::Asynchronous(x) => x,
            InterfaceType::Future(x) => &x.interface,
        }
    }
}

#[derive(Debug)]
pub struct Interface<D>
where
    D: DocReference,
{
    pub name: Name,
    pub mode: InterfaceCategory,
    pub callbacks: Vec<CallbackFunction<D>>,
    pub doc: Doc<D>,
    pub settings: Rc<LibrarySettings>,
}

impl Interface<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Handle<Interface<Validated>>> {
        let callbacks: BindResult<Vec<CallbackFunction<Validated>>> =
            self.callbacks.iter().map(|x| x.validate(lib)).collect();

        Ok(Handle::new(Interface {
            name: self.name.clone(),
            mode: self.mode,
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

/// Declares that the contained interface is asynchronous
///
/// Acts as a "New Type" around an interface handle to restrict where it can be used in the API model
#[derive(Debug, Clone)]
pub struct AsynchronousInterface {
    pub inner: InterfaceHandle,
}

/// Declares that the contained interface is synchronous only
///
/// Acts as a "New Type" around an interface handle to restrict where it can be used in the API model
#[derive(Debug, Clone)]
pub struct SynchronousInterface {
    pub inner: InterfaceHandle,
}

#[derive(Debug, Clone)]
pub struct FutureInterface<D>
where
    D: DocReference,
{
    pub value_type: CallbackArgument,
    pub value_type_doc: DocString<D>,
    pub error_type: OptionalErrorType<D>,
    pub interface: Handle<Interface<D>>,
}

impl FutureInterface<Unvalidated> {
    pub(crate) fn new(
        value_type: CallbackArgument,
        error_type: OptionalErrorType<Unvalidated>,
        interface: Handle<Interface<Unvalidated>>,
        value_type_doc: DocString<Unvalidated>,
    ) -> Self {
        Self {
            value_type,
            error_type,
            value_type_doc,
            interface,
        }
    }

    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<FutureInterface<Validated>> {
        Ok(FutureInterface {
            value_type: self.value_type.clone(),
            error_type: self.error_type.validate(lib)?,
            value_type_doc: self.value_type_doc.validate(&self.interface.name, lib)?,
            interface: self.interface.validate(lib)?,
        })
    }
}
