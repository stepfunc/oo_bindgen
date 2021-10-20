use std::collections::HashSet;

use crate::doc::{Doc, DocString};
use crate::iterator::IteratorHandle;
use crate::return_type::ReturnType;
use crate::structs::{CallbackArgStructField, CallbackStructHandle, UniversalStructHandle};
use crate::types::{Arg, DurationType, StringType, TypeValidator, ValidatedType};
use crate::*;

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

impl From<CallbackStructHandle> for CallbackArgument {
    fn from(x: CallbackStructHandle) -> Self {
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

pub type CReturnType = ReturnType<CallbackReturnValue>;

#[derive(Debug)]
pub struct CallbackFunction {
    pub name: String,
    pub return_type: CReturnType,
    pub arguments: Vec<Arg<CallbackArgument>>,
    pub doc: Doc,
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub callbacks: Vec<CallbackFunction>,
    pub doc: Doc,
}

impl Interface {
    pub fn find_callback<T: AsRef<str>>(&self, name: T) -> Option<&CallbackFunction> {
        self.callbacks
            .iter()
            .find(|callback| callback.name == name.as_ref())
    }

    pub fn is_functional(&self) -> bool {
        self.callbacks.len() == 1
    }
}

pub type InterfaceHandle = Handle<Interface>;

pub struct InterfaceBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    callbacks: Vec<CallbackFunction>,
    callback_names: HashSet<String>,
    doc: Doc,
}

impl<'a> InterfaceBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String, doc: Doc) -> Self {
        Self {
            lib,
            name,
            callbacks: Vec::new(),
            callback_names: Default::default(),
            doc,
        }
    }

    pub fn begin_callback<T: Into<String>, D: Into<Doc>>(
        mut self,
        name: T,
        doc: D,
    ) -> BindResult<CallbackFunctionBuilder<'a>> {
        let name = name.into();
        self.check_unique_callback_name(&name)?;
        Ok(CallbackFunctionBuilder::new(self, name, doc.into()))
    }

    pub fn build(self) -> BindResult<InterfaceHandle> {
        let handle = InterfaceHandle::new(Interface {
            name: self.name,
            callbacks: self.callbacks,
            doc: self.doc,
        });

        self.lib
            .add_statement(Statement::InterfaceDefinition(handle.clone()))?;
        Ok(handle)
    }

    fn check_unique_callback_name(&mut self, name: &str) -> BindResult<()> {
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
                interface_name: self.name.clone(),
                element_name: name.to_string(),
            })
        }
    }
}

pub struct CallbackFunctionBuilder<'a> {
    builder: InterfaceBuilder<'a>,
    name: String,
    return_type: Option<CReturnType>,
    arguments: Vec<Arg<CallbackArgument>>,
    doc: Doc,
}

impl<'a> CallbackFunctionBuilder<'a> {
    pub(crate) fn new(builder: InterfaceBuilder<'a>, name: String, doc: Doc) -> Self {
        Self {
            builder,
            name,
            return_type: None,
            arguments: Vec::new(),
            doc,
        }
    }

    pub fn param<S: Into<String>, D: Into<DocString>, P: Into<CallbackArgument>>(
        mut self,
        name: S,
        arg_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let arg_type = arg_type.into();
        let name = name.into();

        if name == CTX_VARIABLE_NAME {
            return Err(BindingError::CallbackMethodArgumentWithReservedName {
                name: CTX_VARIABLE_NAME,
            });
        }

        self.builder.lib.validate_type(&arg_type)?;
        self.arguments.push(Arg::new(arg_type, name, doc.into()));
        Ok(self)
    }

    pub fn returns<T: Into<CallbackReturnValue>, D: Into<DocString>>(
        self,
        t: T,
        d: D,
    ) -> BindResult<Self> {
        self.return_type(CReturnType::new(t, d))
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        self.return_type(CReturnType::Void)
    }

    fn return_type(mut self, return_type: CReturnType) -> BindResult<Self> {
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
