use crate::backend::*;
use crate::model::*;

trait TypeConversion {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn is_unsafe(&self) -> bool {
        false
    }
}

pub(crate) enum TypeConverter {
    String(StringType),
    ValidatedEnum(Handle<Enum<Validated>>),
    UnvalidatedEnum(Handle<Enum<Unvalidated>>),
    Struct(StructDeclarationHandle),
    Duration(DurationType),
}

impl TypeConverter {
    pub(crate) fn convert_to_c(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            TypeConverter::String(x) => x.convert_to_c(f, from, to),
            TypeConverter::ValidatedEnum(x) => x.convert_to_c(f, from, to),
            TypeConverter::UnvalidatedEnum(x) => x.convert_to_c(f, from, to),
            TypeConverter::Struct(x) => x.convert_to_c(f, from, to),
            TypeConverter::Duration(x) => x.convert_to_c(f, from, to),
        }
    }

    pub(crate) fn convert_from_c(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            TypeConverter::String(x) => x.convert_from_c(f, from, to),
            TypeConverter::ValidatedEnum(x) => x.convert_from_c(f, from, to),
            TypeConverter::UnvalidatedEnum(x) => x.convert_from_c(f, from, to),
            TypeConverter::Struct(x) => x.convert_from_c(f, from, to),
            TypeConverter::Duration(x) => x.convert_from_c(f, from, to),
        }
    }

    pub(crate) fn is_unsafe(&self) -> bool {
        match self {
            TypeConverter::String(x) => x.is_unsafe(),
            TypeConverter::ValidatedEnum(x) => x.is_unsafe(),
            TypeConverter::UnvalidatedEnum(x) => x.is_unsafe(),
            TypeConverter::Struct(x) => x.is_unsafe(),
            TypeConverter::Duration(x) => x.is_unsafe(),
        }
    }
}

impl TypeConversion for StringType {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.as_ptr()", to, from))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}std::ffi::CStr::from_ptr({})", to, from))
    }

    fn is_unsafe(&self) -> bool {
        true
    }
}

impl<D> TypeConversion for Handle<Enum<D>>
where
    D: DocReference,
{
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.into()", to, from))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.into()", to, from))
    }
}

impl TypeConversion for StructDeclarationHandle {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.map_or(std::ptr::null(), |val| val as *const _)",
            to, from
        ))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.as_ref()", to, from))
    }
}

impl TypeConversion for DurationType {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self {
            DurationType::Milliseconds => f.writeln(&format!("{}{}.as_millis() as u64", to, from)),
            DurationType::Seconds => f.writeln(&format!("{}{}.as_secs()", to, from)),
        }
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self {
            DurationType::Milliseconds => {
                f.writeln(&format!("{}std::time::Duration::from_millis({})", to, from))
            }
            DurationType::Seconds => {
                f.writeln(&format!("{}std::time::Duration::from_secs({})", to, from))
            }
        }
    }
}
