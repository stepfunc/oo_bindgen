use oo_bindgen::formatting::*;

/*
struct DocumentationPrinter<'a> {
    inner: &'a mut dyn Printer,
}

impl<'a> DocumentationPrinter<'a> {
    fn new(printer: &'a mut dyn Printer) -> Self {
        Self { inner: printer }
    }
}

impl<'a> Printer for DocumentationPrinter<'a> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write(" * ")
    }
}
*/

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
