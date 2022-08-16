use std::rc::Rc;

use crate::model::*;

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

impl Method<Validated> {
    /// retrieve a list of arguments skipping the first class parameter
    pub fn arguments(&self) -> impl Iterator<Item = &Arg<FunctionArgument, Validated>> {
        self.native_function.arguments.iter().skip(1)
    }
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

    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Method<Validated>> {
        Ok(Method {
            name: self.name.clone(),
            associated_class: self.associated_class.clone(),
            native_function: self.native_function.validate(lib)?,
        })
    }
}

pub type MethodHandle = Method<Unvalidated>;

/// represents a static method associated with a class
///
/// name given to the class method may differ from the name of the native function
#[derive(Debug)]
pub struct StaticMethod<T>
where
    T: DocReference,
{
    pub name: Name,
    pub native_function: Handle<Function<T>>,
}

impl StaticMethod<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<StaticMethod<Validated>> {
        Ok(StaticMethod {
            name: self.name.clone(),
            native_function: self.native_function.validate(lib)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestructionMode {
    /// Object is automatically deleted by the GC
    Automatic,
    /// Object is disposed of manually by calling a custom method
    ///
    /// For safety, if the user never calls the destruction method, the object
    /// will still be deleted by the GC at some point. However, it is
    /// strongly advised to take care of the destruction manually.
    Custom(Name),
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
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Handle<Class<Validated>>> {
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
        lib: &LibraryFields,
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
