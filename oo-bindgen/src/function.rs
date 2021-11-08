use crate::collection::CollectionHandle;
use crate::doc::{Doc, DocReference, DocString, Unvalidated, Validated};
use crate::name::{IntoName, Name};
use crate::return_type::ReturnType;
use crate::structs::{
    FunctionArgStructDeclaration, FunctionArgStructField, FunctionArgStructHandle,
    FunctionReturnStructDeclaration, FunctionReturnStructField, FunctionReturnStructHandle,
    UniversalDeclarationOr, UniversalOr, UniversalStructDeclaration, UniversalStructHandle,
};
use crate::types::{Arg, DurationType, StringType, TypeValidator, ValidatedType};
use crate::*;
use std::rc::Rc;

/// types that can be returns from native functions
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionReturnValue {
    Basic(BasicType),
    String(StringType),
    ClassRef(ClassDeclarationHandle),
    Struct(UniversalOr<FunctionReturnStructField>),
    StructRef(UniversalDeclarationOr<FunctionReturnStructField>),
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

impl From<FunctionReturnStructDeclaration> for FunctionReturnValue {
    fn from(x: FunctionReturnStructDeclaration) -> Self {
        FunctionReturnValue::StructRef(UniversalDeclarationOr::Specific(x))
    }
}

impl From<UniversalStructDeclaration> for FunctionReturnValue {
    fn from(x: UniversalStructDeclaration) -> Self {
        FunctionReturnValue::StructRef(UniversalDeclarationOr::Universal(x))
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

impl From<CollectionClassDeclaration> for FunctionReturnValue {
    fn from(x: CollectionClassDeclaration) -> Self {
        Self::ClassRef(x.inner)
    }
}

pub type FunctionReturnType<D> = ReturnType<FunctionReturnValue, D>;

/// Types that can be used as native function arguments
#[derive(Debug, Clone)]
pub enum FunctionArgument {
    Basic(BasicType),
    String(StringType),
    Collection(CollectionHandle),
    Struct(UniversalOr<FunctionArgStructField>),
    StructRef(FunctionArgStructDeclaration),
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
            FunctionArgument::StructRef(x) => x.inner.get_validated_type(),
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

impl From<FunctionArgStructDeclaration> for FunctionArgument {
    fn from(x: FunctionArgStructDeclaration) -> Self {
        FunctionArgument::StructRef(x)
    }
}

impl From<UniversalStructDeclaration> for FunctionArgument {
    fn from(x: UniversalStructDeclaration) -> Self {
        FunctionArgument::StructRef(FunctionArgStructDeclaration::new(x.inner))
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

impl From<IteratorClassDeclaration> for FunctionArgument {
    fn from(x: IteratorClassDeclaration) -> Self {
        Self::ClassRef(x.inner)
    }
}

impl From<CollectionClassDeclaration> for FunctionArgument {
    fn from(x: CollectionClassDeclaration) -> Self {
        Self::ClassRef(x.inner)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionCategory {
    Native,
    CollectionCreate,
    CollectionDestroy,
    CollectionAdd,
    IteratorNext,
}

/// C function
#[derive(Debug)]
pub struct Function<T>
where
    T: DocReference,
{
    pub name: Name,
    pub category: FunctionCategory,
    pub return_type: ReturnType<FunctionReturnValue, T>,
    pub parameters: Vec<Arg<FunctionArgument, T>>,
    pub error_type: Option<ErrorType<T>>,
    pub settings: Rc<LibrarySettings>,
    pub doc: Doc<T>,
}

impl Function<Unvalidated> {
    pub(crate) fn validate(
        &self,
        lib: &UnvalidatedFields,
    ) -> BindResult<Handle<Function<Validated>>> {
        let parameters: BindResult<Vec<Arg<FunctionArgument, Validated>>> =
            self.parameters.iter().map(|x| x.validate(lib)).collect();
        let error_type = match &self.error_type {
            Some(x) => Some(x.validate(lib)?),
            None => None,
        };

        let arguments: Vec<Name> = self.parameters.iter().map(|x| x.name.clone()).collect();

        Ok(Handle::new(Function {
            name: self.name.clone(),
            category: self.category,
            return_type: self.return_type.validate(&self.name, lib)?,
            parameters: parameters?,
            error_type,
            settings: self.settings.clone(),
            doc: self
                .doc
                .validate_with_args(&self.name, lib, Some(&arguments))?,
        }))
    }
}

#[derive(Debug, Clone)]
pub enum SignatureType {
    /// function that cannot fail and returns nothing
    NoErrorNoReturn,
    /// function that cannot fail and returns something
    NoErrorWithReturn(FunctionReturnValue, DocString<Validated>),
    /// function that can fail, but does not return a value
    ErrorNoReturn(ErrorType<Validated>),
    /// function that can fail and returns something via an out parameter
    ErrorWithReturn(
        ErrorType<Validated>,
        FunctionReturnValue,
        DocString<Validated>,
    ),
}

impl Function<Validated> {
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

pub type FunctionHandle = Handle<Function<Unvalidated>>;

pub struct FunctionBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    function_type: FunctionCategory,
    return_type: Option<ReturnType<FunctionReturnValue, Unvalidated>>,
    params: Vec<Arg<FunctionArgument, Unvalidated>>,
    doc: Option<Doc<Unvalidated>>,
    error_type: Option<ErrorType<Unvalidated>>,
}

impl<'a> FunctionBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        name: Name,
        function_type: FunctionCategory,
    ) -> Self {
        Self {
            lib,
            name,
            function_type,
            return_type: None,
            params: Vec::new(),
            doc: None,
            error_type: None,
        }
    }

    pub fn param<T: IntoName, D: Into<DocString<Unvalidated>>, P: Into<FunctionArgument>>(
        mut self,
        name: T,
        param_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let param_type = param_type.into();
        let name = name.into_name()?;

        self.lib.validate_type(&param_type)?;
        self.params.push(Arg {
            name,
            arg_type: param_type,
            doc: doc.into(),
        });
        Ok(self)
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        self.return_type(ReturnType::Void)
    }

    pub fn returns<D: Into<DocString<Unvalidated>>, T: Into<FunctionReturnValue>>(
        self,
        return_type: T,
        doc: D,
    ) -> BindResult<Self> {
        self.return_type(ReturnType::new(return_type, doc))
    }

    fn return_type(mut self, return_type: FunctionReturnType<Unvalidated>) -> BindResult<Self> {
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

    pub fn fails_with(mut self, err: ErrorType<Unvalidated>) -> BindResult<Self> {
        if let Some(x) = self.error_type {
            return Err(BindingError::ErrorTypeAlreadyDefined {
                function: self.name,
                error_type: x.inner.name.clone(),
            });
        }

        self.error_type = Some(err);
        Ok(self)
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
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
            category: self.function_type,
            return_type,
            parameters: self.params,
            error_type: self.error_type,
            settings: self.lib.settings.clone(),
            doc,
        });

        self.lib
            .add_statement(Statement::FunctionDefinition(handle.clone()))?;

        Ok(handle)
    }
}

pub struct ClassMethodBuilder<'a> {
    method_name: Name,
    class: ClassDeclarationHandle,
    inner: FunctionBuilder<'a>,
}

impl<'a> ClassMethodBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        method_name: Name,
        class: ClassDeclarationHandle,
    ) -> BindResult<Self> {
        if method_name.contains(class.name.as_ref()) {
            return Err(BindingError::BadMethodName { class, method_name });
        }

        let instance_arg_name = lib.settings.class.method_instance_argument_name.clone();

        let builder = lib
            .define_function(class.name.append(&method_name))?
            .param(
                instance_arg_name,
                class.clone(),
                format!("Instance of {{class:{}}}", class.name),
            )?;

        Ok(Self {
            method_name,
            class,
            inner: builder,
        })
    }

    pub fn param<T: IntoName, D: Into<DocString<Unvalidated>>, P: Into<FunctionArgument>>(
        self,
        name: T,
        param_type: P,
        doc: D,
    ) -> BindResult<Self> {
        Ok(Self {
            method_name: self.method_name,
            class: self.class,
            inner: self.inner.param(name, param_type, doc)?,
        })
    }

    pub fn returns_nothing(self) -> BindResult<Self> {
        Ok(Self {
            method_name: self.method_name,
            class: self.class,
            inner: self.inner.return_type(ReturnType::Void)?,
        })
    }

    pub fn returns<D: Into<DocString<Unvalidated>>, T: Into<FunctionReturnValue>>(
        self,
        return_type: T,
        doc: D,
    ) -> BindResult<Self> {
        Ok(Self {
            method_name: self.method_name,
            class: self.class,
            inner: self.inner.return_type(ReturnType::new(return_type, doc))?,
        })
    }

    pub fn fails_with(self, err: ErrorType<Unvalidated>) -> BindResult<Self> {
        Ok(Self {
            method_name: self.method_name,
            class: self.class,
            inner: self.inner.fails_with(err)?,
        })
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(self, doc: D) -> BindResult<Self> {
        Ok(Self {
            method_name: self.method_name,
            class: self.class,
            inner: self.inner.doc(doc)?,
        })
    }

    pub fn build(self) -> BindResult<ClassMethod<Unvalidated>> {
        let function = self.inner.build()?;
        Ok(ClassMethod::new(self.method_name, self.class, function))
    }
}
