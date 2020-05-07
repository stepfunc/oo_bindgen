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

pub fn cpp_guard<'a, F, T>(f: &'a mut dyn Printer, cb: F) -> FormattingResult<T>
where F: FnOnce(&mut dyn Printer) -> FormattingResult<T> {
    f.writeln("#ifdef __cplusplus")?;
    f.writeln("extern \"C\" {")?;
    f.writeln("#endif")?;

    let result = cb(f)?;

    f.writeln("#ifdef __cplusplus")?;
    f.writeln("}")?;
    f.writeln("#endif")?;

    Ok(result)
}
