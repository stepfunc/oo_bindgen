use super::formatting::*;
use crate::*;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::formatting::*;
use oo_bindgen::*;

pub(crate) fn generate_enums_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.name);

    let mut filename = config.rust_source_dir();
    filename.push("enums");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    // Top-level enums struct
    f.writeln("pub struct Enums")?;
    blocked(&mut f, |f| {
        for enumeration in lib.native_enums() {
            f.writeln(&format!(
                "pub enum_{}: Enum{},",
                enumeration.name.to_snake_case(),
                enumeration.name.to_camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Enums")?;
    blocked(&mut f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln("Self")?;
            blocked(f, |f| {
                for enumeration in lib.native_enums() {
                    f.writeln(&format!(
                        "enum_{}: Enum{}::init(env),",
                        enumeration.name.to_snake_case(),
                        enumeration.name.to_camel_case()
                    ))?;
                }
                Ok(())
            })
        })
    })?;

    // Each enum implementation
    for enumeration in lib.native_enums() {
        let enum_name = enumeration.name.to_camel_case();
        let enum_sig = format!("\"L{}/{};\"", lib_path, enum_name);

        f.writeln(&format!("pub struct Enum{}", enum_name))?;
        blocked(&mut f, |f| {
            f.writeln("value_field: jni::objects::JFieldID<'static>,")?;
            for variant in &enumeration.variants {
                f.writeln(&format!(
                    "variant_{}: jni::objects::GlobalRef,",
                    variant.name.to_snake_case()
                ))?;
            }

            Ok(())
        })?;

        f.newline()?;

        f.writeln(&format!("impl Enum{}", enum_name))?;
        blocked(&mut f, |f| {
            f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "let class = env.find_class({}).expect(\"Unable to find {}\");",
                    enum_sig, enum_name
                ))?;
                f.writeln("Self")?;
                blocked(f, |f| {
                    f.writeln(&format!("value_field: env.get_field_id(class, \"value\", \"I\").map(|mid| mid.into_inner().into()).expect(\"Unable to get value field of {}\"),", enum_name))?;
                    for variant in &enumeration.variants {
                        f.writeln(&format!("variant_{}: env.new_global_ref(env.get_static_field(class, \"{}\", {}).expect(\"Unable to find variant {}\").l().unwrap()).unwrap(),", variant.name.to_snake_case(), variant.name.to_shouty_snake_case(), enum_sig, variant.name.to_shouty_snake_case()))?;
                    }
                    Ok(())
                })
            })?;

            f.newline()?;

            f.writeln("pub fn enum_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> std::os::raw::c_int")?;
            blocked(f, |f| {
                f.writeln("env.get_field_unchecked(obj, self.value_field, jni::signature::JavaType::Primitive(jni::signature::Primitive::Int)).unwrap().i().unwrap()")
            })?;

            f.newline()?;

            f.writeln("pub fn enum_from_rust(&self, env: &jni::JNIEnv, value: std::os::raw::c_int) -> jni::sys::jobject")?;
            blocked(f, |f| {
                f.writeln("match value")?;
                blocked(f, |f| {
                    for variant in &enumeration.variants {
                        f.writeln(&format!(
                            "{} => self.variant_{}.as_obj().into_inner(),",
                            variant.value,
                            variant.name.to_snake_case()
                        ))?;
                    }
                    f.writeln(&format!(
                        "_ => panic!(\"Invalid {} value: {{}}\", value),",
                        enum_name
                    ))
                })
            })
        })?;

        f.newline()?;
    }

    Ok(())
}
