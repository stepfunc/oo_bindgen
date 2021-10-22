use crate::cpp::conversion::*;
use crate::cpp::formatting::namespace;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::formatting::{indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::Library;
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

        for e in lib.enums() {
            print_enum_conversions(lib, f, e)?;
        }

        /* TODO
        for interface in lib.interfaces() {
            print_interface_conversions(lib, f, interface)?;
        }

        for handle in lib.iterators() {
            print_iterator_conversions(lib, f, handle)?;
        }
         */

        Ok(())
    })?;

    // TODO - print_exception_wrappers(lib, f)?;

    // enum to string helpers
    for e in lib.enums() {
        print_enum_to_string_impl(f, e)?;
    }

    /* TODO
    print_friend_class_impl(lib, f)?;


    // struct constructors
    for handle in lib.structs() {
        print_struct_constructor_impl(f, handle.get_any_struct())?;
    }
    */

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

fn print_enum_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &EnumHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "{} to_cpp({}_{}_t value)",
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
        "{}_{}_t from_cpp({} value)",
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
