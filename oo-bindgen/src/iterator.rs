use crate::structs::FunctionReturnStructField;
use crate::*;

#[derive(Debug)]
pub struct Iterator {
    pub has_lifetime_annotation: bool,
    pub function: FunctionHandle,
    pub iter_type: ClassDeclarationHandle,
    pub item_type: UniversalOr<FunctionReturnStructField>,
}

impl Iterator {
    pub(crate) fn new(
        has_lifetime_annotation: bool,
        function: &FunctionHandle,
        item_type: UniversalOr<FunctionReturnStructField>,
    ) -> BindResult<Iterator> {
        match &function.return_type {
            FunctionReturnType::Void => {
                return Err(BindingError::IteratorReturnTypeNotStructRef {
                    handle: function.clone(),
                })
            }
            FunctionReturnType::Type(return_type, _) => {
                if *return_type != FunctionReturnValue::StructRef(item_type.declaration()) {
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
            if let FunctionArgument::ClassRef(iter_type) = &param.arg_type {
                if iter.next().is_some() {
                    return Err(BindingError::IteratorNotSingleClassRefParam {
                        handle: function.clone(),
                    });
                }

                Ok(Iterator {
                    has_lifetime_annotation,
                    function: function.clone(),
                    iter_type: iter_type.clone(),
                    item_type,
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
