pub(crate) mod callback_arg_type;
pub(crate) mod constructor_arg_type;
pub(crate) mod core_type;
mod formatting;
pub(crate) mod struct_type;

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::constants::{ConstantSetHandle, ConstantValue, Representation};
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::formatting::{indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::structs::{
    Constructor, ConstructorType, Struct, StructDeclaration, StructFieldType, Visibility,
};
use oo_bindgen::{Handle, Library, Statement, StructType};

use oo_bindgen::class::ClassDeclarationHandle;

use std::path::Path;

use crate::cpp::callback_arg_type::*;
use crate::cpp::constructor_arg_type::CppConstructorArgType;
use crate::cpp::core_type::CoreType;
use crate::cpp::formatting::namespace;
use crate::cpp::struct_type::CppStructType;
use oo_bindgen::interface::InterfaceHandle;
use oo_bindgen::iterator::IteratorHandle;

pub(crate) fn by_ref(expr: String) -> String {
    format!("{}&", expr)
}

pub(crate) fn by_const_ref(expr: String) -> String {
    format!("const {}&", expr)
}

const FRIEND_CLASS_NAME: &str = "InternalFriendClass";

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

fn print_enum(f: &mut dyn Printer, e: &EnumHandle) -> FormattingResult<()> {
    f.writeln(&format!("enum class {} {{", e.core_type()))?;
    indented(f, |f| {
        for v in &e.variants {
            f.writeln(&format!("{} = {},", v.core_type(), v.value))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()?;
    f.writeln(&format!("const char* to_string({} value);", e.core_type()))?;
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
                v.core_type(),
                get_value(v.value)
            ))?;
        }
        Ok(())
    })?;
    f.writeln("}")?;
    f.newline()
}

fn print_exception(f: &mut dyn Printer, e: &ErrorType) -> FormattingResult<()> {
    f.writeln(&format!(
        "class {} : public std::logic_error {{",
        e.core_type()
    ))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln("// underlying error enum")?;
        f.writeln(&format!("{} error;", e.inner.core_type()))?;
        f.writeln(&format!(
            "{}({} error);",
            e.core_type(),
            e.inner.core_type()
        ))?;
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_struct_decl(f: &mut dyn Printer, s: &StructDeclaration) -> FormattingResult<()> {
    f.writeln(&format!("struct {};", s.core_type()))?;
    f.newline()
}

fn print_constructor_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T>>,
    constructor: &Handle<Constructor>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppConstructorArgType,
{
    let args = handle
        .constructor_args(constructor.clone())
        .map(|x| {
            format!(
                "{} {}",
                x.field_type.get_cpp_constructor_arg_type(),
                x.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    match constructor.constructor_type {
        ConstructorType::Normal => f.writeln(&format!("{}({});", handle.core_type(), args))?,
        ConstructorType::Static => f.writeln(&format!(
            "static {} {}({});",
            handle.core_type(),
            constructor.name.to_camel_case(),
            args
        ))?,
    }

    Ok(())
}

fn print_struct_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T>>,
    _lib: &Library,
) -> FormattingResult<()>
where
    T: StructFieldType + CppStructType + CppConstructorArgType,
{
    f.writeln(&format!("struct {} {{", handle.core_type()))?;
    //f.writeln(&format!("    friend class {};", FRIEND_CLASS_NAME))?;
    if let Visibility::Private = handle.visibility {
        f.writeln("private:")?;
    }
    indented(f, |f| {
        // delete the default constructor unless the struct has one
        if !handle.has_default_constructor() {
            f.newline()?;
            f.writeln(&format!("{}() = delete;", handle.core_type()))?;
        }

        // write the constructors
        for c in &handle.constructors {
            f.newline()?;
            print_constructor_definition(f, handle, c)?;
        }

        f.newline()?;
        for field in &handle.fields {
            f.writeln(&format!(
                "{} {};",
                field.field_type.struct_member_type(),
                field.core_type()
            ))?;
        }

        Ok(())
    })?;

    f.writeln("};")?;
    f.newline()
}

fn print_interface(f: &mut dyn Printer, handle: &InterfaceHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.core_type()))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!("virtual ~{}() = default;", handle.core_type()))?;
        f.newline()?;
        for cb in &handle.callbacks {
            let args: String = cb
                .arguments
                .iter()
                .map(|arg| {
                    format!(
                        "{} {}",
                        arg.arg_type.get_cpp_callback_arg_type(),
                        arg.core_type()
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            f.writeln(&format!(
                "virtual {} {}({}) = 0;",
                cb.return_type.get_cpp_callback_return_type(),
                cb.core_type(),
                args
            ))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

/*
fn print_deleted_copy_and_assignment(f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
    f.writeln("// non-copyable")?;
    f.writeln(&format!("{}(const {}&) = delete;", name, name))?;
    f.writeln(&format!("{}& operator=(const {}&) = delete;", name, name))
}
 */

fn print_class_decl(f: &mut dyn Printer, handle: &ClassDeclarationHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {};", handle.core_type()))?;
    f.newline()
}

/*
fn cpp_arguments<'a, T>(iter: T) -> String
where
    T: Iterator<Item = &'a Arg<FArgument>>,
{
    iter.map(|p| {
        format!(
            "{} {}",
            p.arg_type.to_any_type().get_cpp_func_argument_type(),
            p.core_type()
        )
    })
    .collect::<Vec<String>>()
    .join(", ")
}


fn print_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.parameters.iter().skip(1));

    f.writeln(&format!(
        "{} {}({});",
        method.native_function.return_type.get_cpp_return_type(),
        method.core_type(),
        args
    ))
}

fn print_static_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.parameters.iter());

    f.writeln(&format!(
        "static {} {}({});",
        method.native_function.return_type.get_cpp_return_type(),
        method.core_type(),
        args
    ))
}

fn print_async_method(f: &mut dyn Printer, method: &AsyncMethod) -> FormattingResult<()> {
    let args: String = cpp_arguments(method.native_function.parameters.iter().skip(1));

    f.writeln(&format!(
        "{} {}({});",
        method.native_function.return_type.get_cpp_return_type(),
        method.core_type(),
        args
    ))
}

fn print_class(f: &mut dyn Printer, handle: &ClassHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.core_type()))?;
    indented(f, |f| {
        f.writeln(&format!("friend class {};", FRIEND_CLASS_NAME))?;
        f.writeln("// pointer to the underlying C type")?;
        f.writeln("void* self;")?;
        f.writeln("// constructor only accessible internally")?;
        f.writeln(&format!(
            "{}(void* self): self(self) {{}}",
            handle.core_type()
        ))?;
        print_deleted_copy_and_assignment(f, &handle.core_type())
    })?;
    f.newline()?;
    f.writeln("public:")?;
    indented(f, |f| {
        if let Some(x) = &handle.constructor {
            let args = cpp_arguments(x.parameters.iter());
            f.writeln(&format!("{}({});", handle.core_type(), args))?;
        };
        if handle.destructor.is_some() {
            f.writeln(&format!("~{}();", handle.core_type()))?;
        };
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
*/

/*
fn print_static_class(f: &mut dyn Printer, handle: &StaticClassHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.core_type()))?;
    indented(f, |f| {
        f.writeln(&format!("{}() = delete;", handle.core_type()))
    })?;
    f.writeln("public:")?;
    indented(f, |f| {
        for method in &handle.static_methods {
            print_static_method(f, method)?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}
*/

fn print_iterator_definition(f: &mut dyn Printer, iter: &IteratorHandle) -> FormattingResult<()> {
    let iterator = include_str!("./snippet/iterator.hpp");
    for line in iterator.lines() {
        let substituted = line
            .replace("<name>", &iter.core_type())
            .replace("<iter_type>", &iter.item_type.core_type());
        f.writeln(&substituted)?;
    }
    f.newline()
}

fn print_header_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    print_version(lib, f)?;

    f.writeln("// forward declare the friend class which can access C++ class internals")?;
    f.writeln(&format!("class {};", FRIEND_CLASS_NAME))?;
    f.newline()?;

    for statement in lib.statements() {
        match &statement {
            Statement::Constants(x) => print_constants(f, x)?,
            Statement::EnumDefinition(x) => print_enum(f, x)?,
            Statement::ErrorType(x) => print_exception(f, x)?,
            Statement::StructDeclaration(x) => print_struct_decl(f, x)?,
            Statement::StructDefinition(x) => match x {
                StructType::FStruct(x) => print_struct_definition(f, x, lib)?,
                StructType::RStruct(x) => print_struct_definition(f, x, lib)?,
                StructType::CStruct(x) => print_struct_definition(f, x, lib)?,
                StructType::UStruct(x) => print_struct_definition(f, x, lib)?,
            },
            Statement::InterfaceDefinition(x) => print_interface(f, x)?,
            Statement::ClassDeclaration(x) => print_class_decl(f, x)?,
            Statement::ClassDefinition(_x) => {} //print_class(f, x)?,
            Statement::StaticClassDefinition(_x) => {} //print_static_class(f, x)?,
            Statement::IteratorDeclaration(x) => {
                print_iterator_definition(f, x)?;
            }
            Statement::CollectionDeclaration(_) => {
                // only used for transforms ATM
            }
            Statement::FunctionDefinition(_) => {
                // not used in C++
            }
        }
    }

    Ok(())
}

/* TODO
fn convert_native_struct_elem_to_cpp(elem: &AnyStructField) -> String {
    let base_name = format!("x.{}", elem.name);
    convert_to_cpp(&elem.field_type.to_any_type(), base_name)
}

fn convert_native_struct_ptr_elem_to_cpp(elem: &AnyStructField) -> String {
    let base_name = format!("x->{}", elem.name);
    convert_to_cpp(&elem.field_type.to_any_type(), base_name)
}

fn convert_native_struct_elem_from_cpp(elem: &AnyStructField) -> String {
    let base_name = format!("x.{}", elem.name);
    convert_to_c(&elem.field_type.to_any_type(), base_name)
}

fn convert_native_struct_ptr_elem_from_cpp(elem: &AnyStructField) -> String {
    let base_name = format!("x->{}", elem.name);
    convert_to_c(&elem.field_type.to_any_type(), base_name)
}
*/

/*
fn print_friend_class_decl(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", FRIEND_CLASS_NAME))?;
    indented(f, |f| {
        f.writeln("public:")?;

        for handle in lib.structs() {
            f.writeln(&format!(
                "static {} to_cpp(const {}& x);",
                handle.declaration().core_type(),
                //handle.to_c_type(&lib.c_ffi_prefix)
                todo!()
            ))?;
            f.writeln(&format!(
                "static std::unique_ptr<{}> to_cpp_ref(const {}* x);",
                handle.declaration().core_type(),
                //handle.to_c_type(&lib.c_ffi_prefix)
                todo!()
            ))?;
            f.writeln(&format!(
                "static {} from_cpp(const {}& x);",
                //handle.to_c_type(&lib.c_ffi_prefix),
                todo!(),
                handle.declaration().core_type()
            ))?;
            f.writeln(&format!(
                "static std::unique_ptr<{}> from_cpp_ref(const {}* x);",
                //handle.to_c_type(&lib.c_ffi_prefix),
                todo!(),
                handle.declaration().core_type()
            ))?;
            f.newline()?;
        }

        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}
 */

/* TODO
fn print_friend_class_impl(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    for handle in lib.structs() {
        print_struct_conversion_impl(lib, f, handle.get_any_struct())?;
    }

    f.newline()
}

fn print_struct_conversion_impl(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &AnyStructHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "{} {}::to_cpp(const {}& x)",
        handle.declaration().core_type(),
        FRIEND_CLASS_NAME,
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln(&format!("return {}(", handle.declaration.core_type()))?;
        indented(f, |f| {
            for (elem, last) in handle.fields.iter().with_last() {
                let conversion = convert_native_struct_elem_to_cpp(elem);

                if last {
                    f.writeln(&conversion)?;
                } else {
                    f.writeln(&format!("{},", conversion))?;
                }
            }
            Ok(())
        })?;
        f.writeln(");")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "std::unique_ptr<{}> {}::to_cpp_ref(const {}* x)",
        handle.declaration().core_type(),
        FRIEND_CLASS_NAME,
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("if (x) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "return std::make_unique<{}>(",
                handle.declaration.core_type()
            ))?;
            indented(f, |f| {
                for (elem, last) in handle.fields.iter().with_last() {
                    let conversion = convert_native_struct_ptr_elem_to_cpp(elem);
                    if last {
                        f.writeln(&conversion)?;
                    } else {
                        f.writeln(&format!("{},", conversion))?;
                    }
                }
                Ok(())
            })?;
            f.writeln(");")
        })?;
        f.writeln("}")?;
        f.writeln("else {")?;
        indented(f, |f| f.writeln("return nullptr;"))?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "{} {}::from_cpp(const {}& x)",
        handle.to_c_type(&lib.c_ffi_prefix),
        FRIEND_CLASS_NAME,
        handle.declaration().core_type(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("return {")?;
        indented(f, |f| {
            for (elem, last) in handle.fields.iter().with_last() {
                let conversion = convert_native_struct_elem_from_cpp(elem);
                if last {
                    f.writeln(&conversion)?;
                } else {
                    f.writeln(&format!("{},", conversion))?;
                }
            }
            Ok(())
        })?;
        f.writeln("};")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "std::unique_ptr<{}> {}::from_cpp_ref(const {}* x)",
        handle.to_c_type(&lib.c_ffi_prefix),
        FRIEND_CLASS_NAME,
        handle.declaration().core_type(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("if (x) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "return std::unique_ptr<{}>(new {} {{",
                handle.to_c_type(&lib.c_ffi_prefix),
                handle.to_c_type(&lib.c_ffi_prefix)
            ))?;
            indented(f, |f| {
                for (elem, last) in handle.fields.iter().with_last() {
                    let conversion = convert_native_struct_ptr_elem_from_cpp(elem);
                    if last {
                        f.writeln(&conversion)?;
                    } else {
                        f.writeln(&format!("{},", conversion))?;
                    }
                }
                Ok(())
            })?;
            f.writeln("});")
        })?;
        f.writeln("}")?;
        f.writeln("else {")?;
        indented(f, |f| f.writeln("return nullptr;"))?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()
}
*/

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

/*
fn convert_basic_type_to_cpp(typ: &BasicType, expr: String) -> String {
    match typ {
        BasicType::Bool => expr,
        BasicType::U8 => expr,
        BasicType::S8 => expr,
        BasicType::U16 => expr,
        BasicType::S16 => expr,
        BasicType::U32 => expr,
        BasicType::S32 => expr,
        BasicType::U64 => expr,
        BasicType::S64 => expr,
        BasicType::Float32 => expr,
        BasicType::Double64 => expr,
        BasicType::Duration(t) => match t {
            DurationType::Milliseconds => {
                format!("convert::from_sec_u64({})", expr)
            }
            DurationType::Seconds => {
                format!("convert::from_msec_u64({})", expr)
            }
        },
        BasicType::Enum(_) => format!("convert::to_cpp({})", expr),
    }
}


fn convert_to_cpp(typ: &AnyType, expr: String) -> String {
    match typ {
        AnyType::Basic(x) => convert_basic_type_to_cpp(x, expr),
        AnyType::String => format!("std::string({})", expr),
        AnyType::Struct(_) => format!("{}::to_cpp({})", FRIEND_CLASS_NAME, expr),
        AnyType::StructRef(_) => format!("{}::to_cpp_ref({})", FRIEND_CLASS_NAME, expr),
        AnyType::ClassRef(_) => format!("{}::to_cpp({})", FRIEND_CLASS_NAME, expr),
        AnyType::Interface(_) => "nullptr".to_string(), // Conversion from C to C++ is not allowed
        AnyType::Iterator(_) => format!("convert::to_vec({})", expr),
        AnyType::Collection(_) => "nullptr".to_string(), // Conversion from C to C++ is not allowed
    }
}
 */

/*
fn convert_basic_type_to_c(t: &BasicType, expr: String) -> String {
    match t {
        BasicType::Bool => expr,
        BasicType::U8 => expr,
        BasicType::S8 => expr,
        BasicType::U16 => expr,
        BasicType::S16 => expr,
        BasicType::U32 => expr,
        BasicType::S32 => expr,
        BasicType::U64 => expr,
        BasicType::S64 => expr,
        BasicType::Float32 => expr,
        BasicType::Double64 => expr,
        BasicType::Duration(t) => match t {
            DurationType::Milliseconds => {
                format!("convert::to_sec_u64({})", expr)
            }
            DurationType::Seconds => {
                format!("convert::to_msec_u64({})", expr)
            }
        },
        BasicType::Enum(_) => format!("convert::from_cpp({})", expr),
    }
}
 */

/* TODO
fn convert_to_c(typ: &AnyType, expr: String) -> String {
    match typ {
        AnyType::Basic(t) => convert_basic_type_to_c(t, expr),
        AnyType::String => format!("{}.c_str()", expr),
        AnyType::Struct(_) => format!("{}::from_cpp({})", FRIEND_CLASS_NAME, expr),
        AnyType::StructRef(_) => format!("{}::from_cpp_ref({})", FRIEND_CLASS_NAME, expr),
        AnyType::ClassRef(_) => unimplemented!(),
        AnyType::Interface(_) => format!("convert::from_cpp({})", expr),
        AnyType::Iterator(_) => "nullptr".to_string(), // Conversion not supported
        AnyType::Collection(_) => unimplemented!(),
    }
}
 */

/* TODO
fn print_interface_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &InterfaceHandle,
) -> FormattingResult<()> {
    fn get_invocation(handle: &InterfaceHandle, func: &CallbackFunction) -> String {
        let args = func
            .arguments
            .iter()
            .map(|p| convert_to_cpp(&p.arg_type.clone().into(), p.core_type()))
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "reinterpret_cast<{}*>({})->{}({})",
            handle.core_type(),
            CTX_VARIABLE_NAME.to_snake_case(),
            func.core_type(),
            args
        )
    }

    f.writeln(&format!(
        "{}_{}_t from_cpp(std::unique_ptr<{}> value)",
        lib.c_ffi_prefix,
        handle.name.to_snake_case(),
        handle.core_type(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        // Note: Designated initializers (i.e. C-style struct initialization) were standardized in C99,
        // but only to C++ in C++20. Therefore, we cannot use it here as we target a lower version of C++.
        f.writeln("return {")?;
        indented(f, |f| {
            for cb in &handle.callbacks {
                f.writeln(&format!(
                    "[]({}) -> {} {{",
                    crate::chelpers::callback_parameters_with_var_names(lib, cb),
                    cb.return_type.to_c_type(&lib.c_ffi_prefix)
                ))?;
                indented(f, |f| {
                    match &cb.return_type {
                        CReturnType::Type(t, _) => {
                            let value = get_invocation(handle, cb);

                            f.writeln(&format!(
                                "return {};",
                                "TODO!!!"
                                // convert_to_c(&AnyType::from(t.clone()), value)
                            ))?;
                        }
                        CReturnType::Void => {
                            f.writeln(&format!("{};", get_invocation(handle, cb)))?;
                        }
                    }
                    Ok(())
                })?;
                f.writeln("},")?;
            }

            f.writeln("value.release(),")?;

            f.writeln(&format!(
                "[](void* {}) {{ delete reinterpret_cast<{}*>({}); }},",
                CTX_VARIABLE_NAME,
                handle.core_type(),
                CTX_VARIABLE_NAME
            ))?;

            Ok(())
        })?;
        f.writeln("};")
    })?;
    f.writeln("}")?;
    f.newline()
}
*/

/*
fn print_iterator_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &IteratorHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "std::vector<{}> to_vec({}* x)",
        //handle.item_type.core_type(),
        //handle.iter_type.to_c_type(&lib.c_ffi_prefix)
        todo!(),
        todo!(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        let function_name = format!("{}_{}", lib.c_ffi_prefix, handle.function.name);
        f.writeln(&format!(
            "auto result = std::vector<{}>();",
            //handle.item_type.core_type(),
            todo!()
        ))?;
        f.writeln(&format!("auto it = {}(x);", function_name))?;
        f.writeln("while (it != nullptr) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "result.push_back({});",
                "TODO!!"
                /*
                convert_to_cpp(
                    &AnyType::Struct(handle.item_type.to_any_struct()),
                    "*it".to_string()
                )
                 */
            ))?;
            f.writeln(&format!("it = {}(x);", function_name))
        })?;
        f.writeln("}")?;
        f.newline()?;
        f.writeln("return result;")
    })?;
    f.writeln("}")?;
    f.newline()
}
 */

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

/*
fn print_struct_constructor_impl(
    f: &mut dyn Printer,
    handle: &AnyStructHandle,
) -> FormattingResult<()> {
    let name = handle.core_type();
    f.writeln(&format!(
        "{}::{}({}) :",
        name,
        name,
        get_struct_default_constructor_args(handle)
    ))?;
    indented(f, |f| {
        let last = handle.fields.len() - 1;
        for (i, e) in handle.fields.iter().enumerate() {
            if i == last {
                f.writeln(&format!("{}({})", e.core_type(), get_initializer_value(e)))?;
            } else {
                f.writeln(&format!("{}({}),", e.core_type(), get_initializer_value(e)))?;
            }
        }
        Ok(())
    })?;
    f.writeln("{}")?;
    f.newline()
}
*/

/*
fn print_exception_wrappers(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    if !lib.functions().any(|f| f.error_type.is_some()) {
        return Ok(());
    }

    fn print_check_exception(f: &mut dyn Printer, err: &ErrorType) -> FormattingResult<()> {
        f.writeln("if(error) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "throw {}(convert::to_cpp(error));",
                err.exception_name.to_camel_case()
            ))
        })?;
        f.writeln("}")
    }

    fn print_with_returned_value(
        lib: &Library,
        f: &mut dyn Printer,
        func: &FunctionHandle,
        err: &ErrorType,
    ) -> FormattingResult<()> {
        let args = func
            .parameters
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        f.writeln(&format!(
            "{} returned_value;",
            func.return_type.to_c_type(&lib.c_ffi_prefix)
        ))?;
        f.writeln(&format!(
            "const auto error = {}_{}({}, &returned_value);",
            lib.c_ffi_prefix,
            func.name.to_snake_case(),
            args
        ))?;
        print_check_exception(f, err)?;
        f.writeln("return returned_value;")
    }

    fn print_without_returned_value(
        lib: &Library,
        f: &mut dyn Printer,
        func: &FunctionHandle,
        err: &ErrorType,
    ) -> FormattingResult<()> {
        let args = func
            .parameters
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        f.writeln(&format!(
            "const auto error = {}_{}({});",
            lib.c_ffi_prefix,
            func.name.to_snake_case(),
            args
        ))?;
        print_check_exception(f, err)
    }

    // write native function wrappers
    namespace(f, "ex_wrap", |f| {
        for func in lib.functions() {
            if let Some(err) = &func.error_type {
                let args = func
                    .parameters
                    .iter()
                    .map(|p| {
                        format!(
                            "{} {}",
                            p.arg_type.to_any_type().to_c_type(&lib.c_ffi_prefix),
                            p.name.to_snake_case()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                f.writeln(&format!(
                    "{} {}({})",
                    &func.return_type.to_c_type(&lib.c_ffi_prefix),
                    func.name,
                    args
                ))?;
                f.writeln("{")?;
                indented(f, |f| {
                    match func.return_type {
                        FReturnType::Void => {
                            print_without_returned_value(lib, f, func, err)?;
                        }
                        FReturnType::Type(_, _) => {
                            print_with_returned_value(lib, f, func, err)?;
                        }
                    }
                    Ok(())
                })?;
                f.writeln("}")?;
                f.newline()?;
            }
        }
        Ok(())
    })?;
    f.newline()
}
*/

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

pub(crate) fn generate_cpp_header(lib: &Library, path: &Path) -> FormattingResult<()> {
    // Open the file
    std::fs::create_dir_all(&path)?;
    let filename = path.join(format!("{}.hpp", lib.name));
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
        print_header_namespace_contents(lib, f)
    })?;

    Ok(())
}

pub(crate) fn generate_cpp_impl(lib: &Library, path: &Path) -> FormattingResult<()> {
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
