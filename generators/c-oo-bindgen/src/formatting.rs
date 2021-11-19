use oo_bindgen::backend::*;
use oo_bindgen::model::Library;

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

pub(crate) fn print_license(f: &mut dyn Printer, lib: &Library) -> FormattingResult<()> {
    commented(f, |f| {
        for line in lib.info.license_description.iter() {
            f.writeln(line)?;
        }
        Ok(())
    })
}
