use oo_bindgen::formatting::{FormattingResult, Printer};

pub(crate) fn mut_ref(expr: String) -> String {
    format!("{}&", expr)
}
pub(crate) fn const_ref(expr: String) -> String {
    format!("const {}&", expr)
}
pub(crate) fn unique_ptr(expr: String) -> String {
    format!("std::unique_ptr<{}>", expr)
}
pub(crate) fn friend_class(expr: String) -> String {
    format!("Cpp{}Friend", expr)
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
