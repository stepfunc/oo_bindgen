use std::collections::HashSet;

use crate::model::*;

pub struct InterfaceBuilder<'a> {
    lib: &'a mut LibraryBuilder,
    name: Name,
    callbacks: Vec<CallbackFunction<Unvalidated>>,
    callback_names: HashSet<String>,
    doc: Doc<Unvalidated>,
}

impl<'a> InterfaceBuilder<'a> {
    pub(crate) fn new(lib: &'a mut LibraryBuilder, name: Name, doc: Doc<Unvalidated>) -> Self {
        Self {
            lib,
            name,
            callbacks: Vec::new(),
            callback_names: Default::default(),
            doc,
        }
    }

    pub fn begin_callback<T: IntoName, D: Into<Doc<Unvalidated>>>(
        mut self,
        name: T,
        doc: D,
    ) -> BindResult<CallbackFunctionBuilder<'a>> {
        let name = name.into_name()?;
        self.check_unique_callback_name(&name)?;
        Ok(CallbackFunctionBuilder::new(self, name, doc.into()))
    }

    /// Build the interface and mark it as only used in a synchronous context.
    ///
    /// A synchronous interface is one that is invoked only during a function call which
    /// takes it as an argument. The Rust backend will NOT generate `Send` and `Sync`
    /// implementations so that it be cannot be transferred across thread boundaries.
    pub fn build_sync(self) -> BindResult<SynchronousInterface> {
        let (handle, lib) = self.build(InterfaceCategory::Synchronous);
        lib.add_statement(Statement::InterfaceDefinition(InterfaceType::Synchronous(
            handle.clone(),
        )))?;
        Ok(SynchronousInterface { inner: handle })
    }

    /// Build the interface and mark it as used in an asynchronous context.
    ///
    /// An asynchronous interface is one that is invoked some time after it is
    /// passed as a function argument. The Rust backend will mark the C representation
    /// of this interface as `Send` and `Sync` so that it be transferred across thread
    /// boundaries.
    pub fn build_async(self) -> BindResult<AsynchronousInterface> {
        let (handle, lib) = self.build(InterfaceCategory::Asynchronous);
        lib.add_statement(Statement::InterfaceDefinition(InterfaceType::Asynchronous(
            handle.clone(),
        )))?;
        Ok(AsynchronousInterface { inner: handle })
    }

    pub(crate) fn build(
        self,
        mode: InterfaceCategory,
    ) -> (InterfaceHandle, &'a mut LibraryBuilder) {
        let handle = Handle::new(Interface {
            name: self.name,
            mode,
            callbacks: self.callbacks,
            doc: self.doc,
            settings: self.lib.settings.clone(),
        });

        (handle, self.lib)
    }

    fn check_unique_callback_name(&mut self, name: &Name) -> BindResult<()> {
        if name == &self.lib.settings.interface.destroy_func_name {
            return Err(BindingError::InterfaceMethodWithReservedName {
                name: self.lib.settings.interface.destroy_func_name.clone(),
            });
        }

        if name == &self.lib.settings.interface.context_variable_name.clone() {
            return Err(BindingError::InterfaceMethodWithReservedName {
                name: self.lib.settings.interface.context_variable_name.clone(),
            });
        }

        if self.callback_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::InterfaceDuplicateCallbackName {
                interface_name: self.name.clone(),
                callback_name: name.clone(),
            })
        }
    }
}

pub struct CallbackFunctionBuilder<'a> {
    builder: InterfaceBuilder<'a>,
    name: Name,
    functional_transform: FunctionalTransform,
    return_type: OptionalReturnType<CallbackReturnValue, Unvalidated>,
    arguments: Vec<Arg<CallbackArgument, Unvalidated>>,
    doc: Doc<Unvalidated>,
}

impl<'a> CallbackFunctionBuilder<'a> {
    pub(crate) fn new(builder: InterfaceBuilder<'a>, name: Name, doc: Doc<Unvalidated>) -> Self {
        Self {
            builder,
            name,
            functional_transform: FunctionalTransform::No,
            return_type: OptionalReturnType::new(),
            arguments: Vec::new(),
            doc,
        }
    }

    pub fn enable_functional_transform(mut self) -> Self {
        self.functional_transform = FunctionalTransform::Yes;
        self
    }

    pub fn param<S: IntoName, D: Into<DocString<Unvalidated>>, P: Into<CallbackArgument>>(
        mut self,
        name: S,
        arg_type: P,
        doc: D,
    ) -> BindResult<Self> {
        let arg_type = arg_type.into();
        let name = name.into_name()?;

        if name == self.builder.lib.settings.interface.context_variable_name {
            return Err(BindingError::CallbackMethodArgumentWithReservedName {
                name: self
                    .builder
                    .lib
                    .settings
                    .interface
                    .context_variable_name
                    .clone(),
            });
        }

        self.arguments.push(Arg::new(arg_type, name, doc.into()));
        Ok(self)
    }

    pub fn returns<T: Into<CallbackReturnValue>, D: Into<DocString<Unvalidated>>>(
        mut self,
        t: T,
        d: D,
    ) -> BindResult<Self> {
        self.return_type.set(&self.name, t.into(), d.into())?;
        Ok(self)
    }

    pub fn end_callback(mut self) -> BindResult<InterfaceBuilder<'a>> {
        let cb = CallbackFunction {
            name: self.name,
            functional_transform: self.functional_transform,
            return_type: self.return_type,
            arguments: self.arguments,
            doc: self.doc,
        };

        self.builder.callbacks.push(cb);
        Ok(self.builder)
    }
}
