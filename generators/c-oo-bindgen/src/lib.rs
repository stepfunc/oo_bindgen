use oo_bindgen::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;
use std::fmt::{Display};
use std::path::PathBuf;
use crate::formatting::*;

mod formatting;

pub struct CBindgenConfig {
    pub output_dir: PathBuf,
}

pub fn generate_c_header(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    let uppercase_name = lib.name.to_uppercase();

    // Open file
    let mut filename = config.output_dir.clone();
    filename.push("test");
    filename.set_extension("h");
    let mut f = FilePrinter::new(filename)?;

    // Print license
    let mut f = CommentedPrinter::new(&mut f);
    for line in lib.license.iter() {
        f.writeln(line)?;
    }
    let f = f.close();

    // Header guard
    f.writeln("#pragma once")?;
    f.newline()?;

    // C++ guard
    let mut f = CppGuardPrinter::new(f)?;
    f.newline()?;

    // Version number
    f.writeln(&format!("#define {}_VERSION_MAJOR {}", uppercase_name, lib.version.major))?;
    f.writeln(&format!("#define {}_VERSION_MINOR {}", uppercase_name, lib.version.minor))?;
    f.writeln(&format!("#define {}_VERSION_PATCH {}", uppercase_name, lib.version.patch))?;
    f.writeln(&format!("#define {}_VERSION_STRING \"{}\"", uppercase_name, lib.version.to_string()))?;
    f.newline()?;

    // Standard includes needed
    f.writeln("#include <stdint.h>")?;
    f.newline()?;

    // Iterate through each statement and print them
    for statement in lib.into_iter() {
        match statement {
            Statement::StructDeclaration(handle) => {
                f.writeln(&format!("struct {};", handle.name))?;
            },
            Statement::StructDefinition(handle) => write_struct_definition(&mut f, handle)?,
            Statement::ClassDeclaration(handle) => {
                f.writeln(&format!("struct {};", handle.name))?;
            }
            Statement::NativeFunctionDeclaration(handle) => write_function(&mut f, handle)?,
            _ => (),
        }
        f.newline()?;
    }

    Ok(())
}

fn write_struct_definition(f: &mut impl Printer, handle: &NativeStructHandle) -> FormattingResult<()> {
    f.writeln(&format!("typedef struct {}", handle.name()))?;
    f.writeln("{")?;

    let mut file = IndentedPrinter::new(f);
    for element in &handle.elements {
        file.writeln(&format!("{} {};", CType(&element.element_type), element.name))?;
    }
    let file = file.close();

    file.writeln(&format!("}} {};", handle.name()))
}

fn write_function(f: &mut impl Printer, handle: &NativeFunctionHandle) -> FormattingResult<()> {
    f.newline()?;
    f.write(&format!("{} {}(", CReturnType(&handle.return_type), handle.name))?;
    
    f.write(
        &handle.parameters.iter()
            .map(|param| format!("{} {}", CType(&param.param_type), param.name))
            .collect::<Vec<String>>()
            .join(", ")
    )?;

    f.write(");")
}

struct CReturnType<'a>(&'a ReturnType);

impl <'a> Display for CReturnType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            ReturnType::Void => write!(f, "void"),
            ReturnType::Type(return_type) => write!(f, "{}", CType(&return_type)),
        }
    }
}

struct CType<'a>(&'a Type);

impl<'a> Display for CType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Type::Bool => write!(f, "bool"),
            Type::Uint8 => write!(f, "uint8_t"),
            Type::Sint8 => write!(f, "int8_t"),
            Type::Uint16 => write!(f, "uint16_t"),
            Type::Sint16 => write!(f, "int16_t"),
            Type::Uint32 => write!(f, "uint32_t"),
            Type::Sint32 => write!(f, "int32_t"),
            Type::Uint64 => write!(f, "uint64_t"),
            Type::Sint64 => write!(f, "int64_t"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::String => unimplemented!(),
            Type::Struct(handle) => write!(f, "{}", handle.name()),
            Type::StructRef(handle) => write!(f, "{}*", handle.name),
            Type::ClassRef(handle) => write!(f, "{}*", handle.name),
        }
    }
}
