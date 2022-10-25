use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub(crate) type FormattingResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(crate) trait Printer {
    fn write(&mut self, s: &str) -> FormattingResult<()>;
    fn newline(&mut self) -> FormattingResult<()>;

    fn writeln(&mut self, s: &str) -> FormattingResult<()> {
        self.newline()?;
        self.write(s)
    }
}

pub(crate) struct FilePrinter {
    writer: BufWriter<File>,
    first_newline: bool,
}

impl FilePrinter {
    pub(crate) fn new<T: AsRef<Path>>(filepath: T) -> FormattingResult<Self> {
        tracing::info!("Create file: {}", filepath.as_ref().display());
        let file = File::create(filepath)?;
        let writer = BufWriter::new(file);
        Ok(Self {
            writer,
            first_newline: false,
        })
    }
}

impl Drop for FilePrinter {
    fn drop(&mut self) {
        // UNIX newline at the end of file
        self.newline().unwrap();
    }
}

impl Printer for FilePrinter {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.writer
            .write(s.as_bytes())
            .map(|_| {})
            .map_err(|e| e.into())
    }

    fn newline(&mut self) -> FormattingResult<()> {
        if !self.first_newline {
            self.first_newline = true;
            Ok(())
        } else {
            writeln!(self.writer).map_err(|e| e.into())
        }
    }
}

pub(crate) struct PrefixPrinter<'a, 'b> {
    inner: &'a mut dyn Printer,
    prefix: &'b str,
}

impl<'a, 'b> PrefixPrinter<'a, 'b> {
    pub(crate) fn new(printer: &'a mut dyn Printer, prefix: &'b str) -> Self {
        Self {
            inner: printer,
            prefix,
        }
    }
}

impl<'a, 'b> Printer for PrefixPrinter<'a, 'b> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write(self.prefix)
    }
}

pub(crate) struct IndentedPrinter<'a> {
    inner: PrefixPrinter<'a, 'static>,
}

impl<'a> IndentedPrinter<'a> {
    pub(crate) fn new(printer: &'a mut dyn Printer) -> Self {
        Self {
            inner: PrefixPrinter::new(printer, "    "),
        }
    }
}

impl<'a> Printer for IndentedPrinter<'a> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()
    }
}

pub(crate) struct CommentedPrinter<'a> {
    inner: PrefixPrinter<'a, 'static>,
}

impl<'a> CommentedPrinter<'a> {
    pub(crate) fn new(f: &'a mut dyn Printer) -> Self {
        Self {
            inner: PrefixPrinter::new(f, "// "),
        }
    }
}

impl<'a> Printer for CommentedPrinter<'a> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()
    }
}

pub(crate) struct DoxygenPrinter<'a> {
    inner: PrefixPrinter<'a, 'static>,
}

impl<'a> DoxygenPrinter<'a> {
    pub(crate) fn new(printer: &'a mut dyn Printer) -> Self {
        Self {
            inner: PrefixPrinter::new(printer, "/// "),
        }
    }
}

impl<'a> Printer for DoxygenPrinter<'a> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()
    }
}

pub(crate) fn indented<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    let mut printer = IndentedPrinter::new(f);
    cb(&mut printer)
}

pub(crate) fn commented<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    let mut printer = CommentedPrinter::new(f);
    cb(&mut printer)
}

pub(crate) fn doxygen<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    let mut printer = DoxygenPrinter::new(f);
    cb(&mut printer)
}

pub(crate) fn blocked_open_close<F, T>(
    f: &mut dyn Printer,
    open: &str,
    close: &str,
    cb: F,
) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    f.writeln(open)?;
    let result = indented(f, |f| cb(f))?;
    f.writeln(close)?;

    Ok(result)
}

pub(crate) fn blocked<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    blocked_open_close(f, "{", "}", cb)
}
