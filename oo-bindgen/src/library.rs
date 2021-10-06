use crate::class::*;
use crate::collection::CollectionHandle;
use crate::constants::{ConstantSetBuilder, ConstantSetHandle};
use crate::doc::Doc;
use crate::enum_type::{EnumBuilder, EnumHandle};
use crate::error_type::{ErrorType, ErrorTypeBuilder, ExceptionType};
use crate::function::{FunctionBuilder, FunctionHandle};
use crate::interface::{InterfaceBuilder, InterfaceHandle};
use crate::iterator::IteratorHandle;
use crate::structs::any_struct::{AnyStructBuilder, AnyStructField, AnyStructHandle};
use crate::structs::common::{StructDeclaration, StructDeclarationHandle, Visibility};
use crate::structs::function_struct::{FStructBuilder, FStructHandle};
use crate::types::{AnyType, BasicType};
use crate::*;
use crate::{BindingError, Version};

use crate::structs::callback_struct::{CStructBuilder, CStructHandle};
use crate::structs::function_return_struct::{RStructBuilder, RStructHandle};
use crate::structs::univeral_struct::{UStructBuilder, UStructHandle};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Symbol {
    Function(FunctionHandle),
    Struct(StructType),
    Enum(EnumHandle),
    Class(ClassHandle),
    StaticClass(StaticClassHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
}

#[derive(Clone, Debug)]
pub enum Statement {
    Constants(ConstantSetHandle),
    StructDeclaration(StructDeclarationHandle),
    StructDefinition(StructType),
    EnumDefinition(EnumHandle),
    ErrorType(ErrorType),
    ClassDeclaration(ClassDeclarationHandle),
    ClassDefinition(ClassHandle),
    StaticClassDefinition(StaticClassHandle),
    InterfaceDefinition(InterfaceHandle),
    IteratorDeclaration(IteratorHandle),
    CollectionDeclaration(CollectionHandle),
    FunctionDefinition(FunctionHandle),
}

impl Statement {
    pub(crate) fn unique_name(&self) -> Option<&str> {
        match self {
            Statement::Constants(x) => Some(x.name.as_str()),
            Statement::StructDeclaration(x) => Some(x.name.as_str()),
            Statement::StructDefinition(_) => {
                // the name is shared with the declaration
                None
            }
            Statement::EnumDefinition(x) => Some(x.name.as_str()),
            Statement::ErrorType(x) => Some(x.exception_name.as_str()),
            Statement::ClassDeclaration(x) => Some(x.name.as_str()),
            Statement::ClassDefinition(_) => {
                // the name is shared with the declaration
                None
            }
            Statement::StaticClassDefinition(x) => Some(x.name.as_str()),
            Statement::InterfaceDefinition(x) => Some(x.name.as_str()),
            Statement::IteratorDeclaration(_) => {
                // the name is derived in a language specific way
                None
            }
            Statement::CollectionDeclaration(_) => {
                // the name is derived in a language specific way
                None
            }
            Statement::FunctionDefinition(x) => Some(x.name.as_str()),
        }
    }
}

pub struct DeveloperInfo {
    /// Full name of the developer
    pub name: String,
    /// Email of the developer
    pub email: String,
    /// Name of the organization the developer is working for
    pub organization: String,
    /// Organization website URL
    pub organization_url: String,
}

pub struct LibraryInfo {
    /// Description of the library
    pub description: String,
    /// URL of the project
    pub project_url: String,
    /// GitHub organisation and repo name (e.g. stepfunc/oo_bindgen)
    pub repository: String,
    /// License name
    pub license_name: String,
    /// Short description of the license (to put on every generated file)
    pub license_description: Vec<String>,
    /// Path to the license file from the root directory
    pub license_path: PathBuf,
    /// List of developers
    pub developers: Vec<DeveloperInfo>,
}

pub struct Library {
    pub name: String,
    pub version: Version,
    pub c_ffi_prefix: String,
    pub info: LibraryInfo,
    statements: Vec<Statement>,
    structs: HashMap<StructDeclarationHandle, StructType>,
    _classes: HashMap<ClassDeclarationHandle, ClassHandle>,
    symbols: HashMap<String, Symbol>,
}

impl Library {
    pub fn statements(&self) -> impl Iterator<Item = &Statement> {
        self.statements.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &FunctionHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::FunctionDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn structs(&self) -> impl Iterator<Item = &StructType> {
        self.structs.values()
    }

    pub fn find_struct<T: AsRef<str>>(&self, name: T) -> Option<&StructType> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Struct(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn constants(&self) -> impl Iterator<Item = &ConstantSetHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::Constants(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &EnumHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::EnumDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn error_types(&self) -> impl Iterator<Item = &ErrorType> {
        self.statements().filter_map(|statement| match statement {
            Statement::ErrorType(err) => Some(err),
            _ => None,
        })
    }

    pub fn find_enum<T: AsRef<str>>(&self, name: T) -> Option<&EnumHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Enum(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn classes(&self) -> impl Iterator<Item = &ClassHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::ClassDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_class_declaration<T: AsRef<str>>(
        &self,
        name: T,
    ) -> Option<&ClassDeclarationHandle> {
        self.symbol(name).iter().find_map(|symbol| match symbol {
            Symbol::Class(handle) => Some(&handle.declaration),
            Symbol::Iterator(handle) => Some(&handle.iter_type),
            Symbol::Collection(handle) => Some(&handle.collection_type),
            _ => None,
        })
    }

    pub fn find_class<T: AsRef<str>>(&self, name: T) -> Option<&ClassHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Class(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn static_classes(&self) -> impl Iterator<Item = &StaticClassHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::StaticClassDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_static_class<T: AsRef<str>>(&self, name: T) -> Option<&StaticClassHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::StaticClass(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn interfaces(&self) -> impl Iterator<Item = &InterfaceHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::InterfaceDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_interface<T: AsRef<str>>(&self, name: T) -> Option<&InterfaceHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Interface(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn iterators(&self) -> impl Iterator<Item = &IteratorHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::IteratorDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_iterator<T: AsRef<str>>(&self, name: T) -> Option<&IteratorHandle> {
        self.statements.iter().find_map(|statement| {
            if let Statement::IteratorDeclaration(handle) = statement {
                if handle.name() == name.as_ref() {
                    return Some(handle);
                }
            }

            None
        })
    }

    pub fn collections(&self) -> impl Iterator<Item = &CollectionHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::CollectionDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_collection<T: AsRef<str>>(&self, name: T) -> Option<&CollectionHandle> {
        self.statements.iter().find_map(|statement| {
            if let Statement::CollectionDeclaration(handle) = statement {
                if handle.name() == name.as_ref() {
                    return Some(handle);
                }
            }

            None
        })
    }

    pub fn symbol<T: AsRef<str>>(&self, symbol_name: T) -> Option<&Symbol> {
        self.symbols.get(symbol_name.as_ref())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StructType {
    /// this will disappear when we only have specialized structs
    Any(AnyStructHandle),
    /// structs that may be used as native function parameters
    FStruct(FStructHandle, AnyStructHandle),
    /// structs than can be used as native function return values
    RStruct(RStructHandle, AnyStructHandle),
    /// structs that may be used as callback function arguments in interfaces
    CStruct(CStructHandle, AnyStructHandle),
    /// structs that can be used in any context and only contain basic types
    UStruct(UStructHandle, AnyStructHandle),
}

impl From<AnyStructHandle> for StructType {
    fn from(x: AnyStructHandle) -> Self {
        StructType::Any(x)
    }
}

impl From<FStructHandle> for StructType {
    fn from(x: FStructHandle) -> Self {
        StructType::FStruct(x.clone(), x.to_any_struct())
    }
}

impl StructType {
    pub fn get_any_struct(&self) -> &AnyStructHandle {
        match self {
            StructType::Any(x) => x,
            StructType::FStruct(_, x) => x,
            StructType::CStruct(_, x) => x,
            StructType::RStruct(_, x) => x,
            StructType::UStruct(_, x) => x,
        }
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
        match self {
            StructType::Any(x) => x.declaration.clone(),
            StructType::FStruct(_, x) => x.declaration.clone(),
            StructType::CStruct(_, x) => x.declaration.clone(),
            StructType::RStruct(_, x) => x.declaration.clone(),
            StructType::UStruct(_, x) => x.declaration.clone(),
        }
    }

    pub fn doc(&self) -> &Doc {
        match self {
            StructType::Any(x) => &x.doc,
            StructType::FStruct(_, x) => &x.doc,
            StructType::CStruct(_, x) => &x.doc,
            StructType::RStruct(_, x) => &x.doc,
            StructType::UStruct(_, x) => &x.doc,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            StructType::Any(x) => x.name(),
            StructType::FStruct(_, x) => x.name(),
            StructType::CStruct(_, x) => x.name(),
            StructType::RStruct(_, x) => x.name(),
            StructType::UStruct(_, x) => x.name(),
        }
    }

    pub fn visibility(&self) -> Visibility {
        match self {
            StructType::Any(x) => x.visibility,
            StructType::FStruct(_, x) => x.visibility,
            StructType::CStruct(_, x) => x.visibility,
            StructType::RStruct(_, x) => x.visibility,
            StructType::UStruct(_, x) => x.visibility,
        }
    }

    pub fn fields(&self) -> impl Iterator<Item = &AnyStructField> {
        match self {
            StructType::Any(x) => x.fields.iter(),
            StructType::FStruct(_, x) => x.fields.iter(),
            StructType::CStruct(_, x) => x.fields.iter(),
            StructType::RStruct(_, x) => x.fields.iter(),
            StructType::UStruct(_, x) => x.fields.iter(),
        }
    }

    pub fn all_fields_have_defaults(&self) -> bool {
        match self {
            StructType::Any(x) => x.all_fields_have_defaults(),
            StructType::FStruct(_, x) => x.all_fields_have_defaults(),
            StructType::CStruct(_, x) => x.all_fields_have_defaults(),
            StructType::RStruct(_, x) => x.all_fields_have_defaults(),
            StructType::UStruct(_, x) => x.all_fields_have_defaults(),
        }
    }

    pub fn find_field<T: AsRef<str>>(&self, field_name: T) -> Option<&AnyStructField> {
        self.fields().find(|f| *f.name == field_name.as_ref())
    }
}

pub struct LibraryBuilder {
    name: String,
    version: Version,
    c_ffi_prefix: Option<String>,
    info: LibraryInfo,

    // a record of statements preserved in order
    statements: Vec<Statement>,
    // names of symbols used in the library
    symbol_names: HashSet<String>,

    // native stuff
    structs_declarations: HashSet<StructDeclarationHandle>,
    structs: HashMap<StructDeclarationHandle, StructType>,
    functions: HashSet<FunctionHandle>,
    enums: HashSet<EnumHandle>,

    // oo stuff
    class_declarations: HashSet<ClassDeclarationHandle>,
    classes: HashMap<ClassDeclarationHandle, ClassHandle>,
    static_classes: HashSet<StaticClassHandle>,
    interfaces: HashSet<InterfaceHandle>,

    // specialized types
    iterators: HashSet<IteratorHandle>,
    collections: HashSet<CollectionHandle>,
}

impl LibraryBuilder {
    pub fn new<T: Into<String>>(name: T, version: Version, info: LibraryInfo) -> Self {
        Self {
            name: name.into(),
            version,
            c_ffi_prefix: None,
            info,

            statements: Vec::new(),
            symbol_names: HashSet::new(),

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

    pub(crate) fn add_statement(&mut self, statement: Statement) -> BindResult<()> {
        if let Some(name) = statement.unique_name() {
            self.check_unique_symbol(name)?;
        }

        self.statements.push(statement.clone());

        match statement {
            Statement::Constants(_) => {}
            Statement::StructDeclaration(x) => {
                self.structs_declarations.insert(x);
            }
            Statement::StructDefinition(x) => {
                self.structs.insert(x.declaration(), x);
            }
            Statement::EnumDefinition(x) => {
                self.enums.insert(x);
            }
            Statement::ErrorType(_) => {}
            Statement::ClassDeclaration(x) => {
                self.class_declarations.insert(x);
            }
            Statement::ClassDefinition(x) => {
                self.classes.insert(x.declaration.clone(), x);
            }
            Statement::StaticClassDefinition(x) => {
                self.static_classes.insert(x);
            }
            Statement::InterfaceDefinition(x) => {
                self.interfaces.insert(x);
            }
            Statement::IteratorDeclaration(x) => {
                self.iterators.insert(x);
            }
            Statement::CollectionDeclaration(x) => {
                self.collections.insert(x);
            }
            Statement::FunctionDefinition(x) => {
                self.functions.insert(x);
            }
        }

        Ok(())
    }

    pub fn build(self) -> BindResult<Library> {
        // Build symbols map
        let mut symbols = HashMap::new();
        for statement in &self.statements {
            match statement {
                Statement::Constants(_) => {}
                Statement::StructDeclaration(handle) => {
                    symbols.insert(
                        handle.name.clone(),
                        Symbol::Struct(self.structs.get(handle).unwrap().clone()),
                    );
                }
                Statement::StructDefinition(_) => (),
                Statement::EnumDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::Enum(handle.clone()));
                }
                Statement::ErrorType(_) => {}
                Statement::ClassDeclaration(_) => (),
                Statement::ClassDefinition(handle) => {
                    symbols.insert(handle.name().to_string(), Symbol::Class(handle.clone()));
                }
                Statement::StaticClassDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::StaticClass(handle.clone()));
                }
                Statement::InterfaceDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::Interface(handle.clone()));
                }
                Statement::IteratorDeclaration(handle) => {
                    symbols.insert(handle.name().to_string(), Symbol::Iterator(handle.clone()));
                }
                Statement::CollectionDeclaration(handle) => {
                    symbols.insert(
                        handle.name().to_string(),
                        Symbol::Collection(handle.clone()),
                    );
                }
                Statement::FunctionDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::Function(handle.clone()));
                }
            }
        }

        let lib = Library {
            name: self.name.clone(),
            version: self.version,
            c_ffi_prefix: self.c_ffi_prefix.unwrap_or(self.name),
            info: self.info,
            statements: self.statements,
            structs: self.structs,
            _classes: self.classes,
            symbols,
        };

        crate::doc::validate_library_docs(&lib)?;

        Ok(lib)
    }

    pub fn c_ffi_prefix<T: Into<String>>(&mut self, c_ffi_prefix: T) -> BindResult<()> {
        match self.c_ffi_prefix {
            Some(_) => Err(BindingError::FfiPrefixAlreadySet),
            None => {
                self.c_ffi_prefix = Some(c_ffi_prefix.into());
                Ok(())
            }
        }
    }

    pub fn define_error_type<T: Into<String>>(
        &mut self,
        error_name: T,
        exception_name: T,
        exception_type: ExceptionType,
    ) -> BindResult<ErrorTypeBuilder> {
        let builder = self
            .define_enum(error_name)
            .push("Ok", "Success, i.e. no error occurred")?;

        Ok(ErrorTypeBuilder::new(
            exception_name.into(),
            exception_type,
            builder,
        ))
    }

    pub fn define_constants<T: Into<String>>(&mut self, name: T) -> ConstantSetBuilder {
        ConstantSetBuilder::new(self, name.into())
    }

    /// Forward declare a struct
    pub fn declare_struct<T: Into<String>>(
        &mut self,
        name: T,
    ) -> BindResult<StructDeclarationHandle> {
        let name = name.into();
        let handle = StructDeclarationHandle::new(StructDeclaration::new(name));
        self.add_statement(Statement::StructDeclaration(handle.clone()))?;
        Ok(handle)
    }

    /// Define ANY native structure - TODO - this will be removed in favor of specialized struct types
    pub fn define_any_struct(
        &mut self,
        declaration: &StructDeclarationHandle,
    ) -> BindResult<AnyStructBuilder> {
        self.validate_struct_declaration(declaration)?;
        if !self.structs.contains_key(declaration) {
            Ok(AnyStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    /// Define a structure that can be used in any context
    pub fn define_ustruct(
        &mut self,
        declaration: &StructDeclarationHandle,
    ) -> BindResult<UStructBuilder> {
        self.validate_struct_declaration(declaration)?;
        if !self.structs.contains_key(declaration) {
            Ok(UStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    /// Define a structure that can be used in callback function arguments
    pub fn define_cstruct(
        &mut self,
        declaration: &StructDeclarationHandle,
    ) -> BindResult<CStructBuilder> {
        self.validate_struct_declaration(declaration)?;
        if !self.structs.contains_key(declaration) {
            Ok(CStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    /// Define a structure that can be used in callback function arguments
    pub fn define_rstruct(
        &mut self,
        declaration: &StructDeclarationHandle,
    ) -> BindResult<RStructBuilder> {
        self.validate_struct_declaration(declaration)?;
        if !self.structs.contains_key(declaration) {
            Ok(RStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    /// Define a structure that can be used in native function arguments
    pub fn define_fstruct(
        &mut self,
        declaration: &StructDeclarationHandle,
    ) -> BindResult<FStructBuilder> {
        self.validate_struct_declaration(declaration)?;
        if !self.structs.contains_key(declaration) {
            Ok(FStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    /// Define an enumeration
    pub fn define_enum<T: Into<String>>(&mut self, name: T) -> EnumBuilder {
        EnumBuilder::new(self, name.into())
    }

    pub fn define_function<T: Into<String>>(&mut self, name: T) -> FunctionBuilder {
        FunctionBuilder::new(self, name.into())
    }

    pub fn declare_class<T: Into<String>>(
        &mut self,
        name: T,
    ) -> BindResult<ClassDeclarationHandle> {
        let handle = ClassDeclarationHandle::new(ClassDeclaration::new(name.into()));
        self.add_statement(Statement::ClassDeclaration(handle.clone()))?;
        Ok(handle)
    }

    pub fn define_class(
        &mut self,
        declaration: &ClassDeclarationHandle,
    ) -> BindResult<ClassBuilder> {
        self.validate_class_declaration(declaration)?;
        if !self.classes.contains_key(declaration) {
            Ok(ClassBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::ClassAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    pub fn define_static_class<T: Into<String>>(&mut self, name: T) -> StaticClassBuilder {
        StaticClassBuilder::new(self, name.into())
    }

    pub fn define_interface<T: Into<String>, D: Into<Doc>>(
        &mut self,
        name: T,
        doc: D,
    ) -> InterfaceBuilder {
        InterfaceBuilder::new(self, name.into(), doc.into())
    }

    pub fn define_iterator(
        &mut self,
        native_func: &FunctionHandle,
        item_type: &RStructHandle,
    ) -> BindResult<IteratorHandle> {
        self.define_iterator_impl(false, native_func, item_type)
    }

    pub fn define_iterator_with_lifetime(
        &mut self,
        native_func: &FunctionHandle,
        item_type: &RStructHandle,
    ) -> BindResult<IteratorHandle> {
        self.define_iterator_impl(true, native_func, item_type)
    }

    fn define_iterator_impl(
        &mut self,
        has_lifetime: bool,
        native_func: &FunctionHandle,
        item_type: &RStructHandle,
    ) -> BindResult<IteratorHandle> {
        let iter = IteratorHandle::new(crate::iterator::Iterator::new(
            has_lifetime,
            native_func,
            item_type,
        )?);
        self.add_statement(Statement::IteratorDeclaration(iter.clone()))?;
        Ok(iter)
    }

    pub fn define_collection(
        &mut self,
        create_func: &FunctionHandle,
        delete_func: &FunctionHandle,
        add_func: &FunctionHandle,
    ) -> BindResult<CollectionHandle> {
        let collection = CollectionHandle::new(crate::collection::Collection::new(
            create_func,
            delete_func,
            add_func,
        )?);
        self.add_statement(Statement::CollectionDeclaration(collection.clone()))?;
        Ok(collection)
    }

    fn check_unique_symbol(&mut self, name: &str) -> BindResult<()> {
        if self.symbol_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::SymbolAlreadyUsed {
                name: name.to_string(),
            })
        }
    }

    pub(crate) fn validate_function(&self, native_function: &FunctionHandle) -> BindResult<()> {
        if self.functions.contains(native_function) {
            Ok(())
        } else {
            Err(BindingError::FunctionNotPartOfThisLib {
                handle: native_function.clone(),
            })
        }
    }

    pub(crate) fn validate_type(&self, type_to_validate: &AnyType) -> BindResult<()> {
        match type_to_validate {
            AnyType::StructRef(native_struct) => self.validate_struct_declaration(native_struct),
            AnyType::Struct(native_struct) => {
                self.validate_struct(&StructType::Any(native_struct.clone()))
            }
            AnyType::Basic(BasicType::Enum(native_enum)) => self.validate_enum(native_enum),
            AnyType::Interface(interface) => self.validate_interface(interface),
            AnyType::ClassRef(class_declaration) => {
                self.validate_class_declaration(class_declaration)
            }
            AnyType::Iterator(iter) => self.validate_iterator(iter),
            AnyType::Collection(collection) => self.validate_collection(collection),
            _ => Ok(()),
        }
    }

    fn validate_struct_declaration(
        &self,
        native_struct: &StructDeclarationHandle,
    ) -> BindResult<()> {
        if self.structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingError::StructNotPartOfThisLib {
                handle: native_struct.clone(),
            })
        }
    }

    fn validate_struct(&self, native_struct: &StructType) -> BindResult<()> {
        if self.structs.contains_key(&native_struct.declaration()) {
            Ok(())
        } else {
            Err(BindingError::StructNotPartOfThisLib {
                handle: native_struct.declaration(),
            })
        }
    }

    fn validate_enum(&self, native_enum: &EnumHandle) -> BindResult<()> {
        if self.enums.contains(native_enum) {
            Ok(())
        } else {
            Err(BindingError::EnumNotPartOfThisLib {
                handle: native_enum.clone(),
            })
        }
    }

    fn validate_interface(&self, interface: &InterfaceHandle) -> BindResult<()> {
        if self.interfaces.contains(interface) {
            Ok(())
        } else {
            Err(BindingError::InterfaceNotPartOfThisLib {
                handle: interface.clone(),
            })
        }
    }

    fn validate_class_declaration(
        &self,
        class_declaration: &ClassDeclarationHandle,
    ) -> BindResult<()> {
        if self.class_declarations.contains(class_declaration) {
            Ok(())
        } else {
            Err(BindingError::ClassNotPartOfThisLib {
                handle: class_declaration.clone(),
            })
        }
    }

    fn validate_iterator(&self, iter: &IteratorHandle) -> BindResult<()> {
        if self.iterators.contains(iter) {
            Ok(())
        } else {
            Err(BindingError::IteratorNotPartOfThisLib {
                handle: iter.clone(),
            })
        }
    }

    fn validate_collection(&self, collection: &CollectionHandle) -> BindResult<()> {
        if self.collections.contains(collection) {
            Ok(())
        } else {
            Err(BindingError::CollectionNotPartOfThisLib {
                handle: collection.clone(),
            })
        }
    }
}
