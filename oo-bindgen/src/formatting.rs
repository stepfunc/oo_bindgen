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

pub struct IndentedPrinter<'a, T: Printer> {
    inner: &'a mut T,
}

impl<'a, T: Printer> IndentedPrinter<'a, T> {
    pub fn new(printer: &'a mut T) -> Self {
        Self { inner: printer }
    }

    pub fn close(self) -> &'a mut T {
        self.inner
    }
}

impl<'a, T: Printer> Printer for IndentedPrinter<'a, T> {
    fn write(&mut self, s: &str) -> FormattingResult<()> {
        self.inner.write(s)
    }

    fn newline(&mut self) -> FormattingResult<()> {
        self.inner.newline()?;
        self.inner.write("    ")
    }
}
