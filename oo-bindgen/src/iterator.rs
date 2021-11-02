use crate::name::Name;
use crate::structs::FunctionReturnStructField;
use crate::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct Iterator {
    /// underlying Rust iterator may have an associated lifetime annotation
    pub has_lifetime_annotation: bool,
    /// function used to retrieve the next value
    /// it takes the `iter_class` and returns a pointer to the `iter_type`
    pub next_function: FunctionHandle,
    /// opaque c struct type for the iterator
    pub iter_class: ClassDeclarationHandle,
    /// type of the value returned as a possibly null pointer
    pub item_type: UniversalOr<FunctionReturnStructField>,
    /// library settings
    pub settings: Rc<LibrarySettings>,
}

impl Iterator {
    pub(crate) fn new(
        has_lifetime_annotation: bool,
        iter_class: ClassDeclarationHandle,
        next_function: FunctionHandle,
        item_type: UniversalOr<FunctionReturnStructField>,
        settings: Rc<LibrarySettings>,
    ) -> Iterator {
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

pub type IteratorHandle = Handle<Iterator>;
