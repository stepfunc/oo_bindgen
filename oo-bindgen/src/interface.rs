use crate::doc::{Doc, DocString};
use crate::iterator::IteratorHandle;
use crate::structs::callback_struct::CStructHandle;
use crate::types::{Arg, DurationType};
use crate::*;
use std::collections::HashSet;

pub const CTX_VARIABLE_NAME: &str = "ctx";
pub const DESTROY_FUNC_NAME: &str = "on_destroy";

/// Types that can be used as callback function arguments
#[derive(Debug, Clone, PartialEq)]
pub enum CArgument {
    Basic(BasicType),
    String,
    Iterator(IteratorHandle),
    Struct(CStructHandle),
    //StructRef(StructDeclarationHandle),
    //ClassRef(ClassDeclarationHandle),
    //Interface(InterfaceHandle),
}

impl From<CArgument> for AnyType {
    fn from(x: CArgument) -> Self {
        match x {
            CArgument::Basic(x) => Self::Basic(x),
            CArgument::String => Self::String,
            CArgument::Iterator(x) => Self::Iterator(x),
            CArgument::Struct(x) => Self::Struct(x.to_any_struct()),
        }
    }
}

impl From<BasicType> for CArgument {
    fn from(x: BasicType) -> Self {
        Self::Basic(x)
    }
}

impl From<DurationType> for CArgument {
    fn from(x: DurationType) -> Self {
        CArgument::Basic(BasicType::Duration(x))
    }
}

#[derive(Debug)]
pub struct CallbackFunction {
    pub name: String,
    pub return_type: FReturnType,
    pub arguments: Vec<Arg<CArgument>>,
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

impl From<InterfaceHandle> for AnyType {
    fn from(x: InterfaceHandle) -> Self {
        AnyType::Interface(x)
    }
}

impl From<InterfaceHandle> for AnyStructFieldType {
    fn from(x: InterfaceHandle) -> Self {
        AnyStructFieldType::Interface(x)
    }
}

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

    pub fn callback<T: Into<String>, D: Into<Doc>>(
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
    return_type: Option<ReturnType<FReturnValue>>,
    arguments: Vec<Arg<CArgument>>,
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

    pub fn param<S: Into<String>, D: Into<DocString>, P: Into<CArgument>>(
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

        self.builder.lib.validate_type(&arg_type.clone().into())?;
        self.arguments.push(Arg::new(arg_type, name, doc.into()));
        Ok(self)
    }

    pub fn returns<T: Into<FReturnValue>, D: Into<DocString>>(
        self,
        t: T,
        d: D,
    ) -> BindResult<Self> {
        self.return_type(ReturnType::new(t, d))
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        self.return_type(ReturnType::Void)
    }

    fn return_type(mut self, return_type: FReturnType) -> BindResult<Self> {
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

    pub fn build(mut self) -> BindResult<InterfaceBuilder<'a>> {
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
