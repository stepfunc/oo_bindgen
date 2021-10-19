use crate::structs::FunctionReturnStructHandle;
use crate::*;

#[derive(Debug)]
pub struct Iterator {
    pub has_lifetime_annotation: bool,
    pub function: FunctionHandle,
    pub iter_type: ClassDeclarationHandle,
    pub item_type: FunctionReturnStructHandle,
}

impl Iterator {
    pub(crate) fn new(
        has_lifetime_annotation: bool,
        function: &FunctionHandle,
        item_type: &FunctionReturnStructHandle,
    ) -> BindResult<Iterator> {
        match &function.return_type {
            FReturnType::Void => {
                return Err(BindingError::IteratorReturnTypeNotStructRef {
                    handle: function.clone(),
                })
            }
            FReturnType::Type(return_type, _) => {
                if *return_type != FReturnValue::StructRef(item_type.declaration.clone()) {
                    return Err(BindingError::IteratorReturnTypeNotStructRef {
                        handle: function.clone(),
                    });
                }
            }
        }

        if function.error_type.is_some() {
            return Err(BindingError::IteratorFunctionsCannotFail {
                handle: function.clone(),
            });
        }

        let mut iter = function.parameters.iter();
        if let Some(param) = iter.next() {
            if let FArgument::ClassRef(iter_type) = &param.arg_type {
                if iter.next().is_some() {
                    return Err(BindingError::IteratorNotSingleClassRefParam {
                        handle: function.clone(),
                    });
                }

                Ok(Iterator {
                    has_lifetime_annotation,
                    function: function.clone(),
                    iter_type: iter_type.clone(),
                    item_type: item_type.clone(),
                })
            } else {
                Err(BindingError::IteratorNotSingleClassRefParam {
                    handle: function.clone(),
                })
            }
        } else {
            Err(BindingError::IteratorNotSingleClassRefParam {
                handle: function.clone(),
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.iter_type.name
    }
}

pub type IteratorHandle = Handle<Iterator>;
