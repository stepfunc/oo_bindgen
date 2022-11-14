use crate::backend::*;
use crate::model::*;

use crate::backend::java::jni::conversion::*;
use crate::backend::java::jni::JniBindgenConfig;

pub(crate) fn generate_interfaces_cache(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JniBindgenConfig,
) -> FormattingResult<()> {
    let lib_path = config.java_signature_path(&lib.settings.name);

    // Top-level enums struct
    f.writeln("pub struct Interfaces")?;
    blocked(f, |f| {
        for interface in lib.untyped_interfaces() {
            f.writeln(&format!(
                "pub {}: {},",
                interface.name,
                interface.name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl Interfaces")?;
    blocked(f, |f| {
        f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
        blocked(f, |f| {
            f.writeln("Self")?;
            blocked(f, |f| {
                for interface in lib.untyped_interfaces() {
                    f.writeln(&format!(
                        "{}: {}::init(env),",
                        interface.name,
                        interface.name.camel_case()
                    ))?;
                }

                Ok(())
            })
        })
    })?;

    let ctx_variable_name = lib.settings.interface.context_variable_name.clone();
    let destroy_func_name = lib.settings.interface.destroy_func_name.clone();

    // Each interface implementation
    for interface in lib.untyped_interfaces() {
        let interface_name = interface.name.camel_case();

        f.writeln(&format!("pub struct {}", interface_name))?;
        blocked(f, |f| {
            f.writeln("_class: jni::objects::GlobalRef,")?;
            for callback in &interface.callbacks {
                f.writeln(&format!(
                    "{}: jni::objects::JMethodID<'static>,",
                    callback.name
                ))?;
            }

            Ok(())
        })?;

        f.newline()?;

        f.writeln(&format!("impl {}", interface_name))?;
        blocked(f, |f| {
            write_interface_init(f, &interface_name, &lib_path, &interface.callbacks)?;

            f.newline()?;

            let rust_struct_name =
                format!("{}::ffi::{}", config.ffi_name, interface.name.camel_case());
            f.writeln(&format!(
                "pub(crate) fn to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> {}",
                rust_struct_name
            ))?;
            blocked(f, |f| {
                f.writeln(&rust_struct_name)?;
                blocked(f, |f| {
                    for cb in &interface.callbacks {
                        f.writeln(&format!(
                            "{}: Some({}_{}),",
                            cb.name, interface.name, cb.name
                        ))?;
                    }

                    f.writeln(&format!(
                        "{}: Some({}_{}),",
                        destroy_func_name, interface.name, destroy_func_name
                    ))?;

                    f.writeln(&format!(
                        "{}: Box::into_raw(Box::new(env.new_global_ref(obj).unwrap())) as *mut _,",
                        ctx_variable_name
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
                        arg.name,
                        arg.arg_type.get_rust_type(config.ffi_name)
                    )
                })
                .chain(std::iter::once(format!(
                    "{}: *mut std::ffi::c_void",
                    ctx_variable_name
                )))
                .collect::<Vec<_>>()
                .join(", ");

            f.writeln(&format!(
                "extern \"C\" fn {}_{}({}) -> {}",
                interface.name,
                cb.name,
                params,
                cb.return_type.get_rust_type(config.ffi_name)
            ))?;
            blocked(f, |f| {
                call_java_callback(
                    f,
                    &interface.name,
                    &cb.name,
                    &ctx_variable_name,
                    &cb.arguments,
                    &cb.return_type,
                )?;

                // Convert return value
                if let Some(return_type) = &cb.return_type.get_value() {
                    let unwrapped = format!("_result.{}", return_type.unwrap_value());

                    if let Some(converted) = return_type.to_rust(&unwrapped) {
                        let ret = return_type.call_site(&converted).unwrap_or(converted);
                        f.writeln(&format!("return {};", ret))?;
                    } else {
                        f.writeln(&format!("return {};", unwrapped))?;
                    }
                }

                Ok(())
            })?;

            f.newline()?;
        }

        // write the destroy stub
        f.writeln(&format!(
            "extern \"C\" fn {}_{}(ctx: *mut std::ffi::c_void)",
            interface.name, destroy_func_name
        ))?;
        blocked(f, |f| {
            f.writeln("unsafe { Box::from_raw(ctx as *mut jni::objects::GlobalRef) };")
        })?;
    }

    Ok(())
}

fn write_interface_init(
    f: &mut dyn Printer,
    interface_name: &str,
    lib_path: &str,
    callbacks: &[CallbackFunction<Validated>],
) -> FormattingResult<()> {
    f.writeln("pub fn init(env: &jni::JNIEnv) -> Self")?;
    blocked(f, |f| {
        f.writeln(&format!(
            "let class = env.find_class(\"{}/{}\").expect(\"Unable to find {}\");",
            lib_path, interface_name, interface_name
        ))?;
        for callback in callbacks {
            let method_sig = format!(
                "({}){}",
                callback
                    .arguments
                    .iter()
                    .map(|arg| { arg.arg_type.jni_type_id().as_string(lib_path) })
                    .collect::<Vec<_>>()
                    .join(""),
                callback.return_type.jni_type_id().as_string(lib_path)
            );
            f.writeln(&format!("let {} = env.get_method_id(class, \"{method_mixed}\", \"{method_sig}\").map(|mid| mid.into_inner().into()).expect(\"Unable to find method {method_mixed}\");", callback.name, method_mixed=callback.name.mixed_case(), method_sig=method_sig))?;
        }
        f.writeln("Self")?;
        blocked(f, |f| {
            f.writeln("_class: env.new_global_ref(class).unwrap(),")?;
            for callback in callbacks {
                f.writeln(&format!("{},", callback.name))?;
            }
            Ok(())
        })
    })
}

fn call_java_callback(
    f: &mut dyn Printer,
    interface_name: &Name,
    callback_name: &str,
    arg_name: &str,
    args: &[Arg<CallbackArgument, Validated>],
    return_type: &OptionalReturnType<CallbackReturnValue, Validated>,
) -> FormattingResult<()> {
    f.writeln("// setup")?;
    f.writeln("let _cache = crate::get_cache();")?;
    f.writeln("let _env = _cache.vm.attach_current_thread_permanently().unwrap();")?;
    f.writeln(&format!(
        "let _ctx = unsafe {{ &mut *({} as *mut jni::objects::GlobalRef) }};",
        arg_name
    ))?;
    f.writeln("// automatically free any local references that are created")?;
    f.writeln("// the upper bound on the number of references required is the number of arguments to the callback")?;
    f.writeln(&format!(
        "let _frame = crate::util::local_frame(_env, {}).unwrap();",
        args.len()
    ))?;
    f.writeln("// convert the arguments")?;
    // Perform the conversion of the parameters
    for param in args {
        if let Some(conversion) = param.arg_type.maybe_convert(&param.name) {
            f.writeln(&format!("let {} = {};", param.name, conversion))?;
        }
    }

    let invocation =
        format!(
        "_env.call_method_unchecked(_ctx.as_obj(), _cache.interfaces.{}.{}, {}, &[{}]).unwrap()",
        interface_name,
        callback_name,
        return_type.jni_java_type(),
        args.iter().map(|param| format!("{}.into()", param.name)).collect::<Vec<_>>().join(", ")
    );

    f.writeln("// invoke the callback")?;
    if return_type.is_some() {
        f.writeln(&format!("let _result = {};", invocation))?;
    } else {
        f.writeln(&format!("{};", invocation))?;
    }
    Ok(())
}
