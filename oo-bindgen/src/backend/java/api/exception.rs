use super::doc::*;
use super::*;

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
        "public final class {exception_name} extends {exception_type}"
    ))?;
    blocked(f, |f| {
        documentation(f, |f| f.writeln("Error detail"))?;
        f.writeln(&format!("public final {error_name} error;"))?;
        f.newline()?;
        f.writeln(&format!("{exception_name}({error_name} error)"))?;
        blocked(f, |f| {
            f.writeln("super(error.toString());")?;
            f.writeln("this.error = error;")
        })
    })
}
