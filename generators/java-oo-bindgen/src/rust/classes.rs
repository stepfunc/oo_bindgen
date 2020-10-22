use super::formatting::*;
use crate::*;
use heck::{CamelCase, SnakeCase};
use oo_bindgen::formatting::*;

pub(crate) fn generate_classes_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.name);

    let mut filename = config.rust_source_dir();
    filename.push("classes");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    f.writeln("pub struct Classes")?;
    blocked(&mut f, |f| {
        for class in lib.classes().filter(|class| !class.is_static()) {
            f.writeln(&format!(
                "class_{}: jni::objects::GlobalRef,",
                class.name().to_snake_case()
            ))?;
            f.writeln(&format!(
                "{}_constructor: jni::objects::JMethodID<'static>,",
                class.name().to_snake_case()
            ))?;
            f.writeln(&format!(
                "{}_self_field: jni::objects::JFieldID<'static>,",
                class.name().to_snake_case()
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Classes")?;
    blocked(&mut f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            for class in lib.classes().filter(|class| !class.is_static()) {
                let class_name = class.name().to_camel_case();
                let snake_name = class.name().to_snake_case();
                f.writeln(&format!("let class_{snake_name} = env.find_class(\"L{lib_path}/{class_name};\").expect(\"Unable to find class {class_name}\");", snake_name=snake_name, class_name=class_name, lib_path=lib_path))?;
                f.writeln(&format!("let {snake_name}_constructor = env.get_method_id(class_{snake_name}, \"<init>\", \"(J)V\").map(|mid| mid.into_inner().into()).expect(\"Unable to find constructor of {class_name}\");", snake_name=snake_name, class_name=class_name))?;
                f.writeln(&format!("let {snake_name}_self_field = env.get_field_id(class_{snake_name}, \"self\", \"J\").map(|mid| mid.into_inner().into()).expect(\"Unable to find self field of {class_name}\");", snake_name=snake_name, class_name=class_name))?;
            }

            f.writeln("Self")?;
            blocked(f, |f| {
                for class in lib.classes().filter(|class| !class.is_static()) {
                    let snake_name = class.name().to_snake_case();
                    f.writeln(&format!(
                        "class_{}: env.new_global_ref(class_{}).unwrap(),",
                        snake_name, snake_name
                    ))?;
                    f.writeln(&format!("{}_constructor,", snake_name))?;
                    f.writeln(&format!("{}_self_field,", snake_name))?;
                }
                Ok(())
            })
        })?;

        f.newline()?;

        for class in lib.classes().filter(|class| !class.is_static()) {
            let snake_name = class.name().to_snake_case();

            f.writeln(&format!("pub fn {snake_name}_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> *mut {ffi_name}::{class_name}", snake_name=snake_name, ffi_name=config.ffi_name, class_name=class.name()))?;
            blocked(f, |f| {
                f.writeln(&format!("env.get_field_unchecked(obj, self.{snake_name}_self_field, jni::signature::JavaType::Primitive(jni::signature::Primitive::Long)).unwrap().j().unwrap() as *mut _", snake_name=snake_name))
            })?;

            f.newline()?;

            f.writeln(&format!("pub fn {snake_name}_from_rust(&self, env: &jni::JNIEnv, value: *mut {ffi_name}::{class_name}) -> jni::sys::jobject", snake_name=snake_name, ffi_name=config.ffi_name, class_name=class.name()))?;
            blocked(f, |f| {
                f.writeln(&format!("env.new_object_unchecked(&self.class_{snake_name}, self.{snake_name}_constructor, &[jni::objects::JValue::Long(value as i64)]).unwrap().into_inner()", snake_name=snake_name))
            })?;
        }

        Ok(())
    })
}
