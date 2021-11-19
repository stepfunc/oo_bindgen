use std::path::Path;

use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::ctype::CType;
use crate::doc::{docstring_print, doxygen_print};
use crate::formatting::{cpp_guard, print_license};

pub(crate) fn generate_c_header(lib: &Library, path: &Path) -> FormattingResult<()> {
    let uppercase_name = lib.settings.c_ffi_prefix.to_uppercase();

    // Open file
    std::fs::create_dir_all(&path)?;
    let filename = path.join(format!("{}.h", lib.settings.name));
    let mut f = FilePrinter::new(filename)?;

    // Print license
    print_license(&mut f, lib)?;

    // Header guard
    f.writeln("#pragma once")?;
    f.newline()?;

    // C++ guard
    cpp_guard(&mut f, |f| {
        f.newline()?;

        // Version number
        f.writeln(&format!(
            "#define {}_VERSION_MAJOR {}",
            uppercase_name, lib.version.major
        ))?;
        f.writeln(&format!(
            "#define {}_VERSION_MINOR {}",
            uppercase_name, lib.version.minor
        ))?;
        f.writeln(&format!(
            "#define {}_VERSION_PATCH {}",
            uppercase_name, lib.version.patch
        ))?;
        f.writeln(&format!(
            "#define {}_VERSION_STRING \"{}\"",
            uppercase_name, lib.version
        ))?;
        f.newline()?;

        // Standard includes needed
        f.writeln("#include <stdbool.h>")?;
        f.writeln("#include <stdint.h>")?;
        f.newline()?;

        // Doxygen needs the @file tag
        f.writeln(&format!(
            "/// @file C API for the {} library",
            lib.settings.name
        ))?;
        f.newline()?;

        // Iterate through each statement and print them
        for statement in lib.statements() {
            match statement {
                Statement::Constants(handle) => write_constants_definition(f, handle)?,
                Statement::StructDeclaration(handle) => {
                    let c_type = handle.to_c_type();
                    f.writeln(&format!("typedef struct {} {};", c_type, c_type))?;
                }
                Statement::StructDefinition(st) => match st {
                    StructType::FunctionArg(x) => write_struct_definition(f, x)?,
                    StructType::FunctionReturn(x) => write_struct_definition(f, x)?,
                    StructType::CallbackArg(x) => write_struct_definition(f, x)?,
                    StructType::Universal(x) => write_struct_definition(f, x)?,
                },
                Statement::EnumDefinition(handle) => write_enum_definition(f, handle)?,
                Statement::ClassDeclaration(handle) => write_class_declaration(f, handle)?,
                Statement::FunctionDefinition(handle) => write_function(f, handle)?,
                Statement::InterfaceDefinition(handle) => write_interface(f, handle.untyped())?,
                _ => (),
            }
            f.newline()?;
        }

        Ok(())
    })
}

fn write_constants_definition(
    f: &mut dyn Printer,
    handle: &Handle<ConstantSet<Validated>>,
) -> FormattingResult<()> {
    fn get_constant_value(value: ConstantValue) -> String {
        match value {
            ConstantValue::U8(value, Representation::Hex) => format!("0x{:02X?}", value),
        }
    }

    for item in &handle.values {
        doxygen(f, |f| doxygen_print(f, &item.doc))?;
        f.writeln(&format!(
            "#define {}_{}_{} {}",
            handle.settings.c_ffi_prefix.capital_snake_case(),
            handle.name.capital_snake_case(),
            item.name.capital_snake_case(),
            get_constant_value(item.value)
        ))?;
    }
    Ok(())
}

fn write_struct_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + TypeExtractor + CType,
{
    let doc = match handle.visibility {
        Visibility::Public => handle.doc.clone(),
        Visibility::Private => handle
            .doc
            .clone()
            .warning("This struct should never be initialized or modified by user code"),
    };

    doxygen(f, |f| doxygen_print(f, &doc))?;

    // Write the struct definition
    f.writeln(&format!("typedef struct {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.fields {
            doxygen(f, |f| {
                doxygen_print(f, &element.doc)?;

                if let Some(t) = &element.field_type.get_duration_type() {
                    f.writeln(&format!("@note The unit is {}", t.unit()))?;
                }

                Ok(())
            })?;
            f.writeln(&format!(
                "{} {};",
                element.field_type.to_c_type(),
                element.name,
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))?;

    // user should never try to initialize opaque structs, so don't suggest this is OK
    if handle.visibility != Visibility::Private {
        f.newline()?;
        for c in &handle.initializers {
            write_struct_initializer(f, c, handle)?;
            f.newline()?;
        }
    }

    Ok(())
}

fn get_default_value(default: &ValidatedDefaultValue) -> String {
    match default {
        ValidatedDefaultValue::Bool(x) => x.to_string(),
        ValidatedDefaultValue::Number(x) => match x {
            NumberValue::U8(x) => x.to_string(),
            NumberValue::S8(x) => x.to_string(),
            NumberValue::U16(x) => x.to_string(),
            NumberValue::S16(x) => x.to_string(),
            NumberValue::U32(x) => x.to_string(),
            NumberValue::S32(x) => x.to_string(),
            NumberValue::U64(x) => x.to_string(),
            NumberValue::S64(x) => x.to_string(),
            NumberValue::Float(x) => format!("{}f", x),
            NumberValue::Double(x) => x.to_string(),
        },
        ValidatedDefaultValue::Duration(t, x) => t.get_value_string(*x),
        ValidatedDefaultValue::Enum(x, variant) => {
            format!(
                "{}_{}_{}",
                x.settings.c_ffi_prefix.capital_snake_case(),
                x.name.capital_snake_case(),
                variant.capital_snake_case()
            )
        }
        ValidatedDefaultValue::String(x) => format!("\"{}\"", x),
        ValidatedDefaultValue::DefaultStruct(handle, _, name) => {
            format!(
                "{}_{}_{}()",
                &handle.settings().c_ffi_prefix,
                handle.name(),
                name
            )
        }
    }
}

fn write_struct_initializer<T>(
    f: &mut dyn Printer,
    initializer: &Initializer<Validated>,
    handle: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CType + TypeExtractor,
{
    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(f, &initializer.doc.brief)?;

        if !initializer.values.is_empty() {
            f.newline()?;
            f.writeln("@note")?;
            for value in initializer.values.iter() {
                f.writeln(&format!("{} is initialized to {}", value.name, value.value))?;
            }
            f.newline()?;
        }

        f.writeln("@returns ")?;
        docstring_print(f, &text(&format!("New instance of {}", handle.name())))?;

        Ok(())
    })?;

    let params = handle
        .fields()
        .filter(|f| !initializer.values.iter().any(|cf| cf.name == f.name))
        .map(|el| format!("{} {}", el.field_type.to_c_type(), el.name))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "static {} {}_{}_{}({})",
        handle.to_c_type(),
        handle.declaration.inner.settings.c_ffi_prefix,
        handle.name(),
        initializer.name,
        params
    ))?;

    blocked(f, |f| {
        f.writeln(&format!("{} _return_value = {{", handle.to_c_type()))?;
        indented(f, |f| {
            for (field, last) in handle.fields.iter().with_last() {
                let value: String = match initializer.values.iter().find(|x| x.name == field.name) {
                    Some(x) => get_default_value(&x.value),
                    None => field.name.to_string(),
                };
                let value = if last { value } else { format!("{},", value) };
                f.writeln(&value)?;
            }
            Ok(())
        })?;

        f.writeln("};")?;
        f.writeln("return _return_value;")
    })?;

    Ok(())
}

fn write_enum_definition(
    f: &mut dyn Printer,
    handle: &Handle<Enum<Validated>>,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc))?;

    f.writeln(&format!("typedef enum {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for variant in &handle.variants {
            doxygen(f, |f| doxygen_print(f, &variant.doc))?;
            f.writeln(&format!(
                "{}_{}_{} = {},",
                handle.settings.c_ffi_prefix.capital_snake_case(),
                handle.name.capital_snake_case(),
                variant.name.capital_snake_case(),
                variant.value
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))?;

    f.newline()?;

    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(f, &text("Converts the enum to a string"))?;
        f.writeln("@param value Enum to convert")?;
        f.writeln("@returns String representation")
    })?;
    f.writeln(&format!(
        "static const char* {}_{}_to_string({} value)",
        handle.settings.c_ffi_prefix,
        handle.name,
        handle.to_c_type()
    ))?;
    blocked(f, |f| {
        f.writeln("switch (value)")?;
        blocked(f, |f| {
            for variant in &handle.variants {
                f.writeln(&format!(
                    "case {}_{}_{}: return \"{}\";",
                    handle.settings.c_ffi_prefix.capital_snake_case(),
                    handle.name.capital_snake_case(),
                    variant.name.capital_snake_case(),
                    variant.name
                ))?;
            }
            f.writeln(&format!(
                "default: return \"unknown {} value\";",
                handle.name
            ))
        })
    })
}

fn write_class_declaration(
    f: &mut dyn Printer,
    handle: &ClassDeclarationHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "typedef struct {} {};",
        handle.to_c_type(),
        handle.to_c_type()
    ))
}

fn write_function_docs(
    f: &mut dyn Printer,
    handle: &Handle<Function<Validated>>,
) -> FormattingResult<()> {
    doxygen(f, |f| {
        // Print top-level documentation
        doxygen_print(f, &handle.doc)?;

        // Print each parameter value
        for param in &handle.arguments {
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc)?;
            if let FunctionArgument::Basic(BasicType::Duration(mapping)) = param.arg_type {
                f.write(&format!(" ({})", mapping.unit()))?;
            }
        }

        fn write_error_return_doc(f: &mut dyn Printer) -> FormattingResult<()> {
            f.writeln("@return Error code")
        }

        match handle.get_signature_type() {
            SignatureType::NoErrorNoReturn => {}
            SignatureType::NoErrorWithReturn(ret, doc) => {
                f.writeln("@return ")?;
                docstring_print(f, &doc)?;
                if let FunctionReturnValue::Basic(BasicType::Duration(mapping)) = ret {
                    f.write(&format!(" ({})", mapping.unit()))?;
                }
            }
            SignatureType::ErrorNoReturn(_) => {
                write_error_return_doc(f)?;
            }
            SignatureType::ErrorWithReturn(_, ret, doc) => {
                f.writeln("@param out ")?;
                docstring_print(f, &doc)?;
                if let FunctionReturnValue::Basic(BasicType::Duration(mapping)) = ret {
                    f.write(&format!(" ({})", mapping.unit()))?;
                }
                write_error_return_doc(f)?;
            }
        }

        Ok(())
    })
}

fn write_function(
    f: &mut dyn Printer,
    handle: &Handle<Function<Validated>>,
) -> FormattingResult<()> {
    write_function_docs(f, handle)?;

    if let Some(error_type) = &handle.error_type.get() {
        f.writeln(&format!(
            "{} {}_{}(",
            error_type.inner.to_c_type(),
            &handle.settings.c_ffi_prefix,
            handle.name
        ))?;
    } else {
        f.writeln(&format!(
            "{} {}_{}(",
            handle.return_type.to_c_type(),
            &handle.settings.c_ffi_prefix,
            handle.name
        ))?;
    }

    f.write(
        &handle
            .arguments
            .iter()
            .map(|param| format!("{} {}", param.arg_type.to_c_type(), param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    if handle.error_type.is_some() {
        if let Some(x) = &handle.return_type.get_value() {
            if !handle.arguments.is_empty() {
                f.write(", ")?;
                f.write(&format!("{}* out", x.to_c_type()))?;
            }
        }
    }

    f.write(");")
}

pub(crate) fn callback_parameters(func: &CallbackFunction<Validated>) -> String {
    func.arguments
        .iter()
        .map(|arg| arg.arg_type.to_c_type())
        .chain(std::iter::once("void*".to_string()))
        .collect::<Vec<String>>()
        .join(", ")
}

fn write_interface(
    f: &mut dyn Printer,
    handle: &Handle<Interface<Validated>>,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc))?;

    let struct_name = handle.to_c_type();

    let ctx_variable_name = handle.settings.interface.context_variable_name.clone();
    let destroy_func_name = handle.settings.interface.destroy_func_name.clone();

    f.writeln(&format!("typedef struct {}", struct_name))?;
    f.writeln("{")?;
    indented(f, |f| {
        for cb in &handle.callbacks {
            f.newline()?;

            // Print the documentation
            doxygen(f, |f| {
                // Print top-level documentation
                doxygen_print(f, &cb.doc)?;

                // Print each argument value
                for arg in &cb.arguments {
                    f.writeln(&format!("@param {} ", arg.name))?;
                    docstring_print(f, &arg.doc)?;
                }

                f.writeln(&format!("@param {} ", ctx_variable_name))?;
                docstring_print(f, &text("Context data"))?;

                // Print return documentation
                if let Some(doc) = &cb.return_type.get_doc() {
                    f.writeln("@return ")?;
                    docstring_print(f, doc)?;
                }

                Ok(())
            })?;

            f.newline()?;

            // Print function signature
            f.write(&format!("{} (*{})(", cb.return_type.to_c_type(), cb.name))?;

            f.write(&callback_parameters(cb))?;

            f.write(");")?;
        }

        doxygen(f, |f| {
            f.writeln(
                "@brief Callback when the underlying owner doesn't need the interface anymore",
            )?;
            f.writeln("@param arg Context data")
        })?;
        f.writeln(&format!("void (*{})(void* arg);", destroy_func_name))?;

        doxygen(f, |f| f.writeln("@brief Context data"))?;
        f.writeln(&format!("void* {};", ctx_variable_name))?;

        Ok(())
    })?;
    f.writeln(&format!("}} {};", struct_name))?;

    f.newline()?;

    // Write init helper
    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(f, &text("Initialize an instance of the interface"))?;
        for cb in &handle.callbacks {
            f.writeln(&format!("@param {} ", cb.name))?;
            docstring_print(f, &cb.doc.brief)?;
        }
        f.writeln(&format!(
            "@param {} Callback when the underlying owner doesn't need the interface anymore",
            destroy_func_name
        ))?;
        f.writeln(&format!("@param {} Context data", ctx_variable_name))?;
        Ok(())
    })?;
    f.writeln(&format!(
        "static {} {}_{}_init(",
        struct_name, &handle.settings.c_ffi_prefix, handle.name
    ))?;
    indented(f, |f| {
        for cb in &handle.callbacks {
            f.writeln(&format!("{} (*{})(", cb.return_type.to_c_type(), cb.name,))?;

            f.write(&callback_parameters(cb))?;
            f.write("),")?;
        }

        f.writeln(&format!("void (*{})(void* arg),", destroy_func_name))?;
        f.writeln(&format!("void* {}", ctx_variable_name))?;

        Ok(())
    })?;
    f.writeln(")")?;

    blocked(f, |f| {
        f.writeln(&format!("{} _return_value = {{", struct_name))?;
        indented(f, |f| {
            for cb in &handle.callbacks {
                f.writeln(&format!("{},", cb.name))?;
            }
            f.writeln(&format!("{},", destroy_func_name))?;
            f.writeln(ctx_variable_name.as_ref())
        })?;
        f.writeln("};")?;
        f.writeln("return _return_value;")
    })?;

    Ok(())
}
