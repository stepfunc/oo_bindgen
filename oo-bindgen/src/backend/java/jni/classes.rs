use crate::model::Library;

use crate::backend::java::jni::JniBindgenConfig;
use crate::backend::*;

pub(crate) fn generate_classes_cache(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JniBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.settings.name);

    f.writeln("pub(crate) struct ClassInfo {")?;
    indented(f, |f| {
        f.writeln("class: jni::objects::GlobalRef,")?;
        f.writeln("constructor: jni::objects::JMethodID<'static>,")?;
        f.writeln("self_field: jni::objects::JFieldID<'static>,")
    })?;
    f.writeln("}")?;

    for class in lib.classes() {
        let class_name = class.name().camel_case();
        f.newline()?;
        f.writeln(&format!("pub(crate) struct {} {{", class_name))?;
        indented(f, |f| f.writeln("info: ClassInfo"))?;
        f.writeln("}")?;
        f.newline()?;
        f.writeln(&format!("impl {} {{", class_name))?;
        indented(f, |f| {
            f.writeln("fn init(env: &jni::JNIEnv) -> Self {")?;
            indented(f, |f| {
                let class_name = class.name().camel_case();
                f.writeln(&format!(
                    "let class = env.find_class(\"{}/{}\").expect(\"Unable to find class {}\");",
                    lib_path, class_name, class_name
                ))?;
                f.writeln(&format!("let constructor = env.get_method_id(class, \"<init>\", \"(J)V\").map(|mid| mid.into_inner().into()).expect(\"Unable to find constructor of {}\");", class_name))?;
                f.writeln(&format!("let self_field = env.get_field_id(class, \"self\", \"J\").map(|mid| mid.into_inner().into()).expect(\"Unable to find self field of {}\");", class_name))?;
                f.writeln("Self {")?;
                indented(f, |f| {
                    f.writeln("info: ClassInfo { class: env.new_global_ref(class).unwrap(), constructor, self_field }")
                })?;
                f.writeln("}")
            })?;
            f.writeln("}")?;

            f.newline()?;

            f.writeln(&format!("pub(crate) fn to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> *mut {}::{} {{", config.ffi_name, class_name))?;
            indented(f, |f| {
                f.writeln("env.get_field_unchecked(obj, self.info.self_field, jni::signature::JavaType::Primitive(jni::signature::Primitive::Long)).unwrap().j().unwrap() as *mut _")
            })?;
            f.writeln("}")?;

            f.newline()?;

            f.writeln(&format!("pub(crate) fn to_jni(&self, env: &jni::JNIEnv, value: *mut {}::{}) -> jni::sys::jobject {{", config.ffi_name, class_name))?;
            indented(f, |f| {
                f.writeln("env.new_object_unchecked(&self.info.class, self.info.constructor, &[jni::objects::JValue::Long(value as i64)]).unwrap().into_inner()")
            })?;
            f.writeln("}")
        })?;
        f.writeln("}")?;
    }

    f.newline()?;

    f.writeln("pub(crate) struct Classes {")?;
    indented(f, |f| {
        for class in lib.classes() {
            f.writeln(&format!(
                "pub(crate) {}: {},",
                class.name(),
                class.name().camel_case()
            ))?;
        }

        Ok(())
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln("impl Classes {")?;
    indented(f, |f| {
        f.writeln("pub(crate) fn init(env: &jni::JNIEnv) -> Self {")?;
        indented(f, |f| {
            f.writeln("Self {")?;
            indented(f, |f| {
                for class in lib.classes() {
                    f.writeln(&format!(
                        "{}: {}::init(env),",
                        class.name(),
                        class.name().camel_case()
                    ))?;
                }
                Ok(())
            })?;
            f.writeln("}")
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")
}
