use crate::doc::Doc;
use crate::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum CallbackParameter {
    Arg(String),
    Parameter(Parameter),
}

#[derive(Debug)]
pub struct CallbackFunction {
    pub name: String,
    pub return_type: ReturnType,
    pub parameters: Vec<CallbackParameter>,
    pub arg_name: String,
    pub doc: Doc,
}

impl CallbackFunction {
    pub fn params(&self) -> impl Iterator<Item = &Parameter> {
        self.parameters.iter().filter_map(|param| match param {
            CallbackParameter::Parameter(param) => Some(param),
            _ => None,
        })
    }
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
}

pub type InterfaceHandle = Handle<Interface>;

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

    pub fn callback<D: Into<Doc>>(
        mut self,
        name: &str,
        doc: D,
    ) -> Result<CallbackFunctionBuilder<Self>> {
        self.check_unique_name(name)?;
        Ok(CallbackFunctionBuilder::new(
            self,
            name.to_string(),
            doc.into(),
        ))
    }

    pub fn destroy_callback(mut self, name: &str) -> Result<Self> {
        match self.destroy_name {
            None => {
                self.check_unique_name(name)?;
                self.destroy_name = Some(name.to_string());
                self.elements
                    .push(InterfaceElement::DestroyFunction(name.to_string()));
                Ok(self)
            }
            Some(_) => Err(BindingError::InterfaceDestroyCallbackAlreadyDefined {
                interface_name: self.name,
            }),
        }
    }

    pub fn arg(mut self, name: &str) -> Result<Self> {
        match self.arg_name {
            None => {
                self.check_unique_name(name)?;
                self.arg_name = Some(name.to_string());
                self.elements.push(InterfaceElement::Arg(name.to_string()));
                Ok(self)
            }
            Some(_) => Err(BindingError::InterfaceArgNameAlreadyDefined {
                interface_name: self.name,
            }),
        }
    }

    pub fn build(self) -> Result<InterfaceHandle> {
        let arg_name = self
            .arg_name
            .ok_or(BindingError::InterfaceArgNameNotDefined {
                interface_name: self.name.clone(),
            })?;

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

        self.lib.interfaces.insert(handle.clone());
        self.lib
            .statements
            .push(Statement::InterfaceDefinition(handle.clone()));

        Ok(handle)
    }

    fn check_unique_name(&mut self, name: &str) -> Result<()> {
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

#[derive(Debug)]
pub enum OneTimeCallbackElement {
    Arg(String),
    CallbackFunction(CallbackFunction),
}

#[derive(Debug)]
pub struct OneTimeCallback {
    pub name: String,
    pub elements: Vec<OneTimeCallbackElement>,
    pub arg_name: String,
    pub doc: Doc,
}

impl OneTimeCallback {
    pub fn callbacks(&self) -> impl Iterator<Item = &CallbackFunction> {
        self.elements.iter().filter_map(|el| match el {
            OneTimeCallbackElement::CallbackFunction(cb) => Some(cb),
            _ => None,
        })
    }
}

pub type OneTimeCallbackHandle = Handle<OneTimeCallback>;

pub struct OneTimeCallbackBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    elements: Vec<OneTimeCallbackElement>,
    element_names: HashSet<String>,
    arg_name: Option<String>,
    doc: Doc,
}

impl<'a> OneTimeCallbackBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String, doc: Doc) -> Self {
        Self {
            lib,
            name,
            elements: Vec::new(),
            element_names: HashSet::new(),
            arg_name: None,
            doc,
        }
    }

    pub fn callback<D: Into<Doc>>(
        mut self,
        name: &str,
        doc: D,
    ) -> Result<CallbackFunctionBuilder<Self>> {
        self.check_unique_name(name)?;
        Ok(CallbackFunctionBuilder::new(
            self,
            name.to_string(),
            doc.into(),
        ))
    }

    pub fn arg(mut self, name: &str) -> Result<Self> {
        match self.arg_name {
            None => {
                self.check_unique_name(name)?;
                self.arg_name = Some(name.to_string());
                self.elements
                    .push(OneTimeCallbackElement::Arg(name.to_string()));
                Ok(self)
            }
            Some(_) => Err(BindingError::InterfaceArgNameAlreadyDefined {
                interface_name: self.name,
            }),
        }
    }

    pub fn build(self) -> Result<OneTimeCallbackHandle> {
        let arg_name = self
            .arg_name
            .ok_or(BindingError::InterfaceArgNameNotDefined {
                interface_name: self.name.clone(),
            })?;

        let handle = OneTimeCallbackHandle::new(OneTimeCallback {
            name: self.name,
            elements: self.elements,
            arg_name,
            doc: self.doc,
        });

        self.lib.one_time_callbacks.insert(handle.clone());
        self.lib
            .statements
            .push(Statement::OneTimeCallbackDefinition(handle.clone()));

        Ok(handle)
    }

    fn check_unique_name(&mut self, name: &str) -> Result<()> {
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

pub trait CallbackFunctionBuilderTarget {
    fn lib(&mut self) -> &mut LibraryBuilder;
    fn push_callback(&mut self, cb: CallbackFunction);
}

impl<'a> CallbackFunctionBuilderTarget for InterfaceBuilder<'a> {
    fn lib(&mut self) -> &mut LibraryBuilder {
        self.lib
    }

    fn push_callback(&mut self, cb: CallbackFunction) {
        self.elements.push(InterfaceElement::CallbackFunction(cb));
    }
}

impl<'a> CallbackFunctionBuilderTarget for OneTimeCallbackBuilder<'a> {
    fn lib(&mut self) -> &mut LibraryBuilder {
        self.lib
    }

    fn push_callback(&mut self, cb: CallbackFunction) {
        self.elements
            .push(OneTimeCallbackElement::CallbackFunction(cb));
    }
}

pub struct CallbackFunctionBuilder<T: CallbackFunctionBuilderTarget> {
    target: T,
    name: String,
    return_type: Option<ReturnType>,
    params: Vec<CallbackParameter>,
    arg_name: Option<String>,
    doc: Doc,
}

impl<T: CallbackFunctionBuilderTarget> CallbackFunctionBuilder<T> {
    pub(crate) fn new(target: T, name: String, doc: Doc) -> Self {
        Self {
            target,
            name,
            return_type: None,
            params: Vec::new(),
            arg_name: None,
            doc,
        }
    }

    pub fn param<D: Into<Doc>>(mut self, name: &str, param_type: Type, doc: D) -> Result<Self> {
        self.target.lib().validate_type(&param_type)?;
        self.params.push(CallbackParameter::Parameter(Parameter {
            name: name.to_string(),
            param_type,
            doc: doc.into(),
        }));
        Ok(self)
    }

    pub fn arg(mut self, name: &str) -> Result<Self> {
        match self.arg_name {
            None => {
                self.arg_name = Some(name.to_string());
                self.params.push(CallbackParameter::Arg(name.to_string()));
                Ok(self)
            }
            Some(_) => Err(BindingError::InterfaceArgNameAlreadyDefined {
                interface_name: self.name,
            }),
        }
    }

    pub fn return_type(mut self, return_type: ReturnType) -> Result<Self> {
        match self.return_type {
            None => {
                self.return_type = Some(return_type);
                Ok(self)
            }
            Some(return_type) => Err(BindingError::ReturnTypeAlreadyDefined {
                native_func_name: self.name,
                return_type,
            }),
        }
    }

    pub fn build(mut self) -> Result<T> {
        let return_type = self.return_type.ok_or(BindingError::ReturnTypeNotDefined {
            native_func_name: self.name.clone(),
        })?;
        let arg_name = self
            .arg_name
            .ok_or(BindingError::InterfaceArgNameNotDefined {
                interface_name: self.name.clone(),
            })?;

        let cb = CallbackFunction {
            name: self.name,
            return_type,
            parameters: self.params,
            arg_name,
            doc: self.doc,
        };

        self.target.push_callback(cb);
        Ok(self.target)
    }
}
