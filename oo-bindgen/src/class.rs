use crate::doc::DocString;
use crate::*;

/// C-style structure forward declaration
#[derive(Debug)]
pub struct ClassDeclaration {
    pub name: String,
}

impl ClassDeclaration {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

pub type ClassDeclarationHandle = Handle<ClassDeclaration>;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub native_function: FunctionHandle,
}

#[derive(Debug)]
pub struct AsyncMethod {
    pub name: String,
    pub native_function: FunctionHandle,
    pub return_type: CallbackArgument,
    pub return_type_doc: DocString,
    pub one_time_callback_name: String,
    pub one_time_callback_param_name: String,
    pub callback_name: String,
    pub callback_param_name: String,
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
pub struct Class {
    pub declaration: ClassDeclarationHandle,
    pub constructor: Option<FunctionHandle>,
    pub destructor: Option<FunctionHandle>,
    pub methods: Vec<Method>,
    pub static_methods: Vec<Method>,
    pub async_methods: Vec<AsyncMethod>,
    pub doc: Doc,
    pub destruction_mode: DestructionMode,
}

impl Class {
    pub fn name(&self) -> &str {
        &self.declaration.name
    }

    pub fn declaration(&self) -> ClassDeclarationHandle {
        self.declaration.clone()
    }

    pub fn find_method<T: AsRef<str>>(&self, method_name: T) -> Option<&FunctionHandle> {
        for method in &self.methods {
            if method.name == method_name.as_ref() {
                return Some(&method.native_function);
            }
        }

        for method in &self.static_methods {
            if method.name == method_name.as_ref() {
                return Some(&method.native_function);
            }
        }

        for method in &self.async_methods {
            if method.name == method_name.as_ref() {
                return Some(&method.native_function);
            }
        }

        None
    }
}

pub type ClassHandle = Handle<Class>;

pub struct ClassBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    declaration: ClassDeclarationHandle,
    constructor: Option<FunctionHandle>,
    destructor: Option<FunctionHandle>,
    methods: Vec<Method>,
    static_methods: Vec<Method>,
    async_methods: Vec<AsyncMethod>,
    doc: Option<Doc>,
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

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> BindResult<Self> {
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

    pub fn method<T: Into<String>>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        self.lib.validate_function(native_function)?;
        self.validate_first_param(native_function)?;

        self.methods.push(Method {
            name: name.into(),
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn static_method<T: Into<String>>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        self.lib.validate_function(native_function)?;

        self.static_methods.push(Method {
            name: name.into(),
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn async_method<T: Into<String>>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        self.lib.validate_function(native_function)?;
        self.validate_first_param(native_function)?;

        // Check that native method has a single callback with a single method,
        // with a single argument

        let name = name.into();
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
                            name: name.to_string(),
                            native_function: native_function.clone(),
                            return_type: cb_param.arg_type.clone(),
                            return_type_doc: cb_param.doc.clone(),
                            one_time_callback_name: ot_cb.name.clone(),
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
pub struct StaticClass {
    pub name: String,
    pub static_methods: Vec<Method>,
    pub doc: Doc,
}

pub type StaticClassHandle = Handle<StaticClass>;

pub struct StaticClassBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: String,
    static_methods: Vec<Method>,
    doc: Option<Doc>,
}

impl<'a> StaticClassBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: String) -> Self {
        Self {
            lib,
            name,
            static_methods: Vec::new(),
            doc: None,
        }
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

    pub fn static_method<T: Into<String>>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        self.lib.validate_function(native_function)?;

        self.static_methods.push(Method {
            name: name.into(),
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
