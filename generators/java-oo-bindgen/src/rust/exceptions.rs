use super::formatting::*;
use crate::*;
use heck::{CamelCase, SnakeCase};

pub(crate) fn generate_exceptions_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.name);

    let mut filename = config.rust_source_dir();
    filename.push("exceptions");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    // Top-level exceptions struct
    f.writeln("pub struct Exceptions")?;
    blocked(&mut f, |f| {
        for error in lib.error_types() {
            f.writeln(&format!(
                "exception_{}_class: jni::objects::GlobalRef,",
                error.exception_name.to_snake_case(),
            ))?;
            f.writeln(&format!(
                "exception_{}_constructor: jni::objects::JMethodID<'static>,",
                error.exception_name.to_snake_case(),
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Exceptions")?;
    blocked(&mut f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            for error in lib.error_types() {
                let snake_name = error.exception_name.to_snake_case();
                let camel_name = error.exception_name.to_camel_case();
                let enum_name = error.inner.name.to_camel_case();
                f.writeln(&format!(
                    "let exception_{snake_name}_class = env.find_class(\"L{lib_path}/{camel_name};\").expect(\"Unable to find exception {camel_name}\");",
                    snake_name=snake_name, camel_name=camel_name, lib_path=lib_path,
                ))?;
                f.writeln(&format!(
                    "let exception_{}_constructor = env.get_method_id(exception_{snake_name}_class, \"<init>\", \"(L{lib_path}/{enum_name};)V\").map(|mid| mid.into_inner().into()).expect(\"Unable to find constructor of {camel_name}\");",
                    snake_name=snake_name, camel_name=camel_name, lib_path=lib_path, enum_name=enum_name,
                ))?;
            }

            f.writeln("Self")?;
            blocked(f, |f| {
                for error in lib.error_types() {
                    let snake_name = error.exception_name.to_snake_case();
                    f.writeln(&format!(
                        "exception_{snake_name}_class: env.new_global_ref(exception_{snake_name}_class).unwrap(),",
                        snake_name=snake_name
                    ))?;
                    f.writeln(&format!("exception_{}_constructor", snake_name))?;
                }
                Ok(())
            })
        })?;

        for error in lib.error_types() {
            let snake_name = error.exception_name.to_snake_case();
            f.writeln(&format!(
                "pub fn throw_{}(&self, env: &jni::JNIEnv, error: jni::sys::jobject)",
                snake_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("let obj = env.new_object_unchecked(&self.exception_{snake_name}_class, self.exception_{snake_name}_constructor, &[jni::objects::JValue::Object(error.into())]).unwrap();", snake_name=snake_name))?;
                f.writeln("env.throw(jni::objects::JThrowable::from(obj)).unwrap();")
            })?;
        }

        Ok(())
    })
}
