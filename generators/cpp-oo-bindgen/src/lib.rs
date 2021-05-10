#![deny(
// dead_code,
arithmetic_overflow,
invalid_type_param_default,
//missing_fragment_specifier,
mutable_transmutes,
no_mangle_const_items,
overflowing_literals,
patterns_in_fns_without_body,
pub_use_of_private_extern_crate,
unknown_crate_types,
const_err,
order_dependent_trait_objects,
illegal_floating_point_literal_pattern,
improper_ctypes,
late_bound_lifetime_arguments,
non_camel_case_types,
non_shorthand_field_patterns,
non_snake_case,
non_upper_case_globals,
no_mangle_generic_items,
private_in_public,
stable_features,
type_alias_bounds,
tyvar_behind_raw_pointer,
unconditional_recursion,
unused_comparisons,
unreachable_pub,
anonymous_parameters,
missing_copy_implementations,
// missing_debug_implementations,
// missing_docs,
trivial_casts,
trivial_numeric_casts,
unused_import_braces,
unused_qualifications,
clippy::all
)]
#![forbid(
    unsafe_code,
    // intra_doc_link_resolution_failure, broken_intra_doc_links
    safe_packed_borrows,
    while_true,
    bare_trait_objects
)]

mod formatting;
mod name_traits;
mod type_traits;

use heck::SnakeCase;
use oo_bindgen::callback::{CallbackParameter, InterfaceElement, InterfaceHandle};
use oo_bindgen::constants::{ConstantSetHandle, ConstantValue, Representation};
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::formatting::{indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::native_enum::NativeEnumHandle;
use oo_bindgen::native_struct::{NativeStructDeclaration, NativeStructHandle, NativeStructType};
use oo_bindgen::{Library, Statement};
use std::path::Path;

use crate::formatting::namespace;
use crate::name_traits::*;
use crate::type_traits::*;
use oo_bindgen::class::{ClassDeclarationHandle, ClassHandle, Method, AsyncMethod};

const FRIEND_CLASS_NAME : &'static str = "InternalFriendClass";

fn print_version(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    let name = lib.c_ffi_prefix.to_snake_case();

    // Version number
    f.writeln(&format!(
        "constexpr uint64_t {}_version_major = {};",
        name, lib.version.major
    ))?;
    f.writeln(&format!(
        "constexpr uint64_t {}_version_minor = {};",
        name, lib.version.minor
    ))?;
    f.writeln(&format!(
        "constexpr uint64_t {}_version_patch = {};",
        name, lib.version.patch
    ))?;
    f.writeln(&format!(
        "constexpr char const* {}_version_string = \"{}\";",
        name,
        lib.version.to_string()
    ))?;
    f.newline()
}

fn print_enum(f: &mut dyn Printer, e: &NativeEnumHandle) -> FormattingResult<()> {
    f.writeln(&format!("enum class {} {{", e.cpp_name()))?;
    indented(f, |f| {
        for v in &e.variants {
            f.writeln(&format!("{} = {},", v.cpp_name(), v.value))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_constants(f: &mut dyn Printer, c: &ConstantSetHandle) -> FormattingResult<()> {
    fn get_value(v: ConstantValue) -> String {
        match v {
            ConstantValue::U8(v, Representation::Hex) => format!("0x{:02X}", v),
        }
    }

    fn get_type(v: ConstantValue) -> &'static str {
        match v {
            ConstantValue::U8(_, _) => "uint8_t",
        }
    }

    f.writeln(&format!("namespace {} {{", c.name.to_snake_case()))?;
    indented(f, |f| {
        for v in &c.values {
            f.writeln(&format!(
                "constexpr {} {} = {};",
                get_type(v.value),
                v.cpp_name(),
                get_value(v.value)
            ))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_exception(f: &mut dyn Printer, e: &ErrorType) -> FormattingResult<()> {
    f.writeln(&format!(
        "class {} : public std::logic_error {{",
        e.cpp_name()
    ))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln("// underlying error enum")?;
        f.writeln(&format!("{} error;", e.inner.cpp_name()))?;
        f.writeln(&format!("{}({} error);", e.cpp_name(), e.inner.cpp_name()))?;
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_native_struct_decl(f: &mut dyn Printer, s: &NativeStructDeclaration) -> FormattingResult<()> {
    f.writeln(&format!("class {};", s.cpp_name()))?;
    f.newline()
}

fn print_native_struct(f: &mut dyn Printer, handle: &NativeStructHandle) -> FormattingResult<()> {
    let constructor_args = handle
        .elements
        .iter()
        .flat_map(|x| {
            if x.element_type.has_default() {
                None
            } else {
                Some(format!("{} {}", x.element_type.to_type().get_cpp_struct_member_type(), x.name))
            }
        })
        .collect::<Vec<String>>()
        .join(", ");


    f.writeln(&format!("class {} {{", handle.cpp_name()))?;
    if let NativeStructType::Public  = handle.struct_type {
        f.writeln("public:")?;
    }
    indented(f, |f| {
        f.writeln(&format!("{}({});", handle.cpp_name(), constructor_args))?;
        f.newline()?;
        for field in &handle.elements {
            f.writeln(&format!(
                "{} {};",
                field.element_type.to_type().get_cpp_struct_member_type(),
                field.cpp_name()
            ))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_interface(f: &mut dyn Printer, handle: &InterfaceHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.cpp_name()))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!("virtual ~{}() {{}}", handle.cpp_name()))?;
        f.newline()?;
        for elem in &handle.elements {
            if let InterfaceElement::CallbackFunction(func) = elem {
                let args: String = func
                    .parameters
                    .iter()
                    .flat_map(|p| match p {
                        CallbackParameter::Arg(_) => None,
                        CallbackParameter::Parameter(p) => Some(format!(
                            "{} {}",
                            p.param_type.get_cpp_to_native_argument(),
                            p.cpp_name()
                        )),
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                f.writeln(&format!(
                    "virtual {} {}({}) = 0;",
                    func.return_type.get_cpp_type(),
                    func.cpp_name(),
                    args
                ))?;
            }
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_deleted_copy_and_assignment(f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
    f.writeln("// non-copyable")?;
    f.writeln(&format!("{}(const {}&) = delete;", name, name))?;
    f.writeln(&format!("{}& operator=(const {}&) = delete;", name, name))
}

/*
fn print_iterator(f: &mut dyn Printer, handle: &IteratorHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.cpp_name()))?;
    indented(f, |f| {
        f.writeln("// internal friend class used for construction")?;
        f.writeln(&format!("friend class {};", FRIEND_CLASS_NAME))?;
        f.writeln("// pointer to the underlying C iterator type")?;
        f.writeln("void* self;")?;
        f.writeln("// pointer to the current C value")?;
        f.writeln("void* current;")?;
        f.writeln("// constructor only accessible internally")?;
        f.writeln(&format!("{}(void* self, void* current): self(self), current(current) {{}}", handle.cpp_name()))?;
        print_deleted_copy_and_assignment(f, &handle.cpp_name())
    })?;
    f.writeln("public:")?;
    f.newline()?;
    indented(f, |f| {
        f.writeln("// @brief move to the next value")?;
        f.writeln("bool next();")?;
        f.writeln("// @brief get the current value")?;
        f.writeln(&format!("{} get() const;", handle.item_type.cpp_name()))
    })?;
    f.writeln("};")?;
    f.newline()
}
*/

fn print_class_decl(f: &mut dyn Printer, handle: &ClassDeclarationHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {};", handle.cpp_name()))?;
    f.newline()
}

fn print_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args: String = method.native_function
        .parameters
        .iter()
        .skip(1)
        .map(|p| {
            format!(
                "{} {}",
                p.param_type.get_cpp_func_argument_type(),
                p.cpp_name()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "{} {}({});",
        method.native_function.return_type.get_cpp_type(),
        method.cpp_name(),
        args
    ))
}

fn print_static_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args: String = method.native_function
        .parameters
        .iter()
        .map(|p| {
            format!(
                "{} {}",
                p.param_type.get_cpp_func_argument_type(),
                p.cpp_name()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "static {} {}({});",
        method.native_function.return_type.get_cpp_type(),
        method.cpp_name(),
        args
    ))
}

fn print_async_method(f: &mut dyn Printer, method: &AsyncMethod) -> FormattingResult<()> {
    let args: String = method.native_function
        .parameters
        .iter()
        .skip(1)
        .map(|p| {
            format!(
                "{} {}",
                p.param_type.get_cpp_func_argument_type(),
                p.cpp_name()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "{} {}({});",
        method.native_function.return_type.get_cpp_type(),
        method.cpp_name(),
        args
    ))
}

fn print_class(f: &mut dyn Printer, handle: &ClassHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.cpp_name()))?;
    indented(f, |f| {
        f.writeln(&format!("friend class {};", FRIEND_CLASS_NAME))?;
        f.writeln("// pointer to the underlying C type")?;
        f.writeln("void* self;")?;
        f.writeln("// constructor only accessible internally")?;
        f.writeln(&format!("{}(void* self): self(self) {{}}", handle.cpp_name()))?;
        print_deleted_copy_and_assignment(f, &handle.cpp_name())
    })?;
    f.newline()?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!("~{}();", handle.cpp_name()))?;
        for method in &handle.methods {
            print_method(f, method)?;
        }
        for method in &handle.static_methods {
            print_static_method(f, method)?;
        }
        for method in &handle.async_methods {
            print_async_method(f, method)?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}


fn print_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    print_version(lib, f)?;

    f.writeln("// forward declare the friend class which can access C++ class internals")?;
    f.writeln(&format!("class {};", FRIEND_CLASS_NAME))?;
    f.newline()?;

    for statement in lib.into_iter() {
        match statement {
            Statement::Constants(x) => print_constants(f, &x)?,
            Statement::EnumDefinition(x) => print_enum(f, &x)?,
            Statement::ErrorType(x) => print_exception(f, &x)?,
            Statement::NativeStructDeclaration(x) => print_native_struct_decl(f, &x)?,
            Statement::NativeStructDefinition(x) => print_native_struct(f, &x)?,
            Statement::InterfaceDefinition(x) => print_interface(f, &x)?,
            Statement::IteratorDeclaration(_) => {
                // we don't do anything with this ATM b/c we transform to vector
            },
            Statement::ClassDeclaration(x) => print_class_decl(f, &x)?,
            Statement::ClassDefinition(x) => print_class(f, &x)?,
            Statement::StructDefinition(_) => {
                // ignoring these for now
            },
            Statement::StaticClassDefinition(_) => {},
            Statement::CollectionDeclaration(_) => {},
            // not used in C++
            Statement::NativeFunctionDeclaration(_) => {},


        }
    }

    Ok(())
}

pub fn generate_cpp_header<P: AsRef<Path>>(lib: &Library, path: P) -> FormattingResult<()> {
    // Open the file
    std::fs::create_dir_all(&path)?;
    let filename = path.as_ref().join(format!("{}.hpp", lib.name));
    let mut f = FilePrinter::new(filename)?;

    // include guard
    f.writeln("#pragma once")?;
    f.newline()?;
    f.writeln("#include <cstdint>")?;
    f.writeln("#include <stdexcept>")?;
    f.writeln("#include <chrono>")?;
    f.writeln("#include <memory>")?;
    f.writeln("#include <vector>")?;
    f.newline()?;

    namespace(&mut f, &lib.c_ffi_prefix, |f| {
        print_namespace_contents(lib, f)
    })?;

    Ok(())
}
