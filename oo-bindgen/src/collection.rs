use crate::types::{AnyType, BasicType};
use crate::BindResult;
use crate::*;

#[derive(Debug)]
pub struct Collection {
    pub create_func: FunctionHandle,
    pub delete_func: FunctionHandle,
    pub add_func: FunctionHandle,
    pub collection_type: ClassDeclarationHandle,
    pub item_type: AnyType,
    pub has_reserve: bool,
}

impl Collection {
    pub(crate) fn new(
        create_func: &FunctionHandle,
        delete_func: &FunctionHandle,
        add_func: &FunctionHandle,
    ) -> BindResult<Collection> {
        // Validate constructor
        let collection_type = if let ReturnType::Type(AnyType::ClassRef(collection_type), _) =
            &create_func.return_type
        {
            collection_type
        } else {
            return Err(BindingError::CollectionCreateFuncInvalidSignature {
                handle: create_func.clone(),
            });
        };

        if create_func.error_type.is_some() {
            return Err(BindingError::CollectionFunctionsCannotFail {
                handle: create_func.clone(),
            });
        }

        let mut iter = create_func.parameters.iter();
        let has_reserve = if let Some(param) = iter.next() {
            if param.arg_type != FArgument::Basic(BasicType::Uint32) {
                return Err(BindingError::CollectionCreateFuncInvalidSignature {
                    handle: create_func.clone(),
                });
            }

            if iter.next().is_some() {
                return Err(BindingError::CollectionCreateFuncInvalidSignature {
                    handle: create_func.clone(),
                });
            }

            true
        } else {
            false
        };

        // Validate destructor
        let mut iter = delete_func.parameters.iter();
        if let Some(param) = iter.next() {
            if let FArgument::ClassRef(iter_type) = &param.arg_type {
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
                });
            }
        } else {
            return Err(BindingError::CollectionDeleteFuncInvalidSignature {
                handle: delete_func.clone(),
            });
        }

        if delete_func.error_type.is_some() {
            return Err(BindingError::CollectionFunctionsCannotFail {
                handle: delete_func.clone(),
            });
        }

        // Validate add function
        let mut iter = add_func.parameters.iter();
        let item_type = if let Some(param) = iter.next() {
            if let FArgument::ClassRef(iter_type) = &param.arg_type {
                if iter_type != collection_type {
                    return Err(BindingError::CollectionAddFuncInvalidSignature {
                        handle: add_func.clone(),
                    });
                }

                let item_type = if let Some(item_type) = iter.next() {
                    item_type.arg_type.clone()
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
                });
            }
        } else {
            return Err(BindingError::CollectionAddFuncInvalidSignature {
                handle: add_func.clone(),
            });
        };

        if add_func.error_type.is_some() {
            return Err(BindingError::CollectionFunctionsCannotFail {
                handle: add_func.clone(),
            });
        }

        Ok(Collection {
            create_func: create_func.clone(),
            delete_func: delete_func.clone(),
            add_func: add_func.clone(),
            collection_type: collection_type.clone(),
            item_type: item_type.into(),
            has_reserve,
        })
    }

    pub fn name(&self) -> &str {
        &self.collection_type.name
    }
}

pub type CollectionHandle = Handle<Collection>;

impl From<CollectionHandle> for AnyType {
    fn from(x: CollectionHandle) -> Self {
        AnyType::Collection(x)
    }
}
