use crate::types::AnyType;
use crate::Result;
use crate::*;

#[derive(Debug)]
pub struct Iterator {
    pub has_lifetime_annotation: bool,
    pub native_func: FunctionHandle,
    pub iter_type: ClassDeclarationHandle,
    pub item_type: AnyStructHandle,
}

impl Iterator {
    pub(crate) fn new(
        has_lifetime_annotation: bool,
        native_func: &FunctionHandle,
        item_type: &AnyStructHandle,
    ) -> Result<Iterator> {
        match &native_func.return_type {
            ReturnType::Void => {
                return Err(BindingError::IteratorReturnTypeNotStructRef {
                    handle: native_func.clone(),
                })
            }
            ReturnType::Type(return_type, _) => {
                if *return_type != AnyType::StructRef(item_type.declaration()) {
                    return Err(BindingError::IteratorReturnTypeNotStructRef {
                        handle: native_func.clone(),
                    });
                }
            }
        }

        if native_func.error_type.is_some() {
            return Err(BindingError::IteratorFunctionsCannotFail {
                handle: native_func.clone(),
            });
        }

        let mut iter = native_func.parameters.iter();
        if let Some(param) = iter.next() {
            if let FArgument::ClassRef(iter_type) = &param.arg_type {
                if iter.next().is_some() {
                    return Err(BindingError::IteratorNotSingleClassRefParam {
                        handle: native_func.clone(),
                    });
                }

                Ok(Iterator {
                    has_lifetime_annotation,
                    native_func: native_func.clone(),
                    iter_type: iter_type.clone(),
                    item_type: item_type.clone(),
                })
            } else {
                Err(BindingError::IteratorNotSingleClassRefParam {
                    handle: native_func.clone(),
                })
            }
        } else {
            Err(BindingError::IteratorNotSingleClassRefParam {
                handle: native_func.clone(),
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.iter_type.name
    }
}

pub type IteratorHandle = Handle<Iterator>;

impl From<IteratorHandle> for AnyType {
    fn from(x: IteratorHandle) -> Self {
        Self::Iterator(x)
    }
}

impl From<IteratorHandle> for AnyStructFieldType {
    fn from(x: IteratorHandle) -> Self {
        AnyStructFieldType::Iterator(x)
    }
}
