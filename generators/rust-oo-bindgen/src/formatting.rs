use oo_bindgen::formatting::*;

pub(crate) fn blocked<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where F: FnOnce(&mut dyn Printer) -> FormattingResult<T> {
    f.write(" {")?;
    let result = indented(f, |f| cb(f))?;
    f.writeln("}")?;

    Ok(result)
}
