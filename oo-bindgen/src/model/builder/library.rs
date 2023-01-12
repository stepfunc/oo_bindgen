use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::model::*;

pub(crate) struct LibraryFields {
    // a record of statements preserved in order
    pub(crate) statements: Vec<Statement<Unvalidated>>,

    // native stuff
    pub(crate) structs_declarations: HashSet<StructDeclarationHandle>,
    pub(crate) structs: HashMap<StructDeclarationHandle, StructType<Unvalidated>>,
    pub(crate) functions: HashSet<Handle<Function<Unvalidated>>>,
    pub(crate) enums: HashSet<Handle<Enum<Unvalidated>>>,

    // oo stuff
    pub(crate) class_declarations: HashSet<ClassDeclarationHandle>,
    pub(crate) classes: HashMap<ClassDeclarationHandle, Handle<Class<Unvalidated>>>,
    pub(crate) static_classes: HashSet<Handle<StaticClass<Unvalidated>>>,
    pub(crate) interfaces: HashSet<Handle<Interface<Unvalidated>>>,

    // specialized types
    pub(crate) iterators: HashSet<Handle<AbstractIterator<Unvalidated>>>,
    pub(crate) collections: HashSet<Handle<Collection<Unvalidated>>>,
}

impl LibraryFields {
    pub(crate) fn find_struct<T: AsRef<str>>(&self, name: T) -> Option<&StructType<Unvalidated>> {
        self.structs.values().find(|x| x.name() == name.as_ref())
    }

    pub(crate) fn find_enum<T: AsRef<str>>(&self, name: T) -> Option<&Handle<Enum<Unvalidated>>> {
        self.enums.iter().find(|x| x.name == name.as_ref())
    }

    pub(crate) fn find_class_declaration<T: AsRef<str>>(
        &self,
        name: T,
    ) -> Option<&ClassDeclarationHandle> {
        self.class_declarations
            .iter()
            .find(|x| x.name == name.as_ref())
    }

    pub(crate) fn find_class<T: AsRef<str>>(&self, name: T) -> Option<&Handle<Class<Unvalidated>>> {
        self.classes.values().find(|x| x.name() == name.as_ref())
    }

    pub(crate) fn find_interface<T: AsRef<str>>(
        &self,
        name: T,
    ) -> Option<&Handle<Interface<Unvalidated>>> {
        self.interfaces.iter().find(|x| x.name == name.as_ref())
    }

    pub(crate) fn new() -> Self {
        Self {
            statements: Vec::new(),

            structs_declarations: HashSet::new(),
            structs: HashMap::new(),

            enums: HashSet::new(),

            class_declarations: HashSet::new(),
            classes: HashMap::new(),
            static_classes: HashSet::new(),

            interfaces: HashSet::new(),

            iterators: HashSet::new(),
            collections: HashSet::new(),

            functions: HashSet::new(),
        }
    }
}

pub struct LibraryBuilder {
    version: Version,
    info: Rc<LibraryInfo>,

    settings: Rc<LibrarySettings>,

    // names of symbols used in the library
    symbol_names: HashSet<String>,
    fields: LibraryFields,
}

impl LibraryBuilder {
    pub fn new(version: Version, info: LibraryInfo, settings: Rc<LibrarySettings>) -> Self {
        Self {
            version,
            info: Rc::new(info),
            settings,
            symbol_names: HashSet::new(),
            fields: LibraryFields::new(),
        }
    }

    pub(crate) fn settings(&self) -> &LibrarySettings {
        &self.settings
    }

    pub(crate) fn clone_settings(&self) -> Rc<LibrarySettings> {
        self.settings.clone()
    }

    pub(crate) fn add_statement(&mut self, statement: Statement<Unvalidated>) -> BindResult<()> {
        // verify that all the pieces of the statement are from this library
        self.check_statement(&statement)?;

        if let Some(name) = statement.unique_name() {
            self.check_unique_symbol(name)?;
        }

        self.fields.statements.push(statement.clone());

        match statement {
            Statement::Constants(_) => {}
            Statement::StructDeclaration(x) => {
                self.fields.structs_declarations.insert(x);
            }
            Statement::StructDefinition(x) => {
                self.fields.structs.insert(x.declaration(), x);
            }
            Statement::EnumDefinition(x) => {
                self.fields.enums.insert(x);
            }
            Statement::ErrorType(_) => {}
            Statement::ClassDeclaration(x) => {
                self.fields.class_declarations.insert(x);
            }
            Statement::ClassDefinition(x) => {
                self.fields.classes.insert(x.declaration.clone(), x);
            }
            Statement::StaticClassDefinition(x) => {
                self.fields.static_classes.insert(x);
            }
            Statement::InterfaceDefinition(x) => {
                self.fields.interfaces.insert(x.untyped().clone());
            }
            Statement::IteratorDeclaration(x) => {
                self.fields.iterators.insert(x);
            }
            Statement::CollectionDeclaration(x) => {
                self.fields.collections.insert(x);
            }
            Statement::FunctionDefinition(x) => {
                self.fields.functions.insert(x);
            }
        }

        Ok(())
    }

    fn validate_statement(
        &self,
        statement: &Statement<Unvalidated>,
    ) -> BindResult<Statement<Validated>> {
        match statement {
            Statement::Constants(x) => Ok(Statement::Constants(x.validate(&self.fields)?)),
            Statement::StructDeclaration(x) => Ok(Statement::StructDeclaration(x.clone())),
            Statement::StructDefinition(x) => {
                Ok(Statement::StructDefinition(x.validate(&self.fields)?))
            }
            Statement::EnumDefinition(x) => {
                Ok(Statement::EnumDefinition(x.validate(&self.fields)?))
            }
            Statement::ErrorType(x) => Ok(Statement::ErrorType(x.validate(&self.fields)?)),
            Statement::ClassDeclaration(x) => Ok(Statement::ClassDeclaration(x.clone())),
            Statement::ClassDefinition(x) => {
                Ok(Statement::ClassDefinition(x.validate(&self.fields)?))
            }
            Statement::StaticClassDefinition(x) => {
                Ok(Statement::StaticClassDefinition(x.validate(&self.fields)?))
            }
            Statement::InterfaceDefinition(x) => {
                Ok(Statement::InterfaceDefinition(x.validate(&self.fields)?))
            }
            Statement::IteratorDeclaration(x) => {
                Ok(Statement::IteratorDeclaration(x.validate(&self.fields)?))
            }
            Statement::CollectionDeclaration(x) => {
                Ok(Statement::CollectionDeclaration(x.validate(&self.fields)?))
            }
            Statement::FunctionDefinition(x) => {
                Ok(Statement::FunctionDefinition(x.validate(&self.fields)?))
            }
        }
    }

    pub fn build(mut self) -> BindResult<Library> {
        // Add the version function
        self.define_function("version")?
            .returns(StringType, "Version number")?
            .doc("Get the version of the library as a string")?
            .build()?;

        let statements: BindResult<Vec<Statement<Validated>>> = self
            .fields
            .statements
            .iter()
            .map(|s| self.validate_statement(s))
            .collect();

        Ok(Library::new(
            self.version,
            self.info,
            self.settings,
            statements?,
        ))
    }

    pub fn define_error_type<T: IntoName>(
        &mut self,
        error_name: T,
        exception_name: T,
        exception_type: ExceptionType,
    ) -> BindResult<ErrorTypeBuilder> {
        let exception_name = exception_name.into_name()?;
        let builder = self
            .define_enum(error_name)?
            .push("ok", "Success, i.e. no error occurred")?;

        Ok(ErrorTypeBuilder::new(
            exception_name,
            exception_type,
            builder,
        ))
    }

    pub fn define_constants<T: IntoName>(&mut self, name: T) -> BindResult<ConstantSetBuilder> {
        Ok(ConstantSetBuilder::new(self, name.into_name()?))
    }

    pub(crate) fn declare_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<StructDeclarationHandle> {
        let name = name.into_name()?;
        let handle = Handle::new(StructDeclaration::new(name, self.settings.clone()));
        self.add_statement(Statement::StructDeclaration(handle.clone()))?;
        Ok(handle)
    }

    pub fn declare_universal_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<UniversalStructDeclaration> {
        Ok(UniversalStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    pub fn declare_function_argument_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<FunctionArgStructDeclaration> {
        Ok(FunctionArgStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    pub fn declare_function_return_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<FunctionReturnStructDeclaration> {
        Ok(FunctionReturnStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    pub fn declare_callback_argument_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<CallbackArgStructDeclaration> {
        Ok(CallbackArgStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    /// Define a structure that can be used in any context.
    ///
    /// Backends will generate bi-directional conversion routines
    /// for this type of struct.
    pub fn define_universal_struct(
        &mut self,
        declaration: UniversalStructDeclaration,
    ) -> BindResult<UniversalStructBuilder> {
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingErrorVariant::StructAlreadyDefined {
                handle: declaration.inner,
            }
            .into())
        } else {
            Ok(UniversalStructBuilder::new(self, declaration))
        }
    }

    /// Define an opaque structure which must be of universal type
    pub fn define_opaque_struct(
        &mut self,
        declaration: UniversalStructDeclaration,
    ) -> BindResult<UniversalStructBuilder> {
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingErrorVariant::StructAlreadyDefined {
                handle: declaration.inner,
            }
            .into())
        } else {
            Ok(UniversalStructBuilder::opaque(self, declaration))
        }
    }

    /// Define a structure that can be only be used in callback function arguments
    pub fn define_callback_argument_struct<T>(
        &mut self,
        declaration: T,
    ) -> BindResult<CallbackArgStructBuilder>
    where
        T: Into<CallbackArgStructDeclaration>,
    {
        let declaration = declaration.into();
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingErrorVariant::StructAlreadyDefined {
                handle: declaration.inner,
            }
            .into())
        } else {
            Ok(CallbackArgStructBuilder::new(self, declaration))
        }
    }

    /// Define a structure that can only be used as function return value
    pub fn define_function_return_struct<T>(
        &mut self,
        declaration: T,
    ) -> BindResult<FunctionReturnStructBuilder>
    where
        T: Into<FunctionReturnStructDeclaration>,
    {
        let declaration = declaration.into();
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingErrorVariant::StructAlreadyDefined {
                handle: declaration.inner,
            }
            .into())
        } else {
            Ok(FunctionReturnStructBuilder::new(self, declaration))
        }
    }

    /// Define a structure that can only be be used as a function argument
    pub fn define_function_argument_struct<T>(
        &mut self,
        declaration: T,
    ) -> BindResult<FunctionArgStructBuilder>
    where
        T: Into<FunctionArgStructDeclaration>,
    {
        let declaration = declaration.into();
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingErrorVariant::StructAlreadyDefined {
                handle: declaration.inner,
            }
            .into())
        } else {
            Ok(FunctionArgStructBuilder::new(self, declaration))
        }
    }

    /// Define an enumeration
    pub fn define_enum<T: IntoName>(&mut self, name: T) -> BindResult<EnumBuilder> {
        Ok(EnumBuilder::new(self, name.into_name()?))
    }

    pub fn define_function<T: IntoName>(&mut self, name: T) -> BindResult<FunctionBuilder> {
        self.define_function_with_category(name, FunctionCategory::Native)
    }

    pub fn define_method<T: IntoName>(
        &mut self,
        name: T,
        class: ClassDeclarationHandle,
    ) -> BindResult<ClassMethodBuilder> {
        ClassMethodBuilder::new(self, name.into_name()?, class)
    }

    pub fn define_future_method<T: IntoName>(
        &mut self,
        name: T,
        class: ClassDeclarationHandle,
        future: FutureInterface<Unvalidated>,
    ) -> BindResult<FutureMethodBuilder> {
        FutureMethodBuilder::new(self, name.into_name()?, class, future)
    }

    pub fn define_constructor(
        &mut self,
        class: ClassDeclarationHandle,
    ) -> BindResult<ClassConstructorBuilder> {
        ClassConstructorBuilder::new(self, class)
    }

    pub fn define_destructor<D: Into<Doc<Unvalidated>>>(
        &mut self,
        class: ClassDeclarationHandle,
        doc: D,
    ) -> BindResult<ClassDestructor<Unvalidated>> {
        ClassDestructor::new(self, class, doc.into())
    }

    pub(crate) fn define_function_with_category<T: IntoName>(
        &mut self,
        name: T,
        category: FunctionCategory,
    ) -> BindResult<FunctionBuilder> {
        Ok(FunctionBuilder::new(self, name.into_name()?, category))
    }

    pub fn declare_class<T: IntoName>(&mut self, name: T) -> BindResult<ClassDeclarationHandle> {
        self.declare_any_class(name, ClassType::Normal)
    }

    fn declare_iterator<T: IntoName>(&mut self, name: T) -> BindResult<IteratorClassDeclaration> {
        Ok(IteratorClassDeclaration::new(
            self.declare_any_class(name, ClassType::Iterator)?,
        ))
    }

    fn declare_collection<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<CollectionClassDeclaration> {
        Ok(CollectionClassDeclaration::new(self.declare_any_class(
            name.into_name()?,
            ClassType::Collection,
        )?))
    }

    fn declare_any_class<T: IntoName>(
        &mut self,
        name: T,
        class_type: ClassType,
    ) -> BindResult<ClassDeclarationHandle> {
        let name = name.into_name()?;
        let handle = ClassDeclarationHandle::new(ClassDeclaration::new(
            name,
            class_type,
            self.settings.clone(),
        ));
        self.add_statement(Statement::ClassDeclaration(handle.clone()))?;
        Ok(handle)
    }

    pub fn define_class(
        &mut self,
        declaration: &ClassDeclarationHandle,
    ) -> BindResult<ClassBuilder> {
        self.check_class_declaration(declaration)?;
        if self.fields.classes.contains_key(declaration) {
            Err(BindingErrorVariant::ClassAlreadyDefined {
                handle: declaration.clone(),
            }
            .into())
        } else {
            Ok(ClassBuilder::new(self, declaration.clone()))
        }
    }

    pub fn define_static_class<T: IntoName>(&mut self, name: T) -> BindResult<StaticClassBuilder> {
        Ok(StaticClassBuilder::new(self, name.into_name()?))
    }

    /// A future interface is a specialized asynchronous which consists of
    /// a single callback method providing a single value. The callback
    /// represents the completion of a "future" and is mapped appropriately
    /// in backends.
    pub fn define_future_interface<
        T: IntoName,
        D: Into<Doc<Unvalidated>>,
        E: Into<DocString<Unvalidated>>,
        V: Into<CallbackArgument>,
    >(
        &mut self,
        name: T,
        interface_docs: D,
        value_type: V,
        value_type_docs: E,
        error_type: ErrorType<Unvalidated>,
    ) -> BindResult<FutureInterface<Unvalidated>> {
        let value_type = value_type.into();
        let value_type_docs = value_type_docs.into();
        let name = name.into_name()?;
        let success_callback_name = self.settings.future.success_callback_method_name.clone();
        let success_parameter_name = self.settings.future.success_single_parameter_name.clone();
        let failure_callback_name = self.settings.future.failure_callback_method_name.clone();
        let failure_parameter_name = self.settings.future.failure_single_parameter_name.clone();

        let builder = self
            .define_interface(name, interface_docs)?
            .begin_callback(
                success_callback_name,
                "Invoked when the asynchronous operation completes successfully",
            )?
            .param(
                success_parameter_name,
                value_type.clone(),
                value_type_docs.clone(),
            )?
            .enable_functional_transform()
            .end_callback()?;

        let builder = builder
            .begin_callback(
                failure_callback_name,
                "Invoked when the asynchronous operation fails",
            )?
            .param(
                failure_parameter_name,
                CallbackArgument::Basic(BasicType::Enum(error_type.clone_enum())),
                "Enumeration indicating which error occurred",
            )?
            .enable_functional_transform()
            .end_callback()?;

        let (interface, lib) = builder.build(InterfaceCategory::Future);
        let ret = FutureInterface::new(value_type, error_type, interface, value_type_docs);
        lib.add_statement(Statement::InterfaceDefinition(InterfaceType::Future(
            ret.clone(),
        )))?;

        Ok(ret)
    }

    pub fn define_interface<T: IntoName, D: Into<Doc<Unvalidated>>>(
        &mut self,
        name: T,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        Ok(InterfaceBuilder::new(self, name.into_name()?, doc.into()))
    }

    pub fn define_iterator<N: IntoName, T: Into<IteratorItemType>>(
        &mut self,
        class_name: N,
        item_type: T,
    ) -> BindResult<AbstractIteratorHandle> {
        self.define_iterator_impl(class_name, false, item_type)
    }

    pub fn define_iterator_with_lifetime<N: IntoName, T: Into<IteratorItemType>>(
        &mut self,
        class_name: N,
        item_type: T,
    ) -> BindResult<AbstractIteratorHandle> {
        self.define_iterator_impl(class_name, true, item_type)
    }

    fn define_iterator_impl<N: IntoName, T: Into<IteratorItemType>>(
        &mut self,
        class_name: N,
        has_lifetime: bool,
        item_type: T,
    ) -> BindResult<AbstractIteratorHandle> {
        let class_name = class_name.into_name()?;
        let item_type = item_type.into();

        let class = self.declare_iterator(&class_name)?;
        let next_function = self
            .define_function_with_category(
                class_name.append(&self.settings.iterator.next_function_suffix),
                FunctionCategory::IteratorNext,
            )?
            .param(
                "iter",
                class.clone(),
                "opaque iterator on which to retrieve the next value",
            )?
            .doc("returns a pointer to the next value or NULL")?
            .returns(item_type.get_function_return_value(), "next value or NULL")?
            .build()?;

        let iter = AbstractIteratorHandle::new(crate::model::iterator::AbstractIterator::new(
            has_lifetime,
            class.inner,
            next_function,
            item_type,
            self.settings.clone(),
        ));
        self.add_statement(Statement::IteratorDeclaration(iter.clone()))?;
        Ok(iter)
    }

    pub fn define_collection<N: IntoName, A: Into<FunctionArgument>>(
        &mut self,
        class_name: N,
        value_type: A,
        has_reserve: bool,
    ) -> BindResult<CollectionHandle> {
        let class_name = class_name.into_name()?;
        let value_type = value_type.into();

        let class_decl = self.declare_collection(&class_name)?;

        let builder = self
            .define_function_with_category(
                class_name.append(&self.settings.collection.create_function_suffix),
                FunctionCategory::CollectionCreate,
            )?
            .doc("Creates an instance of the collection")?;

        let builder = if has_reserve {
            builder.param(
                "reserve_size",
                Primitive::U32,
                "preallocate a particular size",
            )?
        } else {
            builder
        };

        let create_func = builder
            .returns(class_decl.clone(), "Allocated opaque collection instance")?
            .build()?;

        let destroy_func = self
            .define_function_with_category(
                class_name.append(&self.settings.collection.destroy_function_suffix),
                FunctionCategory::CollectionDestroy,
            )?
            .doc("Destroys an instance of the collection")?
            .param("instance", class_decl.clone(), "instance to destroy")?
            .build()?;

        let add_func = self
            .define_function_with_category(
                class_name.append(&self.settings.collection.add_function_suffix),
                FunctionCategory::CollectionAdd,
            )?
            .doc("Add a value to the collection")?
            .param(
                "instance",
                class_decl.clone(),
                "instance to which to add the value",
            )?
            .param("value", value_type.clone(), "value to add to the instance")?
            .build()?;

        let collection = Handle::new(crate::model::collection::Collection::new(
            class_decl.inner,
            value_type,
            create_func,
            destroy_func,
            add_func,
            has_reserve,
        ));

        self.add_statement(Statement::CollectionDeclaration(collection.clone()))?;
        Ok(collection)
    }

    fn check_unique_symbol(&mut self, name: &Name) -> BindResult<()> {
        if self.symbol_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingErrorVariant::SymbolAlreadyUsed { name: name.clone() }.into())
        }
    }

    fn check_statement(&self, statement: &Statement<Unvalidated>) -> BindResult<()> {
        match statement {
            // no internals that can be from another library
            Statement::Constants(_) => Ok(()),
            Statement::StructDeclaration(_) => Ok(()),
            Statement::EnumDefinition(_) => Ok(()),
            Statement::ClassDeclaration(_) => Ok(()),
            // these types have internals that must be checked
            Statement::StructDefinition(x) => self.check_struct_declaration(&x.declaration()),
            Statement::ErrorType(x) => self.check_enum(&x.inner),
            Statement::ClassDefinition(x) => {
                self.check_class_declaration(&x.declaration)?;
                if let Some(x) = &x.constructor {
                    self.check_function(&x.function)?;
                }
                if let Some(x) = &x.destructor {
                    self.check_function(&x.function)?;
                }
                for x in x.static_methods.iter() {
                    self.check_function(&x.native_function)?
                }
                for x in x.methods.iter() {
                    self.check_function(&x.native_function)?
                }
                for x in x.future_methods.iter() {
                    self.check_function(&x.native_function)?
                }
                Ok(())
            }
            Statement::StaticClassDefinition(x) => {
                for x in x.static_methods.iter() {
                    self.check_function(&x.native_function)?;
                }
                Ok(())
            }
            Statement::InterfaceDefinition(x) => {
                for cb in x.untyped().callbacks.iter() {
                    for arg in cb.arguments.iter() {
                        self.check_callback_argument(&arg.arg_type)?;
                    }
                    if let Some(ret) = cb.return_type.get() {
                        self.check_callback_return_value(&ret.value)?;
                    }
                }
                Ok(())
            }
            Statement::IteratorDeclaration(x) => {
                self.check_class_declaration(&x.iter_class)?;
                self.check_function(&x.next_function)?;
                match &x.item_type {
                    IteratorItemType::Struct(x) => self.check_struct_declaration(&x.declaration()),
                    IteratorItemType::Primitive(_) => Ok(()),
                }
            }
            Statement::CollectionDeclaration(x) => {
                self.check_class_declaration(&x.collection_class)?;
                self.check_function(&x.create_func)?;
                self.check_function(&x.add_func)?;
                self.check_function(&x.delete_func)?;
                self.check_function_argument(&x.item_type)?;
                Ok(())
            }
            Statement::FunctionDefinition(x) => {
                for p in x.arguments.iter() {
                    self.check_function_argument(&p.arg_type)?;
                }
                if let Some(r) = x.return_type.get() {
                    self.check_function_return_type(&r.value)?;
                }
                if let Some(err) = x.error_type.get() {
                    self.check_enum(&err.inner)?;
                }
                Ok(())
            }
        }
    }

    fn check_struct_declaration(&self, native_struct: &StructDeclarationHandle) -> BindResult<()> {
        if self.fields.structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingErrorVariant::NotPartOfThisLibrary {
                name: native_struct.name.clone(),
            }
            .into())
        }
    }

    fn check_function(&self, native_function: &FunctionHandle) -> BindResult<()> {
        if self.fields.functions.contains(native_function) {
            Ok(())
        } else {
            Err(BindingErrorVariant::NotPartOfThisLibrary {
                name: native_function.name.clone(),
            }
            .into())
        }
    }

    fn check_enum(&self, native_enum: &Handle<Enum<Unvalidated>>) -> BindResult<()> {
        if self.fields.enums.contains(native_enum) {
            Ok(())
        } else {
            Err(BindingErrorVariant::NotPartOfThisLibrary {
                name: native_enum.name.clone(),
            }
            .into())
        }
    }

    fn check_interface(&self, interface: &InterfaceHandle) -> BindResult<()> {
        if self.fields.interfaces.contains(interface) {
            Ok(())
        } else {
            Err(BindingErrorVariant::NotPartOfThisLibrary {
                name: interface.name.clone(),
            }
            .into())
        }
    }

    fn check_class_declaration(
        &self,
        class_declaration: &ClassDeclarationHandle,
    ) -> BindResult<()> {
        if self.fields.class_declarations.contains(class_declaration) {
            Ok(())
        } else {
            Err(BindingErrorVariant::NotPartOfThisLibrary {
                name: class_declaration.name.clone(),
            }
            .into())
        }
    }

    fn check_function_argument(&self, arg: &FunctionArgument) -> BindResult<()> {
        match arg {
            FunctionArgument::Basic(x) => self.check_basic_type(x),
            FunctionArgument::String(_) => Ok(()),
            FunctionArgument::Collection(x) => self.check_collection(x),
            FunctionArgument::Struct(x) => self.check_struct_declaration(&x.declaration()),
            FunctionArgument::StructRef(x) => self.check_struct_declaration(&x.inner),
            FunctionArgument::ClassRef(x) => self.check_class_declaration(x),
            FunctionArgument::Interface(x) => self.check_interface(x),
        }
    }

    fn check_callback_argument(&self, arg: &CallbackArgument) -> BindResult<()> {
        match arg {
            CallbackArgument::Basic(x) => self.check_basic_type(x),
            CallbackArgument::String(_) => Ok(()),
            CallbackArgument::Iterator(x) => self.check_iterator(x),
            CallbackArgument::Class(x) => self.check_class_declaration(x),
            CallbackArgument::Struct(x) => self.check_struct_declaration(&x.declaration()),
        }
    }

    fn check_callback_return_value(&self, arg: &CallbackReturnValue) -> BindResult<()> {
        match arg {
            CallbackReturnValue::Basic(x) => self.check_basic_type(x),
            CallbackReturnValue::Struct(x) => self.check_struct_declaration(&x.declaration()),
        }
    }

    fn check_basic_type(&self, arg: &BasicType) -> BindResult<()> {
        match arg {
            BasicType::Primitive(_) => Ok(()),
            BasicType::Duration(_) => Ok(()),
            BasicType::Enum(x) => self.check_enum(x),
        }
    }

    fn check_function_return_type(&self, value: &FunctionReturnValue) -> BindResult<()> {
        match value {
            FunctionReturnValue::Basic(x) => self.check_basic_type(x),
            FunctionReturnValue::PrimitiveRef(_) => Ok(()),
            FunctionReturnValue::String(_) => Ok(()),
            FunctionReturnValue::ClassRef(x) => self.check_class_declaration(x),
            FunctionReturnValue::Struct(x) => self.check_struct_declaration(&x.declaration()),
            FunctionReturnValue::StructRef(x) => self.check_struct_declaration(x.untyped()),
        }
    }

    fn check_iterator(&self, iter: &AbstractIteratorHandle) -> BindResult<()> {
        if self.fields.iterators.contains(iter) {
            Ok(())
        } else {
            Err(BindingErrorVariant::NotPartOfThisLibrary {
                name: iter.iter_class.name.clone(),
            }
            .into())
        }
    }

    fn check_collection(&self, collection: &CollectionHandle) -> BindResult<()> {
        if self.fields.collections.contains(collection) {
            Ok(())
        } else {
            Err(BindingErrorVariant::NotPartOfThisLibrary {
                name: collection.collection_class.name.clone(),
            }
            .into())
        }
    }
}
