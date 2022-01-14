use std::fs;

use conversion::*;
use oo_bindgen::model::*;

use crate::*;

mod classes;
mod conversion;
mod enums;
mod exceptions;
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
    write_functions(&mut f, lib, config)?;
    write_collection_conversions(&mut f, lib, config)?;
    write_iterator_conversions(&mut f, lib, config)?;

    // Create the cache modules
    classes::generate_classes_cache(lib, config)?;
    enums::generate_enums_cache(lib, config)?;
    structs::generate(lib, config)?;
    interface::generate_interfaces_cache(lib, config)?;
    exceptions::generate_exceptions_cache(lib, config)?;

    // Copy the modules that never change
    filename.set_file_name("primitives.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/primitives.rs"))?;

    filename.set_file_name("unsigned.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/unsigned.rs"))?;

    filename.set_file_name("duration.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/duration.rs"))?;

    filename.set_file_name("collection.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("./copy/collection.rs"))?;

    filename.set_file_name("util.rs");
    let mut f = FilePrinter::new(&filename)?;
    f.write(include_str!("copy/util.rs"))?;

    Ok(())
}

fn generate_toml(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    let ffi_project_name = config.ffi_path.file_name().unwrap();
    let path_to_ffi_lib = pathdiff::diff_paths(&config.ffi_path, &config.rust_output_dir).unwrap();
    let path_to_ffi_lib = path_to_ffi_lib.to_string_lossy().replace('\\', "/");

    let mut filename = config.rust_output_dir.clone();
    filename.push("Cargo");
    filename.set_extension("toml");
    let mut f = FilePrinter::new(filename)?;

    f.writeln("[package]")?;
    f.writeln(&format!("name = \"{}\"", config.java_ffi_name()))?;
    f.writeln(&format!("version = \"{}\"", lib.version))?;
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
    f.writeln("mod duration;")?;
    f.writeln("mod classes;")?;
    f.writeln("mod enums;")?;
    f.writeln("mod collection;")?;
    f.writeln("mod structs;")?;
    f.writeln("mod interfaces;")?;
    f.writeln("mod exceptions;")?;
    f.writeln("mod unsigned;")?;
    f.writeln("mod util;")?;
    f.newline()?;
    // Create cache
    f.writeln("struct JCache")?;
    blocked(f, |f| {
        f.writeln("vm: jni::JavaVM,")?;
        f.writeln("primitives: primitives::Primitives,")?;
        f.writeln("unsigned: unsigned::Unsigned,")?;
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
            f.writeln("let unsigned = unsigned::Unsigned::init(&env);")?;
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
                f.writeln("unsigned,")?;
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

    f.writeln("fn get_cache<'a>() -> &'a JCache {")?;
    indented(f, |f| {
        f.writeln("unsafe { crate::JCACHE.as_ref().unwrap() }")
    })?;
    f.writeln("}")?;

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

fn write_collection_conversions(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    f.newline()?;
    f.writeln("/// convert Java lists into native API collections")?;
    f.writeln("mod collections {")?;
    indented(f, |f| {
        for col in lib.collections() {
            f.newline()?;
            write_collection_guard(f, config, col)?;
        }
        Ok(())
    })?;
    f.writeln("}")
}

fn write_iterator_conversions(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    f.newline()?;
    f.writeln("/// functions that convert native API iterators into Java lists")?;
    f.writeln("mod iterators {")?;
    indented(f, |f| {
        for iter in lib.iterators() {
            f.newline()?;
            write_iterator_conversion(f, config, iter)?;
        }
        Ok(())
    })?;
    f.writeln("}")
}

fn write_iterator_conversion(
    f: &mut dyn Printer,
    config: &JavaBindgenConfig,
    iter: &Handle<AbstractIterator<Validated>>,
) -> FormattingResult<()> {
    f.writeln(&format!("pub(crate) fn {}(_env: &jni::JNIEnv, _cache: &crate::JCache, iter: {}) -> jni::sys::jobject {{", iter.name(), iter.iter_class.get_rust_type(&config.ffi_name)))?;
    indented(f, |f| {
        f.writeln("let list = _cache.collection.new_array_list(&_env);")?;
        f.writeln(&format!(
            "while let Some(next) = unsafe {{ {}::ffi::{}_{}(iter).as_ref() }} {{",
            config.ffi_name, iter.iter_class.settings.c_ffi_prefix, iter.next_function.name
        ))?;
        indented(f, |f| {
            if let Some(conversion) = iter.item_type.maybe_convert("next") {
                f.writeln(&format!("let next = _env.auto_local({});", conversion))?;
            }
            f.writeln("_cache.collection.add_to_array_list(&_env, list, next.as_obj().into());")
        })?;
        f.writeln("}")?;
        f.writeln("list.into_inner()")
    })?;
    f.writeln("}")
}

fn write_collection_guard(
    f: &mut dyn Printer,
    config: &JavaBindgenConfig,
    col: &Handle<Collection<Validated>>,
) -> FormattingResult<()> {
    let collection_name = col.collection_class.name.camel_case();
    let c_ffi_prefix = col.collection_class.settings.c_ffi_prefix.clone();

    f.writeln("/// Guard that builds the C collection type from a Java list")?;
    f.writeln(&format!("pub(crate) struct {} {{", collection_name))?;
    indented(f, |f| {
        f.writeln(&format!(
            "inner: *mut {}::{}",
            config.ffi_name, collection_name
        ))
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!("impl std::ops::Deref for {} {{", collection_name))?;
    indented(f, |f| {
        f.writeln(&format!(
            "type Target = *mut {}::{};",
            config.ffi_name, collection_name
        ))?;
        f.newline()?;
        f.writeln("fn deref(&self) -> &Self::Target {")?;
        indented(f, |f| f.writeln("&self.inner"))?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!("impl {} {{", collection_name))?;
    indented(f, |f| {
        f.writeln("pub(crate) fn new(_env: jni::JNIEnv, list: jni::sys::jobject) -> Result<Self, jni::errors::Error> {")?;
        indented(f, |f| {
            f.writeln("let cache = crate::get_cache();")?;
            let size = if col.has_reserve {
                f.writeln("let size = cache.collection.get_size(&_env, list.into());")?;
                "size"
            } else {
                ""
            };
            f.writeln(&format!(
                "let col = Self {{ inner: unsafe {{ {}::ffi::{}_{}({}) }} }};",
                config.ffi_name, c_ffi_prefix, col.create_func.name, size
            ))?;
            f.writeln(
                "let it = _env.auto_local(cache.collection.get_iterator(&_env, list.into()));",
            )?;
            f.writeln("while cache.collection.has_next(&_env, it.as_obj()) {")?;
            indented(f, |f| {
                f.writeln(
                    "let next = _env.auto_local(cache.collection.next(&_env, it.as_obj()));",
                )?;
                if let Some(converted) = col.item_type.to_rust_from_object("next.as_obj()") {
                    // perform  primary conversion that shadows the variable
                    f.writeln(&format!("let next = {};", converted))?;
                }
                let arg = col
                    .item_type
                    .call_site("next")
                    .unwrap_or_else(|| "next".to_string());
                f.writeln(&format!(
                    "unsafe {{ {}::ffi::{}_{}(col.inner, {}) }};",
                    config.ffi_name, c_ffi_prefix, col.add_func.name, arg
                ))?;
                Ok(())
            })?;
            f.writeln("}")?;
            f.writeln("Ok(col)")?;
            Ok(())
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln("/// Destroy the C collection on drop")?;
    f.writeln(&format!("impl Drop for {} {{", collection_name))?;
    indented(f, |f| {
        f.writeln("fn drop(&mut self) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "unsafe {{ {}::ffi::{}_{}(self.inner) }}",
                config.ffi_name, c_ffi_prefix, col.delete_func.name
            ))
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")
}

fn write_functions(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    fn skip(c: FunctionCategory) -> bool {
        match c {
            FunctionCategory::Native => false,
            // these all get used internally to the JNI and
            // don't need external wrappers accessed from Java
            FunctionCategory::CollectionCreate => true,
            FunctionCategory::CollectionDestroy => true,
            FunctionCategory::CollectionAdd => true,
            FunctionCategory::IteratorNext => true,
        }
    }

    for handle in lib.functions().filter(|f| !skip(f.category)) {
        f.newline()?;
        write_function(f, lib, config, handle)?;
    }
    Ok(())
}

fn write_function_signature(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JavaBindgenConfig,
    handle: &Handle<Function<Validated>>,
) -> FormattingResult<()> {
    let args = handle
        .arguments
        .iter()
        .map(|param| format!("{}: {}", param.name, param.arg_type.jni_signature_type()))
        .collect::<Vec<String>>()
        .join(", ");

    let returns = match handle.return_type.get_value() {
        None => "".to_string(),
        Some(x) => {
            format!(" -> {}", x.jni_signature_type())
        }
    };

    f.writeln("#[no_mangle]")?;
    f.writeln(
        &format!(
            "pub extern \"C\" fn Java_{}_{}_NativeFunctions_{}(_env: jni::JNIEnv, _: jni::sys::jobject, {}){}",
            config.group_id.replace('.', "_"),
            lib.settings.name,
            handle.name.replace('_', "_1"),
            args,
            returns
        )
    )
}

fn write_function(
    f: &mut dyn Printer,
    lib: &Library,
    config: &JavaBindgenConfig,
    handle: &Handle<Function<Validated>>,
) -> FormattingResult<()> {
    write_function_signature(f, lib, config, handle)?;
    blocked(f, |f| {
        // Get the JCache
        f.writeln("let _cache = get_cache();")?;

        f.newline()?;

        // Perform the primary conversion of the parameters if required
        for param in &handle.arguments {
            if let Some(converted) = param.arg_type.to_rust(&param.name) {
                let conversion = format!("let {} = {};", param.name, converted);
                f.writeln(&conversion)?;
            }
        }

        f.newline()?;

        let extra_param = match handle.get_signature_type() {
            SignatureType::NoErrorNoReturn => None,
            SignatureType::NoErrorWithReturn(_, _) => None,
            SignatureType::ErrorNoReturn(_) => None,
            SignatureType::ErrorWithReturn(_, _, _) => Some("_out.as_mut_ptr()".to_string()),
        };

        // list of arguments in the invocation
        let args = handle
            .arguments
            .iter()
            .map(|param| {
                param
                    .arg_type
                    .call_site(&param.name)
                    .unwrap_or_else(|| param.name.to_string())
            })
            .chain(extra_param.into_iter())
            .collect::<Vec<String>>()
            .join(", ");

        // the invocation of the native function
        let invocation = format!(
            "unsafe {{ {}::ffi::{}_{}({}) }}",
            config.ffi_name, lib.settings.c_ffi_prefix, handle.name, args
        );

        // Call the C FFI
        match handle.get_signature_type() {
            SignatureType::NoErrorNoReturn => {
                f.writeln(&format!("{};", invocation))?;
            }
            SignatureType::NoErrorWithReturn(_, _) | SignatureType::ErrorNoReturn(_) => {
                f.writeln(&format!("let _result = {};", invocation))?;
            }
            SignatureType::ErrorWithReturn(_, _, _) => {
                f.writeln("let mut _out = std::mem::MaybeUninit::uninit();")?;
                f.writeln(&format!("let _result = {};", invocation))?;
            }
        };

        // Convert return value
        match handle.get_signature_type() {
            SignatureType::NoErrorNoReturn => (),
            SignatureType::NoErrorWithReturn(return_type, _) => {
                if let Some(conversion) = return_type.maybe_convert("_result") {
                    f.writeln(&format!("let _result = {};", conversion))?;
                }
            }
            SignatureType::ErrorNoReturn(error_type) => {
                f.writeln("if _result != 0")?;
                blocked(f, |f| {
                    error_type.inner.convert("_result");
                    f.writeln(&format!(
                        "let _error = {};",
                        error_type.inner.convert("_result")
                    ))?;
                    f.writeln(&format!(
                        "let error = _cache.exceptions.{}.throw(&_env, _error);",
                        error_type.exception_name
                    ))
                })?;
            }
            SignatureType::ErrorWithReturn(error_type, return_type, _) => {
                f.writeln("let _result = if _result == 0")?;
                blocked(f, |f| {
                    f.writeln("let _result = unsafe { _out.assume_init() };")?;
                    if let Some(conversion) = return_type.maybe_convert("_result") {
                        f.writeln(&conversion)?;
                    }
                    Ok(())
                })?;
                f.writeln("else")?;
                blocked(f, |f| {
                    f.writeln(&format!(
                        "let _error = {};",
                        error_type.inner.convert("_result")
                    ))?;
                    f.writeln(&format!(
                        "let error = _cache.exceptions.{}.throw(&_env, _error);",
                        error_type.exception_name
                    ))?;
                    f.writeln(return_type.get_default_value())
                })?;
                f.write(";")?;
            }
        }

        // Return value
        if !handle.return_type.is_none() {
            f.writeln("return _result.into();")?;
        }

        Ok(())
    })
}
