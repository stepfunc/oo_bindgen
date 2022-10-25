use crate::model::*;

#[derive(Debug)]
pub struct Collection<D>
where
    D: DocReference,
{
    pub(crate) collection_class: ClassDeclarationHandle,
    pub(crate) item_type: FunctionArgument,
    pub(crate) create_func: Handle<Function<D>>,
    pub(crate) delete_func: Handle<Function<D>>,
    pub(crate) add_func: Handle<Function<D>>,
    pub(crate) has_reserve: bool,
}

impl<D> Collection<D>
where
    D: DocReference,
{
    pub(crate) fn new(
        collection_class: ClassDeclarationHandle,
        item_type: FunctionArgument,
        create_func: Handle<Function<D>>,
        delete_func: Handle<Function<D>>,
        add_func: Handle<Function<D>>,
        has_reserve: bool,
    ) -> Collection<D> {
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

impl Collection<Unvalidated> {
    pub(crate) fn validate(
        &self,
        lib: &LibraryFields,
    ) -> BindResult<Handle<Collection<Validated>>> {
        Ok(Handle::new(Collection {
            collection_class: self.collection_class.clone(),
            item_type: self.item_type.clone(),
            create_func: self.create_func.validate(lib)?,
            delete_func: self.delete_func.validate(lib)?,
            add_func: self.add_func.validate(lib)?,
            has_reserve: self.has_reserve,
        }))
    }
}

pub type CollectionHandle = Handle<Collection<Unvalidated>>;
