use crate::*;
use crate::Result;

/// C-style structure definition
#[derive(Debug)]
pub struct Iterator {
    pub native_func: NativeFunctionHandle,
    pub iter_type: ClassDeclarationHandle,
    pub item_type: NativeStructHandle,
}

impl Iterator {
    pub(crate) fn new(native_func: &NativeFunctionHandle, item_type: &NativeStructHandle) -> Result<Iterator> {
        if native_func.return_type != ReturnType::Type(Type::StructRef(item_type.declaration())) {
            return Err(BindingError::IteratorReturnTypeNotStructRef{handle: native_func.clone()});
        }

        let mut iter = native_func.parameters.iter();
        if let Some(param) = iter.next() {
            if let Type::ClassRef(iter_type) = &param.param_type {
                if iter.next().is_some() {
                    return Err(BindingError::IteratorNotSingleClassRefParam{handle: native_func.clone()});
                }

                Ok(Iterator {
                    native_func: native_func.clone(),
                    iter_type: iter_type.clone(),
                    item_type: item_type.clone(),
                })
            } else {
                Err(BindingError::IteratorNotSingleClassRefParam{handle: native_func.clone()})
            }
        } else {
            Err(BindingError::IteratorNotSingleClassRefParam{handle: native_func.clone()})
        }
    }

    pub fn name(&self) -> &str {
        &self.iter_type.name
    }
}

pub type IteratorHandle = Handle<Iterator>;
