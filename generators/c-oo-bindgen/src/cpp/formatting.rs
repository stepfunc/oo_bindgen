use oo_bindgen::class::{Class, ClassDeclarationHandle};
use oo_bindgen::formatting::{FormattingResult, Printer};
use oo_bindgen::structs::{Struct, StructDeclarationHandle, StructFieldType};

use heck::CamelCase;
use oo_bindgen::doc::DocReference;
use oo_bindgen::Handle;

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

impl<D> FriendClass for Handle<oo_bindgen::iterator::Iterator<D>>
where
    D: DocReference,
{
    fn friend_class(&self) -> String {
        self.iter_class.friend_class()
    }
}

impl<T, D> FriendClass for Struct<T, D>
where
    D: DocReference,
    T: StructFieldType,
{
    fn friend_class(&self) -> String {
        self.declaration.inner.friend_class()
    }
}

impl<D> FriendClass for Handle<Class<D>>
where
    D: DocReference,
{
    fn friend_class(&self) -> String {
        self.declaration.friend_class()
    }
}
