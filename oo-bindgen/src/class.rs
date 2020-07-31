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
    pub native_function: NativeFunctionHandle,
}

#[derive(Debug)]
pub struct AsyncMethod {
    pub name: String,
    pub native_function: NativeFunctionHandle,
    pub return_type: Type,
    pub return_type_doc: Doc,
    pub one_time_callback_name: String,
    pub one_time_callback_param_name: String,
    pub callback_name: String,
    pub callback_param_name: String,
}

/// Object-oriented class definition
#[derive(Debug)]
pub struct Class {
    pub declaration: ClassDeclarationHandle,
    pub constructor: Option<NativeFunctionHandle>,
    pub destructor: Option<NativeFunctionHandle>,
    pub methods: Vec<Method>,
    pub static_methods: Vec<Method>,
    pub async_methods: Vec<AsyncMethod>,
    pub doc: Doc,
}

impl Class {
    pub fn name(&self) -> &str {
        &self.declaration.name
    }

    pub fn declaration(&self) -> ClassDeclarationHandle {
        self.declaration.clone()
    }

    pub fn is_static(&self) -> bool {
        self.constructor.is_none() && self.destructor.is_none() && self.methods.is_empty()
    }
}

pub type ClassHandle = Handle<Class>;

pub struct ClassBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    declaration: ClassDeclarationHandle,
    constructor: Option<NativeFunctionHandle>,
    destructor: Option<NativeFunctionHandle>,
    methods: Vec<Method>,
    static_methods: Vec<Method>,
    async_methods: Vec<AsyncMethod>,
    doc: Option<Doc>,
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
        }
    }

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> Result<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingError::DocAlreadyDefined {
                symbol_name: self.declaration.name.clone(),
            })
        }
    }

    pub fn constructor(mut self, native_function: &NativeFunctionHandle) -> Result<Self> {
        if self.constructor.is_some() {
            return Err(BindingError::ConstructorAlreadyDefined {
                handle: self.declaration,
            });
        }
        self.lib.validate_native_function(native_function)?;

        if let ReturnType::Type(Type::ClassRef(return_type), _) = &native_function.return_type {
            if return_type == &self.declaration {
                self.constructor = Some(native_function.clone());
                return Ok(self);
            }
        }

        Err(BindingError::ConstructorReturnTypeDoesNotMatch {
            handle: self.declaration,
            native_func: native_function.clone(),
        })
    }

    pub fn destructor(mut self, native_function: &NativeFunctionHandle) -> Result<Self> {
        if self.destructor.is_some() {
            return Err(BindingError::DestructorAlreadyDefined {
                handle: self.declaration,
            });
        }
        self.lib.validate_native_function(native_function)?;
        self.validate_first_param(native_function)?;

        if native_function.parameters.len() != 1 {
            return Err(BindingError::DestructorTakesMoreThanOneParameter {
                handle: self.declaration,
                native_func: native_function.clone(),
            });
        }

        self.destructor = Some(native_function.clone());

        Ok(self)
    }

    pub fn method(mut self, name: &str, native_function: &NativeFunctionHandle) -> Result<Self> {
        self.lib.validate_native_function(native_function)?;
        self.validate_first_param(native_function)?;

        self.methods.push(Method {
            name: name.to_string(),
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn static_method(
        mut self,
        name: &str,
        native_function: &NativeFunctionHandle,
    ) -> Result<Self> {
        self.lib.validate_native_function(native_function)?;

        self.static_methods.push(Method {
            name: name.to_string(),
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn async_method(
        mut self,
        name: &str,
        native_function: &NativeFunctionHandle,
    ) -> Result<Self> {
        self.lib.validate_native_function(native_function)?;
        self.validate_first_param(native_function)?;

        // Check that native method has a single callback with a single method,
        // with a single argument

        let mut async_method = None;
        for param in &native_function.parameters {
            if let Type::OneTimeCallback(ot_cb) = &param.param_type {
                if async_method.is_some() {
                    return Err(BindingError::AsyncNativeMethodTooManyOneTimeCallback {
                        handle: native_function.clone(),
                    });
                }

                let mut cb_iter = ot_cb.callbacks();
                if let Some(cb) = cb_iter.next() {
                    if !cb.return_type.is_void() {
                        return Err(BindingError::AsyncCallbackReturnTypeNotVoid {
                            handle: native_function.clone(),
                        });
                    }

                    let mut iter = cb.params();
                    if let Some(cb_param) = iter.next() {
                        async_method = Some(AsyncMethod {
                            name: name.to_string(),
                            native_function: native_function.clone(),
                            return_type: cb_param.param_type.clone(),
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
                        return Err(BindingError::AsyncOneTimeCallbackNotSingleCallback {
                            handle: native_function.clone(),
                        });
                    }
                } else {
                    return Err(BindingError::AsyncOneTimeCallbackNotSingleCallback {
                        handle: native_function.clone(),
                    });
                }
            }
        }

        if let Some(method) = async_method {
            self.async_methods.push(method);
        } else {
            return Err(BindingError::AsyncNativeMethodNoOneTimeCallback {
                handle: native_function.clone(),
            });
        }

        Ok(self)
    }

    pub fn build(self) -> Result<ClassHandle> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => return Err(BindingError::DocNotDefined {
                symbol_name: self.declaration.name.clone(),
            })
        };

        let handle = ClassHandle::new(Class {
            declaration: self.declaration.clone(),
            constructor: self.constructor,
            destructor: self.destructor,
            methods: self.methods,
            static_methods: self.static_methods,
            async_methods: self.async_methods,
            doc,
        });

        self.lib
            .classes
            .insert(handle.declaration.clone(), handle.clone());
        self.lib
            .statements
            .push(Statement::ClassDefinition(handle.clone()));

        Ok(handle)
    }

    fn validate_first_param(&self, native_function: &NativeFunctionHandle) -> Result<()> {
        if let Some(first_param) = native_function.parameters.first() {
            if let Type::ClassRef(first_param_type) = &first_param.param_type {
                if first_param_type == &self.declaration {
                    return Ok(());
                }
            }
        }

        Err(BindingError::FirstMethodParameterIsNotClassType {
            handle: self.declaration.clone(),
            native_func: native_function.clone(),
        })
    }
}
