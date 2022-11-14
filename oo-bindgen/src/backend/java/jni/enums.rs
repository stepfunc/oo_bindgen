use crate::backend::java::jni::JniBindgenConfig;
use crate::backend::*;
use crate::model::Library;

pub(crate) fn generate_enums_cache(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JniBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.settings.name);

    // Top-level enums struct
    f.writeln("pub struct Enums")?;
    blocked(f, |f| {
        for enumeration in lib.enums() {
            f.writeln(&format!(
                "pub {}: {},",
                enumeration.name,
                enumeration.name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Enums")?;
    blocked(f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln("Self")?;
            blocked(f, |f| {
                for enumeration in lib.enums() {
                    f.writeln(&format!(
                        "{}: {}::init(env),",
                        enumeration.name,
                        enumeration.name.camel_case()
                    ))?;
                }
                Ok(())
            })
        })
    })?;

    // Each enum implementation
    for enumeration in lib.enums() {
        let enum_name = enumeration.name.camel_case();

        f.writeln(&format!("pub struct {}", enum_name))?;
        blocked(f, |f| {
            f.writeln("_value_field: jni::objects::JFieldID<'static>,")?;
            for variant in &enumeration.variants {
                f.writeln(&format!("{}: jni::objects::GlobalRef,", variant.name))?;
            }

            Ok(())
        })?;

        f.newline()?;

        f.writeln(&format!("impl {}", enum_name))?;
        blocked(f, |f| {
            f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "let class = env.find_class({}).expect(\"Unable to find {}\");",
                    format!("\"{}/{}\"", lib_path, enum_name), enum_name
                ))?;
                f.writeln("Self")?;
                blocked(f, |f| {
                    f.writeln(&format!("_value_field: env.get_field_id(class, \"value\", \"I\").map(|mid| mid.into_inner().into()).expect(\"Unable to get value field of {}\"),", enum_name))?;
                    for variant in &enumeration.variants {
                        f.writeln(&format!("{}: env.new_global_ref(env.get_static_field(class, \"{}\", {}).expect(\"Unable to find variant {}\").l().unwrap()).unwrap(),", variant.name, variant.name.capital_snake_case(), format!("\"L{}/{};\"", lib_path, enum_name), variant.name.capital_snake_case()))?;
                    }
                    Ok(())
                })
            })?;

            f.newline()?;

            f.writeln("pub fn to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> std::os::raw::c_int")?;
            blocked(f, |f| {
                f.writeln("env.get_field_unchecked(obj, self._value_field, jni::signature::JavaType::Primitive(jni::signature::Primitive::Int)).unwrap().i().unwrap()")
            })?;

            f.newline()?;

            f.writeln("pub fn to_jni(&self, _env: &jni::JNIEnv, value: std::os::raw::c_int) -> jni::sys::jobject")?;
            blocked(f, |f| {
                f.writeln("match value")?;
                blocked(f, |f| {
                    for variant in &enumeration.variants {
                        f.writeln(&format!(
                            "{} => self.{}.as_obj().into_inner(),",
                            variant.value, variant.name
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
