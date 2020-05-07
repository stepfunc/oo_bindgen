use oo_bindgen::formatting::*;

struct CommentedPrinter<'a> {
    inner: &'a mut dyn Printer,
}

impl<'a> CommentedPrinter<'a> {
    fn new(printer: &'a mut dyn Printer) -> Self {
        Self { inner: printer }
    }
}

impl<'a> Printer for CommentedPrinter<'a> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write("// ")
    }
}

pub fn commented<'a, F, T>(f: &'a mut dyn Printer, cb: F) -> FormattingResult<T>
where F: FnOnce(&mut dyn Printer) -> FormattingResult<T> {
    let mut printer = CommentedPrinter::new(f);
    cb(&mut printer)
}

pub fn namespaced<'a, F, T>(f: &'a mut dyn Printer, namespace: &str, cb: F) -> FormattingResult<T>
where F: FnOnce(&mut dyn Printer) -> FormattingResult<T> {
    f.writeln(&format!("namespace {}", namespace))?;
    blocked(f, |f| cb(f))
}

pub fn blocked<'a, F, T>(f: &'a mut dyn Printer, cb: F) -> FormattingResult<T>
where F: FnOnce(&mut dyn Printer) -> FormattingResult<T> {
    f.writeln("{")?;
    let result = indented(f, |f| cb(f))?;
    f.writeln("}")?;

    Ok(result)
}
