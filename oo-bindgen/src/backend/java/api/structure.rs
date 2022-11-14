use super::doc::*;
use super::*;

use crate::backend::java::api::nullable::IsStruct;

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
            ValidatedDefaultValue::Number(x) => match x {
                NumberValue::U8(x) => format!("UByte.valueOf({})", x),
                NumberValue::S8(x) => format!("(byte) {}", x),
                NumberValue::U16(x) => format!("UShort.valueOf({})", x),
                NumberValue::S16(x) => format!("(short) {}", x),
                NumberValue::U32(x) => format!("UInteger.valueOf({}L)", x),
                NumberValue::S32(x) => x.to_string(),
                NumberValue::U64(x) => format!("ULong.valueOf({}L)", x),
                NumberValue::S64(x) => x.to_string(),
                NumberValue::Float(x) => format!("{}F", x),
                NumberValue::Double(x) => x.to_string(),
            },
            ValidatedDefaultValue::Duration(t, x) => match t {
                DurationType::Milliseconds => {
                    format!("java.time.Duration.ofMillis({})", t.get_value_string(*x))
                }
                DurationType::Seconds => {
                    format!("java.time.Duration.ofSeconds({})", t.get_value_string(*x))
                }
            },
            ValidatedDefaultValue::Enum(x, variant) => {
                format!("{}.{}", x.name.camel_case(), variant.capital_snake_case())
            }
            ValidatedDefaultValue::String(x) => format!("\"{}\"", x),
            ValidatedDefaultValue::DefaultStruct(handle, _, _) => {
                format!("new {}()", handle.name().camel_case(),)
            }
        },
        None => field.name.mixed_case(),
    }
}

fn get_default_value_doc(x: &ValidatedDefaultValue) -> String {
    match x {
        ValidatedDefaultValue::Bool(x) => x.to_string(),
        ValidatedDefaultValue::Number(x) => x.to_string(),
        ValidatedDefaultValue::Duration(DurationType::Milliseconds, x) => {
            format!("{}ms", x.as_millis())
        }
        ValidatedDefaultValue::Duration(DurationType::Seconds, x) => format!("{}s", x.as_secs()),
        ValidatedDefaultValue::Enum(x, variant) => format!(
            "{{@link {}#{}}}",
            x.name.camel_case(),
            variant.capital_snake_case()
        ),
        ValidatedDefaultValue::String(x) => format!("\"{}\"", x),
        ValidatedDefaultValue::DefaultStruct(x, _, _) => {
            format!("Default {{@link {}}}", x.name().camel_case())
        }
    }
}

fn write_constructor_docs<T>(
    f: &mut dyn Printer,
    handle: &Struct<T, Validated>,
    constructor: &Handle<Initializer<Validated>>,
    write_return_info: bool,
) -> FormattingResult<()>
where
    T: StructFieldType,
{
    documentation(f, |f| {
        javadoc_print(f, &constructor.doc)?;

        if !constructor.values.is_empty() {
            f.newline()?;
            f.writeln("<p>Values are initialized to:</p>")?;
            f.writeln("<ul>")?;
            for value in constructor.values.iter() {
                f.writeln(&format!(
                    "<li> {{@link {}#{}}} : {}</li>",
                    handle.name().camel_case(),
                    value.name.mixed_case(),
                    get_default_value_doc(&value.value)
                ))?;
            }
            f.writeln("</ul>")?;
        }

        f.newline()?;

        for field in handle.initializer_args(constructor.clone()) {
            f.writeln(&format!("@param {} ", field.name.mixed_case()))?;
            docstring_print(f, &field.doc.brief)?;
        }

        if write_return_info {
            f.writeln(&format!(
                "@return Initialized {{@link {}}} instance",
                handle.name().camel_case()
            ))?;
        }

        Ok(())
    })
}

fn write_static_method_constructor<T>(
    f: &mut dyn Printer,
    handle: &Struct<T, Validated>,
    constructor: &Handle<Initializer<Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + JavaType,
{
    write_constructor_docs(f, handle, constructor, true)?;

    let invocation_args = handle
        .fields()
        .map(|sf| get_field_value(sf, constructor))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "public static {} {}({})",
        handle.name().camel_case(),
        constructor.name.mixed_case(),
        constructor_args(handle, constructor)
    ))?;

    blocked(f, |f| {
        indented(f, |f| {
            f.writeln(&format!(
                "return new {}({});",
                handle.name().camel_case(),
                invocation_args
            ))
        })
    })
}

fn constructor_args<T>(
    handle: &Struct<T, Validated>,
    constructor: &Handle<Initializer<Validated>>,
) -> String
where
    T: StructFieldType + JavaType,
{
    handle
        .initializer_args(constructor.clone())
        .map(|sf| {
            format!(
                "{} {}",
                sf.field_type.as_java_primitive(),
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
    T: StructFieldType + JavaType,
{
    if visibility == Visibility::Public && handle.visibility == Visibility::Public {
        write_constructor_docs(f, handle, constructor, false)?;
    }

    let visibility = match visibility {
        Visibility::Public => constructor_visibility(handle.visibility),
        Visibility::Private => constructor_visibility(Visibility::Private),
    };

    f.writeln(&format!(
        "{} {}({})",
        visibility,
        handle.name().camel_case(),
        constructor_args(handle, constructor)
    ))?;
    blocked(f, |f| {
        for field in &handle.fields {
            indented(f, |f| {
                f.writeln(&format!(
                    "this.{} = {};",
                    field.name.mixed_case(),
                    get_field_value(field, constructor)
                ))
            })?;
        }
        Ok(())
    })?;

    Ok(())
}

fn write_null_checker<T>(f: &mut dyn Printer, handle: &Struct<T, Validated>) -> FormattingResult<()>
where
    T: StructFieldType + Nullable + IsStruct,
{
    f.writeln("void _assertFieldsNotNull()")?;
    blocked(f, |f| {
        for field in handle.fields.iter() {
            if field.field_type.is_nullable() {
                let field_name = field.name.mixed_case();
                f.writeln(&format!(
                    "java.util.Objects.requireNonNull({}, \"{} cannot be null\");",
                    field_name, field_name
                ))?;
                if field.field_type.is_struct() {
                    f.writeln(&format!("{}._assertFieldsNotNull();", field_name))?;
                }
            }
        }
        Ok(())
    })
}

pub(crate) fn generate<T>(
    f: &mut dyn Printer,
    st: &Struct<T, Validated>,
    generate_builder_methods: bool,
) -> FormattingResult<()>
where
    T: StructFieldType + JavaType + Nullable + IsStruct,
{
    let struct_name = st.name().camel_case();

    let doc = match st.visibility {
        Visibility::Public => st.doc.clone(),
        Visibility::Private => st
            .doc
            .clone()
            .warning("This class is an opaque handle and cannot be constructed by user code"),
    };

    // Documentation
    documentation(f, |f| javadoc_print(f, &doc))?;

    // Structure definition
    f.writeln(&format!("public final class {}", struct_name))?;
    blocked(f, |f| {
        // Write Java structure fields
        for field in st.fields() {
            documentation(f, |f| {
                javadoc_print(f, &field.doc)?;
                Ok(())
            })?;

            f.writeln(&format!(
                "{} {} {};",
                field_visibility(st.visibility),
                field.field_type.as_java_primitive(),
                field.name.mixed_case()
            ))?;
        }

        // Write builder methods
        if st.visibility == Visibility::Public && generate_builder_methods {
            for field in st.fields() {
                documentation(f, |f| {
                    javadoc_print(f, &field.doc)?;
                    Ok(())
                })?;

                f.writeln(&format!(
                    "public {} with{}({} value)",
                    struct_name,
                    field.name.camel_case(),
                    field.field_type.as_java_primitive(),
                ))?;
                blocked(f, |f| {
                    f.writeln(&format!("this.{} = value;", field.name.mixed_case(),))?;
                    f.writeln("return this;")
                })?;
            }
        }

        for c in &st.initializers {
            f.newline()?;
            match &c.initializer_type {
                InitializerType::Normal => {
                    write_constructor(f, Visibility::Public, st, c)?;
                }
                InitializerType::Static => {
                    write_static_method_constructor(f, st, c)?;
                }
            }
        }

        if !st.has_full_initializer() {
            let constructor = Handle::new(Initializer::full(
                InitializerType::Normal,
                brief("Initialize all values"),
            ));

            f.newline()?;
            write_constructor(f, Visibility::Private, st, &constructor)?;
        }

        f.newline()?;
        write_null_checker(f, st)
    })
}
