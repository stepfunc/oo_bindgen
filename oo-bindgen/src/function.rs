use crate::collection::CollectionHandle;
use crate::doc::{Doc, DocString};
use crate::structs::return_struct::RStructHandle;
use crate::types::{Arg, DurationType, StringType};
use crate::*;

/// types that can be returns from native functions
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Basic(BasicType),
    String,
    ClassRef(ClassDeclarationHandle),
    Struct(RStructHandle),
    StructRef(StructDeclarationHandle),
}

impl From<ReturnType> for AnyType {
    fn from(x: ReturnType) -> Self {
        match x {
            ReturnType::Basic(x) => AnyType::Basic(x),
            ReturnType::String => AnyType::String,
            ReturnType::ClassRef(x) => AnyType::ClassRef(x),
            ReturnType::Struct(x) => AnyType::Struct(x.to_any_struct()),
            ReturnType::StructRef(x) => AnyType::StructRef(x),
        }
    }
}

impl From<BasicType> for ReturnType {
    fn from(x: BasicType) -> Self {
        ReturnType::Basic(x)
    }
}

impl From<DurationType> for ReturnType {
    fn from(x: DurationType) -> Self {
        ReturnType::Basic(BasicType::Duration(x))
    }
}

impl From<ClassDeclarationHandle> for ReturnType {
    fn from(x: ClassDeclarationHandle) -> Self {
        ReturnType::ClassRef(x)
    }
}

impl From<StringType> for ReturnType {
    fn from(_: StringType) -> Self {
        ReturnType::String
    }
}

impl From<EnumHandle> for ReturnType {
    fn from(x: EnumHandle) -> Self {
        ReturnType::Basic(BasicType::Enum(x))
    }
}

impl From<RStructHandle> for ReturnType {
    fn from(x: RStructHandle) -> Self {
        ReturnType::Struct(x)
    }
}

impl From<StructDeclarationHandle> for ReturnType {
    fn from(x: StructDeclarationHandle) -> Self {
        ReturnType::StructRef(x)
    }
}

#[derive(Debug)]
pub enum ReturnTypeInfo {
    Void,
    Type(ReturnType, DocString),
}

impl ReturnTypeInfo {
    pub fn void() -> Self {
        ReturnTypeInfo::Void
    }

    pub fn new<D: Into<DocString>, T: Into<ReturnType>>(return_type: T, doc: D) -> Self {
        ReturnTypeInfo::Type(return_type.into(), doc.into())
    }

    pub fn is_void(&self) -> bool {
        if let Self::Void = self {
            return true;
        }
        false
    }
}

/// Types that can be used as native function arguments
#[derive(Debug, Clone, PartialEq)]
pub enum FArgument {
    Basic(BasicType),
    String,
    Collection(CollectionHandle),
    Struct(FStructHandle),
    StructRef(StructDeclarationHandle),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
}

impl FArgument {
    pub fn to_any_type(&self) -> AnyType {
        match self {
            Self::Basic(x) => AnyType::Basic(x.clone()),
            Self::String => AnyType::String,
            Self::Collection(x) => AnyType::Collection(x.clone()),
            Self::Struct(x) => AnyType::Struct(x.to_any_struct()),
            Self::StructRef(x) => AnyType::StructRef(x.clone()),
            Self::ClassRef(x) => AnyType::ClassRef(x.clone()),
            Self::Interface(x) => AnyType::Interface(x.clone()),
        }
    }
}

impl From<ClassDeclarationHandle> for FArgument {
    fn from(x: ClassDeclarationHandle) -> Self {
        FArgument::ClassRef(x)
    }
}

impl From<InterfaceHandle> for FArgument {
    fn from(x: InterfaceHandle) -> Self {
        FArgument::Interface(x)
    }
}

impl From<BasicType> for FArgument {
    fn from(x: BasicType) -> Self {
        FArgument::Basic(x)
    }
}

impl From<DurationType> for FArgument {
    fn from(x: DurationType) -> Self {
        FArgument::Basic(x.into())
    }
}

impl From<StringType> for FArgument {
    fn from(_: StringType) -> Self {
        FArgument::String
    }
}

impl From<CollectionHandle> for FArgument {
    fn from(x: CollectionHandle) -> Self {
        FArgument::Collection(x)
    }
}

impl From<EnumHandle> for FArgument {
    fn from(x: EnumHandle) -> Self {
        FArgument::Basic(BasicType::Enum(x))
    }
}

impl From<StructDeclarationHandle> for FArgument {
    fn from(x: StructDeclarationHandle) -> Self {
        FArgument::StructRef(x)
    }
}

impl From<FArgument> for AnyType {
    fn from(x: FArgument) -> Self {
        x.to_any_type()
    }
}

/// C function
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: ReturnTypeInfo,
    pub parameters: Vec<Arg<FArgument>>,
    pub error_type: Option<ErrorType>,
    pub doc: Doc,
}

#[derive(Debug, Clone)]
pub enum SignatureType {
    /// function that cannot fail and returns nothing
    NoErrorNoReturn,
    /// function that cannot fail and returns something
    NoErrorWithReturn(ReturnType, DocString),
    /// function that can fail, but does not return a value
    ErrorNoReturn(ErrorType),
    /// function that can fail and returns something via an out parameter
    ErrorWithReturn(ErrorType, ReturnType, DocString),
}

impl Function {
    pub fn get_signature_type(&self) -> SignatureType {
        match &self.error_type {
            Some(e) => match &self.return_type {
                ReturnTypeInfo::Void => SignatureType::ErrorNoReturn(e.clone()),
                ReturnTypeInfo::Type(t, d) => {
                    SignatureType::ErrorWithReturn(e.clone(), t.clone(), d.clone())
                }
            },
            None => match &self.return_type {
                ReturnTypeInfo::Void => SignatureType::NoErrorNoReturn,
                ReturnTypeInfo::Type(t, d) => {
                    SignatureType::NoErrorWithReturn(t.clone(), d.clone())
                }
            },
        }
    }
}

pub type FunctionHandle = Handle<Function>;

pub struct FunctionBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    return_type: Option<ReturnTypeInfo>,
    params: Vec<Arg<FArgument>>,
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

    pub fn param<T: Into<String>, D: Into<DocString>, P: Into<FArgument>>(
        mut self,
        name: T,
        param_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let param_type = param_type.into();

        self.lib.validate_type(&param_type.clone().into())?; // TODO
        self.params.push(Arg {
            name: name.into(),
            arg_type: param_type,
            doc: doc.into(),
        });
        Ok(self)
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        self.return_type(ReturnTypeInfo::Void)
    }

    pub fn returns<D: Into<DocString>, T: Into<ReturnType>>(
        self,
        return_type: T,
        doc: D,
    ) -> BindResult<Self> {
        self.return_type(ReturnTypeInfo::new(return_type, doc))
    }

    fn return_type(mut self, return_type: ReturnTypeInfo) -> BindResult<Self> {
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
