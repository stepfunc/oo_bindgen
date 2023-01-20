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
    FutureInterface,
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
            TypeConverter::FutureInterface => unimplemented!(),
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
            TypeConverter::FutureInterface => {
                f.writeln(&format!("{to} crate::ffi::promise::make_promise({from})"))
            }
        }
    }

    pub(crate) fn is_unsafe(&self) -> bool {
        match self {
            TypeConverter::String(x) => x.is_unsafe(),
            TypeConverter::ValidatedEnum(x) => x.is_unsafe(),
            TypeConverter::UnvalidatedEnum(x) => x.is_unsafe(),
            TypeConverter::Struct(x) => x.is_unsafe(),
            TypeConverter::Duration(x) => x.is_unsafe(),
            TypeConverter::FutureInterface => {
                todo!()
            }
        }
    }
}

impl TypeConversion for StringType {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{to}{from}.as_ptr()"))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{to}std::ffi::CStr::from_ptr({from})"))
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
        f.writeln(&format!("{to}{from}.into()"))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{to}{from}.into()"))
    }
}

impl TypeConversion for StructDeclarationHandle {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{to}{from}.map_or(std::ptr::null(), |val| val as *const _)"
        ))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{to}{from}.as_ref()"))
    }
}

impl TypeConversion for DurationType {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self {
            DurationType::Milliseconds => f.writeln(&format!("{to}{from}.as_millis() as u64")),
            DurationType::Seconds => f.writeln(&format!("{to}{from}.as_secs()")),
        }
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self {
            DurationType::Milliseconds => {
                f.writeln(&format!("{to}std::time::Duration::from_millis({from})"))
            }
            DurationType::Seconds => {
                f.writeln(&format!("{to}std::time::Duration::from_secs({from})"))
            }
        }
    }
}
