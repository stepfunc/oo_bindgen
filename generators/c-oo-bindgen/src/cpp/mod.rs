pub(crate) mod conversion;
pub(crate) mod definition;
mod formatting;
pub(crate) mod implementation;

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
