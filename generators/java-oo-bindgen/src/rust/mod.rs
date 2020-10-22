use self::formatting::*;
use crate::*;
use conversion::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;
use std::fs;

mod classes;
mod conversion;
mod enums;
mod formatting;
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

    // Copy the modules that never changes
    filename.set_file_name("joou.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/joou.rs"))?;

    filename.set_file_name("duration.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/duration.rs"))?;

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
    f.writeln("jni = \"0.17\"")?;
    f.writeln(&format!(
        "{} = {{ path = \"{}\" }}",
        ffi_project_name.to_string_lossy(),
        path_to_ffi_lib
    ))?;
    f.newline()?;
    f.writeln("[workspace]")
}

fn generate_cache(f: &mut dyn Printer) -> FormattingResult<()> {
    // Import modules
    f.writeln("mod joou;")?;
    f.writeln("mod duration;")?;
    f.writeln("mod classes;")?;
    f.writeln("mod enums;")?;
    f.writeln("mod structs;")?;

    // Create cache
    f.writeln("struct JCache")?;
    blocked(f, |f| {
        f.writeln("vm: jni::JavaVM,")?;
        f.writeln("joou: joou::Joou,")?;
        f.writeln("duration: duration::Duration,")?;
        f.writeln("classes: classes::Classes,")?;
        f.writeln("enums: enums::Enums,")?;
        f.writeln("structs: structs::Structs,")?;
        // TODO: put the other cache elements here
        Ok(())
    })?;

    f.newline()?;

    f.writeln("impl JCache")?;
    blocked(f, |f| {
        f.writeln("fn init(vm: jni::JavaVM) -> Self")?;
        blocked(f, |f| {
            f.writeln("let env = vm.get_env().unwrap();")?;
            f.writeln("let joou = joou::Joou::init(&env);")?;
            f.writeln("let duration = duration::Duration::init(&env);")?;
            f.writeln("let classes = classes::Classes::init(&env);")?;
            f.writeln("let enums = enums::Enums::init(&env);")?;
            f.writeln("let structs = structs::Structs::init(&env);")?;
            // TODO: initialize all the stuff here
            f.writeln("Self")?;
            blocked(f, |f| {
                f.writeln("vm,")?;
                f.writeln("joou,")?;
                f.writeln("duration,")?;
                f.writeln("classes,")?;
                f.writeln("enums,")?;
                f.writeln("structs,")?;
                // TODO: put everything else here
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
    for handle in lib.native_functions() {
        if let Some(first_param) = handle.parameters.first() {
            // We don't want to generate the `next` methods of iterators
            if let Type::ClassRef(handle) = &first_param.param_type {
                if matches!(lib.symbol(&handle.name).unwrap(), Symbol::Iterator(_)) {
                    continue;
                }
            }
        }

        f.writeln("#[no_mangle]")?;
        f.writeln(&format!("pub extern \"C\" fn Java_{}_{}_NativeFunctions_{}(_env: jni::JNIEnv, _: jni::sys::jobject, ", config.group_id.replace(".", "_"), lib.name, handle.name.replace("_", "_1")))?;
        f.write(
            &handle
                .parameters
                .iter()
                .map(|param| format!("{}: {}", param.name, param.param_type.as_raw_jni_type()))
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

            // Perform the conversion of the parameters
            for param in &handle.parameters {
                if let Some(conversion) = param.param_type.conversion() {
                    conversion.convert_to_rust(
                        f,
                        &param.name,
                        &format!("let {} = ", param.name),
                    )?;
                    f.write(";")?;
                }
            }

            // Call the C FFI
            if !handle.return_type.is_void() {
                f.writeln("let _result = ")?;
            } else {
                f.newline()?;
            }
            f.write(&format!(
                "unsafe {{ {}::ffi::{}(",
                config.ffi_name, handle.name
            ))?;
            f.write(
                &handle
                    .parameters
                    .iter()
                    .map(|param| {
                        if matches!(param.param_type, Type::Struct(_)) {
                            format!("{}.clone()", &param.name)
                        } else {
                            param.name.to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(") };")?;

            // Convert return value
            if let ReturnType::Type(return_type, _) = &handle.return_type {
                if let Some(conversion) = return_type.conversion() {
                    conversion.convert_from_rust(f, "_result", "let _result = ")?;
                    f.write(";")?;
                }
            }

            // Conversion cleanup
            for param in &handle.parameters {
                if let Some(conversion) = param.param_type.conversion() {
                    conversion.convert_to_rust_cleanup(f, &param.name)?;
                }
            }

            // Return value
            if !handle.return_type.is_void() {
                f.writeln("return _result;")?;
            }

            Ok(())
        })?;

        f.newline()?;
    }
    Ok(())
}
