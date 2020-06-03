use oo_bindgen::*;
use oo_bindgen::class::ClassHandle;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::NativeStructHandle;
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
        .add("iin1", Type::Struct(iin1))?
        .add("iin2", Type::Struct(iin2))?
        .build();

    let response_header = lib.declare_native_struct("ResponseHeader")?;
    let response_header = lib.define_native_struct(&response_header)?
        .add("control", Type::Struct(control))?
        .add("func", Type::Enum(response_function))?
        .add("iin", Type::Struct(iin))?
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
        .add("variation", Type::Enum(variation_enum))?
        .add("qualifier", Type::Enum(qualifier_code_enum))?
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
        .add("quality", Type::Enum(time_quality_enum))?
        .build();

    let double_bit_enum = lib.define_native_enum("DoubleBit")?
        .push("Intermediate")?
        .push("DeterminedOff")?
        .push("DeterminedOn")?
        .push("Indeterminate")?
        .build();

    let binary_it = build_iterator("Binary", Type::Bool, lib, &flags_struct, &time_struct)?;
    let double_bit_binary_it = build_iterator("DoubleBitBinary", Type::Enum(double_bit_enum), lib, &flags_struct, &time_struct)?;
    let bos_it = build_iterator("BinaryOutputStatus", Type::Bool, lib, &flags_struct, &time_struct)?;
    let counter_it = build_iterator("Counter", Type::Uint32, lib, &flags_struct, &time_struct)?;
    let frozen_counter_it = build_iterator("FrozenCounter", Type::Uint32, lib, &flags_struct, &time_struct)?;
    let analog_it = build_iterator("Analog", Type::Double, lib, &flags_struct, &time_struct)?;
    let aos_it = build_iterator("AnalogOutputStatus", Type::Double, lib, &flags_struct, &time_struct)?;

    let read_handler_interface = lib.define_interface("ReadHandler")?
        .callback("begin_fragment")?
            .param("header", Type::Struct(response_header.clone()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("end_fragment")?
            .param("header", Type::Struct(response_header))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_binary")?
            .param("info", Type::Struct(header_info.clone()))?
            .param("it", Type::ClassRef(binary_it.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_double_bit_binary")?
            .param("info", Type::Struct(header_info.clone()))?
            .param("it", Type::ClassRef(double_bit_binary_it.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_binary_output_status")?
            .param("info", Type::Struct(header_info.clone()))?
            .param("it", Type::ClassRef(bos_it.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_counter")?
            .param("info", Type::Struct(header_info.clone()))?
            .param("it", Type::ClassRef(counter_it.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_frozen_counter")?
            .param("info", Type::Struct(header_info.clone()))?
            .param("it", Type::ClassRef(frozen_counter_it.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_analog")?
            .param("info", Type::Struct(header_info.clone()))?
            .param("it", Type::ClassRef(analog_it.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .callback("handle_analog_output_status")?
            .param("info", Type::Struct(header_info))?
            .param("it", Type::ClassRef(aos_it.declaration()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    Ok(read_handler_interface)
}

fn build_iterator(name: &str, value_type: Type, lib: &mut LibraryBuilder, flags_struct: &NativeStructHandle, time_struct: &NativeStructHandle) -> Result<ClassHandle, BindingError> {
    let value_struct = lib.declare_native_struct(name)?;
    let value_struct = lib.define_native_struct(&value_struct)?
        .add("index", Type::Uint16)?
        .add("value", value_type)?
        .add("flags", Type::Struct(flags_struct.clone()))?
        .add("time", Type::Struct(time_struct.clone()))?
        .build();

    let value_iterator = lib.declare_class(&format!("{}Iterator", name))?;
    let iterator_next_fn = lib.declare_native_function(&format!("{}_next", name.to_lowercase()))?
        .param("it", Type::ClassRef(value_iterator.clone()))?
        .return_type(ReturnType::Type(Type::StructRef(value_struct.declaration())))?
        .build()?;

    let value_iterator = lib.define_class(&value_iterator)?
        .method("next", &iterator_next_fn)?
        .build();

    Ok(value_iterator)
}
