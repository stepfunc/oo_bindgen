use crate::types::AnyType;
use crate::Handle;

/// struct type affects the type of code the backend will generate
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visibility {
    /// struct members are public
    Public,
    /// struct members are private (except C of course), and the struct is just an opaque "token"
    Private,
}

/// C-style structure forward declaration
#[derive(Debug)]
pub struct StructDeclaration {
    pub name: String,
}

impl StructDeclaration {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

pub type StructDeclarationHandle = Handle<StructDeclaration>;

impl From<StructDeclarationHandle> for AnyType {
    fn from(x: StructDeclarationHandle) -> Self {
        Self::StructRef(x)
    }
}
