use crate::*;
use oo_bindgen::structs::*;
use oo_bindgen::types::DurationType;

trait DotNetVisibility {
    fn to_str(&self) -> &str;
}

impl DotNetVisibility for Visibility {
    fn to_str(&self) -> &str {
        match self {
            Visibility::Public => "public",
            Visibility::Private => "internal",
        }
    }
}

fn get_field_value<T>(
    field: &StructField<T, Validated>,
    constructor: &Initializer<Validated>,
) -> String
where
    T: StructFieldType,
{
    match constructor.values.iter().find(|x| x.name == field.name) {
        Some(x) => match &x.value {
            ValidatedDefaultValue::Bool(x) => x.to_string(),
            ValidatedDefaultValue::Numeric(x) => match x {
                Number::U8(x) => x.to_string(),
                Number::S8(x) => x.to_string(),
                Number::U16(x) => x.to_string(),
                Number::S16(x) => x.to_string(),
                Number::U32(x) => x.to_string(),
                Number::S32(x) => x.to_string(),
                Number::U64(x) => x.to_string(),
                Number::S64(x) => x.to_string(),
                Number::Float(x) => format!("{}F", x),
                Number::Double(x) => x.to_string(),
            },
            ValidatedDefaultValue::Duration(t, x) => match t {
                DurationType::Milliseconds => {
                    format!("TimeSpan.FromMilliseconds({})", t.get_value_string(*x))
                }
                DurationType::Seconds => {
                    format!("TimeSpan.FromSeconds({})", t.get_value_string(*x))
                }
            },
            ValidatedDefaultValue::Enum(x, variant) => {
                format!("{}.{}", x.name.camel_case(), variant.camel_case())
            }
            ValidatedDefaultValue::String(x) => format!("\"{}\"", x),
            ValidatedDefaultValue::DefaultStruct(handle, _, _) => {
                format!("new {}()", handle.name().camel_case())
            }
        },
        None => field.name.mixed_case(),
    }
}

fn write_static_constructor<T>(
    f: &mut dyn Printer,
    handle: &Struct<T, Validated>,
    constructor: &Handle<Initializer<Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + DotnetType,
{
    write_constructor_documentation(f, handle, constructor, true)?;

    let invocation_args = handle
        .fields()
        .map(|sf| get_field_value(sf, constructor))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "public static {} {}({})",
        handle.name().camel_case(),
        constructor.name.camel_case(),
        constructor_parameters(handle, constructor)
    ))?;

    blocked(f, |f| {
        f.writeln(&format!(
            "return new {}({});",
            handle.declaration.name().camel_case(),
            invocation_args
        ))
    })
}

fn write_constructor_documentation<T>(
    f: &mut dyn Printer,
    handle: &Struct<T, Validated>,
    constructor: &Handle<Initializer<Validated>>,
    write_return_info: bool,
) -> FormattingResult<()>
where
    T: StructFieldType + DotnetType,
{
    documentation(f, |f| {
        xmldoc_print(f, &constructor.doc)?;

        for arg in handle.initializer_args(constructor.clone()) {
            f.writeln(&format!("<param name=\"{}\">", arg.name.mixed_case()))?;
            docstring_print(f, &arg.doc.brief)?;
            f.write("</param>")?;
        }

        if write_return_info {
            f.writeln(&format!(
                "<returns> initialized {} instance </returns>",
                handle.name().camel_case()
            ))?;
        }

        Ok(())
    })
}

fn constructor_parameters<T>(
    handle: &Struct<T, Validated>,
    constructor: &Handle<Initializer<Validated>>,
) -> String
where
    T: StructFieldType + DotnetType,
{
    handle
        .initializer_args(constructor.clone())
        .map(|sf| {
            format!(
                "{} {}",
                sf.field_type.as_dotnet_type(),
                sf.name.mixed_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn write_constructor<T>(
    f: &mut dyn Printer,
    visibility: Visibility,
    handle: &Struct<T, Validated>,
    constructor: &Handle<Initializer<Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + DotnetType,
{
    if visibility == Visibility::Public && handle.visibility == Visibility::Public {
        write_constructor_documentation(f, handle, constructor, false)?;
    }

    let visibility = match visibility {
        Visibility::Public => handle.visibility,
        Visibility::Private => Visibility::Private,
    };

    f.writeln(&format!(
        "{} {}({})",
        visibility.to_str(),
        handle.name().camel_case(),
        constructor_parameters(handle, constructor)
    ))?;
    blocked(f, |f| {
        for field in &handle.fields {
            indented(f, |f| {
                f.writeln(&format!(
                    "this.{} = {};",
                    field.name.camel_case(),
                    get_field_value(field, constructor)
                ))
            })?;
        }
        Ok(())
    })?;
    Ok(())
}

pub(crate) fn generate<T>(
    f: &mut dyn Printer,
    handle: &Struct<T, Validated>,
    lib: &Library,
) -> FormattingResult<()>
where
    T: StructFieldType + DotnetType,
{
    let struct_name = handle.name().camel_case();
    let struct_native_name = format!("{}Native", struct_name);

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    let doc = match handle.visibility {
        Visibility::Public => handle.doc.clone(),
        Visibility::Private => handle
            .doc
            .clone()
            .warning("This class is an opaque handle and cannot be constructed by user code"),
    };

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &doc)
        })?;

        f.writeln(&format!("public class {}", struct_name))?;
        blocked(f, |f| {
            // Write .NET structure elements
            for field in handle.fields() {
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &field.doc)?;
                    Ok(())
                })?;

                f.writeln(&format!(
                    "{} {} {};",
                    handle.visibility.to_str(),
                    field.field_type.as_dotnet_type(),
                    field.name.camel_case()
                ))?;
            }

            for c in &handle.initializers {
                match c.initializer_type {
                    InitializerType::Normal => {
                        f.newline()?;
                        write_constructor(f, Visibility::Public, handle, c)?;
                    }
                    InitializerType::Static => {
                        f.newline()?;
                        write_static_constructor(f, handle, c)?;
                    }
                }
            }

            // If the struct doesn't already define a full constructor, write a private one
            if !handle.has_full_initializer() {
                let constructor = Handle::new(Initializer::full(InitializerType::Normal, doc));

                f.newline()?;
                write_constructor(f, Visibility::Private, handle, &constructor)?;
            }

            if !handle.has_default_initializer() {
                // Internal parameter-less constructor
                f.newline()?;
                f.writeln(&format!("internal {}() {{ }}", handle.name().camel_case()))?;
            }

            Ok(())
        })?;

        f.newline()?;

        // Write native struct
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}", struct_native_name))?;
        blocked(f, |f| {
            // Write native elements
            for el in handle.fields() {
                f.writeln(&format!(
                    "{} {};",
                    el.field_type.as_native_type(),
                    el.name.camel_case()
                ))?;
            }

            f.newline()?;

            // Convert from .NET to native
            f.writeln(&format!(
                "internal static {} ToNative({} self)",
                struct_native_name, struct_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result;", struct_native_name))?;
                for el in handle.fields() {
                    let el_name = el.name.camel_case();

                    let conversion = el
                        .field_type
                        .convert_to_native(&format!("self.{}", el_name))
                        .unwrap_or(format!("self.{}", el_name));
                    f.writeln(&format!("result.{} = {};", el_name, conversion))?;
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Convert from native to .NET
            f.writeln(&format!(
                "internal static {} FromNative({} native)",
                struct_name, struct_native_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result = new {}();", struct_name, struct_name))?;
                for el in handle.fields() {
                    let el_name = el.name.camel_case();

                    let conversion = el
                        .field_type
                        .convert_from_native(&format!("native.{}", el_name))
                        .unwrap_or(format!("native.{}", el_name));
                    f.writeln(&format!("result.{} = {};", el_name, conversion))?;
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Convert from .NET to native reference
            f.writeln(&format!(
                "internal static IntPtr ToNativeRef({} self)",
                struct_name
            ))?;
            blocked(f, |f| {
                f.writeln("var handle = IntPtr.Zero;")?;
                f.writeln("if (self != null)")?;
                blocked(f, |f| {
                    f.writeln("var nativeStruct = ToNative(self);")?;
                    f.writeln("handle = Marshal.AllocHGlobal(Marshal.SizeOf(nativeStruct));")?;
                    f.writeln("Marshal.StructureToPtr(nativeStruct, handle, false);")?;
                    f.writeln("nativeStruct.Dispose();")
                })?;
                f.writeln("return handle;")
            })?;

            f.newline()?;

            // Ref cleanup
            f.writeln("internal static void NativeRefCleanup(IntPtr native)")?;
            blocked(f, |f| {
                f.writeln("if (native != IntPtr.Zero)")?;
                blocked(f, |f| f.writeln("Marshal.FreeHGlobal(native);"))
            })?;

            f.newline()?;

            // Convert from native ref to .NET
            f.writeln(&format!(
                "internal static {} FromNativeRef(IntPtr native)",
                struct_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} handle = null;", struct_name))?;
                f.writeln("if (native != IntPtr.Zero)")?;
                blocked(f, |f| {
                    f.writeln(&format!(
                        "var nativeStruct = Marshal.PtrToStructure<{}>(native);",
                        struct_native_name
                    ))?;
                    f.writeln("handle = FromNative(nativeStruct);")
                })?;
                f.writeln("return handle;")
            })?;

            f.newline()?;

            // Finalizer
            f.writeln("internal void Dispose()")?;
            blocked(f, |f| {
                for el in handle.fields() {
                    let el_name = el.name.camel_case();

                    if let Some(cleanup) = el.field_type.cleanup(&format!("this.{}", el_name)) {
                        f.writeln(&cleanup)?;
                    }
                }
                Ok(())
            })
        })
    })
}
