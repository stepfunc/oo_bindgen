use crate::doc::{DocReference, DocString, Unvalidated, Validated};
use crate::name::{IntoName, Name};
use crate::*;
use std::rc::Rc;

/// Different types of classes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClassType {
    /// A normal user class which will have a constructor, destructor, methods, etc
    Normal,
    /// An iterator class
    Iterator,
    /// A collection class
    Collection,
}

/// C-style structure forward declaration
#[derive(Debug)]
pub struct ClassDeclaration {
    pub name: Name,
    pub class_type: ClassType,
    pub settings: Rc<LibrarySettings>,
}

#[derive(Debug, Clone)]
pub struct IteratorClassDeclaration {
    pub inner: ClassDeclarationHandle,
}

impl IteratorClassDeclaration {
    pub(crate) fn new(inner: ClassDeclarationHandle) -> Self {
        Self { inner }
    }
}

#[derive(Debug, Clone)]
pub struct CollectionClassDeclaration {
    pub inner: ClassDeclarationHandle,
}

impl CollectionClassDeclaration {
    pub(crate) fn new(inner: ClassDeclarationHandle) -> Self {
        Self { inner }
    }
}

impl ClassDeclaration {
    pub(crate) fn new(name: Name, class_type: ClassType, settings: Rc<LibrarySettings>) -> Self {
        Self {
            name,
            class_type,
            settings,
        }
    }
}

pub type ClassDeclarationHandle = Handle<ClassDeclaration>;

#[derive(Debug)]
pub struct Method<T>
where
    T: DocReference,
{
    pub name: Name,
    pub native_function: Handle<Function<T>>,
}

impl Method<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Method<Validated>> {
        Ok(Method {
            name: self.name.clone(),
            native_function: self.native_function.validate(lib)?,
        })
    }
}

#[derive(Debug)]
pub struct AsyncMethod<T>
where
    T: DocReference,
{
    pub name: Name,
    pub native_function: Handle<Function<T>>,
    pub return_type: CallbackArgument,
    pub return_type_doc: DocString<T>,
    pub one_time_callback: Handle<Interface<T>>,
    pub one_time_callback_param_name: Name,
    pub callback_name: Name,
    pub callback_param_name: Name,
}

impl AsyncMethod<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<AsyncMethod<Validated>> {
        Ok(AsyncMethod {
            name: self.name.clone(),
            native_function: self.native_function.validate(lib)?,
            return_type: self.return_type.clone(),
            return_type_doc: self.return_type_doc.validate(&self.name, lib)?,
            one_time_callback: self.one_time_callback.validate(lib)?,
            one_time_callback_param_name: self.one_time_callback_param_name.clone(),
            callback_name: self.callback_name.clone(),
            callback_param_name: self.callback_param_name.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DestructionMode {
    /// Object is automatically deleted by the GC
    Automatic,
    /// Object is disposed of manually by calling a custom method
    ///
    /// For safety, if the user never calls the destruction method, the object
    /// will still be deleted by the GC at some point. However, it is
    /// strongly advised to take care of the destruction manually.
    Custom(String),
    /// Object is disposed of manually by calling a dispose()/close() method
    ///
    /// For safety, if the user never calls the destruction method, the object
    /// will still be deleted by the GC at some point. However, it is
    /// strongly advised to take care of the destruction manually.
    Dispose,
}

impl DestructionMode {
    pub fn is_manual_destruction(&self) -> bool {
        match self {
            Self::Automatic => false,
            Self::Custom(_) => true,
            Self::Dispose => true,
        }
    }
}

/// Object-oriented class definition
#[derive(Debug)]
pub struct Class<T>
where
    T: DocReference,
{
    pub declaration: ClassDeclarationHandle,
    pub constructor: Option<Handle<Function<T>>>,
    pub destructor: Option<Handle<Function<T>>>,
    pub methods: Vec<Method<T>>,
    pub static_methods: Vec<Method<T>>,
    pub async_methods: Vec<AsyncMethod<T>>,
    pub doc: Doc<T>,
    pub destruction_mode: DestructionMode,
    pub settings: Rc<LibrarySettings>,
}

impl Class<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Handle<Class<Validated>>> {
        let constructor = match &self.constructor {
            None => None,
            Some(x) => Some(x.validate(lib)?),
        };
        let destructor = match &self.destructor {
            None => None,
            Some(x) => Some(x.validate(lib)?),
        };
        let methods: BindResult<Vec<Method<Validated>>> =
            self.methods.iter().map(|x| x.validate(lib)).collect();
        let static_methods: BindResult<Vec<Method<Validated>>> = self
            .static_methods
            .iter()
            .map(|x| x.validate(lib))
            .collect();
        let async_methods: BindResult<Vec<AsyncMethod<Validated>>> =
            self.async_methods.iter().map(|x| x.validate(lib)).collect();

        Ok(Handle::new(Class {
            declaration: self.declaration.clone(),
            constructor,
            destructor,
            methods: methods?,
            static_methods: static_methods?,
            async_methods: async_methods?,
            doc: self.doc.validate(self.name(), lib)?,
            destruction_mode: self.destruction_mode.clone(),
            settings: self.settings.clone(),
        }))
    }
}

impl<T> Class<T>
where
    T: DocReference,
{
    pub fn name(&self) -> &Name {
        &self.declaration.name
    }

    pub fn declaration(&self) -> ClassDeclarationHandle {
        self.declaration.clone()
    }
}

impl Class<Unvalidated> {
    pub(crate) fn find_method<S: AsRef<str>>(
        &self,
        method_name: S,
    ) -> Option<(Name, FunctionHandle)> {
        let method_name = method_name.as_ref();

        for method in &self.methods {
            if method.name.as_ref() == method_name {
                return Some((method.name.clone(), method.native_function.clone()));
            }
        }

        for method in &self.static_methods {
            if method.name.as_ref() == method_name {
                return Some((method.name.clone(), method.native_function.clone()));
            }
        }

        for method in &self.async_methods {
            if method.name.as_ref() == method_name {
                return Some((method.name.clone(), method.native_function.clone()));
            }
        }

        None
    }
}

pub type ClassHandle = Handle<Class<Unvalidated>>;

pub struct ClassBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    declaration: ClassDeclarationHandle,
    constructor: Option<Handle<Function<Unvalidated>>>,
    destructor: Option<Handle<Function<Unvalidated>>>,
    methods: Vec<Method<Unvalidated>>,
    static_methods: Vec<Method<Unvalidated>>,
    async_methods: Vec<AsyncMethod<Unvalidated>>,
    doc: Option<Doc<Unvalidated>>,
    destruction_mode: DestructionMode,
}

impl<'a> ClassBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, declaration: ClassDeclarationHandle) -> Self {
        Self {
            lib,
            declaration,
            constructor: None,
            destructor: None,
            methods: Vec::new(),
            static_methods: Vec::new(),
            async_methods: Vec::new(),
            doc: None,
            destruction_mode: DestructionMode::Automatic,
        }
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingError::DocAlreadyDefined {
                symbol_name: self.declaration.name.clone(),
            }),
        }
    }

    pub fn constructor(mut self, native_function: &FunctionHandle) -> BindResult<Self> {
        if self.constructor.is_some() {
            return Err(BindingError::ConstructorAlreadyDefined {
                handle: self.declaration,
            });
        }
        self.lib.validate_function(native_function)?;

        if let FunctionReturnType::Type(FunctionReturnValue::ClassRef(return_type), _) =
            &native_function.return_type
        {
            if return_type == &self.declaration {
                self.constructor = Some(native_function.clone());
                return Ok(self);
            }
        }

        Err(BindingError::ConstructorReturnTypeDoesNotMatch {
            handle: self.declaration,
            function: native_function.clone(),
        })
    }

    pub fn destructor(mut self, native_function: &FunctionHandle) -> BindResult<Self> {
        if self.destructor.is_some() {
            return Err(BindingError::DestructorAlreadyDefined {
                handle: self.declaration,
            });
        }
        self.lib.validate_function(native_function)?;
        self.validate_first_param(native_function)?;

        if native_function.error_type.is_some() {
            return Err(BindingError::DestructorCannotFail {
                handle: self.declaration,
            });
        }

        if native_function.parameters.len() != 1 {
            return Err(BindingError::DestructorTakesMoreThanOneParameter {
                handle: self.declaration,
                function: native_function.clone(),
            });
        }

        self.destructor = Some(native_function.clone());

        Ok(self)
    }

    pub fn method<T: IntoName>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        let name = name.into_name()?;

        self.lib.validate_function(native_function)?;
        self.validate_first_param(native_function)?;

        self.methods.push(Method {
            name,
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn static_method<T: IntoName>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        self.lib.validate_function(native_function)?;

        self.static_methods.push(Method {
            name,
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn async_method<T: IntoName>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        self.lib.validate_function(native_function)?;
        self.validate_first_param(native_function)?;

        // Check that native method has a single callback with a single method,
        // with a single argument

        let name = name.into_name()?;
        let mut async_method = None;
        for param in &native_function.parameters {
            if let FunctionArgument::Interface(ot_cb) = &param.arg_type {
                if async_method.is_some() {
                    return Err(BindingError::AsyncMethodTooManyInterface {
                        handle: native_function.clone(),
                    });
                }

                let mut cb_iter = ot_cb.callbacks.iter();
                if let Some(cb) = cb_iter.next() {
                    if !cb.return_type.is_void() {
                        return Err(BindingError::AsyncCallbackReturnTypeNotVoid {
                            handle: native_function.clone(),
                        });
                    }

                    let mut iter = cb.arguments.iter();
                    if let Some(cb_param) = iter.next() {
                        async_method = Some(AsyncMethod {
                            name: name.clone(),
                            native_function: native_function.clone(),
                            return_type: cb_param.arg_type.clone(),
                            return_type_doc: cb_param.doc.clone(),
                            one_time_callback: ot_cb.clone(),
                            one_time_callback_param_name: param.name.clone(),
                            callback_name: cb.name.clone(),
                            callback_param_name: cb_param.name.clone(),
                        });

                        if iter.next().is_some() {
                            return Err(BindingError::AsyncCallbackNotSingleParam {
                                handle: native_function.clone(),
                            });
                        }
                    } else {
                        return Err(BindingError::AsyncCallbackNotSingleParam {
                            handle: native_function.clone(),
                        });
                    }

                    if cb_iter.next().is_some() {
                        return Err(BindingError::AsyncInterfaceNotSingleCallback {
                            handle: native_function.clone(),
                        });
                    }
                } else {
                    return Err(BindingError::AsyncInterfaceNotSingleCallback {
                        handle: native_function.clone(),
                    });
                }
            }
        }

        if let Some(method) = async_method {
            self.async_methods.push(method);
        } else {
            return Err(BindingError::AsyncMethodNoInterface {
                handle: native_function.clone(),
            });
        }

        Ok(self)
    }

    pub fn custom_destroy<T: Into<String>>(mut self, name: T) -> BindResult<Self> {
        if self.destructor.is_none() {
            return Err(BindingError::NoDestructorForManualDestruction {
                handle: self.declaration,
            });
        }

        self.destruction_mode = DestructionMode::Custom(name.into());
        Ok(self)
    }

    pub fn disposable_destroy(mut self) -> BindResult<Self> {
        if self.destructor.is_none() {
            return Err(BindingError::NoDestructorForManualDestruction {
                handle: self.declaration,
            });
        }

        self.destruction_mode = DestructionMode::Dispose;
        Ok(self)
    }

    pub fn build(self) -> BindResult<ClassHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        let handle = ClassHandle::new(Class {
            declaration: self.declaration.clone(),
            constructor: self.constructor,
            destructor: self.destructor,
            methods: self.methods,
            static_methods: self.static_methods,
            async_methods: self.async_methods,
            doc,
            destruction_mode: self.destruction_mode,
            settings: self.lib.settings.clone(),
        });

        self.lib
            .add_statement(Statement::ClassDefinition(handle.clone()))?;

        Ok(handle)
    }

    fn validate_first_param(&self, native_function: &FunctionHandle) -> BindResult<()> {
        if let Some(first_param) = native_function.parameters.first() {
            if let FunctionArgument::ClassRef(first_param_type) = &first_param.arg_type {
                if first_param_type == &self.declaration {
                    return Ok(());
                }
            }
        }

        Err(BindingError::FirstMethodParameterIsNotClassType {
            handle: self.declaration.clone(),
            function: native_function.clone(),
        })
    }
}

/// Static class definition
#[derive(Debug)]
pub struct StaticClass<T>
where
    T: DocReference,
{
    pub name: Name,
    pub static_methods: Vec<Method<T>>,
    pub doc: Doc<T>,
}

impl StaticClass<Unvalidated> {
    pub(crate) fn validate(
        &self,
        lib: &UnvalidatedFields,
    ) -> BindResult<Handle<StaticClass<Validated>>> {
        let methods: BindResult<Vec<Method<Validated>>> = self
            .static_methods
            .iter()
            .map(|x| x.validate(lib))
            .collect();
        Ok(Handle::new(StaticClass {
            name: self.name.clone(),
            static_methods: methods?,
            doc: self.doc.validate(&self.name, lib)?,
        }))
    }
}

pub type StaticClassHandle = Handle<StaticClass<Unvalidated>>;

pub struct StaticClassBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    static_methods: Vec<Method<Unvalidated>>,
    doc: Option<Doc<Unvalidated>>,
}

impl<'a> StaticClassBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: Name) -> Self {
        Self {
            lib,
            name,
            static_methods: Vec::new(),
            doc: None,
        }
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

    pub fn static_method<T: IntoName>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        self.lib.validate_function(native_function)?;

        self.static_methods.push(Method {
            name,
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn build(self) -> BindResult<StaticClassHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.name.clone(),
                })
            }
        };

        let handle = StaticClassHandle::new(StaticClass {
            name: self.name,
            static_methods: self.static_methods,
            doc,
        });

        self.lib
            .add_statement(Statement::StaticClassDefinition(handle.clone()))?;

        Ok(handle)
    }
}
