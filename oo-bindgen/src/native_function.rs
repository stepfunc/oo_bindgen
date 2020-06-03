use crate::*;

#[derive(Debug, Clone)]
pub enum Type {
    Bool,
    Uint8,
    Sint8,
    Uint16,
    Sint16,
    Uint32,
    Sint32,
    Uint64,
    Sint64,
    Float,
    Double,
    String,

    // Complex types
    Struct(NativeStructHandle),
    StructRef(NativeStructDeclarationHandle),
    Enum(NativeEnumHandle),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),

    // Not native types
    Duration(DurationMapping),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DurationMapping {
    // Duration is the number of milliseconds in a u64 value
    Milliseconds,
    // Duration is the number of seconds in a u64 value
    Seconds,
    // Duration is the number of seconds and fractional part in a f32 value
    SecondsFloat,
}

#[derive(Debug)]
pub enum ReturnType {
    Void,
    Type(Type),
}

impl ReturnType {
    pub fn is_void(&self) -> bool {
        if let Self::Void = self {
            return true
        }
        false
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

/// C function
#[derive(Debug)]
pub struct NativeFunction {
    pub name: String,
    pub return_type: ReturnType,
    pub parameters: Vec<Parameter>,
}

pub type NativeFunctionHandle = Handle<NativeFunction>;

pub struct NativeFunctionBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    return_type: Option<ReturnType>,
    params: Vec<Parameter>,
}

impl<'a> NativeFunctionBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String) -> Self {
        Self {
            lib,
            name,
            return_type: None,
            params: Vec::new(),
        }
    }

    pub fn param(mut self, name: &str, param_type: Type) -> Result<Self> {
        self.lib.validate_type(&param_type)?;
        self.params.push(Parameter {
            name: name.to_string(),
            param_type
        });
        Ok(self)
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

    pub fn build(self) -> Result<NativeFunctionHandle> {
        if let Some(return_type) = self.return_type {
            let handle = NativeFunctionHandle::new(NativeFunction {
                name: self.name,
                return_type,
                parameters: self.params,
            });

            self.lib.native_functions.insert(handle.clone());
            self.lib.statements.push(Statement::NativeFunctionDeclaration(handle.clone()));

            Ok(handle)
        } else {
            Err(BindingError::ReturnTypeNotDefined{native_func_name: self.name})
        }
    }
}
