use super::conversion::*;
use super::formatting::*;
use crate::*;
use heck::{CamelCase, MixedCase, SnakeCase};
use oo_bindgen::formatting::*;

pub(crate) fn generate_structs_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.name);

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
        for structure in lib.native_structs() {
            f.writeln(&format!(
                "pub struct_{}: Struct{},",
                structure.name().to_snake_case(),
                structure.name().to_camel_case()
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
                for structure in lib.native_structs() {
                    f.writeln(&format!(
                        "struct_{}: Struct{}::init(env),",
                        structure.name().to_snake_case(),
                        structure.name().to_camel_case()
                    ))?;
                }
                Ok(())
            })
        })
    })?;

    // Each struct implementation
    for structure in lib.native_structs() {
        let struct_name = structure.name().to_camel_case();
        let struct_sig = format!("\"L{}/{};\"", lib_path, struct_name);

        f.writeln(&format!("pub struct Struct{}", struct_name))?;
        blocked(&mut f, |f| {
            f.writeln("class: jni::objects::GlobalRef,")?;
            for field in &structure.fields {
                f.writeln(&format!(
                    "field_{}: jni::objects::JFieldID<'static>,",
                    field.name.to_snake_case()
                ))?;
            }

            Ok(())
        })?;

        f.newline()?;

        f.writeln(&format!("impl Struct{}", struct_name))?;
        blocked(&mut f, |f| {
            f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "let class = env.find_class({}).expect(\"Unable to find {}\");",
                    struct_sig, struct_name
                ))?;
                for field in &structure.fields {
                    f.writeln(&format!("let field_{field_snake} = env.get_field_id(class, \"{field_mixed}\", \"{field_sig}\").map(|mid| mid.into_inner().into()).expect(\"Unable to find field {field_mixed}\");", field_snake=field.name.to_snake_case(), field_mixed=field.name.to_mixed_case(), field_sig=field.field_type.to_all_types().as_jni_sig(&lib_path)))?;
                }
                f.writeln("Self")?;
                blocked(f, |f| {
                    f.writeln("class: env.new_global_ref(class).unwrap(),")?;
                    for field in &structure.fields {
                        f.writeln(&format!("field_{},", field.name.to_snake_case()))?;
                    }
                    Ok(())
                })
            })?;

            f.newline()?;

            let ffi_struct_name = format!("{}::ffi::{}", config.ffi_name, struct_name);

            f.writeln(&format!("pub(crate) fn struct_to_rust(&self, _cache: &super::JCache, _env: &jni::JNIEnv, obj: jni::sys::jobject) -> {}", ffi_struct_name))?;
            blocked(f, |f| {
                for field in &structure.fields {
                    f.writeln(&format!("let {} = _env.get_field_unchecked(obj, self.field_{}, jni::signature::JavaType::from_str(\"{}\").unwrap()).unwrap().{};", field.name.to_snake_case(), field.name.to_snake_case(), field.field_type.to_all_types().as_jni_sig(&lib_path), field.field_type.to_all_types().convert_jvalue()))?;
                }

                f.writeln(&ffi_struct_name)?;
                blocked(f, |f| {
                    for field in &structure.fields {
                        if let Some(conversion) = field
                            .field_type
                            .to_all_types()
                            .conversion(&config.ffi_name, &lib.c_ffi_prefix)
                        {
                            conversion.convert_to_rust(
                                f,
                                &field.name.to_snake_case(),
                                &format!("{}: ", field.name.to_snake_case()),
                            )?;
                            f.write(",")?;
                        } else {
                            f.writeln(&format!("{},", field.name.to_snake_case()))?;
                        }
                    }

                    Ok(())
                })?;

                Ok(())
            })?;

            f.newline()?;

            f.writeln(&format!("pub(crate) fn struct_to_rust_cleanup(&self, _cache: &super::JCache, _env: &jni::JNIEnv, _value: &{})", ffi_struct_name))?;
            blocked(f, |f| {
                for field in &structure.fields {
                    if let Some(conversion) = field
                        .field_type
                        .to_all_types()
                        .conversion(&config.ffi_name, &lib.c_ffi_prefix)
                    {
                        conversion.convert_to_rust_cleanup(
                            f,
                            &format!("_value.{}", field.name.to_snake_case()),
                        )?;
                    }
                }

                Ok(())
            })?;

            f.newline()?;

            f.writeln(&format!("pub(crate) fn struct_from_rust(&self, _cache: &super::JCache, _env: &jni::JNIEnv, value: &{}) -> jni::sys::jobject", ffi_struct_name))?;
            blocked(f, |f| {
                f.writeln("let obj = _env.alloc_object(&self.class).unwrap();")?;
                for field in &structure.fields {
                    if let Some(conversion) = field
                        .field_type
                        .to_all_types()
                        .conversion(&config.ffi_name, &lib.c_ffi_prefix)
                    {
                        conversion.convert_from_rust(
                            f,
                            &format!("value.{}", field.name.to_snake_case()),
                            "let temp = ",
                        )?;
                        f.write(";")?;
                    } else {
                        f.writeln(&format!("let temp = value.{};", field.name.to_snake_case()))?;
                    }
                    f.writeln(&format!(
                        "_env.set_field_unchecked(obj, self.field_{}, temp.into()).unwrap();",
                        field.name.to_snake_case()
                    ))?;

                    if field.field_type.to_all_types().requires_local_ref_cleanup() {
                        f.writeln("_env.delete_local_ref(temp.into()).unwrap();")?;
                    }
                }

                f.writeln("obj.into_inner()")
            })?;

            f.newline()?;

            // Check for null elements helper function
            f.writeln("pub(crate) fn check_null(&self, _cache: &super::JCache, _env: &jni::JNIEnv, obj: jni::sys::jobject) -> Result<(), String>")?;
            blocked(f, |f| {
                for el in &structure.fields {
                    f.writeln(&format!("let temp = _env.get_field_unchecked(obj, self.field_{}, jni::signature::JavaType::from_str(\"{}\").unwrap()).unwrap().{};", el.name.to_snake_case(), el.field_type.to_all_types().as_jni_sig(&lib_path), el.field_type.to_all_types().convert_jvalue()))?;
                    el.field_type.to_all_types().check_null(f, "temp")?;
                }
                f.writeln("Ok(())")
            })
        })?;

        f.newline()?;
    }

    Ok(())
}
