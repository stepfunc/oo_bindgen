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
}

impl CallbackFunction {
    pub fn params(&self) -> impl Iterator<Item = &Parameter> {
        self.parameters.iter().filter_map(|param| {
            match param {
                CallbackParameter::Parameter(param) => Some(param),
                _ => None,
            }
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
}

impl Interface {
    pub fn callbacks(&self) -> impl Iterator<Item = &CallbackFunction> {
        self.elements.iter().filter_map(|el| {
            match el {
                InterfaceElement::CallbackFunction(cb) => Some(cb),
                _ => None,
            }
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
}

impl<'a> InterfaceBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String) -> Self {
        Self {
            lib,
            name,
            elements: Vec::new(),
            element_names: HashSet::new(),
            arg_name: None,
            destroy_name: None,
        }
    }

    pub fn callback(mut self, name: &str) -> Result<CallbackFunctionBuilder<'a>> {
        self.check_unique_name(name)?;
        Ok(CallbackFunctionBuilder::new(self, name.to_string()))
    }

    pub fn destroy_callback(mut self, name: &str) -> Result<Self> {
        match self.destroy_name {
            None => {
                self.check_unique_name(name)?;
                self.destroy_name = Some(name.to_string());
                self.elements.push(InterfaceElement::DestroyFunction(name.to_string()));
                Ok(self)
            }
            Some(_) => Err(BindingError::InterfaceDestroyCallbackAlreadyDefined{
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
            Some(_) => Err(BindingError::InterfaceArgNameAlreadyDefined{
                interface_name: self.name,
            }),
        }
    }

    pub fn build(self) -> Result<InterfaceHandle> {
        let arg_name = self.arg_name.ok_or(BindingError::InterfaceArgNameNotDefined{interface_name: self.name.clone()})?;
        let destroy_name = self.destroy_name.ok_or(BindingError::InterfaceDestroyCallbackNotDefined{interface_name: self.name.clone()})?;
        let handle = InterfaceHandle::new(Interface {
            name: self.name,
            elements: self.elements,
            arg_name,
            destroy_name,
        });

        self.lib.interfaces.insert(handle.clone());
        self.lib.statements.push(Statement::InterfaceDefinition(handle.clone()));

        Ok(handle)
    }

    fn check_unique_name(&mut self, name: &str) -> Result<()> {
        if self.element_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::InterfaceHasElementWithSameName{
                interface_name: self.name.clone(),
                element_name: name.to_string(),
            })
        }
    }
}

pub struct CallbackFunctionBuilder<'a> {
    interface: InterfaceBuilder<'a>,
    name: String,
    return_type: Option<ReturnType>,
    params: Vec<CallbackParameter>,
    arg_name: Option<String>,
}

impl<'a> CallbackFunctionBuilder<'a> {
    pub(crate) fn new(interface: InterfaceBuilder<'a>, name: String) -> Self {
        Self {
            interface,
            name,
            return_type: None,
            params: Vec::new(),
            arg_name: None,
        }
    }

    pub fn param(mut self, name: &str, param_type: Type) -> Result<Self> {
        self.interface.lib.validate_type(&param_type)?;
        self.params.push(CallbackParameter::Parameter(Parameter {
            name: name.to_string(),
            param_type
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
            Some(_) => Err(BindingError::InterfaceArgNameAlreadyDefined{
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
            Some(return_type) => Err(BindingError::ReturnTypeAlreadyDefined{
                native_func_name: self.name,
                return_type,
            }),
        }
    }

    pub fn build(mut self) -> Result<InterfaceBuilder<'a>> {
        let return_type = self.return_type.ok_or(BindingError::ReturnTypeNotDefined{native_func_name: self.name.clone()})?;
        let arg_name = self.arg_name.ok_or(BindingError::InterfaceArgNameNotDefined{interface_name: self.name.clone()})?;
        
        let cb = CallbackFunction {
            name: self.name,
            return_type,
            parameters: self.params,
            arg_name,
        };

        self.interface.elements.push(InterfaceElement::CallbackFunction(cb));
        Ok(self.interface)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
}
