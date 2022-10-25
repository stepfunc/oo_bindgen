use crate::model::*;

pub struct FunctionBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    function_type: FunctionCategory,
    return_type: OptionalReturnType<FunctionReturnValue, Unvalidated>,
    params: Vec<Arg<FunctionArgument, Unvalidated>>,
    doc: OptionalDoc,
    error_type: OptionalErrorType<Unvalidated>,
}

impl<'a> FunctionBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        name: Name,
        function_type: FunctionCategory,
    ) -> Self {
        Self {
            lib,
            name: name.clone(),
            function_type,
            return_type: OptionalReturnType::new(),
            params: Vec::new(),
            doc: OptionalDoc::new(name),
            error_type: OptionalErrorType::new(),
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
        self.params.push(Arg {
            name,
            arg_type: param_type,
            doc: doc.into(),
        });
        Ok(self)
    }

    pub fn returns<D: Into<DocString<Unvalidated>>, T: Into<FunctionReturnValue>>(
        mut self,
        return_type: T,
        doc: D,
    ) -> BindResult<Self> {
        self.return_type
            .set(&self.name, return_type.into(), doc.into())?;
        Ok(self)
    }

    pub fn fails_with(mut self, err: ErrorType<Unvalidated>) -> BindResult<Self> {
        self.error_type.set(&self.name, &err)?;
        Ok(self)
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        self.doc.set(doc.into())?;
        Ok(self)
    }

    pub fn build(self) -> BindResult<FunctionHandle> {
        let handle = Handle::new(Function {
            name: self.name,
            category: self.function_type,
            return_type: self.return_type,
            arguments: self.params,
            error_type: self.error_type,
            settings: self.lib.clone_settings(),
            doc: self.doc.extract()?,
        });

        self.lib
            .add_statement(Statement::FunctionDefinition(handle.clone()))?;

        Ok(handle)
    }

    /// Build a static method with a different name than the native function
    pub fn build_static<N: IntoName>(self, name: N) -> BindResult<StaticMethod<Unvalidated>> {
        let handle = self.build()?;
        Ok(StaticMethod {
            name: name.into_name()?,
            native_function: handle,
        })
    }

    /// Build a static method with the same name as the native function
    pub fn build_static_with_same_name(self) -> BindResult<StaticMethod<Unvalidated>> {
        let handle = self.build()?;
        Ok(StaticMethod {
            name: handle.name.clone(),
            native_function: handle,
        })
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
            return Err(BindingErrorVariant::BadMethodName { class, method_name }.into());
        }

        let instance_arg_name = lib.settings().class.method_instance_argument_name.clone();

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

    pub fn returns<D: Into<DocString<Unvalidated>>, T: Into<FunctionReturnValue>>(
        self,
        return_type: T,
        doc: D,
    ) -> BindResult<Self> {
        let return_type = return_type.into();
        let doc = doc.into();
        Ok(Self {
            method_name: self.method_name,
            class: self.class,
            inner: self.inner.returns(return_type, doc)?,
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

    pub fn build(self) -> BindResult<Method<Unvalidated>> {
        let function = self.inner.build()?;
        Ok(Method::new(self.method_name, self.class, function))
    }
}

pub struct ClassConstructorBuilder<'a> {
    class: ClassDeclarationHandle,
    inner: FunctionBuilder<'a>,
}

impl<'a> ClassConstructorBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        class: ClassDeclarationHandle,
    ) -> BindResult<Self> {
        let builder = lib
            .define_function(
                class
                    .name
                    .append(&lib.settings().class.class_constructor_suffix),
            )?
            .returns(
                class.clone(),
                format!("Instance of {{class:{}}}", class.name),
            )?;

        Ok(Self {
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
            class: self.class,
            inner: self.inner.param(name, param_type, doc)?,
        })
    }

    pub fn fails_with(self, err: ErrorType<Unvalidated>) -> BindResult<Self> {
        Ok(Self {
            class: self.class,
            inner: self.inner.fails_with(err)?,
        })
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(self, doc: D) -> BindResult<Self> {
        Ok(Self {
            class: self.class,
            inner: self.inner.doc(doc)?,
        })
    }

    pub fn build(self) -> BindResult<ClassConstructor<Unvalidated>> {
        Ok(ClassConstructor::new(self.class, self.inner.build()?))
    }
}

pub struct FutureMethodBuilder<'a> {
    future: FutureInterface<Unvalidated>,
    inner: ClassMethodBuilder<'a>,
}

impl<'a> FutureMethodBuilder<'a> {
    pub(crate) fn new(
        lib: &'a mut LibraryBuilder,
        method_name: Name,
        class: ClassDeclarationHandle,
        future: FutureInterface<Unvalidated>,
    ) -> BindResult<Self> {
        let builder = lib.define_method(method_name, class)?;

        Ok(Self {
            future,
            inner: builder,
        })
    }

    pub fn param<T: IntoName, D: Into<DocString<Unvalidated>>, P: Into<FunctionArgument>>(
        self,
        name: T,
        param_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        let param_type = param_type.into();
        let builder = self.inner.param(name, param_type, doc)?;
        Ok(Self {
            future: self.future,
            inner: builder,
        })
    }

    pub fn fails_with(self, err: ErrorType<Unvalidated>) -> BindResult<Self> {
        Ok(Self {
            future: self.future,
            inner: self.inner.fails_with(err)?,
        })
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(self, doc: D) -> BindResult<Self> {
        Ok(Self {
            future: self.future,
            inner: self.inner.doc(doc)?,
        })
    }

    pub fn build(self) -> BindResult<FutureMethod<Unvalidated>> {
        let future = self.future.clone();
        let callback_parameter_name = self
            .inner
            .inner
            .lib
            .settings()
            .future
            .async_method_callback_parameter_name
            .clone();
        let method = self
            .inner
            .param(
                callback_parameter_name,
                FunctionArgument::Interface(self.future.interface),
                "callback invoked when the operation completes",
            )?
            .build()?;

        Ok(FutureMethod {
            name: method.name,
            associated_class: method.associated_class,
            future,
            native_function: method.native_function,
        })
    }
}
