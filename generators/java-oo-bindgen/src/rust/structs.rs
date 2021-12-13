use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::*;

use crate::rust::conversion::*;

pub(crate) fn generate(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    let mut filename = config.rust_source_dir();
    filename.push("structs");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    f.newline()?;

    generate_top_level_cache(&mut f, lib)?;

    f.newline()?;

    f.writeln("mod instances {")?;

    indented(&mut f, |f| {
        f.writeln("use std::str::FromStr;")?;
        generate_structs(f, lib, config)
    })?;
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
    config: &JavaBindgenConfig,
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
    config: &JavaBindgenConfig,
) -> FormattingResult<()>
where
    T: StructFieldType + JniTypeId + UnwrapValue + ConvertibleToRust,
{
    let lib_path = config.java_signature_path(&structure.declaration.inner.settings.name);
    let struct_name = structure.name().camel_case();
    let ffi_struct_name = format!("{}::ffi::{}", config.ffi_name, struct_name);

    f.newline()?;
    f.writeln(&format!("impl {}", struct_name))?;
    blocked(f, |f| {
        f.writeln(&format!("pub(crate) fn to_rust(&self, _cache: &crate::JCache, _env: &jni::JNIEnv, obj: jni::sys::jobject) -> {}", ffi_struct_name))?;
        blocked(f, |f| {
            // retrieve the fields from the jobject
            for field in structure.fields() {
                f.writeln(&format!("let {} = _env.get_field_unchecked(obj, self.{}, jni::signature::JavaType::from_str(\"{}\").unwrap()).unwrap().{};", field.name, field.name, field.field_type.jni_type_id().as_string(&lib_path), field.field_type.unwrap_value()))?;
            }

            f.newline()?;

            // transform the fields and shadow the variables
            for field in structure.fields() {
                if let Some(converted) = field.field_type.to_rust(&field.name) {
                    f.writeln(&format!("let {} = {};", field.name, converted))?;
                }
            }

            f.newline()?;
            f.writeln(&ffi_struct_name)?;
            blocked(f, |f| {
                for field in structure.fields() {
                    if let Some(converted) = field.field_type.call_site(&field.name) {
                        f.writeln(&format!("{}: {},", field.name, converted))?;
                    } else {
                        f.writeln(&format!("{},", field.name))?;
                    }
                }

                Ok(())
            })?;

            Ok(())
        })
    })
}

fn generate_conversion_to_jni<T>(
    f: &mut dyn Printer,
    structure: &Handle<Struct<T, Validated>>,
    config: &JavaBindgenConfig,
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
            f.writeln("_env.with_local_frame(0, || {")?;
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
    config: &JavaBindgenConfig,
) -> FormattingResult<()>
where
    T: StructFieldType + JniTypeId,
{
    let lib_path = config.java_signature_path(&structure.declaration.inner.settings.name);
    let struct_name = structure.name().camel_case();
    let struct_sig = format!("\"L{}/{};\"", lib_path, struct_name);

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
