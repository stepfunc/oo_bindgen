use oo_bindgen::formatting::*;

pub struct CommentedPrinter<'a, T: Printer> {
    inner: &'a mut T,
}

impl<'a, T: Printer> CommentedPrinter<'a, T> {
    pub fn new(printer: &'a mut T) -> Self {
        Self { inner: printer }
    }

    pub fn close(self) -> &'a mut T {
        self.inner
    }
}

impl<'a, T: Printer> Printer for CommentedPrinter<'a, T> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write("// ")
    }
}

pub struct CppGuardPrinter<'a, T: Printer> {
    inner: &'a mut T,
}

impl<'a, T: Printer> CppGuardPrinter<'a, T> {
    pub fn new(printer: &'a mut T) -> FormattingResult<Self> {
        printer.writeln("#ifdef __cplusplus")?;
        printer.writeln("extern \"C\" {")?;
        printer.writeln("#endif")?;

        Ok(Self { inner: printer })
    }
}

impl<'a, T: Printer> Drop for CppGuardPrinter<'a, T> {
    fn drop(&mut self) {
        self.inner.writeln("#ifdef __cplusplus").unwrap();
        self.inner.writeln("}").unwrap();
        self.inner.writeln("#endif").unwrap();
    }
}

impl<'a, T: Printer> Printer for CppGuardPrinter<'a, T> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()
    }
}
