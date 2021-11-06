use crate::doc::{DocReference, Unvalidated, Validated};
use crate::name::Name;
use crate::structs::{
    FunctionReturnStructField, FunctionReturnStructHandle, StructDeclarationHandle,
    UniversalStructHandle,
};
use crate::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum IteratorItemType {
    Struct(UniversalOr<FunctionReturnStructField>),
}

impl From<UniversalOr<FunctionReturnStructField>> for IteratorItemType {
    fn from(x: UniversalOr<FunctionReturnStructField>) -> Self {
        IteratorItemType::Struct(x)
    }
}

impl From<UniversalStructHandle> for IteratorItemType {
    fn from(x: UniversalStructHandle) -> Self {
        Self::Struct(UniversalOr::Universal(x))
    }
}

impl From<FunctionReturnStructHandle> for IteratorItemType {
    fn from(x: FunctionReturnStructHandle) -> Self {
        Self::Struct(UniversalOr::Specific(x))
    }
}

impl IteratorItemType {
    pub fn name(&self) -> &Name {
        match self {
            IteratorItemType::Struct(x) => x.name(),
        }
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
        match self {
            IteratorItemType::Struct(x) => x.declaration(),
        }
    }

    pub(crate) fn get_function_return_value(&self) -> FunctionReturnValue {
        match self {
            IteratorItemType::Struct(x) => FunctionReturnValue::StructRef(x.typed_declaration()),
        }
    }
}

#[derive(Debug)]
pub struct Iterator<D>
where
    D: DocReference,
{
    /// underlying Rust iterator may have an associated lifetime annotation
    pub has_lifetime_annotation: bool,
    /// function used to retrieve the next value
    /// it takes the `iter_class` and returns a pointer to the `iter_type`
    pub next_function: Handle<Function<D>>,
    /// opaque c struct type for the iterator
    pub iter_class: ClassDeclarationHandle,
    /// type of the value returned as a possibly null pointer
    pub item_type: IteratorItemType,
    /// library settings
    pub settings: Rc<LibrarySettings>,
}

impl Iterator<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Iterator<Validated>> {
        Ok(Iterator {
            has_lifetime_annotation: self.has_lifetime_annotation,
            next_function: Handle::new(self.next_function.validate(lib)?),
            iter_class: self.iter_class.clone(),
            item_type: self.item_type.clone(),
            settings: self.settings.clone(),
        })
    }
}

impl<D> Iterator<D>
where
    D: DocReference,
{
    pub(crate) fn new(
        has_lifetime_annotation: bool,
        iter_class: ClassDeclarationHandle,
        next_function: Handle<Function<D>>,
        item_type: IteratorItemType,
        settings: Rc<LibrarySettings>,
    ) -> Iterator<D> {
        Iterator {
            has_lifetime_annotation,
            next_function,
            iter_class,
            item_type,
            settings,
        }
    }

    pub fn name(&self) -> &Name {
        &self.iter_class.name
    }
}

pub type IteratorHandle = Handle<Iterator<Unvalidated>>;
