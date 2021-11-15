use self::conversion::*;
use self::formatting::*;
use crate::JavaBindgenConfig;
use oo_bindgen::formatting::*;
use oo_bindgen::function::*;
use oo_bindgen::platforms::Platform;
use oo_bindgen::structs::StructType;
use oo_bindgen::*;
use std::fs;

mod class;
mod constant;
mod conversion;
mod doc;
mod enumeration;
mod exception;
mod formatting;
mod interface;
mod structure;

const NATIVE_FUNCTIONS_CLASSNAME: &str = "NativeFunctions";

const SUPPORTED_PLATFORMS: &[Platform] = &[
    Platform::WinX64Msvc,
    Platform::LinuxX64Gnu,
    Platform::LinuxArm8Gnu,
];

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
        .filter(|x| SUPPORTED_PLATFORMS.iter().any(|y| *y == x.platform))
    {
        let mut target_dir = config.java_resource_dir();
        target_dir.push(platform.platform.as_string());
        fs::create_dir_all(&target_dir)?;

        let mut source_file = platform.location.clone();
        source_file.push(platform.bin_filename(&ffi_name));

        let mut target_file = target_dir.clone();
        target_file.push(platform.bin_filename(&ffi_name));

        fs::copy(&source_file, &target_file)?;
    }

    // Copy the extra files
    fs::copy(
        &lib.info.license_path,
        config
            .java_resource_dir()
            .join(lib.info.license_path.file_name().unwrap()),
    )?;
    for path in &config.extra_files {
        let dest = config.java_resource_dir().join(path.file_name().unwrap());
        fs::copy(path, dest)?;
    }

    // Create the source directory
    fs::create_dir_all(&config.java_source_dir(lib))?;

    // Create all the direct mappings
    generate_native_func_class(lib, config)?;

    // Generate the user-facing stuff
    generate_constants(lib, config)?;
    generate_exceptions(lib, config)?;
    generate_structs(lib, config)?;
    generate_enums(lib, config)?;
    generate_classes(lib, config)?;
    generate_interfaces(lib, config)?;

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
            lib.settings.name.kebab_case()
        ))?;
        f.writeln(&format!("<version>{}</version>", lib.version.to_string()))?;

        f.newline()?;

        // General metadata
        f.writeln(&format!("<name>{}</name>", lib.settings.name))?;
        f.writeln(&format!(
            "<description>{}</description>",
            lib.info.description
        ))?;
        f.writeln(&format!("<url>{}</url>", lib.info.project_url))?;
        f.writeln("<scm>")?;
        f.writeln(&format!(
            "    <connection>scm:git:git://github.com/{}.git</connection>",
            lib.info.repository
        ))?;
        f.writeln(&format!(
            "    <developerConnection>scm:git:ssh://github.com:{}.git</developerConnection>",
            lib.info.repository
        ))?;
        f.writeln(&format!(
            "    <url>http://github.com/{}/tree/master</url>",
            lib.info.repository
        ))?;
        f.writeln("</scm>")?;
        f.writeln("<developers>")?;
        for developer in &lib.info.developers {
            f.writeln("<developer>")?;
            f.writeln(&format!("    <name>{}</name>", developer.name))?;
            f.writeln(&format!("    <email>{}</email>", developer.email))?;
            f.writeln(&format!(
                "    <organization>{}</organization>",
                developer.organization
            ))?;
            f.writeln(&format!(
                "    <organizationUrl>{}</organizationUrl>",
                developer.organization_url
            ))?;
            f.writeln("</developer>")?;
        }
        f.writeln("</developers>")?;

        f.newline()?;

        // License description
        f.writeln("<licenses>")?;
        f.writeln("    <license>")?;
        f.writeln(&format!("        <name>{}</name>", lib.info.license_name))?;
        f.writeln(&format!(
            "        <url>https://github.com/{}/blob/master/{}</url>",
            lib.info.repository,
            lib.info.license_path.to_string_lossy()
        ))?;
        f.writeln("    </license>")?;
        f.writeln("</licenses>")?;

        f.newline()?;

        // General properties
        f.writeln("<properties>")?;
        f.writeln("    <project.java.version>1.8</project.java.version>")?;
        f.writeln("    <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>")?;
        f.writeln("    <maven.compiler.target>1.8</maven.compiler.target>")?;
        f.writeln("    <maven.compiler.source>1.8</maven.compiler.source>")?;
        f.writeln("</properties>")?;

        f.newline()?;

        // Dependencies
        f.writeln("<dependencies>")?;
        f.writeln("    <dependency>")?;
        f.writeln("        <groupId>org.jooq</groupId>")?;
        f.writeln("        <artifactId>joou-java-6</artifactId>")?;
        f.writeln("        <version>0.9.4</version>")?;
        f.writeln("    </dependency>")?;
        f.writeln("</dependencies>")?;

        f.newline()?;

        // Other central repository requirements
        f.writeln("<build>")?;
        f.writeln("  <plugins>")?;
        f.writeln("    <plugin>")?;
        f.writeln("      <groupId>org.apache.maven.plugins</groupId>")?;
        f.writeln("      <artifactId>maven-source-plugin</artifactId>")?;
        f.writeln("      <version>2.2.1</version>")?;
        f.writeln("      <executions>")?;
        f.writeln("        <execution>")?;
        f.writeln("          <id>attach-sources</id>")?;
        f.writeln("          <goals>")?;
        f.writeln("            <goal>jar-no-fork</goal>")?;
        f.writeln("          </goals>")?;
        f.writeln("        </execution>")?;
        f.writeln("      </executions>")?;
        f.writeln("    </plugin>")?;
        f.writeln("    <plugin>")?;
        f.writeln("      <groupId>org.apache.maven.plugins</groupId>")?;
        f.writeln("      <artifactId>maven-javadoc-plugin</artifactId>")?;
        f.writeln("      <version>2.9.1</version>")?;
        f.writeln("      <executions>")?;
        f.writeln("        <execution>")?;
        f.writeln("        <id>attach-javadocs</id>")?;
        f.writeln("        <goals>")?;
        f.writeln("          <goal>jar</goal>")?;
        f.writeln("        </goals>")?;
        f.writeln("        </execution>")?;
        f.writeln("      </executions>")?;
        f.writeln("    </plugin>")?;
        f.writeln("  </plugins>")?;
        f.writeln("</build>")?;

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
                let env_variable_name = format!(
                    "{}_NATIVE_LIB_LOCATION",
                    lib.settings.name.capital_snake_case()
                );
                f.writeln(&format!(
                    "String nativeLibLocation = System.getenv(\"{}\");",
                    env_variable_name
                ))?;
                f.writeln("if(nativeLibLocation != null)")?;
                blocked(f, |f| f.writeln("System.load(nativeLibLocation);"))?;

                f.writeln("else")?;
                blocked(f, |f| {
                    f.writeln("boolean loaded = false;")?;
                    let libname = format!("{}_java", config.ffi_name);
                    for platform in config.platforms.iter() {
                        match platform.platform {
                            Platform::WinX64Msvc => {
                                f.writeln("if(!loaded)")?;
                                blocked(f, |f| {
                                    f.writeln(&format!(
                                        "loaded = loadLibrary(\"{}\", \"{}\", \"dll\");",
                                        platform.platform.as_string(),
                                        libname
                                    ))
                                })?;
                            }
                            Platform::LinuxX64Gnu => {
                                f.writeln("if(!loaded)")?;
                                blocked(f, |f| {
                                    f.writeln(&format!(
                                        "loaded = loadLibrary(\"{}\", \"lib{}\", \"so\");",
                                        platform.platform.as_string(),
                                        libname
                                    ))
                                })?;
                            }
                            Platform::LinuxArm8Gnu => {
                                f.writeln("if(!loaded)")?;
                                blocked(f, |f| {
                                    f.writeln(&format!(
                                        "loaded = loadLibrary(\"{}\", \"lib{}\", \"so\");",
                                        platform.platform.as_string(),
                                        libname
                                    ))
                                })?;
                            }
                            _ => (), // Other platforms are not supported
                        }
                    }

                    f.writeln("if(!loaded)")?;
                    blocked(f, |f| {
                        f.writeln("throw new Exception(\"Unable to load any of the included native library\");")
                    })
                })
            })?;
            f.writeln("catch(Exception e)")?;
            blocked(f, |f| {
                f.writeln("System.err.println(\"Native code library failed to load: \" + e);")?;
                f.writeln("System.exit(1);")
            })
        })?;

        f.newline()?;

        // Load library helper function
        f.writeln("private static boolean loadLibrary(String directory, String name, String extension) throws Exception")?;
        blocked(f, |f| {
            f.writeln("try")?;
            blocked(f, |f| {
                f.writeln("java.io.InputStream stream = NativeFunctions.class.getResourceAsStream(\"/\" + directory + \"/\" + name + \".\" + extension);")?;
                f.writeln("java.nio.file.Path tempFilePath = java.nio.file.Files.createTempFile(name, \".\" + extension);")?;
                f.writeln("tempFilePath.toFile().deleteOnExit();")?;
                f.writeln("java.nio.file.Files.copy(stream, tempFilePath, java.nio.file.StandardCopyOption.REPLACE_EXISTING);")?;
                f.writeln("System.load(tempFilePath.toString());")?;
                f.writeln("return true;")
            })?;
            f.writeln("catch(Exception e)")?;
            blocked(f, |f| f.writeln("return false;"))
        })?;

        f.newline()?;

        fn skip(c: FunctionCategory) -> bool {
            match c {
                FunctionCategory::Native => false,
                // we don't generate any of these
                FunctionCategory::CollectionCreate => true,
                FunctionCategory::CollectionDestroy => true,
                FunctionCategory::CollectionAdd => true,
                FunctionCategory::IteratorNext => true,
            }
        }

        for handle in lib.functions().filter(|func| !skip(func.category)) {
            f.writeln(&format!(
                "static native {} {}(",
                handle.return_type.as_java_primitive(),
                handle.name
            ))?;

            let args = handle
                .arguments
                .iter()
                .map(|param| format!("{} {}", param.arg_type.as_java_primitive(), param.name))
                .collect::<Vec<String>>()
                .join(", ");

            f.write(&args)?;
            f.write(");")?;
            f.newline()?;
        }

        Ok(())
    })
}

fn generate_constants(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for set in lib.constants() {
        let mut f = create_file(&set.name.camel_case(), config, lib)?;
        constant::generate(&mut f, set)?;
    }

    Ok(())
}

fn generate_exceptions(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for error in lib.error_types() {
        let mut f = create_file(&error.exception_name.camel_case(), config, lib)?;
        exception::generate(&mut f, error)?;
    }

    Ok(())
}

fn generate_structs(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for st in lib.structs() {
        let mut f = create_file(&st.name().camel_case(), config, lib)?;
        match st {
            StructType::FunctionArg(x) => structure::generate(&mut f, x)?,
            StructType::FunctionReturn(x) => structure::generate(&mut f, x)?,
            StructType::CallbackArg(x) => structure::generate(&mut f, x)?,
            StructType::Universal(x) => structure::generate(&mut f, x)?,
        }
    }

    Ok(())
}

fn generate_enums(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for native_enum in lib.enums() {
        let mut f = create_file(&native_enum.name.camel_case(), config, lib)?;
        enumeration::generate(&mut f, native_enum)?;
    }

    Ok(())
}

fn generate_classes(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for class in lib.classes() {
        let mut f = create_file(&class.name().camel_case(), config, lib)?;
        class::generate(&mut f, class)?;
    }

    for class in lib.static_classes() {
        let mut f = create_file(&class.name.camel_case(), config, lib)?;
        class::generate_static(&mut f, class)?;
    }

    Ok(())
}

fn generate_interfaces(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for interface in lib.untyped_interfaces() {
        let mut f = create_file(&interface.name.camel_case(), config, lib)?;
        interface::generate(&mut f, interface)?;
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

    print_license(&mut f, &lib.info.license_description)?;
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
        lib.settings.name.kebab_case()
    ))?;
    f.newline()?;
    f.writeln("import org.joou.*;")
}
