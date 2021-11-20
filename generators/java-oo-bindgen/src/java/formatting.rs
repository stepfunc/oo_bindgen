use oo_bindgen::backend::*;

pub(crate) fn documentation<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    f.writeln("/**")?;
    let mut printer = PrefixPrinter::new(f, " * ");
    let result = cb(&mut printer)?;
    f.writeln(" */")?;

    Ok(result)
}
