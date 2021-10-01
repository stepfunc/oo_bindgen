use self::formatting::*;
use crate::*;
use conversion::*;
use heck::SnakeCase;
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;
use oo_bindgen::types::AnyType;
use std::fs;

mod classes;
mod conversion;
mod enums;
mod exceptions;
mod formatting;
mod interface;
mod structs;

pub fn generate_java_ffi(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    fs::create_dir_all(&config.rust_output_dir)?;

    // Create the Cargo.toml
    generate_toml(lib, config)?;

    // Create the source directory
    fs::create_dir_all(&config.rust_source_dir())?;

    // Create the root file
    let mut filename = config.rust_source_dir();
    filename.push("lib");
    filename.set_extension("rs");
    let mut f = FilePrinter::new(&filename)?;

    generate_cache(&mut f)?;
    f.newline()?;
    generate_functions(&mut f, lib, config)?;

    // Create the cache modules
    classes::generate_classes_cache(lib, config)?;
    enums::generate_enums_cache(lib, config)?;
    structs::generate_structs_cache(lib, config)?;
    interface::generate_interfaces_cache(lib, config)?;
    exceptions::generate_exceptions_cache(lib, config)?;

    // Copy the modules that never changes
    filename.set_file_name("primitives.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/primitives.rs"))?;

    filename.set_file_name("joou.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/joou.rs"))?;

    filename.set_file_name("duration.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/duration.rs"))?;

    filename.set_file_name("collection.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/collection.rs"))?;

    Ok(())
}

fn generate_toml(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    let ffi_project_name = config.ffi_path.file_name().unwrap();
    let path_to_ffi_lib = pathdiff::diff_paths(&config.ffi_path, &config.rust_output_dir).unwrap();
    let path_to_ffi_lib = path_to_ffi_lib.to_string_lossy().replace("\\", "/");

    let mut filename = config.rust_output_dir.clone();
    filename.push("Cargo");
    filename.set_extension("toml");
    let mut f = FilePrinter::new(filename)?;

    f.writeln("[package]")?;
    f.writeln(&format!("name = \"{}\"", config.java_ffi_name()))?;
    f.writeln(&format!("version = \"{}\"", lib.version.to_string()))?;
    f.writeln("edition = \"2018\"")?;
    f.newline()?;
    f.writeln("[lib]")?;
    f.writeln("crate-type = [\"cdylib\"]")?;
    f.newline()?;
    f.writeln("[dependencies]")?;
    f.writeln("jni = \"0.19\"")?;
    f.writeln(&format!(
        "{} = {{ path = \"{}\" }}",
        ffi_project_name.to_string_lossy(),
        path_to_ffi_lib
    ))?;
    f.newline()?;
    f.writeln("[workspace]")
}

fn generate_cache(f: &mut dyn Printer) -> FormattingResult<()> {
    // Disable some warnings, otherwise I won't see the light of day
    f.writeln("#![allow(dead_code)]")?;
    f.writeln("#![allow(irrefutable_let_patterns)]")?;
    f.writeln("#![allow(non_snake_case)]")?;
    f.writeln("#![allow(unused_variables)]")?;

    f.newline()?;

    // Import modules
    f.writeln("mod primitives;")?;
    f.writeln("mod joou;")?;
    f.writeln("mod duration;")?;
    f.writeln("mod classes;")?;
    f.writeln("mod enums;")?;
    f.writeln("mod collection;")?;
    f.writeln("mod structs;")?;
    f.writeln("mod interfaces;")?;
    f.writeln("mod exceptions;")?;

    // Create cache
    f.writeln("struct JCache")?;
    blocked(f, |f| {
        f.writeln("vm: jni::JavaVM,")?;
        f.writeln("primitives: primitives::Primitives,")?;
        f.writeln("joou: joou::Joou,")?;
        f.writeln("duration: duration::Duration,")?;
        f.writeln("collection: collection::Collection,")?;
        f.writeln("classes: classes::Classes,")?;
        f.writeln("enums: enums::Enums,")?;
        f.writeln("structs: structs::Structs,")?;
        f.writeln("interfaces: interfaces::Interfaces,")?;
        f.writeln("exceptions: exceptions::Exceptions,")?;
        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl JCache")?;
    blocked(f, |f| {
        f.writeln("fn init(vm: jni::JavaVM) -> Self")?;
        blocked(f, |f| {
            f.writeln("let env = vm.get_env().unwrap();")?;
            f.writeln("let primitives = primitives::Primitives::init(&env);")?;
            f.writeln("let joou = joou::Joou::init(&env);")?;
            f.writeln("let duration = duration::Duration::init(&env);")?;
            f.writeln("let collection = collection::Collection::init(&env);")?;
            f.writeln("let classes = classes::Classes::init(&env);")?;
            f.writeln("let enums = enums::Enums::init(&env);")?;
            f.writeln("let structs = structs::Structs::init(&env);")?;
            f.writeln("let interfaces = interfaces::Interfaces::init(&env);")?;
            f.writeln("let exceptions = exceptions::Exceptions::init(&env);")?;
            f.writeln("Self")?;
            blocked(f, |f| {
                f.writeln("vm,")?;
                f.writeln("primitives,")?;
                f.writeln("joou,")?;
                f.writeln("duration,")?;
                f.writeln("collection,")?;
                f.writeln("classes,")?;
                f.writeln("enums,")?;
                f.writeln("structs,")?;
                f.writeln("interfaces,")?;
                f.writeln("exceptions,")?;
                Ok(())
            })
        })
    })?;

    f.newline()?;

    f.writeln("static mut JCACHE: Option<JCache> = None;")?;

    f.newline()?;

    // OnLoad function
    f.writeln("#[no_mangle]")?;
    f.writeln("pub extern \"C\" fn JNI_OnLoad(vm: *mut jni::sys::JavaVM, _: *mut std::ffi::c_void) -> jni::sys::jint")?;
    blocked(f, |f| {
        f.writeln("let vm = unsafe { jni::JavaVM::from_raw(vm).unwrap() };")?;
        f.writeln("let jcache = JCache::init(vm);")?;
        f.writeln("unsafe { JCACHE.replace(jcache) };")?;
        f.writeln("jni::JNIVersion::V8.into()")
    })?;

    f.newline()?;

    // OnUnload function
    f.writeln("#[no_mangle]")?;
    f.writeln("pub extern \"C\" fn JNI_OnUnload(_vm: *mut jni::sys::JavaVM, _: *mut std::ffi::c_void) -> jni::sys::jint")?;
    blocked(f, |f| {
        f.writeln("unsafe { JCACHE.take().unwrap(); }")?;
        f.writeln("return 0;")
    })
}

fn generate_functions(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    for handle in lib.functions() {
        if let Some(first_param) = handle.parameters.first() {
            if let FArgument::ClassRef(class_handle) = &first_param.arg_type {
                // We don't want to generate the `next` methods of iterators
                if let Some(it) = lib.find_iterator(&class_handle.name) {
                    if &it.native_func == handle {
                        continue;
                    }
                }
                // We don't want to generate the `add` and `delete` methods of collections
                if let Some(col) = lib.find_collection(&class_handle.name) {
                    if &col.add_func == handle || &col.delete_func == handle {
                        continue;
                    }
                }
            }
        }

        if let ReturnType::Type(AnyType::ClassRef(class_handle), _) = &handle.return_type {
            // We don't want to generate the `create` method of collections
            if lib.find_collection(&class_handle.name).is_some() {
                continue;
            }
        }

        f.writeln("#[no_mangle]")?;
        f.writeln(&format!("pub extern \"C\" fn Java_{}_{}_NativeFunctions_{}(_env: jni::JNIEnv, _: jni::sys::jobject, ", config.group_id.replace(".", "_"), lib.name, handle.name.replace("_", "_1")))?;
        f.write(
            &handle
                .parameters
                .iter()
                .map(|param| {
                    format!(
                        "{}: {}",
                        param.name.to_snake_case(),
                        param.arg_type.to_any_type().as_raw_jni_type()
                    )
                })
                .collect::<Vec<String>>()
                .join(", "),
        )?;
        f.write(")")?;

        if let ReturnType::Type(return_type, _) = &handle.return_type {
            f.write(&format!(" -> {}", return_type.as_raw_jni_type()))?;
        }

        blocked(f, |f| {
            // Get the JCache
            f.writeln("let _cache = unsafe { JCACHE.as_ref().unwrap() };")?;

            f.newline()?;

            // Check for illegal null
            //
            // "Illégale!
            // Tu m'fais faire des bêtises,
            // Dans les rues d'Montréal.
            // Quand y faut que j'me maîtrise,
            // Tu m'fais piquer des crises.
            // Illégale!"
            f.writeln("if let Err(msg) = (|| -> Result<(), String>")?;
            blocked(f, |f| {
                for param in &handle.parameters {
                    param
                        .arg_type
                        .to_any_type()
                        .check_null(f, &param.name.to_snake_case())?;
                }
                f.writeln("Ok(())")
            })?;
            f.write(")()")?;
            blocked(f, |f| {
                f.writeln("_env.throw_new(\"java/lang/IllegalArgumentException\", msg).unwrap();")?;
                if let ReturnType::Type(return_type, _) = &handle.return_type {
                    f.writeln(&format!("return {}", return_type.default_value()))?;
                } else {
                    f.writeln("return;")?;
                }
                Ok(())
            })?;

            f.newline()?;

            // Perform the conversion of the parameters
            for param in &handle.parameters {
                if let Some(conversion) = param
                    .arg_type
                    .to_any_type()
                    .conversion(&config.ffi_name, &lib.c_ffi_prefix)
                {
                    conversion.convert_to_rust(
                        f,
                        &param.name,
                        &format!("let {} = ", param.name.to_snake_case()),
                    )?;
                    f.write(";")?;
                }
            }

            f.newline()?;

            // Call the C FFI
            let extra_param = match handle.get_signature_type() {
                SignatureType::NoErrorNoReturn => {
                    f.newline()?;
                    None
                }
                SignatureType::NoErrorWithReturn(_, _) => {
                    f.writeln("let _result = ")?;
                    None
                }
                SignatureType::ErrorNoReturn(_) => {
                    f.writeln("let _result = ")?;
                    None
                }
                SignatureType::ErrorWithReturn(_, _, _) => {
                    f.writeln("let mut _out = std::mem::MaybeUninit::uninit();")?;
                    f.writeln("let _result = ")?;
                    Some("_out.as_mut_ptr()".to_string())
                }
            };

            f.write(&format!(
                "unsafe {{ {}::ffi::{}_{}(",
                config.ffi_name, lib.c_ffi_prefix, handle.name
            ))?;
            f.write(
                &handle
                    .parameters
                    .iter()
                    .map(|param| {
                        if matches!(param.arg_type, FArgument::Struct(_)) {
                            format!("{}.clone()", &param.name.to_snake_case())
                        } else {
                            param.name.to_snake_case()
                        }
                    })
                    .chain(extra_param.into_iter())
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(") };")?;

            f.newline()?;

            // Convert return value
            match handle.get_signature_type() {
                SignatureType::NoErrorNoReturn => (),
                SignatureType::NoErrorWithReturn(return_type, _) => {
                    if let Some(conversion) =
                        return_type.conversion(&config.ffi_name, &lib.c_ffi_prefix)
                    {
                        conversion.convert_from_rust(f, "_result", "let _result = ")?;
                        f.write(";")?;
                    }
                }
                SignatureType::ErrorNoReturn(error_type) => {
                    f.writeln("if _result != 0")?;
                    blocked(f, |f| {
                        EnumConverter(error_type.inner).convert_from_rust(
                            f,
                            "_result",
                            "let _error = ",
                        )?;
                        f.write(";")?;
                        f.writeln(&format!(
                            "let error = _cache.exceptions.throw_{}(&_env, _error);",
                            error_type.exception_name.to_snake_case()
                        ))
                    })?;
                }
                SignatureType::ErrorWithReturn(error_type, return_type, _) => {
                    f.writeln("let _result = if _result == 0")?;
                    blocked(f, |f| {
                        f.writeln("let _result = unsafe { _out.assume_init() };")?;
                        if let Some(conversion) =
                            return_type.conversion(&config.ffi_name, &lib.c_ffi_prefix)
                        {
                            conversion.convert_from_rust(f, "_result", "")?;
                        }
                        Ok(())
                    })?;
                    f.writeln("else")?;
                    blocked(f, |f| {
                        EnumConverter(error_type.inner).convert_from_rust(
                            f,
                            "_result",
                            "let _error = ",
                        )?;
                        f.write(";")?;
                        f.writeln(&format!(
                            "let error = _cache.exceptions.throw_{}(&_env, _error);",
                            error_type.exception_name.to_snake_case()
                        ))?;
                        f.writeln(return_type.default_value())
                    })?;
                    f.write(";")?;
                }
            }

            f.newline()?;

            // Conversion cleanup
            for param in &handle.parameters {
                if let Some(conversion) = param
                    .arg_type
                    .to_any_type()
                    .conversion(&config.ffi_name, &lib.c_ffi_prefix)
                {
                    conversion.convert_to_rust_cleanup(f, &param.name.to_snake_case())?;
                }

                // Because we clone structs that are passed by value, we don't want the drop of interfaces to be called twice
                if matches!(param.arg_type, FArgument::Struct(_)) {
                    f.writeln(&format!(
                        "std::mem::forget({});",
                        param.name.to_snake_case()
                    ))?;
                }
            }

            f.newline()?;

            // Return value
            if !handle.return_type.is_void() {
                f.writeln("return _result.into();")?;
            }

            Ok(())
        })?;

        f.newline()?;
    }
    Ok(())
}
