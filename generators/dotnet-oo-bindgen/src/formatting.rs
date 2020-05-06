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

pub struct NamespacedPrinter<'a, T: Printer> {
    inner: &'a mut T,
}

impl<'a, T: Printer> NamespacedPrinter<'a, T> {
    pub fn new(printer: &'a mut T, namespace: &str) -> FormattingResult<Self> {
        printer.writeln(&format!("namespace {}", namespace))?;
        printer.writeln("{")?;

        Ok(Self { inner: printer })
    }
}

impl<'a, T: Printer> Drop for NamespacedPrinter<'a, T> {
    fn drop(&mut self) {
        self.inner.writeln("}").unwrap();
    }
}

impl<'a, T: Printer> Printer for NamespacedPrinter<'a, T> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write("    ")
    }
}

pub struct ClassPrinter<'a, T: Printer> {
    inner: &'a mut T,
}

impl<'a, T: Printer> ClassPrinter<'a, T> {
    pub fn new(printer: &'a mut T, classname: &str) -> FormattingResult<Self> {
        printer.writeln(&format!("class {}", classname))?;
        printer.writeln("{")?;

        Ok(Self { inner: printer })
    }
}

impl<'a, T: Printer> Drop for ClassPrinter<'a, T> {
    fn drop(&mut self) {
        self.inner.writeln("}").unwrap();
    }
}

impl<'a, T: Printer> Printer for ClassPrinter<'a, T> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write("    ")
    }
}
