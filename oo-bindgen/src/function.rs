use crate::collection::CollectionHandle;
use crate::doc::{Doc, DocString};
use crate::return_type::ReturnType;
use crate::structs::{
    FunctionArgStructField, FunctionArgStructHandle, FunctionReturnStructField,
    FunctionReturnStructHandle, UniversalStructHandle,
};
use crate::types::{Arg, DurationType, StringType, TypeValidator, ValidatedType};
use crate::*;

/// types that can be returns from native functions
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionReturnValue {
    Basic(BasicType),
    String(StringType),
    ClassRef(ClassDeclarationHandle),
    Struct(UniversalOr<FunctionReturnStructField>),
    StructRef(StructDeclarationHandle),
}

impl From<BasicType> for FunctionReturnValue {
    fn from(x: BasicType) -> Self {
        FunctionReturnValue::Basic(x)
    }
}

impl From<DurationType> for FunctionReturnValue {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

impl From<ClassDeclarationHandle> for FunctionReturnValue {
    fn from(x: ClassDeclarationHandle) -> Self {
        FunctionReturnValue::ClassRef(x)
    }
}

impl From<StringType> for FunctionReturnValue {
    fn from(_: StringType) -> Self {
        FunctionReturnValue::String(StringType)
    }
}

impl From<FunctionReturnStructHandle> for FunctionReturnValue {
    fn from(x: FunctionReturnStructHandle) -> Self {
        FunctionReturnValue::Struct(x.into())
    }
}

impl From<StructDeclarationHandle> for FunctionReturnValue {
    fn from(x: StructDeclarationHandle) -> Self {
        FunctionReturnValue::StructRef(x)
    }
}

impl From<EnumHandle> for FunctionReturnValue {
    fn from(x: EnumHandle) -> Self {
        BasicType::Enum(x).into()
    }
}

impl From<UniversalStructHandle> for FunctionReturnValue {
    fn from(x: UniversalStructHandle) -> Self {
        Self::Struct(UniversalOr::Universal(x))
    }
}

pub type FunctionReturnType = ReturnType<FunctionReturnValue>;

/// Types that can be used as native function arguments
#[derive(Debug, Clone)]
pub enum FunctionArgument {
    Basic(BasicType),
    String(StringType),
    Collection(CollectionHandle),
    Struct(UniversalOr<FunctionArgStructField>),
    StructRef(StructDeclarationHandle),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
}

impl TypeValidator for FunctionArgument {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            FunctionArgument::Basic(x) => x.get_validated_type(),
            FunctionArgument::String(_) => None,
            FunctionArgument::Collection(x) => x.get_validated_type(),
            FunctionArgument::Struct(x) => x.get_validated_type(),
            FunctionArgument::StructRef(x) => x.get_validated_type(),
            FunctionArgument::ClassRef(x) => x.get_validated_type(),
            FunctionArgument::Interface(x) => x.get_validated_type(),
        }
    }
}

impl From<UniversalStructHandle> for FunctionArgument {
    fn from(x: UniversalStructHandle) -> Self {
        Self::Struct(UniversalOr::Universal(x))
    }
}

impl From<FunctionArgStructHandle> for FunctionArgument {
    fn from(x: FunctionArgStructHandle) -> Self {
        Self::Struct(x.into())
    }
}

impl From<ClassDeclarationHandle> for FunctionArgument {
    fn from(x: ClassDeclarationHandle) -> Self {
        FunctionArgument::ClassRef(x)
    }
}

impl From<InterfaceHandle> for FunctionArgument {
    fn from(x: InterfaceHandle) -> Self {
        FunctionArgument::Interface(x)
    }
}

impl From<BasicType> for FunctionArgument {
    fn from(x: BasicType) -> Self {
        FunctionArgument::Basic(x)
    }
}

impl From<StringType> for FunctionArgument {
    fn from(x: StringType) -> Self {
        FunctionArgument::String(x)
    }
}

impl From<CollectionHandle> for FunctionArgument {
    fn from(x: CollectionHandle) -> Self {
        FunctionArgument::Collection(x)
    }
}

impl From<StructDeclarationHandle> for FunctionArgument {
    fn from(x: StructDeclarationHandle) -> Self {
        FunctionArgument::StructRef(x)
    }
}

impl From<DurationType> for FunctionArgument {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

impl From<EnumHandle> for FunctionArgument {
    fn from(x: EnumHandle) -> Self {
        BasicType::Enum(x).into()
    }
}

/// C function
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: FunctionReturnType,
    pub parameters: Vec<Arg<FunctionArgument>>,
    pub error_type: Option<ErrorType>,
    pub doc: Doc,
}

#[derive(Debug, Clone)]
pub enum SignatureType {
    /// function that cannot fail and returns nothing
    NoErrorNoReturn,
    /// function that cannot fail and returns something
    NoErrorWithReturn(FunctionReturnValue, DocString),
    /// function that can fail, but does not return a value
    ErrorNoReturn(ErrorType),
    /// function that can fail and returns something via an out parameter
    ErrorWithReturn(ErrorType, FunctionReturnValue, DocString),
}

impl Function {
    pub fn get_signature_type(&self) -> SignatureType {
        match &self.error_type {
            Some(e) => match &self.return_type {
                ReturnType::Void => SignatureType::ErrorNoReturn(e.clone()),
                ReturnType::Type(t, d) => {
                    SignatureType::ErrorWithReturn(e.clone(), t.clone(), d.clone())
                }
            },
            None => match &self.return_type {
                ReturnType::Void => SignatureType::NoErrorNoReturn,
                ReturnType::Type(t, d) => SignatureType::NoErrorWithReturn(t.clone(), d.clone()),
            },
        }
    }
}

pub type FunctionHandle = Handle<Function>;

pub struct FunctionBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    return_type: Option<ReturnType<FunctionReturnValue>>,
    params: Vec<Arg<FunctionArgument>>,
    doc: Option<Doc>,
    error_type: Option<ErrorType>,
}

impl<'a> FunctionBuilder<'a> {
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

    pub fn param<T: Into<String>, D: Into<DocString>, P: Into<FunctionArgument>>(
        mut self,
        name: T,
        param_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let param_type = param_type.into();

        self.lib.validate_type(&param_type)?;
        self.params.push(Arg {
            name: name.into(),
            arg_type: param_type,
            doc: doc.into(),
        });
        Ok(self)
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        self.return_type(ReturnType::Void)
    }

    pub fn returns<D: Into<DocString>, T: Into<FunctionReturnValue>>(
        self,
        return_type: T,
        doc: D,
    ) -> BindResult<Self> {
        self.return_type(ReturnType::new(return_type, doc))
    }

    fn return_type(mut self, return_type: FunctionReturnType) -> BindResult<Self> {
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

    pub fn fails_with(mut self, err: ErrorType) -> BindResult<Self> {
        if let Some(x) = self.error_type {
            return Err(BindingError::ErrorTypeAlreadyDefined {
                function: self.name,
                error_type: x.inner.name.clone(),
            });
        }

        self.error_type = Some(err);
        Ok(self)
    }

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> BindResult<Self> {
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

    pub fn build(self) -> BindResult<FunctionHandle> {
        let return_type = match self.return_type {
            Some(return_type) => return_type,
            None => {
                return Err(BindingError::ReturnTypeNotDefined {
                    func_name: self.name,
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

        let handle = FunctionHandle::new(Function {
            name: self.name,
            return_type,
            parameters: self.params,
            error_type: self.error_type,
            doc,
        });

        self.lib
            .add_statement(Statement::FunctionDefinition(handle.clone()))?;

        Ok(handle)
    }
}
