use self::conversion::*;
use self::formatting::*;
use crate::JavaBindgenConfig;
use heck::{CamelCase, KebabCase};
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;
use oo_bindgen::platforms::Platform;
use oo_bindgen::*;
use std::fs;

mod callback;
mod class;
mod conversion;
mod doc;
mod enumeration;
mod formatting;
mod interface;
mod structure;

const NATIVE_FUNCTIONS_CLASSNAME: &str = "NativeFunctions";

pub fn generate_java_bindings(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    fs::create_dir_all(&config.java_output_dir)?;

    // Create the pom.xml
    generate_pom(lib, config)?;

    // Copy the compiled libraries to the resource folder
    let mut ffi_name = config.ffi_name.clone();
    ffi_name.push_str("_java");
    for platform in config
        .platforms
        .iter()
        .filter(|x| x.platform != Platform::LinuxMusl)
    {
        let mut target_dir = config.java_resource_dir();
        target_dir.push(platform.platform.to_string());
        fs::create_dir_all(&target_dir)?;

        let mut source_file = platform.location.clone();
        source_file.push(platform.bin_filename(&ffi_name));

        let mut target_file = target_dir.clone();
        target_file.push(platform.bin_filename(&ffi_name));

        fs::copy(source_file, target_file)?;
    }

    // Create the source directory
    fs::create_dir_all(&config.java_source_dir(lib))?;

    // Create all the direct mappings
    generate_native_func_class(lib, config)?;

    // Generate the user-facing stuff
    generate_structs(lib, config)?;
    generate_enums(lib, config)?;
    generate_classes(lib, config)?;
    generate_interfaces(lib, config)?;
    generate_callbacks(lib, config)?;

    Ok(())
}

fn generate_pom(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    // Open file
    let mut filename = config.java_output_dir.clone();
    filename.push("pom");
    filename.set_extension("xml");
    let mut f = FilePrinter::new(filename)?;

    f.writeln("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    f.writeln("<project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd\">")?;
    indented(&mut f, |f| {
        f.writeln("<modelVersion>4.0.0</modelVersion>")?;
        f.newline()?;
        f.writeln(&format!("<groupId>{}</groupId>", config.group_id))?;
        f.writeln(&format!(
            "<artifactId>{}</artifactId>",
            lib.name.to_kebab_case()
        ))?;
        f.writeln(&format!("<version>{}</version>", lib.version.to_string()))?;
        f.newline()?;
        f.writeln("<properties>")?;
        f.writeln("    <project.java.version>1.8</project.java.version>")?;
        f.writeln("    <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>")?;
        f.writeln("    <maven.compiler.target>1.8</maven.compiler.target>")?;
        f.writeln("    <maven.compiler.source>1.8</maven.compiler.source>")?;
        f.writeln("</properties>")?;
        f.newline()?;
        f.writeln("<dependencies>")?;
        f.writeln("    <dependency>")?;
        f.writeln("        <groupId>org.jooq</groupId>")?;
        f.writeln("        <artifactId>joou-java-6</artifactId>")?;
        f.writeln("        <version>0.9.4</version>")?;
        f.writeln("    </dependency>")?;
        f.writeln("    <dependency>")?;
        f.writeln("        <groupId>org.apache.commons</groupId>")?;
        f.writeln("        <artifactId>commons-lang3</artifactId>")?;
        f.writeln("        <version>3.11</version>")?;
        f.writeln("    </dependency>")?;
        f.writeln("</dependencies>")?;

        Ok(())
    })?;
    f.writeln("</project>")
}

fn generate_native_func_class(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    let mut f = create_file(NATIVE_FUNCTIONS_CLASSNAME, config, lib)?;

    f.writeln(&format!("class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
    blocked(&mut f, |f| {
        // Load the library
        f.writeln("static")?;
        blocked(f, |f| {
            f.writeln("try")?;
            blocked(f, |f| {
                let libname = format!("{}_java", config.ffi_name);
                for platform in config.platforms.iter() {
                    match platform.platform {
                        Platform::Win64 => {
                            f.writeln("if(org.apache.commons.lang3.SystemUtils.IS_OS_WINDOWS && org.apache.commons.lang3.ArchUtils.getProcessor().is64Bit())")?;
                            blocked(f, |f| {
                                f.writeln(&format!(
                                    "loadLibrary(\"{}\", \"{}\", \"dll\");",
                                    platform.platform.to_string(),
                                    libname
                                ))
                            })?;
                        }
                        Platform::Linux => {
                            f.writeln("if(org.apache.commons.lang3.SystemUtils.IS_OS_LINUX && org.apache.commons.lang3.ArchUtils.getProcessor().is64Bit())")?;
                            blocked(f, |f| {
                                f.writeln(&format!(
                                    "loadLibrary(\"{}\", \"lib{}\", \"so\");",
                                    platform.platform.to_string(),
                                    libname
                                ))
                            })?;
                        }
                        Platform::LinuxMusl => (), // We do not generate Java bindings for Linux musl
                    }
                }
                Ok(())
            })?;
            f.writeln("catch(java.io.IOException e)")?;
            blocked(f, |f| {
                f.writeln("System.err.println(\"Native code library failed to load: \" + e);")?;
                f.writeln("System.exit(1);")
            })
        })?;

        f.newline()?;

        // Load library helper function
        f.writeln("private static void loadLibrary(String directory, String name, String extension) throws java.io.IOException {")?;
        f.writeln("    java.io.InputStream stream = NativeFunctions.class.getResourceAsStream(\"/\" + directory + \"/\" + name + \".\" + extension);")?;
        f.writeln("    java.nio.file.Path tempFilePath = java.nio.file.Files.createTempFile(name, \".\" + extension);")?;
        f.writeln("    tempFilePath.toFile().deleteOnExit();")?;
        f.writeln("    java.nio.file.Files.copy(stream, tempFilePath, java.nio.file.StandardCopyOption.REPLACE_EXISTING);")?;
        f.writeln("    System.load(tempFilePath.toString());")?;
        f.writeln("}")?;

        f.newline()?;

        // Write each native functions
        for handle in lib.native_functions() {
            if let Some(first_param) = handle.parameters.first() {
                if let Type::ClassRef(class_handle) = &first_param.param_type {
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
            if let ReturnType::Type(return_type, _) = &handle.return_type {
                if let Type::ClassRef(class_handle) = &return_type {
                    // We don't want to generate the `create` method of collections
                    if lib.find_collection(&class_handle.name).is_some() {
                        continue;
                    }
                }
            }

            f.writeln(&format!(
                "static native {} {}(",
                handle.return_type.as_java_primitive(),
                handle.name
            ))?;

            f.write(
                &handle
                    .parameters
                    .iter()
                    .map(|param| format!("{} {}", param.param_type.as_java_primitive(), param.name))
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(");")?;
            f.newline()?;
        }

        Ok(())
    })
}

fn generate_structs(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for native_struct in lib.structs() {
        let mut f = create_file(&native_struct.name().to_camel_case(), config, lib)?;
        structure::generate(&mut f, native_struct, lib)?;
    }

    Ok(())
}

fn generate_enums(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for native_enum in lib.native_enums() {
        let mut f = create_file(&native_enum.name.to_camel_case(), config, lib)?;
        enumeration::generate(&mut f, native_enum, lib)?;
    }

    Ok(())
}

fn generate_classes(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for class in lib.classes() {
        let mut f = create_file(&class.name().to_camel_case(), config, lib)?;
        class::generate(&mut f, class, lib)?;
    }

    Ok(())
}

fn generate_interfaces(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for interface in lib.interfaces() {
        let mut f = create_file(&interface.name.to_camel_case(), config, lib)?;
        interface::generate(&mut f, interface, lib)?;
    }

    Ok(())
}

fn generate_callbacks(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for callback in lib.one_time_callbacks() {
        let mut f = create_file(&callback.name.to_camel_case(), config, lib)?;
        callback::generate(&mut f, callback, lib)?;
    }

    Ok(())
}

fn create_file(
    name: &str,
    config: &JavaBindgenConfig,
    lib: &Library,
) -> FormattingResult<FilePrinter> {
    // Open file
    let mut filename = config.java_source_dir(lib);
    filename.push(name);
    filename.set_extension("java");
    let mut f = FilePrinter::new(filename)?;

    print_license(&mut f, &lib.license)?;
    print_package(&mut f, config, lib)?;
    f.newline()?;

    Ok(f)
}

fn print_license(f: &mut dyn Printer, license: &[String]) -> FormattingResult<()> {
    commented(f, |f| {
        for line in license.iter() {
            f.writeln(line)?;
        }
        Ok(())
    })
}

fn print_package(
    f: &mut dyn Printer,
    config: &JavaBindgenConfig,
    lib: &Library,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "package {}.{};",
        config.group_id,
        lib.name.to_kebab_case()
    ))?;
    f.newline()?;
    f.writeln("import org.joou.*;")
}
