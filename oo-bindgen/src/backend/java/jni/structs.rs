use crate::backend::java::jni::conversion::*;
use crate::backend::java::jni::JniBindgenConfig;
use crate::backend::*;
use crate::model::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JniBindgenConfig,
) -> FormattingResult<()> {
    f.newline()?;

    generate_top_level_cache(f, lib)?;

    f.newline()?;

    f.writeln("mod instances {")?;
    indented(f, |f| generate_structs(f, lib, config))?;
    f.writeln("}")
}

fn generate_top_level_cache(f: &mut dyn Printer, lib: &Library) -> FormattingResult<()> {
    // Top-level enums struct
    f.writeln("pub struct Structs")?;
    blocked(f, |f| {
        for structure in lib.structs() {
            f.writeln(&format!(
                "pub {}: instances::{},",
                structure.name(),
                structure.name().camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Structs")?;
    blocked(f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln("Self")?;
            blocked(f, |f| {
                for structure in lib.structs() {
                    f.writeln(&format!(
                        "{}: instances::{}::init(env),",
                        structure.name(),
                        structure.name().camel_case()
                    ))?;
                }
                Ok(())
            })
        })
    })
}

fn generate_structs(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JniBindgenConfig,
) -> FormattingResult<()> {
    // Each struct implementation
    for st in lib.structs() {
        match st {
            StructType::FunctionArg(x) => {
                generate_struct_fields(f, x)?;
                generate_struct_init(f, x, config)?;
                generate_conversion_to_rust(f, x, config)?;
            }
            StructType::FunctionReturn(x) => {
                generate_struct_fields(f, x)?;
                generate_struct_init(f, x, config)?;
                generate_conversion_to_jni(f, x, config)?;
            }
            StructType::CallbackArg(x) => {
                generate_struct_fields(f, x)?;
                generate_struct_init(f, x, config)?;
                generate_conversion_to_jni(f, x, config)?;
            }
            StructType::Universal(x) => {
                generate_struct_fields(f, x)?;
                generate_struct_init(f, x, config)?;
                generate_conversion_to_rust(f, x, config)?;
                generate_conversion_to_jni(f, x, config)?;
            }
        }
    }

    Ok(())
}

fn generate_conversion_to_rust<T>(
    f: &mut dyn Printer,
    structure: &Handle<Struct<T, Validated>>,
    config: &JniBindgenConfig,
) -> FormattingResult<()>
where
    T: StructFieldType + UnwrapValue + ConvertibleToRust + GuardType + JniJavaType,
{
    let struct_name = structure.name().camel_case();
    let ffi_struct_name = format!("{}::ffi::{}", config.ffi_name, struct_name);
    let guard_struct_name = format!("{}Guard", struct_name);

    f.newline()?;

    f.writeln(&format!(
        "/// Guard object ensures a {} struct is valid",
        ffi_struct_name
    ))?;
    f.writeln(&format!("pub(crate) struct {}<'a> {{", guard_struct_name))?;
    indented(f, |f| {
        if !structure
            .fields
            .iter()
            .any(|x| x.field_type.guard_type().is_some())
        {
            f.writeln("/// empty guard objects require this field to make use of the lifetime")?;
            f.writeln("_phantom: std::marker::PhantomData<&'a usize>,")?;
        }

        for x in structure.fields.iter() {
            if let Some(guard_type) = x.field_type.guard_type() {
                f.writeln(&format!("/// guard for the {} field", x.name))?;
                f.writeln(&format!("{}: {},", x.name, guard_type))?;
            }
        }

        Ok(())
    })?;
    f.writeln("}")?;

    f.newline()?;
    f.writeln(&format!("impl {}", struct_name))?;
    blocked(f, |f| {
        f.writeln(&format!("pub(crate) fn to_rust<'a>(&self, _cache: &'a crate::JCache, _env: &'a jni::JNIEnv, obj: jni::sys::jobject) -> ({}<'a>, {})", guard_struct_name, ffi_struct_name))?;
        blocked(f, |f| {
            // retrieve the fields from the jobject
            for field in structure.fields() {
                f.writeln(&format!(
                    "let {} = _env.get_field_unchecked(obj, self.{}, {}).unwrap().{};",
                    field.name,
                    field.name,
                    field.field_type.jni_java_type(),
                    field.field_type.unwrap_value()
                ))?;
            }

            f.newline()?;

            // transform the fields and shadow the variables
            for field in structure.fields() {
                if let Some(converted) = field.field_type.to_rust(&field.name) {
                    f.writeln(&format!("let {} = {};", field.name, converted))?;
                }
            }

            f.newline()?;
            f.writeln(&format!("let _ffi_struct = {} {{", ffi_struct_name))?;
            indented(f, |f| {
                for field in structure.fields() {
                    if let Some(converted) = field.field_type.call_site(&field.name) {
                        f.writeln(&format!("{}: {},", field.name, converted))?;
                    } else {
                        f.writeln(&format!("{},", field.name))?;
                    }
                }

                Ok(())
            })?;
            f.writeln("};")?;

            f.newline()?;

            f.writeln(&format!("let _guard = {} {{", guard_struct_name))?;
            indented(f, |f| {
                if structure
                    .fields
                    .iter()
                    .any(|f| f.field_type.guard_type().is_some())
                {
                    for field in structure
                        .fields
                        .iter()
                        .filter(|x| x.field_type.guard_type().is_some())
                    {
                        if let Some(transform) = field.field_type.guard_transform(&field.name) {
                            f.writeln(&format!("{} : {},", field.name, transform))?;
                        } else {
                            f.writeln(&format!("{},", field.name))?;
                        }
                    }
                } else {
                    f.writeln("_phantom: std::marker::PhantomData::default(),")?;
                }
                Ok(())
            })?;
            f.writeln("};")?;

            f.newline()?;

            f.writeln("(_guard, _ffi_struct)")
        })
    })
}

fn generate_conversion_to_jni<T>(
    f: &mut dyn Printer,
    structure: &Handle<Struct<T, Validated>>,
    config: &JniBindgenConfig,
) -> FormattingResult<()>
where
    T: StructFieldType + MaybeConvertibleToJni,
{
    let struct_name = structure.name().camel_case();
    let ffi_struct_name = format!("{}::ffi::{}", config.ffi_name, struct_name);

    f.newline()?;
    f.writeln(&format!("impl {}", struct_name))?;
    blocked(f, |f| {
        f.writeln(&format!("pub(crate) fn to_jni(&self, _cache: &crate::JCache, _env: &jni::JNIEnv, value: &{}) -> jni::sys::jobject", ffi_struct_name))?;
        blocked(f, |f| {
            f.writeln("// automatically free all local references except for the struct itself which is returned")?;
            f.writeln("// upper bound on the number of references in the local frame is the number of fields + 1 for struct itself")?;
            f.writeln(&format!(
                "_env.with_local_frame({}, || {{",
                structure.fields.len() + 1
            ))?;
            indented(f, |f| {
                f.writeln("let obj = _env.alloc_object(&self._class).unwrap();")?;
                for field in structure.fields() {
                    let field_name = format!("value.{}", field.name);
                    let conversion = field
                        .field_type
                        .maybe_convert(&field_name)
                        .unwrap_or(field_name);
                    f.writeln(&format!(
                        "_env.set_field_unchecked(obj, self.{}, {}.into()).unwrap();",
                        field.name, conversion,
                    ))?;
                }

                f.writeln("Ok(obj)")
            })?;
            f.writeln("}).unwrap().into_inner()")
        })
    })
}

fn generate_struct_fields<T>(
    f: &mut dyn Printer,
    structure: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType,
{
    let struct_name = structure.name().camel_case();

    f.newline()?;
    f.writeln(&format!("pub struct {}", struct_name))?;
    blocked(f, |f| {
        f.writeln("_class: jni::objects::GlobalRef,")?;
        for field in structure.fields() {
            f.writeln(&format!("{}: jni::objects::JFieldID<'static>,", field.name))?;
        }
        Ok(())
    })
}

fn generate_struct_init<T>(
    f: &mut dyn Printer,
    structure: &Handle<Struct<T, Validated>>,
    config: &JniBindgenConfig,
) -> FormattingResult<()>
where
    T: StructFieldType + JniTypeId,
{
    let lib_path = config.java_signature_path(&structure.declaration.inner.settings.name);
    let struct_name = structure.name().camel_case();
    let struct_sig = format!("\"{}/{}\"", lib_path, struct_name);

    f.newline()?;
    f.writeln(&format!("impl {}", struct_name))?;
    blocked(f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln(&format!(
                "let class = env.find_class({}).expect(\"Unable to find {}\");",
                struct_sig, struct_name
            ))?;
            for field in structure.fields() {
                f.writeln(&format!("let {field_snake} = env.get_field_id(class, \"{field_mixed}\", \"{field_sig}\").map(|mid| mid.into_inner().into()).expect(\"Unable to find field {field_mixed}\");", field_snake=field.name, field_mixed=field.name.mixed_case(), field_sig=field.field_type.jni_type_id().as_string(&lib_path)))?;
            }
            f.writeln("Self")?;
            blocked(f, |f| {
                f.writeln("_class: env.new_global_ref(class).unwrap(),")?;
                for field in structure.fields() {
                    f.writeln(&format!("{},", field.name))?;
                }
                Ok(())
            })
        })
    })
}
