use crate::cpp::conversion::*;
use crate::cpp::formatting::{const_ref, mut_ref, namespace, unique_ptr};
use crate::ctype::CType;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::formatting::{blocked, indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::interface::InterfaceHandle;
use oo_bindgen::structs::{Struct, StructFieldType};
use oo_bindgen::{Handle, Library, Statement, StructType};
use std::path::Path;

pub(crate) fn generate_impl(lib: &Library, path: &Path) -> FormattingResult<()> {
    // Open the file
    std::fs::create_dir_all(&path)?;
    let filename = path.join(format!("{}.cpp", lib.name));
    let mut f = FilePrinter::new(filename)?;

    // include guard
    f.writeln(&format!("#include \"{}.hpp\"", lib.name))?;
    f.writeln(&format!("#include \"{}.h\"", lib.name))?;
    f.newline()?;

    namespace(&mut f, &lib.c_ffi_prefix, |f| {
        print_impl_namespace_contents(lib, f)
    })?;

    Ok(())
}

fn print_impl_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    let time_conversions = include_str!("./snippet/convert_time.cpp");

    // TODO - print_friend_class_decl(lib, f)?;

    // conversions
    namespace(f, "convert", |f| {
        for line in time_conversions.lines() {
            f.writeln(line)?;
        }
        f.newline()?;

        // we need to emit the conversions in statement order as some conversions
        // will reference other conversions
        for statement in lib.statements() {
            match statement {
                Statement::StructDefinition(x) => match x {
                    StructType::FunctionArg(x) => {
                        write_cpp_to_native_struct_conversion(f, lib, x)?;
                    }
                    StructType::FunctionReturn(x) => {
                        write_native_to_cpp_struct_conversion(f, lib, x)?;
                    }
                    StructType::CallbackArg(x) => {
                        write_native_to_cpp_struct_conversion(f, lib, x)?;
                    }
                    StructType::Universal(x) => {
                        write_cpp_to_native_struct_conversion(f, lib, x)?;
                        write_native_to_cpp_struct_conversion(f, lib, x)?;
                    }
                },
                Statement::EnumDefinition(x) => {
                    write_enum_conversions(lib, f, x)?;
                    print_enum_to_string_impl(f, x)?;
                }
                Statement::InterfaceDefinition(x) => {
                    write_cpp_interface_to_native_conversion(f, lib, x)?;
                }
                _ => {}
            }
        }

        Ok(())
    })?;

    Ok(())
}

fn write_cpp_to_native_struct_conversion<T>(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &Handle<Struct<T>>,
) -> FormattingResult<()>
where
    T: StructFieldType + ToNativeStructField,
{
    let value_type = if handle.fields.iter().any(|f| f.field_type.requires_move()) {
        mut_ref(handle.core_type())
    } else {
        const_ref(handle.core_type())
    };

    let c_type = handle.to_c_type(&lib.c_ffi_prefix);
    f.writeln(&format!("{} to_native({} value)", c_type, value_type))?;
    blocked(f, |f| {
        f.writeln(&format!("return {} {{", c_type))?;
        indented(f, |f| {
            for field in &handle.fields {
                let cpp_value = format!("value.{}", field.name.to_snake_case());
                let conversion = field.field_type.to_native_struct_field(cpp_value);
                f.writeln(&format!("{},", conversion))?;
            }
            Ok(())
        })?;
        f.writeln("};")?;
        Ok(())
    })?;
    f.newline()?;
    Ok(())
}

fn write_native_to_cpp_struct_conversion<T>(
    _f: &mut dyn Printer,
    _lib: &Library,
    _handle: &Handle<Struct<T>>,
) -> FormattingResult<()>
where
    T: StructFieldType,
{
    Ok(())
}

fn print_enum_to_string_impl(f: &mut dyn Printer, handle: &EnumHandle) -> FormattingResult<()> {
    f.writeln(&format!(
        "const char* to_string({} value)",
        handle.core_type()
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}::{}: return \"{}\";",
                    handle.core_type(),
                    v.core_type(),
                    v.name
                ))?;
            }
            f.writeln(&format!(
                "default: throw std::invalid_argument(\"Undefined value for enum '{}'\");",
                handle.name
            ))
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;
    f.newline()
}

fn write_enum_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &EnumHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "{} enum_from_native({}_{}_t value)",
        handle.core_type(),
        lib.c_ffi_prefix,
        handle.name.to_snake_case()
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}_{}_{}: return {}::{};",
                    lib.c_ffi_prefix.to_shouty_snake_case(),
                    handle.name.to_shouty_snake_case(),
                    v.name.to_shouty_snake_case(),
                    handle.name.to_camel_case(),
                    v.name.to_snake_case()
                ))?;
            }
            f.writeln("default: throw std::invalid_argument(\"bad enum conversion\");")?;
            Ok(())
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "{}_{}_t enum_to_native({} value)",
        lib.c_ffi_prefix,
        handle.name.to_snake_case(),
        handle.core_type(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}::{}: return {}_{}_{};",
                    handle.name.to_camel_case(),
                    v.name.to_snake_case(),
                    lib.c_ffi_prefix.to_shouty_snake_case(),
                    handle.name.to_shouty_snake_case(),
                    v.name.to_shouty_snake_case(),
                ))?;
            }
            f.writeln("default: throw std::invalid_argument(\"bad enum conversion\");")?;
            Ok(())
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;
    f.newline()
}

fn write_cpp_interface_to_native_conversion(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &InterfaceHandle,
) -> FormattingResult<()> {
    let c_type = handle.to_c_type(&lib.c_ffi_prefix);
    f.writeln(&format!(
        "{} to_native({} value)",
        c_type,
        unique_ptr(handle.core_type())
    ))?;
    blocked(f, |f| {
        f.writeln("throw std::logic_error(\"not implemented\");")?;
        Ok(())
    })?;
    f.newline()?;
    Ok(())
}
