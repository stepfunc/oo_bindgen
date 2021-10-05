use super::conversion::*;
use super::formatting::*;
use crate::*;
use heck::{CamelCase, MixedCase, SnakeCase};
use oo_bindgen::formatting::*;
use oo_bindgen::interface::*;
use oo_bindgen::types::Arg;

pub(crate) fn generate_interfaces_cache(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.name);

    let mut filename = config.rust_source_dir();
    filename.push("interfaces");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    // Imports
    f.writeln("use std::str::FromStr;")?;

    f.newline()?;

    // Top-level enums struct
    f.writeln("pub struct Interfaces")?;
    blocked(&mut f, |f| {
        for interface in lib.interfaces() {
            f.writeln(&format!(
                "pub interface_{}: Interface{},",
                interface.name.to_snake_case(),
                interface.name.to_camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Interfaces")?;
    blocked(&mut f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln("Self")?;
            blocked(f, |f| {
                for interface in lib.interfaces() {
                    f.writeln(&format!(
                        "interface_{}: Interface{}::init(env),",
                        interface.name.to_snake_case(),
                        interface.name.to_camel_case()
                    ))?;
                }

                Ok(())
            })
        })
    })?;

    // Each interface implementation
    for interface in lib.interfaces() {
        let interface_name = interface.name.to_camel_case();

        f.writeln(&format!("pub struct Interface{}", interface_name))?;
        blocked(&mut f, |f| {
            f.writeln("_class: jni::objects::GlobalRef,")?;
            for callback in &interface.callbacks {
                f.writeln(&format!(
                    "cb_{}: jni::objects::JMethodID<'static>,",
                    callback.name.to_snake_case()
                ))?;
            }

            Ok(())
        })?;

        f.newline()?;

        f.writeln(&format!("impl Interface{}", interface_name))?;
        blocked(&mut f, |f| {
            write_interface_init(f, &interface_name, &lib_path, &interface.callbacks)?;

            f.newline()?;

            let rust_struct_name = format!(
                "{}::ffi::{}",
                config.ffi_name,
                interface.name.to_camel_case()
            );
            f.writeln(&format!("pub(crate) fn interface_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> {}", rust_struct_name))?;
            blocked(f, |f| {
                f.writeln(&rust_struct_name)?;
                blocked(f, |f| {
                    for cb in &interface.callbacks {
                        f.writeln(&format!(
                            "{}: Some({}_{}),",
                            cb.name.to_snake_case(),
                            interface.name.to_camel_case(),
                            cb.name.to_snake_case()
                        ))?;
                    }

                    f.writeln(&format!(
                        "{}: Some({}_{}),",
                        DESTROY_FUNC_NAME.to_snake_case(),
                        interface.name.to_camel_case(),
                        DESTROY_FUNC_NAME.to_snake_case()
                    ))?;

                    f.writeln(&format!(
                        "{}: Box::into_raw(Box::new(env.new_global_ref(obj).unwrap())) as *mut _,",
                        CTX_VARIABLE_NAME.to_snake_case()
                    ))?;

                    Ok(())
                })
            })
        })?;

        f.newline()?;

        for cb in &interface.callbacks {
            let params = cb
                .arguments
                .iter()
                .map(|arg| {
                    format!(
                        "{}: {}",
                        arg.name.to_snake_case(),
                        arg.arg_type.as_rust_type(&config.ffi_name)
                    )
                })
                .chain(std::iter::once(format!(
                    "{}: *mut std::ffi::c_void",
                    CTX_VARIABLE_NAME
                )))
                .collect::<Vec<_>>()
                .join(", ");

            f.writeln(&format!(
                "extern \"C\" fn {}_{}({}) -> {}",
                interface.name.to_camel_case(),
                cb.name.to_snake_case(),
                params,
                cb.return_type.as_rust_type(&config.ffi_name)
            ))?;
            blocked(&mut f, |f| {
                call_java_callback(
                    f,
                    &format!("interface_{}", interface_name.to_snake_case()),
                    &cb.name.to_snake_case(),
                    &lib_path,
                    &config.ffi_name,
                    &lib.c_ffi_prefix,
                    CTX_VARIABLE_NAME,
                    &cb.arguments,
                    &cb.return_type,
                )?;

                // Convert return value
                if let CReturnType::Type(return_type, _) = &cb.return_type {
                    if let Some(conversion) =
                        return_type.conversion(&config.ffi_name, &lib.c_ffi_prefix)
                    {
                        conversion.convert_to_rust(
                            f,
                            &format!("_result.{}", return_type.convert_jvalue()),
                            "return ",
                        )?;
                        f.write(";")?;
                    } else {
                        f.writeln("return _result;")?;
                    }
                }

                Ok(())
            })?;

            f.newline()?;
        }

        // write the destroy stub
        f.writeln(&format!(
            "extern \"C\" fn {}_{}(ctx: *mut std::ffi::c_void)",
            interface.name.to_camel_case(),
            DESTROY_FUNC_NAME.to_snake_case()
        ))?;
        blocked(&mut f, |f| {
            f.writeln("unsafe { Box::from_raw(ctx as *mut jni::objects::GlobalRef) };")
        })?;
    }

    Ok(())
}

fn write_interface_init(
    f: &mut dyn Printer,
    interface_name: &str,
    lib_path: &str,
    callbacks: &[CallbackFunction],
) -> FormattingResult<()> {
    f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
    blocked(f, |f| {
        f.writeln(&format!(
            "let class = env.find_class(\"L{}/{};\").expect(\"Unable to find {}\");",
            lib_path,
            interface_name.to_camel_case(),
            interface_name
        ))?;
        for callback in callbacks {
            let method_sig = format!(
                "({}){}",
                callback
                    .arguments
                    .iter()
                    .map(|arg| { arg.arg_type.as_jni_sig(lib_path) })
                    .collect::<Vec<_>>()
                    .join(""),
                callback.return_type.as_jni_sig(lib_path)
            );
            f.writeln(&format!("let cb_{method_snake} = env.get_method_id(class, \"{method_mixed}\", \"{method_sig}\").map(|mid| mid.into_inner().into()).expect(\"Unable to find method {method_mixed}\");", method_snake=callback.name.to_snake_case(), method_mixed=callback.name.to_mixed_case(), method_sig=method_sig))?;
        }
        f.writeln("Self")?;
        blocked(f, |f| {
            f.writeln("_class: env.new_global_ref(class).unwrap(),")?;
            for callback in callbacks {
                f.writeln(&format!("cb_{},", callback.name.to_snake_case()))?;
            }
            Ok(())
        })
    })
}

#[allow(clippy::too_many_arguments)]
fn call_java_callback(
    f: &mut dyn Printer,
    interface_name: &str,
    callback_name: &str,
    lib_path: &str,
    ffi_name: &str,
    prefix: &str,
    arg_name: &str,
    args: &[Arg<CArgument>],
    return_type: &CReturnType,
) -> FormattingResult<()> {
    // Extract the global ref
    f.writeln(&format!(
        "let _obj = unsafe {{ &mut *({} as *mut jni::objects::GlobalRef) }};",
        arg_name
    ))?;

    // Get the JCache
    f.writeln("let _cache = unsafe { crate::JCACHE.as_ref().unwrap() };")?;

    // Attach the current thread
    f.writeln("_cache.vm.attach_current_thread_permanently().unwrap();")?;

    // Get the env
    f.writeln("let _env = _cache.vm.get_env().unwrap();")?;

    // Perform the conversion of the parameters
    for param in args {
        if let Some(conversion) = param.arg_type.conversion(ffi_name, prefix) {
            conversion.convert_from_rust(
                f,
                &param.name,
                &format!("let {} = ", param.name.to_snake_case()),
            )?;
            f.write(";")?;
        }
    }

    // Call the Java callback
    if !return_type.is_void() {
        f.writeln("let _result = ")?;
    } else {
        f.newline()?;
    }
    f.write(&format!(
        "_env.call_method_unchecked(_obj.as_obj(), _cache.interfaces.{}.cb_{}, jni::signature::JavaType::from_str(\"{}\").unwrap(), &[{}]).unwrap();",
        interface_name,
        callback_name,
        return_type.as_jni_sig(lib_path),
        args.iter().map(|param| format!("{}.into()", param.name.to_snake_case())).collect::<Vec<_>>().join(", ")
    ))?;

    // Release the local refs
    for param in args {
        if param.arg_type.requires_local_ref_cleanup() {
            f.writeln(&format!(
                "_env.delete_local_ref({}.into()).unwrap();",
                param.name.to_snake_case()
            ))?;
        }
    }

    Ok(())
}
