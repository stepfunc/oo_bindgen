use crate::class::*;
use crate::native_enum::*;
use crate::native_function::*;
use crate::native_struct::*;
use thiserror::Error;
use semver::Version;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

pub mod class;
pub mod formatting;
pub mod platforms;
pub mod native_enum;
pub mod native_function;
pub mod native_struct;

type Result<T> = std::result::Result<T, BindingError>;

#[derive(Error, Debug)]
pub enum BindingError {
    // Global errors
    #[error("Symbol '{}' already used in the library", name)]
    SymbolAlreadyUsed{name: String},
    #[error("Description already set")]
    DescriptionAlreadySet,

    // Native function errors
    #[error("Native struct '{}' is not part of this library", handle.name)]
    NativeFunctionNotPartOfThisLib{handle: NativeFunctionHandle},
    #[error("Return type of native function '{}' was already defined to '{:?}'", native_func_name, return_type)]
    ReturnTypeAlreadyDefined{
        native_func_name: String,
        return_type: ReturnType,
    },
    #[error("Return type of native function '{}' was not defined", native_func_name)]
    ReturnTypeNotDefined{native_func_name: String},

    // Native enum errors
    #[error("Native enum '{}' is not part of this library", handle.name)]
    NativeEnumNotPartOfThisLib{
        handle: NativeEnumHandle,
    },
    #[error("Native enum '{}' already contains a variant with name '{}'", name, variant_name)]
    NativeEnumAlreadyContainsVariantWithSameName{
        name: String,
        variant_name: String,
    },

    // Structure errors
    #[error("Native struct '{}' was already defined", handle.name)]
    NativeStructAlreadyDefined{handle: NativeStructDeclarationHandle},
    #[error("Native struct '{}' is not part of this library", handle.name)]
    NativeStructNotPartOfThisLib{handle: NativeStructDeclarationHandle},
    #[error("Native struct '{}' already contains element with name '{}'", handle.name, element_name)]
    NativeStructAlreadyContainsElementWithSameName{
        handle: NativeStructDeclarationHandle,
        element_name: String,
    },

    // Class errors
    #[error("Class '{}' was already defined", handle.name)]
    ClassAlreadyDefined{handle: ClassDeclarationHandle},
    #[error("Class '{}' is not part of this library", handle.name)]
    ClassNotPartOfThisLib{handle: ClassDeclarationHandle},
    #[error("First parameter of native function '{}' is not of type '{}' as expected for a method", native_func.name, handle.name)]
    FirstMethodParameterIsNotProperType{
        handle: ClassDeclarationHandle,
        native_func: NativeFunctionHandle,
    },
    #[error("Constructor for class '{}' was already defined", handle.name)]
    ConstructorAlreadyDefined{handle: ClassDeclarationHandle},
    #[error("Native function '{}' does not return '{}' as expected for a constructor", native_func.name, handle.name)]
    ConstructorReturnTypeDoesNotMatch{
        handle: ClassDeclarationHandle,
        native_func: NativeFunctionHandle,
    },
    #[error("Destructor for class '{}' was already defined", handle.name)]
    DestructorAlreadyDefined{handle: ClassDeclarationHandle},
    #[error("Native function '{}' does not take a single '{}' parameter as expected for a destructor", native_func.name, handle.name)]
    DestructorTakesMoreThanOneParameter{
        handle: ClassDeclarationHandle,
        native_func: NativeFunctionHandle,
    }
}

pub struct Handle<T>(Rc<T>);

impl<T> Handle<T> {
    fn new(inner: T) -> Self {
        Self(Rc::new(inner))
    }
}

impl<T: Debug> Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&*self.0, state)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug)]
pub enum Statement {
    StructDeclaration(NativeStructDeclarationHandle),
    StructDefinition(NativeStructHandle),
    EnumDefinition(NativeEnumHandle),
    ClassDeclaration(ClassDeclarationHandle),
    ClassDefinition(ClassHandle),
    NativeFunctionDeclaration(NativeFunctionHandle),
}

pub struct Library {
    pub name: String,
    pub version: Version,
    pub description: Option<String>,
    pub license: Vec<String>,
    statements: Vec<Statement>,
    native_structs: HashMap<NativeStructDeclarationHandle, NativeStructHandle>,
    classes: HashMap<ClassDeclarationHandle, ClassHandle>,
}

impl Library {
    pub fn native_functions(&self) -> NativeFunctionIterator {
        NativeFunctionIterator {
            iter: self.into_iter()
        }
    }

    pub fn native_structs(&self) -> NativeStructIterator {
        NativeStructIterator {
            iter: self.into_iter()
        }
    }

    pub fn native_enums(&self) -> NativeEnumIterator {
        NativeEnumIterator {
            iter: self.into_iter()
        }
    }

    pub fn classes(&self) -> ClassIterator {
        ClassIterator {
            iter: self.into_iter()
        }
    }
}

impl<'a> IntoIterator for &'a Library {
    type Item = &'a Statement;
    type IntoIter = std::slice::Iter<'a, Statement>;
    fn into_iter(self) -> Self::IntoIter {
        self.statements.iter()
    }
}

pub struct NativeFunctionIterator<'a> {
    iter: std::slice::Iter<'a, Statement>,
}

impl<'a> Iterator for NativeFunctionIterator<'a> {
    type Item = &'a NativeFunctionHandle;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(statement) = self.iter.next() {
            match statement {
                Statement::NativeFunctionDeclaration(handle) => return Some(handle),
                _ => (),
            }
        }

        None
    }
}

pub struct NativeStructIterator<'a> {
    iter: std::slice::Iter<'a, Statement>,
}

impl<'a> Iterator for NativeStructIterator<'a> {
    type Item = &'a NativeStructHandle;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(statement) = self.iter.next() {
            match statement {
                Statement::StructDefinition(handle) => return Some(handle),
                _ => (),
            }
        }

        None
    }
}

pub struct NativeEnumIterator<'a> {
    iter: std::slice::Iter<'a, Statement>,
}

impl<'a> Iterator for NativeEnumIterator<'a> {
    type Item = &'a NativeEnumHandle;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(statement) = self.iter.next() {
            match statement {
                Statement::EnumDefinition(handle) => return Some(handle),
                _ => (),
            }
        }

        None
    }
}

pub struct ClassIterator<'a> {
    iter: std::slice::Iter<'a, Statement>,
}

impl<'a> Iterator for ClassIterator<'a> {
    type Item = &'a ClassHandle;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(statement) = self.iter.next() {
            match statement {
                Statement::ClassDefinition(handle) => return Some(handle),
                _ => (),
            }
        }

        None
    }
}

pub struct LibraryBuilder {
    name: String,
    version: Version,
    description: Option<String>,
    license: Vec<String>,

    statements: Vec<Statement>,
    symbol_names: HashSet<String>,

    native_structs_declarations: HashSet<NativeStructDeclarationHandle>,
    native_structs: HashMap<NativeStructDeclarationHandle, NativeStructHandle>,

    native_enums: HashSet<NativeEnumHandle>,

    class_declarations: HashSet<ClassDeclarationHandle>,
    classes: HashMap<ClassDeclarationHandle, ClassHandle>,

    native_functions: HashSet<NativeFunctionHandle>,
}

impl LibraryBuilder {
    pub fn new(name: &str, version: Version) -> Self {
        Self {
            name: name.to_string(),
            version: version,
            description: None,
            license: Vec::new(),

            statements: Vec::new(),
            symbol_names: HashSet::new(),

            native_structs_declarations: HashSet::new(),
            native_structs: HashMap::new(),

            native_enums: HashSet::new(),

            class_declarations: HashSet::new(),
            classes: HashMap::new(),

            native_functions: HashSet::new(),
        }
    }

    pub fn build(self) -> Library {
        Library{
            name: self.name,
            version: self.version,
            description: self.description,
            license: self.license,

            statements: self.statements,
            native_structs: self.native_structs,
            classes: self.classes,
        }
    }

    pub fn description(&mut self, description: &str) -> Result<()> {
        match self.description {
            Some(_) => Err(BindingError::DescriptionAlreadySet),
            None => {
                self.description = Some(description.to_string());
                Ok(())
            }
        }
    }

    pub fn license(&mut self, license: Vec<String>) -> Result<()> {
        self.license = license;
        Ok(())
    }

    /// Forward declare a native structure
    pub fn declare_native_struct(&mut self, name: &str) -> Result<NativeStructDeclarationHandle> {
        self.check_unique_symbol(name)?;
        let handle = NativeStructDeclarationHandle::new(NativeStructDeclaration::new(name.to_string()));
        self.native_structs_declarations.insert(handle.clone());
        self.statements.push(Statement::StructDeclaration(handle.clone()));
        Ok(handle)
    }

    /// Define a native structure
    pub fn define_native_struct(&mut self, declaration: &NativeStructDeclarationHandle) -> Result<NativeStructBuilder> {
        self.validate_native_struct_declaration(declaration)?;
        if !self.native_structs.contains_key(&declaration) {
            Ok(NativeStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::NativeStructAlreadyDefined{handle: declaration.clone()})
        }
    }

    /// Define an enumeration
    pub fn define_native_enum(&mut self, name: &str) -> Result<NativeEnumBuilder> {
        self.check_unique_symbol(name)?;
        Ok(NativeEnumBuilder::new(self, name.to_string()))
    }

    pub fn declare_native_function(&mut self, name: &str) -> Result<NativeFunctionBuilder> {
        self.check_unique_symbol(name)?;
        Ok(NativeFunctionBuilder::new(self, name.to_string()))
    }

    pub fn declare_class(&mut self, name: &str) -> Result<ClassDeclarationHandle> {
        self.check_unique_symbol(name)?;
        let handle = ClassDeclarationHandle::new(ClassDeclaration::new(name.to_string()));
        self.class_declarations.insert(handle.clone());
        self.statements.push(Statement::ClassDeclaration(handle.clone()));
        Ok(handle)
    }

    pub fn define_class(&mut self, declaration: &ClassDeclarationHandle) -> Result<ClassBuilder> {
        self.validate_class_declaration(declaration)?;
        if !self.classes.contains_key(&declaration) {
            Ok(ClassBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::ClassAlreadyDefined{handle: declaration.clone()})
        }
    }

    fn check_unique_symbol(&mut self, name: &str) -> Result<()> {
        if self.symbol_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::SymbolAlreadyUsed{name: name.to_string()})
        }
    }

    fn validate_native_function(&self, native_function: &NativeFunctionHandle) -> Result<()> {
        if self.native_functions.contains(native_function) {
            Ok(())
        } else {
            Err(BindingError::NativeFunctionNotPartOfThisLib{handle: native_function.clone()})
        }
    }

    fn validate_type(&self, type_to_validate: &Type) -> Result<()> {
        match type_to_validate {
            Type::StructRef(native_struct) => self.validate_native_struct_declaration(native_struct),
            Type::Struct(native_struct) => self.validate_native_struct(&native_struct),
            Type::Enum(native_enum) => self.validate_native_enum(&native_enum),
            Type::ClassRef(class_declaration) => self.validate_class_declaration(class_declaration),
            _ => Ok(())
        }
    }

    fn validate_native_struct_declaration(&self, native_struct: &NativeStructDeclarationHandle) -> Result<()> {
        if self.native_structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingError::NativeStructNotPartOfThisLib{handle: native_struct.clone()})
        }
    }

    fn validate_native_struct(&self, native_struct: &NativeStructHandle) -> Result<()> {
        if self.native_structs.contains_key(&native_struct.declaration) {
            Ok(())
        } else {
            Err(BindingError::NativeStructNotPartOfThisLib{handle: native_struct.declaration.clone()})
        }
    }

    fn validate_native_enum(&self, native_enum: &NativeEnumHandle) -> Result<()> {
        if self.native_enums.contains(native_enum) {
            Ok(())
        } else {
            Err(BindingError::NativeEnumNotPartOfThisLib{handle: native_enum.clone()})
        }
    }

    fn validate_class_declaration(&self, class_declaration: &ClassDeclarationHandle) -> Result<()> {
        if self.class_declarations.contains(class_declaration) {
            Ok(())
        } else {
            Err(BindingError::ClassNotPartOfThisLib{handle: class_declaration.clone()})
        }
    }
}
