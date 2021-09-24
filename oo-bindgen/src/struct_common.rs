use crate::Handle;
use crate::native_function::Type;

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

impl From<NativeStructDeclarationHandle> for Type {
    fn from(x: NativeStructDeclarationHandle) -> Self {
        Self::StructRef(x)
    }
}