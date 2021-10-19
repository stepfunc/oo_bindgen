use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::structs::{Struct, StructFieldType, Visibility};

/* TODO
fn constructor_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Public => "public",
        Visibility::Private => "private",
    }
}
*/

fn field_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Public => "public",
        Visibility::Private => "private final",
    }
}

pub(crate) fn generate<T>(
    f: &mut impl Printer,
    st: &Struct<T>,
    lib: &Library,
) -> FormattingResult<()>
where
    T: StructFieldType + JavaType,
{
    let struct_name = st.name().to_camel_case();

    let doc = match st.visibility {
        Visibility::Public => st.doc.clone(),
        Visibility::Private => st
            .doc
            .clone()
            .warning("This class is an opaque handle and cannot be constructed by user code"),
    };

    // Documentation
    documentation(f, |f| javadoc_print(f, &doc, lib))?;

    // Structure definition
    f.writeln(&format!("public final class {}", struct_name))?;
    blocked(f, |f| {
        // Write Java structure fields
        for field in st.fields() {
            documentation(f, |f| {
                javadoc_print(f, &field.doc, lib)?;
                Ok(())
            })?;

            f.writeln(&format!(
                "{} {} {};",
                field_visibility(st.visibility),
                field.field_type.as_java_primitive(),
                field.name.to_mixed_case()
            ))?;
        }

        f.newline()?;

        /* TODO
        if !native_struct.all_fields_have_defaults() {
            documentation(f, |f| {
                f.newline()?;
                docstring_print(
                    f,
                    &format!(
                        "Initialize {{struct:{}}} to default values",
                        native_struct.name()
                    )
                    .into(),
                    lib,
                )?;
                f.newline()?;

                for param in native_struct
                    .fields()
                    .filter(|f| !f.field_type.has_default())
                {
                    f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
                    docstring_print(f, &param.doc.brief, lib)?;
                }

                Ok(())
            })?;

            f.writeln(&format!(
                "{} {}(",
                constructor_visibility(native_struct.visibility()),
                struct_name,
            ))?;
            f.write(
                &native_struct
                    .fields()
                    .filter(|el| !el.field_type.has_default())
                    .map(|el| {
                        format!(
                            "{} {}",
                            el.field_type.to_any_type().as_java_primitive(),
                            el.name.to_mixed_case()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(")")?;

            blocked(f, |f| {
                for el in native_struct.fields() {
                    if !el.field_type.has_default() {
                        f.writeln(&format!(
                            "this.{} = {};",
                            el.name.to_mixed_case(),
                            el.name.to_mixed_case()
                        ))?;
                    }
                }
                Ok(())
            })?;

            f.newline()?;
        }
        */

        Ok(())
    })
}
