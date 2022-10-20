use std::rc::Rc;

use crate::model::*;

/// Used for iterator "next" functions to get an optional primitive
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PrimitiveRef {
    pub(crate) inner: Primitive,
}

impl PrimitiveRef {
    pub fn new(inner: Primitive) -> Self {
        Self { inner }
    }
}

/// types that can be returns from native functions
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionReturnValue {
    Basic(BasicType),
    PrimitiveRef(PrimitiveRef),
    String(StringType),
    ClassRef(ClassDeclarationHandle),
    Struct(UniversalOr<FunctionReturnStructField>),
    StructRef(UniversalDeclarationOr<FunctionReturnStructField>),
}

impl From<PrimitiveRef> for FunctionReturnValue {
    fn from(x: PrimitiveRef) -> Self {
        FunctionReturnValue::PrimitiveRef(x)
    }
}

impl From<Primitive> for FunctionReturnValue {
    fn from(x: Primitive) -> Self {
        FunctionReturnValue::Basic(x.into())
    }
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

impl From<Handle<Enum<Unvalidated>>> for FunctionReturnValue {
    fn from(x: Handle<Enum<Unvalidated>>) -> Self {
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

impl From<SynchronousInterface> for FunctionArgument {
    fn from(x: SynchronousInterface) -> Self {
        FunctionArgument::Interface(x.inner)
    }
}

impl From<AsynchronousInterface> for FunctionArgument {
    fn from(x: AsynchronousInterface) -> Self {
        FunctionArgument::Interface(x.inner)
    }
}

impl From<Primitive> for FunctionArgument {
    fn from(x: Primitive) -> Self {
        FunctionArgument::Basic(BasicType::Primitive(x))
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

impl From<BasicType> for FunctionArgument {
    fn from(x: BasicType) -> Self {
        FunctionArgument::Basic(x)
    }
}

impl From<DurationType> for FunctionArgument {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

impl From<Handle<Enum<Unvalidated>>> for FunctionArgument {
    fn from(x: Handle<Enum<Unvalidated>>) -> Self {
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
    pub(crate) name: Name,
    pub(crate) category: FunctionCategory,
    pub(crate) return_type: OptionalReturnType<FunctionReturnValue, T>,
    pub(crate) arguments: Vec<Arg<FunctionArgument, T>>,
    pub(crate) error_type: OptionalErrorType<T>,
    pub(crate) settings: Rc<LibrarySettings>,
    pub(crate) doc: Doc<T>,
}

impl Function<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Handle<Function<Validated>>> {
        let parameters: BindResult<Vec<Arg<FunctionArgument, Validated>>> =
            self.arguments.iter().map(|x| x.validate(lib)).collect();

        let arguments: Vec<Name> = self.arguments.iter().map(|x| x.name.clone()).collect();

        Ok(Handle::new(Function {
            name: self.name.clone(),
            category: self.category,
            return_type: self.return_type.validate(&self.name, lib)?,
            arguments: parameters?,
            error_type: self.error_type.validate(lib)?,
            settings: self.settings.clone(),
            doc: self
                .doc
                .validate_with_args(&self.name, lib, Some(&arguments))?,
        }))
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub(crate) enum SignatureType {
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
    pub(crate) fn get_signature_type(&self) -> SignatureType {
        match self.error_type.get() {
            Some(e) => match self.return_type.get() {
                None => SignatureType::ErrorNoReturn(e.clone()),
                Some(rt) => {
                    SignatureType::ErrorWithReturn(e.clone(), rt.value.clone(), rt.doc.clone())
                }
            },
            None => match self.return_type.get() {
                None => SignatureType::NoErrorNoReturn,
                Some(rt) => SignatureType::NoErrorWithReturn(rt.value.clone(), rt.doc.clone()),
            },
        }
    }
}

pub type FunctionHandle = Handle<Function<Unvalidated>>;

/// represents a method that initiates an asynchronous operation
/// an eventually completes an abstract future
#[derive(Debug, Clone)]
pub struct FutureMethod<T>
where
    T: DocReference,
{
    pub(crate) name: Name,
    pub(crate) associated_class: Handle<ClassDeclaration>,
    pub(crate) future: FutureInterface<T>,
    pub(crate) native_function: Handle<Function<T>>,
}

impl FutureMethod<Validated> {
    pub fn arguments(&self) -> impl Iterator<Item = &Arg<FunctionArgument, Validated>> {
        self.native_function.arguments.iter().skip(1)
    }

    pub fn arguments_without_callback(
        &self,
    ) -> impl Iterator<Item = &Arg<FunctionArgument, Validated>> {
        self.arguments().filter(|param| match &param.arg_type {
            FunctionArgument::Interface(handle) => handle.name != self.future.interface.name,
            _ => true,
        })
    }
}

impl FutureMethod<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<FutureMethod<Validated>> {
        Ok(FutureMethod {
            name: self.name.clone(),
            associated_class: self.associated_class.clone(),
            future: self.future.validate(lib)?,
            native_function: self.native_function.validate(lib)?,
        })
    }
}

pub type FutureMethodHandle = FutureMethod<Unvalidated>;

#[derive(Debug, Clone)]
pub struct ClassDestructor<T>
where
    T: DocReference,
{
    pub(crate) class: ClassDeclarationHandle,
    pub(crate) function: Handle<Function<T>>,
}

impl ClassDestructor<Unvalidated> {
    pub(crate) fn new(
        lib: &mut LibraryBuilder,
        class: ClassDeclarationHandle,
        doc: Doc<Unvalidated>,
    ) -> BindResult<Self> {
        let destructor_function_name = class
            .name
            .append(&lib.settings.class.class_destructor_suffix);
        let instance_name = lib.settings.class.method_instance_argument_name.clone();

        let function = lib
            .define_function(destructor_function_name)?
            .param(
                instance_name,
                class.clone(),
                format!("Instance of {{class:{}}} to destroy", class.name),
            )?
            .doc(doc)?
            .build()?;

        Ok(Self { class, function })
    }

    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<ClassDestructor<Validated>> {
        Ok(ClassDestructor {
            class: self.class.clone(),
            function: self.function.validate(lib)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClassConstructor<T>
where
    T: DocReference,
{
    pub(crate) class: ClassDeclarationHandle,
    pub(crate) function: Handle<Function<T>>,
}

impl ClassConstructor<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<ClassConstructor<Validated>> {
        Ok(ClassConstructor {
            class: self.class.clone(),
            function: self.function.validate(lib)?,
        })
    }

    pub(crate) fn new(
        class: ClassDeclarationHandle,
        function: Handle<Function<Unvalidated>>,
    ) -> Self {
        Self { class, function }
    }
}
