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

/// Object-oriented class definition
#[derive(Debug)]
pub struct Class {
    pub declaration: ClassDeclarationHandle,
    pub constructor: Option<NativeFunctionHandle>,
    pub destructor: Option<NativeFunctionHandle>,
    pub methods: Vec<Method>,
    pub static_methods: Vec<Method>,
}

impl Class {
    pub fn name(&self) -> &str {
        &self.declaration.name
    }

    pub fn declaration(&self) -> ClassDeclarationHandle {
        self.declaration.clone()
    }

    pub fn is_static(&self) -> bool {
        self.constructor.is_none() &&
        self.destructor.is_none() &&
        self.methods.is_empty()
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
        }
    }

    pub fn constructor(mut self, native_function: &NativeFunctionHandle) -> Result<Self> {
        if self.constructor.is_some() {
            return Err(BindingError::ConstructorAlreadyDefined{handle: self.declaration})
        }
        self.lib.validate_native_function(native_function)?;

        if let ReturnType::Type(Type::ClassRef(return_type)) = &native_function.return_type {
            if return_type == &self.declaration {
                self.constructor = Some(native_function.clone());
                return Ok(self)
            }
        }

        Err(BindingError::ConstructorReturnTypeDoesNotMatch{
            handle: self.declaration,
            native_func: native_function.clone(),
        })
    }

    pub fn destructor(mut self, native_function: &NativeFunctionHandle) -> Result<Self> {
        if self.destructor.is_some() {
            return Err(BindingError::DestructorAlreadyDefined{handle: self.declaration})
        }
        self.lib.validate_native_function(native_function)?;
        self.validate_first_param(native_function)?;

        if native_function.parameters.len() != 1 {
            return Err(BindingError::DestructorTakesMoreThanOneParameter{
                handle: self.declaration,
                native_func: native_function.clone(),
            })
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

    pub fn static_method(mut self, name: &str, native_function: &NativeFunctionHandle) -> Result<Self> {
        self.lib.validate_native_function(native_function)?;

        self.static_methods.push(Method {
            name: name.to_string(),
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn build(self) -> ClassHandle {
        let handle = ClassHandle::new(Class {
            declaration: self.declaration.clone(),
            constructor: self.constructor,
            destructor: self.destructor,
            methods: self.methods,
            static_methods: self.static_methods,
        });

        self.lib.classes.insert(handle.declaration.clone(), handle.clone());
        self.lib.statements.push(Statement::ClassDefinition(handle.clone()));

        handle
    }

    fn validate_first_param(&self, native_function: &NativeFunctionHandle) -> Result<()> {
        if let Some(first_param) = native_function.parameters.first() {
            if let Type::ClassRef(first_param_type) = &first_param.param_type {
                if first_param_type == &self.declaration {
                    return Ok(())
                }
            }
        }

        Err(BindingError::FirstMethodParameterIsNotClassType{
            handle: self.declaration.clone(),
            native_func: native_function.clone(),
        })
    }
}
