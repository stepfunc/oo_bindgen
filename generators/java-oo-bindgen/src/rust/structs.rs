use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::*;

use crate::rust::conversion::*;

pub(crate) fn generate_structs_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let mut filename = config.rust_source_dir();
    filename.push("structs");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    // Imports
    f.writeln("use std::str::FromStr;")?;

    f.newline()?;

    // Top-level enums struct
    f.writeln("pub struct Structs")?;
    blocked(&mut f, |f| {
        for structure in lib.structs() {
            f.writeln(&format!(
                "pub struct_{}: Struct{},",
                structure.name(),
                structure.name().camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Structs")?;
    blocked(&mut f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln("Self")?;
            blocked(f, |f| {
                for structure in lib.structs() {
                    f.writeln(&format!(
                        "struct_{}: Struct{}::init(env),",
                        structure.name(),
                        structure.name().camel_case()
                    ))?;
                }
                Ok(())
            })
        })
    })?;

    // Each struct implementation
    for structure in lib.structs() {
        match structure {
            StructType::FunctionArg(x) => generate_struct(&mut f, x, lib, config)?,
            StructType::FunctionReturn(x) => generate_struct(&mut f, x, lib, config)?,
            StructType::CallbackArg(x) => generate_struct(&mut f, x, lib, config)?,
            StructType::Universal(x) => generate_struct(&mut f, x, lib, config)?,
        }
    }

    Ok(())
}

fn generate_struct<T>(
    f: &mut dyn Printer,
    structure: &Handle<Struct<T, Validated>>,
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()>
where
    T: StructFieldType + JniType + JniTypeId,
{
    let lib_path = config.java_signature_path(&lib.settings.name);
    let struct_name = structure.name().camel_case();
    let struct_sig = format!("\"L{}/{};\"", lib_path, struct_name);

    f.writeln(&format!("pub struct Struct{}", struct_name))?;
    blocked(f, |f| {
        f.writeln("class: jni::objects::GlobalRef,")?;
        for field in structure.fields() {
            f.writeln(&format!(
                "field_{}: jni::objects::JFieldID<'static>,",
                field.name
            ))?;
        }
        Ok(())
    })?;

    f.newline()?;

    f.writeln(&format!("impl Struct{}", struct_name))?;
    blocked(f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln(&format!(
                "let class = env.find_class({}).expect(\"Unable to find {}\");",
                struct_sig, struct_name
            ))?;
            for field in structure.fields() {
                f.writeln(&format!("let field_{field_snake} = env.get_field_id(class, \"{field_mixed}\", \"{field_sig}\").map(|mid| mid.into_inner().into()).expect(\"Unable to find field {field_mixed}\");", field_snake=field.name, field_mixed=field.name.mixed_case(), field_sig=field.field_type.jni_type_id(&lib_path)))?;
            }
            f.writeln("Self")?;
            blocked(f, |f| {
                f.writeln("class: env.new_global_ref(class).unwrap(),")?;
                for field in structure.fields() {
                    f.writeln(&format!("field_{},", field.name))?;
                }
                Ok(())
            })
        })?;

        f.newline()?;

        let ffi_struct_name = format!("{}::ffi::{}", config.ffi_name, struct_name);

        f.writeln(&format!("pub(crate) fn struct_to_rust(&self, _cache: &super::JCache, _env: &jni::JNIEnv, obj: jni::sys::jobject) -> {}", ffi_struct_name))?;
        blocked(f, |f| {
            // retrieve the fields from the jobject
            for field in structure.fields() {
                f.writeln(&format!("let {} = _env.get_field_unchecked(obj, self.field_{}, jni::signature::JavaType::from_str(\"{}\").unwrap()).unwrap().{};", field.name, field.name, field.field_type.jni_type_id(&lib_path), field.field_type.convert_jvalue()))?;
            }

            f.newline()?;

            // transform the fields and shadow the variables
            for field in structure.fields() {
                if let Some(converter) = field.field_type.conversion() {
                    let to = format!("let {} = ", field.name);
                    converter.convert_to_rust(f, field.name.as_ref(), &to)?;
                    f.write(";")?;
                }
            }

            f.newline()?;
            f.writeln(&ffi_struct_name)?;
            blocked(f, |f| {
                for field in structure.fields() {
                    if let Some(conversion) = field
                        .field_type
                        .conversion()
                        .and_then(|c| c.convert_parameter_at_call_site(&field.name))
                    {
                        f.writeln(&format!("{}: {},", field.name, conversion))?;
                    } else {
                        f.writeln(&format!("{},", field.name))?;
                    }
                }

                Ok(())
            })?;

            Ok(())
        })?;

        f.newline()?;

        f.writeln(&format!("pub(crate) fn struct_to_rust_cleanup(&self, _cache: &super::JCache, _env: &jni::JNIEnv, _value: &{})", ffi_struct_name))?;
        blocked(f, |f| {
            for field in structure.fields() {
                if let Some(conversion) = field.field_type.conversion() {
                    conversion.convert_to_rust_cleanup(f, &format!("_value.{}", field.name))?;
                }
            }

            Ok(())
        })?;

        f.newline()?;

        f.writeln(&format!("pub(crate) fn struct_from_rust(&self, _cache: &super::JCache, _env: &jni::JNIEnv, value: &{}) -> jni::sys::jobject", ffi_struct_name))?;
        blocked(f, |f| {
            f.writeln("let obj = _env.alloc_object(&self.class).unwrap();")?;
            for field in structure.fields() {
                if let Some(conversion) = field.field_type.conversion() {
                    conversion.convert_from_rust(
                        f,
                        &format!("value.{}", field.name),
                        "let temp = ",
                    )?;
                    f.write(";")?;
                } else {
                    f.writeln(&format!("let temp = value.{};", field.name))?;
                }
                f.writeln(&format!(
                    "_env.set_field_unchecked(obj, self.field_{}, temp.into()).unwrap();",
                    field.name
                ))?;

                if field.field_type.requires_local_ref_cleanup() {
                    f.writeln("_env.delete_local_ref(temp.into()).unwrap();")?;
                }
            }

            f.writeln("obj.into_inner()")
        })?;

        Ok(())
    })?;

    f.newline()
}
