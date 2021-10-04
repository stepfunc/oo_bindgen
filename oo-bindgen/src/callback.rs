use crate::doc::{Doc, DocString};
use crate::types::Arg;
use crate::*;
use std::collections::HashSet;

pub const CTX_VARIABLE_NAME: &str = "ctx";
pub const DESTROY_FUNC_NAME: &str = "on_destroy";

#[derive(Debug)]
pub struct CallbackFunction {
    pub name: String,
    pub return_type: ReturnType,
    pub arguments: Vec<Arg<AnyType>>,
    pub doc: Doc,
}

#[derive(Debug)]
pub enum InterfaceElement {
    Arg(String),
    DestroyFunction(String),
    CallbackFunction(CallbackFunction),
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub elements: Vec<InterfaceElement>,
    pub arg_name: String,
    pub destroy_name: String,
    pub doc: Doc,
}

impl Interface {
    pub fn callbacks(&self) -> impl Iterator<Item = &CallbackFunction> {
        self.elements.iter().filter_map(|el| match el {
            InterfaceElement::CallbackFunction(cb) => Some(cb),
            _ => None,
        })
    }

    pub fn find_callback<T: AsRef<str>>(&self, name: T) -> Option<&CallbackFunction> {
        self.callbacks()
            .find(|callback| callback.name == name.as_ref())
    }

    pub fn is_functional(&self) -> bool {
        self.callbacks().count() == 1
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
    elements: Vec<InterfaceElement>,
    element_names: HashSet<String>,
    arg_name: Option<String>,
    destroy_name: Option<String>,
    doc: Doc,
}

impl<'a> InterfaceBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String, doc: Doc) -> Self {
        Self {
            lib,
            name,
            elements: Vec::new(),
            element_names: HashSet::new(),
            arg_name: None,
            destroy_name: None,
            doc,
        }
    }

    pub fn callback<T: Into<String>, D: Into<Doc>>(
        mut self,
        name: T,
        doc: D,
    ) -> BindResult<CallbackFunctionBuilder<'a>> {
        let name = name.into();
        self.check_unique_name(&name)?;
        Ok(CallbackFunctionBuilder::new(self, name, doc.into()))
    }

    pub fn destroy_callback<T: Into<String>>(mut self, name: T) -> BindResult<Self> {
        match self.destroy_name {
            None => {
                let name = name.into();
                self.check_unique_name(&name)?;
                self.destroy_name = Some(name.to_string());
                self.elements.push(InterfaceElement::DestroyFunction(name));
                Ok(self)
            }
            Some(_) => Err(BindingError::InterfaceDestroyCallbackAlreadyDefined {
                interface_name: self.name,
            }),
        }
    }

    fn ctx<T: Into<String>>(mut self, name: T) -> BindResult<Self> {
        match self.arg_name {
            None => {
                let name = name.into();
                self.check_unique_name(&name)?;
                self.arg_name = Some(name.to_string());
                self.elements.push(InterfaceElement::Arg(name));
                Ok(self)
            }
            Some(_) => Err(BindingError::InterfaceArgNameAlreadyDefined {
                interface_name: self.name,
            }),
        }
    }

    pub fn build(mut self) -> BindResult<InterfaceHandle> {
        let arg_name = if let Some(arg_name) = self.arg_name {
            arg_name
        } else {
            self = self.ctx(CTX_VARIABLE_NAME)?;
            CTX_VARIABLE_NAME.to_string()
        };

        let destroy_name =
            self.destroy_name
                .ok_or(BindingError::InterfaceDestroyCallbackNotDefined {
                    interface_name: self.name.clone(),
                })?;

        let handle = InterfaceHandle::new(Interface {
            name: self.name,
            elements: self.elements,
            arg_name,
            destroy_name,
            doc: self.doc,
        });

        self.lib
            .add_statement(Statement::InterfaceDefinition(handle.clone()))?;
        Ok(handle)
    }

    fn check_unique_name(&mut self, name: &str) -> BindResult<()> {
        if self.element_names.insert(name.to_string()) {
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
    return_type: Option<ReturnType>,
    arguments: Vec<Arg<AnyType>>,
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

    pub fn param<S: Into<String>, D: Into<DocString>, P: Into<AnyType>>(
        mut self,
        name: S,
        arg_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let arg_type = arg_type.into();
        self.builder.lib.validate_type(&arg_type)?;
        self.arguments
            .push(Arg::new(arg_type, name.into(), doc.into()));
        Ok(self)
    }

    pub fn returns<T: Into<AnyType>, D: Into<DocString>>(self, t: T, d: D) -> BindResult<Self> {
        self.return_type(ReturnType::new(t, d))
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        self.return_type(ReturnType::Void)
    }

    fn return_type(mut self, return_type: ReturnType) -> BindResult<Self> {
        match self.return_type {
            None => {
                self.return_type = Some(return_type);
                Ok(self)
            }
            Some(return_type) => Err(BindingError::ReturnTypeAlreadyDefined {
                func_name: self.name,
                return_type,
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

        self.builder
            .elements
            .push(InterfaceElement::CallbackFunction(cb));
        Ok(self.builder)
    }
}
