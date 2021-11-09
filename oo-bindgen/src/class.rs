use crate::doc::{DocCell, DocReference, Unvalidated, Validated};
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

/// Represents an instance method on a class
#[derive(Debug, Clone)]
pub struct Method<T>
where
    T: DocReference,
{
    pub name: Name,
    pub associated_class: Handle<ClassDeclaration>,
    pub native_function: Handle<Function<T>>,
}

impl Method<Unvalidated> {
    pub(crate) fn new(
        name: Name,
        associated_class: Handle<ClassDeclaration>,
        function: Handle<Function<Unvalidated>>,
    ) -> Self {
        Self {
            name,
            associated_class,
            native_function: function,
        }
    }

    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Method<Validated>> {
        Ok(Method {
            name: self.name.clone(),
            associated_class: self.associated_class.clone(),
            native_function: self.native_function.validate(lib)?,
        })
    }
}

/// represents a static method associated with a class
#[derive(Debug)]
pub struct StaticMethod<T>
where
    T: DocReference,
{
    pub name: Name,
    pub native_function: Handle<Function<T>>,
}

impl StaticMethod<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<StaticMethod<Validated>> {
        Ok(StaticMethod {
            name: self.name.clone(),
            native_function: self.native_function.validate(lib)?,
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
    pub constructor: Option<ClassConstructor<T>>,
    pub destructor: Option<ClassDestructor<T>>,
    pub methods: Vec<Method<T>>,
    pub static_methods: Vec<StaticMethod<T>>,
    pub future_methods: Vec<FutureMethod<T>>,
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
        let static_methods: BindResult<Vec<StaticMethod<Validated>>> = self
            .static_methods
            .iter()
            .map(|x| x.validate(lib))
            .collect();
        let async_methods: BindResult<Vec<FutureMethod<Validated>>> = self
            .future_methods
            .iter()
            .map(|x| x.validate(lib))
            .collect();

        Ok(Handle::new(Class {
            declaration: self.declaration.clone(),
            constructor,
            destructor,
            methods: methods?,
            static_methods: static_methods?,
            future_methods: async_methods?,
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

        for async_method in &self.future_methods {
            if async_method.name.as_ref() == method_name {
                return Some((
                    async_method.name.clone(),
                    async_method.native_function.clone(),
                ));
            }
        }

        None
    }
}

pub type ClassHandle = Handle<Class<Unvalidated>>;

pub struct ClassBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    declaration: ClassDeclarationHandle,
    constructor: Option<ClassConstructor<Unvalidated>>,
    destructor: Option<ClassDestructor<Unvalidated>>,
    methods: Vec<Method<Unvalidated>>,
    static_methods: Vec<StaticMethod<Unvalidated>>,
    async_methods: Vec<FutureMethod<Unvalidated>>,
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

    pub fn constructor(mut self, constructor: ClassConstructor<Unvalidated>) -> BindResult<Self> {
        // make sure the method is defined for this class
        self.check_class(&constructor.function.name, constructor.class.clone())?;

        if self.constructor.is_some() {
            return Err(BindingError::ConstructorAlreadyDefined {
                handle: self.declaration,
            });
        }

        self.constructor = Some(constructor);

        Ok(self)
    }

    pub fn destructor(mut self, destructor: ClassDestructor<Unvalidated>) -> BindResult<Self> {
        if self.destructor.is_some() {
            return Err(BindingError::DestructorAlreadyDefined {
                handle: self.declaration,
            });
        }

        // make sure the method is defined for this class
        self.check_class(&destructor.function.name, destructor.class.clone())?;

        self.destructor = Some(destructor);

        Ok(self)
    }

    pub fn method(mut self, method: Method<Unvalidated>) -> BindResult<Self> {
        // make sure the method is defined for this class
        self.check_class(&method.name, method.associated_class.clone())?;

        self.methods.push(method);

        Ok(self)
    }

    pub fn static_method<T: IntoName>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        self.lib.validate_function(native_function)?;

        self.static_methods.push(StaticMethod {
            name,
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    fn check_class(&self, name: &Name, other: ClassDeclarationHandle) -> BindResult<()> {
        if self.declaration != other {
            return Err(BindingError::ClassMethodWrongAssociatedClass {
                name: name.clone(),
                declared: other,
                added_to: self.declaration.clone(),
            });
        }
        Ok(())
    }

    pub fn async_method(mut self, method: FutureMethod<Unvalidated>) -> BindResult<Self> {
        self.check_class(&method.name, method.associated_class.clone())?;

        self.async_methods.push(method);

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
            future_methods: self.async_methods,
            doc,
            destruction_mode: self.destruction_mode,
            settings: self.lib.settings.clone(),
        });

        self.lib
            .add_statement(Statement::ClassDefinition(handle.clone()))?;

        Ok(handle)
    }
}

/// Static class definition
#[derive(Debug)]
pub struct StaticClass<T>
where
    T: DocReference,
{
    pub name: Name,
    pub static_methods: Vec<StaticMethod<T>>,
    pub doc: Doc<T>,
}

impl StaticClass<Unvalidated> {
    pub(crate) fn validate(
        &self,
        lib: &UnvalidatedFields,
    ) -> BindResult<Handle<StaticClass<Validated>>> {
        let methods: BindResult<Vec<StaticMethod<Validated>>> = self
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
    static_methods: Vec<StaticMethod<Unvalidated>>,
    doc: DocCell,
}

impl<'a> StaticClassBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: Name) -> Self {
        Self {
            lib,
            name: name.clone(),
            static_methods: Vec::new(),
            doc: DocCell::new(name),
        }
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        self.doc.set(doc.into())?;
        Ok(self)
    }

    pub fn static_method<T: IntoName>(
        mut self,
        name: T,
        native_function: &FunctionHandle,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        self.lib.validate_function(native_function)?;

        self.static_methods.push(StaticMethod {
            name,
            native_function: native_function.clone(),
        });

        Ok(self)
    }

    pub fn build(self) -> BindResult<StaticClassHandle> {
        let handle = StaticClassHandle::new(StaticClass {
            name: self.name,
            static_methods: self.static_methods,
            doc: self.doc.extract()?,
        });

        self.lib
            .add_statement(Statement::StaticClassDefinition(handle.clone()))?;

        Ok(handle)
    }
}
