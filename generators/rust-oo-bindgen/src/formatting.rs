use oo_bindgen::formatting::*;

pub fn blocked<'a, F, T>(f: &'a mut dyn Printer, cb: F) -> FormattingResult<T>
where F: FnOnce(&mut dyn Printer) -> FormattingResult<T> {
    f.write(" {")?;
    let result = indented(f, |f| cb(f))?;
    f.writeln("}")?;

    Ok(result)
}
