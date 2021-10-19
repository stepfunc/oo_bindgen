use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::structs::*;
use oo_bindgen::types::DurationType;

fn field_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Private => "internal",
        Visibility::Public => "public",
    }
}

fn constructor_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Private => "internal",
        Visibility::Public => "public",
    }
}

fn get_field_value<T>(field: &StructField<T>, constructor: &Constructor) -> String
where
    T: StructFieldType,
{
    match constructor.values.iter().find(|x| x.name == field.name) {
        Some(x) => match &x.value {
            ValidatedConstructorDefault::Bool(x) => x.to_string(),
            ValidatedConstructorDefault::Uint8(x) => x.to_string(),
            ValidatedConstructorDefault::Sint8(x) => x.to_string(),
            ValidatedConstructorDefault::Uint16(x) => x.to_string(),
            ValidatedConstructorDefault::Sint16(x) => x.to_string(),
            ValidatedConstructorDefault::Uint32(x) => x.to_string(),
            ValidatedConstructorDefault::Sint32(x) => x.to_string(),
            ValidatedConstructorDefault::Uint64(x) => x.to_string(),
            ValidatedConstructorDefault::Sint64(x) => x.to_string(),
            ValidatedConstructorDefault::Float(x) => format!("{}F", x),
            ValidatedConstructorDefault::Double(x) => x.to_string(),
            ValidatedConstructorDefault::Duration(t, x) => match t {
                DurationType::Milliseconds => {
                    format!("TimeSpan.FromMilliseconds({})", t.get_value_string(*x))
                }
                DurationType::Seconds => {
                    format!("TimeSpan.FromSeconds({})", t.get_value_string(*x))
                }
            },
            ValidatedConstructorDefault::Enum(x, variant) => {
                format!("{}.{}", x.name.to_camel_case(), variant.to_camel_case())
            }
            ValidatedConstructorDefault::String(x) => format!("\"{}\"", x),
            ValidatedConstructorDefault::DefaultStruct(handle, _, _) => {
                format!("new {}()", handle.name().to_camel_case(),)
            }
        },
        None => field.name.to_mixed_case(),
    }
}

fn write_constructor<T>(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &Struct<T>,
    constructor: &Constructor,
) -> FormattingResult<()>
where
    T: StructFieldType + DotnetType,
{
    documentation(f, |f| {
        xmldoc_print(f, &constructor.doc, lib)?;

        for param in handle
            .fields()
            .filter(|field| !constructor.values.iter().any(|c| c.name == field.name))
        {
            f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc.brief, lib)?;
            f.write("</param>")?;
        }

        Ok(())
    })?;

    let params = handle
        .fields()
        .filter(|f| !constructor.values.iter().any(|cf| cf.name == f.name))
        .map(|sf| {
            format!(
                "{} {}",
                sf.field_type.as_dotnet_type(),
                sf.name.to_mixed_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "{} {}({})",
        constructor_visibility(handle.visibility),
        handle.name().to_camel_case(),
        params
    ))?;
    blocked(f, |f| {
        for field in &handle.fields {
            indented(f, |f| {
                f.writeln(&format!(
                    "this.{} = {};",
                    field.name.to_camel_case(),
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
    handle: &Struct<T>,
    lib: &Library,
) -> FormattingResult<()>
where
    T: StructFieldType + DotnetType,
{
    let struct_name = handle.name().to_camel_case();
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

    namespaced(f, &lib.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &doc, lib)
        })?;

        f.writeln(&format!("public class {}", struct_name))?;
        blocked(f, |f| {
            // Write .NET structure elements
            for field in handle.fields() {
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &field.doc, lib)?;
                    Ok(())
                })?;

                f.writeln(&format!(
                    "{} {} {};",
                    field_visibility(handle.visibility),
                    field.field_type.as_dotnet_type(),
                    field.name.to_camel_case()
                ))?;
            }

            for c in &handle.constructors {
                f.newline()?;
                write_constructor(f, lib, handle, c)?;
            }

            if !handle.has_default_constructor() {
                // Internal parameterless constructor
                f.newline()?;
                f.writeln(&format!(
                    "internal {}() {{ }}",
                    handle.name().to_camel_case()
                ))?;
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
                    el.name.to_camel_case()
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
                    let el_name = el.name.to_camel_case();

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
                    let el_name = el.name.to_camel_case();

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
                    let el_name = el.name.to_camel_case();

                    if let Some(cleanup) = el.field_type.cleanup(&format!("this.{}", el_name)) {
                        f.writeln(&cleanup)?;
                    }
                }
                Ok(())
            })
        })
    })
}
