use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::structs::{
    Constructor, ConstructorType, Number, Struct, StructField, StructFieldType,
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

fn get_field_value<T>(field: &StructField<T>, constructor: &Constructor) -> String
where
    T: StructFieldType,
{
    match constructor.values.iter().find(|x| x.name == field.name) {
        Some(x) => match &x.value {
            ValidatedConstructorDefault::Bool(x) => x.to_string(),
            ValidatedConstructorDefault::Numeric(x) => match x {
                Number::U8(x) => format!("UByte.valueOf({})", x),
                Number::S8(x) => format!("(byte) {}", x),
                Number::U16(x) => format!("UShort.valueOf({})", x),
                Number::S16(x) => format!("(short) {}", x),
                Number::U32(x) => format!("UInteger.valueOf({}L)", x),
                Number::S32(x) => x.to_string(),
                Number::U64(x) => format!("ULong.valueOf({}L)", x),
                Number::S64(x) => x.to_string(),
                Number::Float(x) => format!("{}F", x),
                Number::Double(x) => x.to_string(),
            },
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
    write_return_info: bool,
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

        if write_return_info {
            f.writeln(&format!(
                "@return initialized {} instance",
                handle.name().to_camel_case()
            ))?;
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
    write_constructor_docs(f, lib, handle, constructor, true)?;

    let invocation_args = handle
        .fields()
        .map(|sf| get_field_value(sf, constructor))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "public static {} {}({})",
        handle.name().to_camel_case(),
        constructor.name.to_mixed_case(),
        constructor_args(handle, constructor)
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

fn constructor_args<T>(handle: &Struct<T>, constructor: &Handle<Constructor>) -> String
where
    T: StructFieldType + JavaType,
{
    handle
        .constructor_args(constructor.clone())
        .map(|sf| {
            format!(
                "{} {}",
                sf.field_type.as_java_primitive(),
                sf.name.to_mixed_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
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
        write_constructor_docs(f, lib, handle, constructor, false)?;
    }

    let visibility = match visibility {
        Visibility::Public => constructor_visibility(handle.visibility),
        Visibility::Private => constructor_visibility(Visibility::Private),
    };

    f.writeln(&format!(
        "{} {}({})",
        visibility,
        handle.name().to_camel_case(),
        constructor_args(handle, constructor)
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
                    write_static_method_constructor(f, lib, st, c)?;
                }
            }
        }

        if !st.has_full_constructor() {
            let constructor = Handle::new(Constructor::full(
                ConstructorType::Normal,
                format!(
                    "Initialize all values of {{struct:{}}}",
                    st.declaration().name
                )
                .into(),
            ));

            f.newline()?;
            write_constructor(f, Visibility::Private, lib, st, &constructor)?;
        }

        Ok(())
    })
}
