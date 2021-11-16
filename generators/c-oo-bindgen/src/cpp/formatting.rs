use oo_bindgen::backend::*;
use oo_bindgen::model::*;

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
pub(crate) fn std_move<S: Into<String>>(expr: S) -> String {
    format!("std::move({})", expr.into())
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
        format!("Cpp{}Friend", self.name.camel_case())
    }
}

impl FriendClass for StructDeclarationHandle {
    fn friend_class(&self) -> String {
        format!("Cpp{}Friend", self.name.camel_case())
    }
}

impl<D> FriendClass for Handle<AbstractIterator<D>>
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
