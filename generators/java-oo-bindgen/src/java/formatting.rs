use oo_bindgen::backend::*;
use oo_bindgen::model::{Arg, BasicType, FunctionArgument, Primitive, Validated};

pub(crate) fn documentation<F, T>(f: &mut dyn Printer, cb: F) -> FormattingResult<T>
where
    F: FnOnce(&mut dyn Printer) -> FormattingResult<T>,
{
    f.writeln("/**")?;
    let mut printer = PrefixPrinter::new(f, " * ");
    let result = cb(&mut printer)?;
    f.writeln(" */")?;

    Ok(result)
}

trait Nullable {
    fn is_nullable(&self) -> bool;
}

impl Nullable for Primitive {
    // the unsigned types are wrapper objects
    fn is_nullable(&self) -> bool {
        match self {
            Primitive::Bool => false,
            Primitive::U8 => true,
            Primitive::S8 => false,
            Primitive::U16 => true,
            Primitive::S16 => false,
            Primitive::U32 => true,
            Primitive::S32 => false,
            Primitive::U64 => true,
            Primitive::S64 => false,
            Primitive::Float => false,
            Primitive::Double => false,
        }
    }
}

impl Nullable for FunctionArgument {
    fn is_nullable(&self) -> bool {
        match self {
            Self::Basic(x) => match x {
                BasicType::Primitive(x) => x.is_nullable(),
                BasicType::Duration(_) => true,
                BasicType::Enum(_) => true,
            },
            Self::String(_) => true,
            Self::Collection(_) => true,
            Self::Struct(_) => true,
            Self::StructRef(_) => true,
            Self::ClassRef(_) => true,
            Self::Interface(_) => true,
        }
    }
}

pub(crate) fn write_null_checks(
    f: &mut dyn Printer,
    args: &[Arg<FunctionArgument, Validated>],
) -> FormattingResult<()> {
    for arg in args.iter().filter(|a| a.arg_type.is_nullable()) {
        let arg_name = arg.name.mixed_case();
        f.writeln(&format!(
            "java.util.Objects.requireNonNull({}, \"{} cannot be null\");",
            arg_name, arg_name
        ))?;
    }
    Ok(())
}
