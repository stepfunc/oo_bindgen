use oo_bindgen::model::Library;

use crate::*;

pub(crate) fn generate_exceptions_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.settings.name);

    let mut filename = config.rust_source_dir();
    filename.push("exceptions");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    f.writeln("/// cached information about an exception")?;
    f.writeln("pub struct ExceptionInfo {")?;
    indented(&mut f, |f| {
        f.writeln("class: jni::objects::GlobalRef,")?;
        f.writeln("constructor: jni::objects::JMethodID<'static>,")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln("impl ExceptionInfo {")?;
    indented(&mut f, |f| {
        f.writeln("pub(crate) fn throw(&self, env: &jni::JNIEnv, error: jni::sys::jobject) {")?;
        indented(f, |f| {
            f.writeln("let obj = env.new_object_unchecked(&self.class, self.constructor, &[jni::objects::JValue::Object(error.into())]).unwrap();")?;
            f.writeln("env.throw(jni::objects::JThrowable::from(obj)).unwrap()")
        })?;
        f.writeln("}")?;
        for error in lib.error_types() {
            let camel_name = error.exception_name.camel_case();
            let enum_name = error.inner.name.camel_case();
            f.newline()?;
            f.writeln(&format!(
                "fn init_{}(env: &jni::JNIEnv) -> Self {{",
                error.exception_name
            ))?;
            indented(f, |f| {
                f.writeln(&format!("let class = env.find_class(\"L{}/{};\").expect(\"Unable to find exception {}\");", lib_path, camel_name, camel_name))?;
                f.writeln(&format!("let constructor = env.get_method_id(class, \"<init>\", \"(L{}/{};)V\").map(|mid| mid.into_inner().into()).expect(\"Unable to find constructor of {}\");", lib_path, enum_name, camel_name))?;
                f.writeln("Self { class : env.new_global_ref(class).unwrap(), constructor }")
            })?;
            f.writeln("}")?;
        }
        Ok(())
    })?;
    f.writeln("}")?;

    f.newline()?;

    // Top-level exceptions struct
    f.writeln("pub struct Exceptions")?;
    blocked(&mut f, |f| {
        for error in lib.error_types() {
            f.writeln(&format!(
                "pub(crate) {}: ExceptionInfo,",
                error.exception_name
            ))?;
        }
        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Exceptions {")?;
    indented(&mut f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self {")?;
        indented(f, |f| {
            f.writeln("Self {")?;
            indented(f, |f| {
                for error in lib.error_types() {
                    f.writeln(&format!(
                        "{}: ExceptionInfo::init_{}(env),",
                        error.exception_name, error.exception_name
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
