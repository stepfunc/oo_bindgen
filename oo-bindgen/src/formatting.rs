use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub type FormattingResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait Printer {
    fn write(&mut self, s: &str) -> FormattingResult<()>;
    fn newline(&mut self) -> FormattingResult<()>;

    fn writeln(&mut self, s: &str) -> FormattingResult<()> {
        self.newline()?;
        self.write(s)
    }
}

pub struct FilePrinter {
    writer: BufWriter<File>,
}

impl FilePrinter {
    pub fn new<T: AsRef<Path>>(filepath: T) -> FormattingResult<Self> {
        let file = File::create(filepath)?;
        let writer = BufWriter::new(file);
        Ok(Self { writer })
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
        self.writer.write(s.as_bytes()).map(|_| {}).map_err(|e| e.into())
    }

    fn newline(&mut self) -> FormattingResult<()> {
        writeln!(self.writer, "").map_err(|e| e.into())
    }
}

struct IndentedPrinter<'a> {
    inner: &'a mut dyn Printer,
}

impl<'a> IndentedPrinter<'a> {
    fn new(printer: &'a mut dyn Printer) -> Self {
        Self { inner: printer }
    }
}

impl<'a> Printer for IndentedPrinter<'a> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write("    ")
    }
}

pub fn indented<'a, F, T>(f: &'a mut dyn Printer, cb: F) -> FormattingResult<T>
where F: FnOnce(&mut dyn Printer) -> FormattingResult<T> {
    let mut printer = IndentedPrinter::new(f);
    cb(&mut printer)
}
