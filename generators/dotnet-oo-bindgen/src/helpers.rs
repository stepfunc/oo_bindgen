use crate::conversion::DotnetType;
use crate::*;
use heck::CamelCase;
use oo_bindgen::collection::CollectionHandle;

pub(crate) fn generate_collection_helpers(
    f: &mut dyn Printer,
    coll: &CollectionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!(
            "internal static class {}Helpers",
            coll.name().to_camel_case()
        ))?;
        blocked(f, |f| {
            // ToNative function
            f.writeln(&format!(
                "internal static IntPtr ToNative(System.Collections.Generic.ICollection<{}> value)",
                coll.item_type.as_dotnet_type()
            ))?;
            blocked(f, |f| {
                if coll.has_reserve {
                    f.writeln(&format!(
                        "var builder = {}.{}((uint)value.Count);",
                        NATIVE_FUNCTIONS_CLASSNAME, coll.create_func.name
                    ))?;
                } else {
                    f.writeln(&format!(
                        "var builder = {}.{}();",
                        NATIVE_FUNCTIONS_CLASSNAME, coll.create_func.name
                    ))?;
                }

                f.writeln("foreach (var el in value)")?;
                blocked(f, |f| {
                    let conversion = coll
                        .item_type
                        .convert_to_native("el")
                        .unwrap_or_else(|| "el".to_string());
                    f.writeln(&format!("var convertedEl = {};", conversion))?;

                    f.writeln(&format!(
                        "{}.{}(builder, convertedEl);",
                        NATIVE_FUNCTIONS_CLASSNAME, coll.add_func.name
                    ))?;

                    if let Some(cleanup) = &coll.item_type.cleanup("convertedEl") {
                        f.writeln(cleanup)?;
                    }

                    Ok(())
                })?;

                f.writeln("return builder;")
            })?;

            // Cleanup function
            f.writeln("internal static void Cleanup(IntPtr value)")?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "{}.{}(value);",
                    NATIVE_FUNCTIONS_CLASSNAME, coll.delete_func.name
                ))
            })?;

            Ok(())
        })
    })
}

pub(crate) fn generate_iterator_helpers(
    f: &mut dyn Printer,
    iter: &iterator::IteratorHandle,
    lib: &Library,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!(
            "internal static class {}Helpers",
            iter.name().to_camel_case()
        ))?;
        blocked(f, |f| {
            // ToNative function
            f.writeln(&format!("internal static System.Collections.Generic.ICollection<{}> FromNative(IntPtr value)", iter.item_type.name().to_camel_case()))?;
            blocked(f, |f| {
                let next_call = format!(
                    "{}.{}(value)",
                    NATIVE_FUNCTIONS_CLASSNAME, iter.function.name
                );

                f.writeln(&format!(
                    "var builder = ImmutableArray.CreateBuilder<{}>();",
                    iter.item_type.name().to_camel_case()
                ))?;
                f.writeln(&format!(
                    "for (var itRawValue = {}; itRawValue != IntPtr.Zero; itRawValue = {})",
                    next_call, next_call
                ))?;
                blocked(f, |f| {
                    f.writeln(&format!(
                        "{} itValue = null;",
                        iter.item_type.name().to_camel_case()
                    ))?;
                    f.writeln(&format!(
                        "itValue = {};",
                        iter.item_type.declaration()
                            .convert_from_native("itRawValue")
                            .unwrap()
                    ))?;
                    f.writeln("builder.Add(itValue);")
                })?;
                f.writeln("return builder.ToImmutable();")
            })
        })
    })
}
