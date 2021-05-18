use oo_bindgen::formatting::{FormattingResult, Printer};

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
