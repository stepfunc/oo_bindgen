use crate::name::Name;
use crate::*;

#[derive(Debug)]
pub struct Collection {
    pub collection_class: ClassDeclarationHandle,
    pub item_type: FunctionArgument,
    pub create_func: FunctionHandle,
    pub delete_func: FunctionHandle,
    pub add_func: FunctionHandle,
    pub has_reserve: bool,
}

impl Collection {
    pub(crate) fn new(
        collection_class: ClassDeclarationHandle,
        item_type: FunctionArgument,
        create_func: FunctionHandle,
        delete_func: FunctionHandle,
        add_func: FunctionHandle,
        has_reserve: bool,
    ) -> Collection {
        Collection {
            collection_class,
            item_type,
            create_func,
            delete_func,
            add_func,
            has_reserve,
        }
    }

    pub fn name(&self) -> &Name {
        &self.collection_class.name
    }
}

pub type CollectionHandle = Handle<Collection>;
