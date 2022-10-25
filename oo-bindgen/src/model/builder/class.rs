use crate::model::*;

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

    pub fn static_method(mut self, method: StaticMethod<Unvalidated>) -> BindResult<Self> {
        self.static_methods.push(method);
        Ok(self)
    }

    fn check_class(&self, name: &Name, other: ClassDeclarationHandle) -> BindResult<()> {
        if self.declaration != other {
            return Err(BindingError::ClassMemberWrongAssociatedClass {
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

    pub fn custom_destroy<T: IntoName>(mut self, name: T) -> BindResult<Self> {
        if self.destructor.is_none() {
            return Err(BindingError::NoDestructorForManualDestruction {
                handle: self.declaration,
            });
        }

        self.destruction_mode = DestructionMode::Custom(name.into_name()?);
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

        let handle = Handle::new(Class {
            declaration: self.declaration.clone(),
            constructor: self.constructor,
            destructor: self.destructor,
            methods: self.methods,
            static_methods: self.static_methods,
            future_methods: self.async_methods,
            doc,
            destruction_mode: self.destruction_mode,
            settings: self.lib.clone_settings(),
        });

        self.lib
            .add_statement(Statement::ClassDefinition(handle.clone()))?;

        Ok(handle)
    }
}

pub struct StaticClassBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    static_methods: Vec<StaticMethod<Unvalidated>>,
    doc: OptionalDoc,
}

impl<'a> StaticClassBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: Name) -> Self {
        Self {
            lib,
            name: name.clone(),
            static_methods: Vec::new(),
            doc: OptionalDoc::new(name),
        }
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        self.doc.set(doc.into())?;
        Ok(self)
    }

    pub fn static_method(mut self, method: StaticMethod<Unvalidated>) -> BindResult<Self> {
        self.static_methods.push(method);
        Ok(self)
    }

    pub fn build(self) -> BindResult<StaticClassHandle> {
        let handle = Handle::new(StaticClass {
            name: self.name,
            static_methods: self.static_methods,
            doc: self.doc.extract()?,
        });

        self.lib
            .add_statement(Statement::StaticClassDefinition(handle.clone()))?;

        Ok(handle)
    }
}
