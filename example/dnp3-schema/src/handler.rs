use oo_bindgen::*;
use oo_bindgen::native_function::*;
use oo_bindgen::interface::InterfaceHandle;

pub fn define(lib: &mut LibraryBuilder) -> Result<InterfaceHandle, BindingError> {
    let control = lib.declare_native_struct("Control")?;
    let control = lib.define_native_struct(&control)?
        .add("fir", Type::Bool)?
        .add("fin", Type::Bool)?
        .add("con", Type::Bool)?
        .add("uns", Type::Bool)?
        .add("seq", Type::Uint8)?
        .build();

    let response_function = lib.define_native_enum("ResponseFunction")?
        .push("Response")?
        .push("UnsolicitedResponse")?
        .build();

    // TODO: add struct methods to isolate bits
    let iin1 = lib.declare_native_struct("IIN1")?;
    let iin1 = lib.define_native_struct(&iin1)?
        .add("value", Type::Uint8)?
        .build();

    let iin2 = lib.declare_native_struct("IIN2")?;
    let iin2 = lib.define_native_struct(&iin2)?
        .add("value", Type::Uint8)?
        .build();

    let iin = lib.declare_native_struct("IIN")?;
    let iin = lib.define_native_struct(&iin)?
        .add("iin1", Type::Struct(iin1.clone()))?
        .add("iin2", Type::Struct(iin2.clone()))?
        .build();

    let response_header = lib.declare_native_struct("ResponseHeader")?;
    let response_header = lib.define_native_struct(&response_header)?
        .add("control", Type::Struct(control.clone()))?
        .add("func", Type::Enum(response_function.clone()))?
        .add("iin", Type::Struct(iin.clone()))?
        .build();

    let variation_enum = lib.define_native_enum("Variation")?
        .push("Group1Var0")?
        .push("Group1Var1")?
        .push("Group1Var2")?
        .push("Group2Var0")?
        .push("Group2Var1")?
        .push("Group2Var2")?
        .push("Group2Var3")?
        .push("Group3Var0")?
        .push("Group3Var1")?
        .push("Group3Var2")?
        .push("Group4Var0")?
        .push("Group4Var1")?
        .push("Group4Var2")?
        .push("Group4Var3")?
        .push("Group10Var0")?
        .push("Group10Var1")?
        .push("Group10Var2")?
        .push("Group11Var0")?
        .push("Group11Var1")?
        .push("Group11Var2")?
        .push("Group12Var0")?
        .push("Group12Var1")?
        .push("Group13Var1")?
        .push("Group13Var2")?
        .push("Group20Var0")?
        .push("Group20Var1")?
        .push("Group20Var2")?
        .push("Group20Var5")?
        .push("Group20Var6")?
        .push("Group21Var0")?
        .push("Group21Var1")?
        .push("Group21Var2")?
        .push("Group21Var5")?
        .push("Group21Var6")?
        .push("Group21Var9")?
        .push("Group21Var10")?
        .push("Group22Var0")?
        .push("Group22Var1")?
        .push("Group22Var2")?
        .push("Group22Var5")?
        .push("Group22Var6")?
        .push("Group23Var0")?
        .push("Group23Var1")?
        .push("Group23Var2")?
        .push("Group23Var5")?
        .push("Group23Var6")?
        .push("Group30Var0")?
        .push("Group30Var1")?
        .push("Group30Var2")?
        .push("Group30Var3")?
        .push("Group30Var4")?
        .push("Group30Var5")?
        .push("Group30Var6")?
        .push("Group32Var0")?
        .push("Group32Var1")?
        .push("Group32Var2")?
        .push("Group32Var3")?
        .push("Group32Var4")?
        .push("Group32Var5")?
        .push("Group32Var6")?
        .push("Group32Var7")?
        .push("Group32Var8")?
        .push("Group40Var0")?
        .push("Group40Var1")?
        .push("Group40Var2")?
        .push("Group40Var3")?
        .push("Group40Var4")?
        .push("Group41Var0")?
        .push("Group41Var1")?
        .push("Group41Var2")?
        .push("Group41Var3")?
        .push("Group41Var4")?
        .push("Group42Var0")?
        .push("Group42Var1")?
        .push("Group42Var2")?
        .push("Group42Var3")?
        .push("Group42Var4")?
        .push("Group42Var5")?
        .push("Group42Var6")?
        .push("Group42Var7")?
        .push("Group42Var8")?
        .push("Group43Var1")?
        .push("Group43Var2")?
        .push("Group43Var3")?
        .push("Group43Var4")?
        .push("Group43Var5")?
        .push("Group43Var6")?
        .push("Group43Var7")?
        .push("Group43Var8")?
        .push("Group50Var1")?
        .push("Group50Var3")?
        .push("Group50Var4")?
        .push("Group51Var1")?
        .push("Group51Var2")?
        .push("Group52Var1")?
        .push("Group52Var2")?
        .push("Group60Var1")?
        .push("Group60Var2")?
        .push("Group60Var3")?
        .push("Group60Var4")?
        .push("Group80Var1")?
        .push("Group110")?
        .push("Group111")?
        .push("Group112")?
        .push("Group113")?
        .build();

    let qualifier_code_enum = lib.define_native_enum("QualifierCode")?
        .push("Range8")?
        .push("Range16")?
        .push("AllObjects")?
        .push("Count8")?
        .push("Count16")?
        .push("CountAndPrefix8")?
        .push("CountAndPrefix16")?
        .push("FreeFormat16")?
        .build();

    let header_info = lib.declare_native_struct("HeaderInfo")?;
    let header_info = lib.define_native_struct(&header_info)?
        .add("variation", Type::Enum(variation_enum.clone()))?
        .add("qualifier", Type::Enum(qualifier_code_enum.clone()))?
        .build();

    let flags_struct = lib.declare_native_struct("Flags")?;
    let flags_struct = lib.define_native_struct(&flags_struct)?
        .add("value", Type::Uint8)?
        .build();

    let time_quality_enum = lib.define_native_enum("TimeQuality")?
        .push("Synchronized")?
        .push("NotSynchronized")?
        .push("Invalid")?
        .build();

    let time_struct = lib.declare_native_struct("Time")?;
    let time_struct = lib.define_native_struct(&time_struct)?
        .add("value", Type::Uint64)?
        .add("quality", Type::Enum(time_quality_enum.clone()))?
        .build();

    let binary_struct = lib.declare_native_struct("Binary")?;
    let binary_struct = lib.define_native_struct(&binary_struct)?
        .add("index", Type::Uint16)?
        .add("value", Type::Bool)?
        .add("flags", Type::Struct(flags_struct.clone()))?
        .add("time", Type::Struct(time_struct))?
        .build();

    let binary_iterator = lib.declare_class("BinaryIterator")?;
    let binary_next_fn = lib.declare_native_function("binary_next")?
        .param("it", Type::ClassRef(binary_iterator.clone()))?
        .return_type(ReturnType::Type(Type::StructRef(binary_struct.declaration())))?
        .build()?;

    let binary_iterator = lib.define_class(&binary_iterator)?
        .method("next", &binary_next_fn)?
        .build();

    let read_handler_interface = lib.define_interface("ReadHandler")?
        .callback("begin_fragment")?
            .param("header", Type::Struct(response_header.clone()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("end_fragment")?
            .param("header", Type::Struct(response_header.clone()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_binary")?
            .param("info", Type::Struct(header_info.clone()))?
            .param("iter", Type::ClassRef(binary_iterator.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    Ok(read_handler_interface)
}
