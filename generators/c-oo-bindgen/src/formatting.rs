use oo_bindgen::formatting::*;

pub(crate) fn cpp_guard<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    f.writeln("#ifdef __cplusplus")?;
    f.writeln("extern \"C\" {")?;
    f.writeln("#endif")?;

    let result = cb(f)?;

    f.writeln("#ifdef __cplusplus")?;
    f.writeln("}")?;
    f.writeln("#endif")?;

    Ok(result)
}
