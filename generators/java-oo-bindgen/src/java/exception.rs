use super::doc::*;
use super::*;
use oo_bindgen::doc::Validated;
use oo_bindgen::error_type::{ErrorType, ExceptionType};

pub(crate) fn generate(f: &mut impl Printer, error: &ErrorType<Validated>) -> FormattingResult<()> {
    // Documentation
    documentation(f, |f| javadoc_print(f, &error.inner.doc))?;

    let exception_name = error.exception_name.camel_case();
    let error_name = error.inner.name.camel_case();

    let exception_type = match error.exception_type {
        ExceptionType::CheckedException => "Exception",
        ExceptionType::UncheckedException => "RuntimeException",
    };

    // Enum definition
    f.writeln(&format!(
        "public final class {} extends {}",
        exception_name, exception_type
    ))?;
    blocked(f, |f| {
        documentation(f, |f| f.writeln("Error detail"))?;
        f.writeln(&format!("public final {} error;", error_name))?;
        f.newline()?;
        f.writeln(&format!("{}({} error)", exception_name, error_name))?;
        blocked(f, |f| {
            f.writeln("super(error.toString());")?;
            f.writeln("this.error = error;")
        })
    })
}
