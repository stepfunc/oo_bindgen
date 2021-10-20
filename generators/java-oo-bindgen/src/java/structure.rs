use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::structs::{
    Constructor, ConstructorType, Struct, StructField, StructFieldType,
    ValidatedConstructorDefault, Visibility,
};
use oo_bindgen::types::DurationType;

fn constructor_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Public => "public",
        Visibility::Private => "private",
    }
}

fn field_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Public => "public",
        Visibility::Private => "private final",
    }
}

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

fn get_field_value<T>(field: &StructField<T>, constructor: &Constructor) -> String
where
    T: StructFieldType,
{
    match constructor.values.iter().find(|x| x.name == field.name) {
        Some(x) => match &x.value {
            ValidatedConstructorDefault::Bool(x) => x.to_string(),
            ValidatedConstructorDefault::Uint8(x) => format!("UByte.valueOf({})", x),
            ValidatedConstructorDefault::Sint8(x) => format!("(byte) {}", x),
            ValidatedConstructorDefault::Uint16(x) => format!("UShort.valueOf({})", x),
            ValidatedConstructorDefault::Sint16(x) => format!("(short) {}", x),
            ValidatedConstructorDefault::Uint32(x) => format!("UInteger.valueOf({}L)", x),
            ValidatedConstructorDefault::Sint32(x) => x.to_string(),
            ValidatedConstructorDefault::Uint64(x) => format!("ULong.valueOf({}L)", x),
            ValidatedConstructorDefault::Sint64(x) => x.to_string(),
            ValidatedConstructorDefault::Float(x) => format!("{}F", x),
            ValidatedConstructorDefault::Double(x) => x.to_string(),
            ValidatedConstructorDefault::Duration(t, x) => match t {
                DurationType::Milliseconds => {
                    format!("java.time.Duration.ofMillis({})", t.get_value_string(*x))
                }
                DurationType::Seconds => {
                    format!("java.time.Duration.ofSeconds({})", t.get_value_string(*x))
                }
            },
            ValidatedConstructorDefault::Enum(x, variant) => {
                format!(
                    "{}.{}",
                    x.name.to_camel_case(),
                    variant.to_shouty_snake_case()
                )
            }
            ValidatedConstructorDefault::String(x) => format!("\"{}\"", x),
            ValidatedConstructorDefault::DefaultStruct(handle, _, _) => {
                format!("new {}()", handle.name().to_camel_case(),)
            }
        },
        None => field.name.to_mixed_case(),
    }
}

fn write_constructor_docs<T>(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &Struct<T>,
    constructor: &Handle<Constructor>,
) -> FormattingResult<()>
where
    T: StructFieldType,
{
    documentation(f, |f| {
        f.newline()?;
        javadoc_print(f, &constructor.doc, lib)?;
        f.newline()?;

        for field in handle.constructor_args(constructor.clone()) {
            f.writeln(&format!("@param {} ", field.name.to_mixed_case()))?;
            docstring_print(f, &field.doc.brief, lib)?;
        }

        Ok(())
    })
}

fn write_static_method_constructor<T>(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &Struct<T>,
    constructor: &Handle<Constructor>,
) -> FormattingResult<()>
where
    T: StructFieldType + JavaType,
{
    write_constructor_docs(f, lib, handle, constructor)?;

    let params = handle
        .constructor_args(constructor.clone())
        .map(|sf| {
            format!(
                "{} {}",
                sf.field_type.as_java_primitive(),
                sf.name.to_mixed_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let invocation_args = handle
        .fields()
        .filter(|f| !constructor.values.iter().any(|cf| cf.name == f.name))
        .map(|sf| sf.name.to_mixed_case())
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "public static {}({})",
        constructor.name.to_camel_case(),
        params
    ))?;

    blocked(f, |f| {
        indented(f, |f| {
            f.writeln(&format!(
                "return new {}({});",
                handle.name().to_camel_case(),
                invocation_args
            ))
        })
    })
}

fn write_constructor<T>(
    f: &mut dyn Printer,
    visibility: Visibility,
    lib: &Library,
    handle: &Struct<T>,
    constructor: &Handle<Constructor>,
) -> FormattingResult<()>
where
    T: StructFieldType + JavaType,
{
    if visibility == Visibility::Public && handle.visibility == Visibility::Public {
        write_constructor_docs(f, lib, handle, constructor)?;
    }

    let params = handle
        .constructor_args(constructor.clone())
        .map(|sf| {
            format!(
                "{} {}",
                sf.field_type.as_java_primitive(),
                sf.name.to_mixed_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let visibility = match visibility {
        Visibility::Public => constructor_visibility(handle.visibility),
        Visibility::Private => constructor_visibility(Visibility::Private),
    };

    f.writeln(&format!(
        "{} {}({})",
        visibility,
        handle.name().to_camel_case(),
        params
    ))?;
    blocked(f, |f| {
        for field in &handle.fields {
            indented(f, |f| {
                f.writeln(&format!(
                    "this.{} = {};",
                    field.name.to_mixed_case(),
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

        for c in &st.constructors {
            f.newline()?;
            match &c.constructor_type {
                ConstructorType::Normal => {
                    write_constructor(f, Visibility::Public, lib, st, c)?;
                }
                ConstructorType::Static => {
                    // write a private constructor that the static method will call
                    write_constructor(f, Visibility::Private, lib, st, c)?;
                    f.newline()?;
                    write_static_method_constructor(f, lib, st, c)?;
                }
            }
        }

        // write a full constructor that initializes all values if none are present
        if st.constructors.is_empty() {
            let constructor = Handle::new(Constructor::full(
                "init".into(),
                ConstructorType::Normal,
                format!(
                    "Initialize all values of {{struct:{}}}",
                    st.declaration().name
                )
                .into(),
            ));
            write_constructor(f, Visibility::Public, lib, st, &constructor)?;
        }

        Ok(())
    })
}
