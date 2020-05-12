use oo_bindgen::*;
use oo_bindgen::native_struct::*;
use oo_bindgen::formatting::*;
use crate::*;

pub fn generate(f: &mut impl Printer, native_struct: &NativeStructHandle, lib: &Library) -> FormattingResult<()> {
    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("public struct {}", native_struct.name()))?;
        blocked(f, |f| {
            for el in &native_struct.elements {
                let dotnet_type = DotnetType(&el.element_type);
                
                // If there is conversion to do, hide the native type under a properties
                if let Some(converter) = dotnet_type.conversion() {
                    // Define backing type in C
                    let native_name = format!("_{}", el.name);
                    f.writeln(&format!("private {} {};", dotnet_type.as_native_type(), native_name))?;
                    
                    // Define property accessors
                    f.writeln(&format!("public {} {}", dotnet_type.as_dotnet_type(), el.name))?;
                    blocked(f, |f| {
                        f.writeln("get")?;
                        blocked(f, |f| {
                            converter.convert_from_native(f, &native_name, "return ")
                        })?;
                        f.writeln("set")?;
                        blocked(f, |f| {
                            converter.convert_to_native(f, "value", &format!("{} = ", &native_name))
                        })
                    })?;
                } else {
                    f.writeln(&format!("public {} {};", dotnet_type.as_native_type(), el.name))?;
                }
                f.newline()?;
            }
            Ok(())
        })
    })
}
