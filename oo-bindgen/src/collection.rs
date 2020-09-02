use crate::Result;
use crate::*;

#[derive(Debug)]
pub struct Collection {
    pub create_func: NativeFunctionHandle,
    pub delete_func: NativeFunctionHandle,
    pub add_func: NativeFunctionHandle,
    pub collection_type: ClassDeclarationHandle,
    pub item_type: Type,
}

impl Collection {
    pub(crate) fn new(
        create_func: &NativeFunctionHandle,
        delete_func: &NativeFunctionHandle,
        add_func: &NativeFunctionHandle,
    ) -> Result<Collection> {
        // Validate constructor
        let collection_type = if let ReturnType::Type(Type::ClassRef(collection_type), _) = &create_func.return_type {
            collection_type
        } else {
            return Err(BindingError::CollectionCreateFuncInvalidSignature {
                handle: create_func.clone(),
            })
        };

        if !create_func.parameters.is_empty() {
            return Err(BindingError::CollectionCreateFuncInvalidSignature {
                handle: create_func.clone(),
            })
        }

        // Validate destructor
        let mut iter = delete_func.parameters.iter();
        if let Some(param) = iter.next() {
            if let Type::ClassRef(iter_type) = &param.param_type {
                if iter_type != collection_type {
                    return Err(BindingError::CollectionDeleteFuncInvalidSignature {
                        handle: delete_func.clone(),
                    });
                }

                if iter.next().is_some() {
                    return Err(BindingError::CollectionDeleteFuncInvalidSignature {
                        handle: delete_func.clone(),
                    });
                }
            } else {
                return Err(BindingError::CollectionDeleteFuncInvalidSignature {
                    handle: delete_func.clone(),
                })
            }
        } else {
            return Err(BindingError::CollectionDeleteFuncInvalidSignature {
                handle: delete_func.clone(),
            })
        }

        // Validate add function
        let mut iter = add_func.parameters.iter();
        let item_type = if let Some(param) = iter.next() {
            if let Type::ClassRef(iter_type) = &param.param_type {
                if iter_type != collection_type {
                    return Err(BindingError::CollectionAddFuncInvalidSignature {
                        handle: add_func.clone(),
                    });
                }

                let item_type = if let Some(item_type) = iter.next() {
                    item_type.param_type.clone()
                } else {
                    return Err(BindingError::CollectionAddFuncInvalidSignature {
                        handle: add_func.clone(),
                    });
                };

                if iter.next().is_some() {
                    return Err(BindingError::CollectionAddFuncInvalidSignature {
                        handle: add_func.clone(),
                    });
                }

                item_type
            } else {
                return Err(BindingError::CollectionAddFuncInvalidSignature {
                    handle: add_func.clone(),
                })
            }
        } else {
            return Err(BindingError::CollectionAddFuncInvalidSignature {
                handle: add_func.clone(),
            })
        };

        Ok(Collection {
            create_func: create_func.clone(),
            delete_func: delete_func.clone(),
            add_func: add_func.clone(),
            collection_type: collection_type.clone(),
            item_type: item_type.clone(),
        })
    }

    pub fn name(&self) -> &str {
        &self.collection_type.name
    }
}

pub type CollectionHandle = Handle<Collection>;
