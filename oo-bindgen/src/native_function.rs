use crate::collection::CollectionHandle;
use crate::doc::{Doc, DocString};
use crate::iterator::IteratorHandle;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
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
    OneTimeCallback(OneTimeCallbackHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),

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
    Type(Type, DocString),
}

impl ReturnType {
    pub fn void() -> Self {
        ReturnType::Void
    }

    pub fn new<D: Into<DocString>>(return_type: Type, doc: D) -> Self {
        ReturnType::Type(return_type, doc.into())
    }

    pub fn is_void(&self) -> bool {
        if let Self::Void = self {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub doc: DocString,
}

/// C function
#[derive(Debug)]
pub struct NativeFunction {
    pub name: String,
    pub return_type: ReturnType,
    pub parameters: Vec<Parameter>,
    pub error_type: Option<ErrorType>,
    pub doc: Doc,
}

pub type NativeFunctionHandle = Handle<NativeFunction>;

pub struct NativeFunctionBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    return_type: Option<ReturnType>,
    params: Vec<Parameter>,
    doc: Option<Doc>,
    error_type: Option<ErrorType>,
}

impl<'a> NativeFunctionBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String) -> Self {
        Self {
            lib,
            name,
            return_type: None,
            params: Vec::new(),
            doc: None,
            error_type: None,
        }
    }

    pub fn param<T: Into<String>, D: Into<DocString>>(
        mut self,
        name: T,
        param_type: Type,
        doc: D,
    ) -> Result<Self> {
        self.lib.validate_type(&param_type)?;
        self.params.push(Parameter {
            name: name.into(),
            param_type,
            doc: doc.into(),
        });
        Ok(self)
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

    pub fn fails_with(mut self, err: ErrorType) -> Result<Self> {
        if let Some(x) = self.error_type {
            return Err(BindingError::ErrorTypeAlreadyDefined {
                function: self.name,
                error_type: x.inner.name.clone(),
            });
        }

        self.error_type = Some(err);
        Ok(self)
    }

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> Result<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingError::DocAlreadyDefined {
                symbol_name: self.name,
            }),
        }
    }

    pub fn build(self) -> Result<NativeFunctionHandle> {
        let return_type = match self.return_type {
            Some(return_type) => return_type,
            None => {
                return Err(BindingError::ReturnTypeNotDefined {
                    native_func_name: self.name,
                })
            }
        };

        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.name,
                })
            }
        };

        let handle = NativeFunctionHandle::new(NativeFunction {
            name: self.name,
            return_type,
            parameters: self.params,
            error_type: self.error_type,
            doc,
        });

        self.lib.native_functions.insert(handle.clone());
        self.lib
            .statements
            .push(Statement::NativeFunctionDeclaration(handle.clone()));

        Ok(handle)
    }
}
