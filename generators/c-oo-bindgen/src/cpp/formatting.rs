use oo_bindgen::class::{ClassDeclarationHandle, ClassHandle};
use oo_bindgen::formatting::{FormattingResult, Printer};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{Struct, StructDeclarationHandle, StructFieldType};

use heck::CamelCase;

pub(crate) fn mut_ref(expr: String) -> String {
    format!("{}&", expr)
}
pub(crate) fn const_ref(expr: String) -> String {
    format!("const {}&", expr)
}
pub(crate) fn unique_ptr(expr: String) -> String {
    format!("std::unique_ptr<{}>", expr)
}
pub(crate) fn pointer(expr: String) -> String {
    format!("{}*", expr)
}
pub(crate) fn std_move(expr: String) -> String {
    format!("std::move({})", expr)
}

pub(crate) fn namespace<F>(f: &mut dyn Printer, namespace: &str, cb: F) -> FormattingResult<()>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<()>,
{
    f.writeln(&format!("namespace {} {{", namespace))?;
    f.newline()?;
    cb(f)?;
    f.writeln(&format!("}} // end namespace {}", namespace))?;
    Ok(())
}

pub(crate) trait FriendClass {
    fn friend_class(&self) -> String;
}

impl FriendClass for ClassDeclarationHandle {
    fn friend_class(&self) -> String {
        format!("Cpp{}Friend", self.name.to_camel_case())
    }
}

impl FriendClass for StructDeclarationHandle {
    fn friend_class(&self) -> String {
        format!("Cpp{}Friend", self.name.to_camel_case())
    }
}

impl FriendClass for IteratorHandle {
    fn friend_class(&self) -> String {
        self.iter_type.friend_class()
    }
}

impl<T> FriendClass for Struct<T>
where
    T: StructFieldType,
{
    fn friend_class(&self) -> String {
        self.declaration.friend_class()
    }
}

impl FriendClass for ClassHandle {
    fn friend_class(&self) -> String {
        self.declaration.friend_class()
    }
}
