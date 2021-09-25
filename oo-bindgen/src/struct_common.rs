use crate::types::AllTypes;
use crate::Handle;

/// C-style structure forward declaration
#[derive(Debug)]
pub struct NativeStructDeclaration {
    pub name: String,
}

impl NativeStructDeclaration {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

pub type NativeStructDeclarationHandle = Handle<NativeStructDeclaration>;

impl From<NativeStructDeclarationHandle> for AllTypes {
    fn from(x: NativeStructDeclarationHandle) -> Self {
        Self::StructRef(x)
    }
}
