use oo_bindgen::formatting::*;

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
        self.inner.write("/// ")
    }
}

pub(crate) fn documentation<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    let mut printer = DocumentationPrinter::new(f);
    cb(&mut printer)
}

pub(crate) fn namespaced<F, T>(f: &mut dyn Printer, namespace: &str, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    f.writeln(&format!("namespace {}", namespace))?;
    blocked(f, |f| cb(f))
}

pub(crate) fn blocked<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    f.writeln("{")?;
    let result = indented(f, |f| cb(f))?;
    f.writeln("}")?;

    Ok(result)
}
