use oo_bindgen::model::Library;

use crate::*;

pub(crate) fn generate_classes_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.settings.name);

    let mut filename = config.rust_source_dir();
    filename.push("classes");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    f.writeln("pub(crate) struct ClassInfo {")?;
    indented(&mut f, |f| {
        f.writeln("class: jni::objects::GlobalRef,")?;
        f.writeln("constructor: jni::objects::JMethodID<'static>,")?;
        f.writeln("self_field: jni::objects::JFieldID<'static>,")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln("impl ClassInfo {")?;
    indented(&mut f, |f| {
        for class in lib.classes() {
            f.newline()?;
            f.writeln(&format!(
                "fn init_{}(env: &jni::JNIEnv) -> Self {{",
                class.name()
            ))?;
            indented(f, |f| {
                let class_name = class.name().camel_case();
                f.writeln(&format!(
                    "let class = env.find_class(\"L{}/{};\").expect(\"Unable to find class {}\");",
                    lib_path, class_name, class_name
                ))?;
                f.writeln(&format!("let constructor = env.get_method_id(class, \"<init>\", \"(J)V\").map(|mid| mid.into_inner().into()).expect(\"Unable to find constructor of {}\");", class_name))?;
                f.writeln(&format!("let self_field = env.get_field_id(class, \"self\", \"J\").map(|mid| mid.into_inner().into()).expect(\"Unable to find self field of {}\");", class_name))?;
                f.writeln(
                    "Self { class: env.new_global_ref(class).unwrap(), constructor, self_field }",
                )
            })?;
            f.writeln("}")?;
        }
        Ok(())
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln("pub(crate) struct Classes {")?;
    indented(&mut f, |f| {
        for class in lib.classes() {
            f.writeln(&format!("{}: ClassInfo,", class.name()))?;
        }

        Ok(())
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln("impl Classes")?;
    blocked(&mut f, |f| {
        f.writeln("pub(crate) fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln("Self")?;
            blocked(f, |f| {
                for class in lib.classes() {
                    f.writeln(&format!(
                        "{}: ClassInfo::init_{}(env),",
                        class.name(),
                        class.name()
                    ))?;
                }
                Ok(())
            })
        })?;

        f.newline()?;

        for class in lib.classes() {
            let class_name = class.name().camel_case();

            f.writeln(&format!("pub(crate) fn {}_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> *mut {}::{}", class.name(), config.ffi_name, class_name))?;
            blocked(f, |f| {
                f.writeln(&format!("env.get_field_unchecked(obj, self.{}.self_field, jni::signature::JavaType::Primitive(jni::signature::Primitive::Long)).unwrap().j().unwrap() as *mut _", class.name()))
            })?;

            f.newline()?;

            f.writeln(&format!("pub(crate) fn {}_from_rust(&self, env: &jni::JNIEnv, value: *mut {}::{}) -> jni::sys::jobject", class.name(), config.ffi_name, class_name))?;
            blocked(f, |f| {
                f.writeln(&format!("env.new_object_unchecked(&self.{}.class, self.{}.constructor, &[jni::objects::JValue::Long(value as i64)]).unwrap().into_inner()", class.name(), class.name()))
            })?;
        }

        Ok(())
    })
}
