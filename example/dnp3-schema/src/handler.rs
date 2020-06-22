use oo_bindgen::callback::InterfaceHandle;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::NativeStructHandle;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder, variation_enum: NativeEnumHandle) -> Result<InterfaceHandle, BindingError> {
    let control = lib.declare_native_struct("Control")?;
    let control = lib
        .define_native_struct(&control)?
        .add("fir", Type::Bool)?
        .add("fin", Type::Bool)?
        .add("con", Type::Bool)?
        .add("uns", Type::Bool)?
        .add("seq", Type::Uint8)?
        .build();

    let response_function = lib
        .define_native_enum("ResponseFunction")?
        .push("Response")?
        .push("UnsolicitedResponse")?
        .build();

    let iin = declare_iin_struct(lib)?;

    let response_header = lib.declare_native_struct("ResponseHeader")?;
    let response_header = lib
        .define_native_struct(&response_header)?
        .add("control", Type::Struct(control))?
        .add("func", Type::Enum(response_function))?
        .add("iin", Type::Struct(iin))?
        .build();

    let qualifier_code_enum = lib
        .define_native_enum("QualifierCode")?
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
    let header_info = lib
        .define_native_struct(&header_info)?
        .add("variation", Type::Enum(variation_enum))?
        .add("qualifier", Type::Enum(qualifier_code_enum))?
        .build();

    let flags_struct = declare_flags_struct(lib)?;

    let time_quality_enum = lib
        .define_native_enum("TimeQuality")?
        .push("Synchronized")?
        .push("NotSynchronized")?
        .push("Invalid")?
        .build();

    let time_struct = lib.declare_native_struct("Time")?;
    let time_struct = lib
        .define_native_struct(&time_struct)?
        .add("value", Type::Uint64)?
        .add("quality", Type::Enum(time_quality_enum))?
        .build();

    let double_bit_enum = lib
        .define_native_enum("DoubleBit")?
        .push("Intermediate")?
        .push("DeterminedOff")?
        .push("DeterminedOn")?
        .push("Indeterminate")?
        .build();

    let binary_it = build_iterator("Binary", Type::Bool, lib, &flags_struct, &time_struct)?;
    let double_bit_binary_it = build_iterator(
        "DoubleBitBinary",
        Type::Enum(double_bit_enum),
        lib,
        &flags_struct,
        &time_struct,
    )?;
    let bos_it = build_iterator(
        "BinaryOutputStatus",
        Type::Bool,
        lib,
        &flags_struct,
        &time_struct,
    )?;
    let counter_it = build_iterator("Counter", Type::Uint32, lib, &flags_struct, &time_struct)?;
    let frozen_counter_it = build_iterator(
        "FrozenCounter",
        Type::Uint32,
        lib,
        &flags_struct,
        &time_struct,
    )?;
    let analog_it = build_iterator("Analog", Type::Double, lib, &flags_struct, &time_struct)?;
    let aos_it = build_iterator(
        "AnalogOutputStatus",
        Type::Double,
        lib,
        &flags_struct,
        &time_struct,
    )?;

    let read_handler_interface = lib
        .define_interface("ReadHandler")?
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
        .param("it", Type::Iterator(binary_it))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .callback("handle_double_bit_binary")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(double_bit_binary_it))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .callback("handle_binary_output_status")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(bos_it))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .callback("handle_counter")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(counter_it))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .callback("handle_frozen_counter")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(frozen_counter_it))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .callback("handle_analog")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(analog_it))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .callback("handle_analog_output_status")?
        .param("info", Type::Struct(header_info))?
        .param("it", Type::Iterator(aos_it))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .destroy_callback("on_destroy")?
        .arg("arg")?
        .build()?;

    Ok(read_handler_interface)
}

fn declare_iin_struct(lib: &mut LibraryBuilder) -> Result<NativeStructHandle, BindingError> {
    let iin1 = lib.declare_native_struct("IIN1")?;
    let iin1 = lib
        .define_native_struct(&iin1)?
        .add("value", Type::Uint8)?
        .build();

    let iin1_flag = lib
        .define_native_enum("IIN1Flag")?
        .push("Broadcast")?
        .push("Class1Events")?
        .push("Class2Events")?
        .push("Class3Events")?
        .push("NeedTime")?
        .push("LocalControl")?
        .push("DeviceTrouble")?
        .push("DeviceRestart")?
        .build();

    let iin1_is_set_fn = lib
        .declare_native_function("iin1_is_set")?
        .param("iin1", Type::StructRef(iin1.declaration()))?
        .param("flag", Type::Enum(iin1_flag))?
        .return_type(ReturnType::Type(Type::Bool))?
        .build()?;

    lib.define_struct(&iin1)?
        .method("IsSet", &iin1_is_set_fn)?
        .build();

    let iin2 = lib.declare_native_struct("IIN2")?;
    let iin2 = lib
        .define_native_struct(&iin2)?
        .add("value", Type::Uint8)?
        .build();

    let iin2_flag = lib
        .define_native_enum("IIN2Flag")?
        .push("NoFuncCodeSupport")?
        .push("ObjectUnknown")?
        .push("ParameterError")?
        .push("EventBufferOverflow")?
        .push("AlreadyExecuting")?
        .push("ConfigCorrupt")?
        .build();

    let iin2_is_set_fn = lib
        .declare_native_function("iin2_is_set")?
        .param("iin2", Type::StructRef(iin2.declaration()))?
        .param("flag", Type::Enum(iin2_flag))?
        .return_type(ReturnType::Type(Type::Bool))?
        .build()?;

    lib.define_struct(&iin2)?
        .method("IsSet", &iin2_is_set_fn)?
        .build();

    let iin = lib.declare_native_struct("IIN")?;
    let iin = lib
        .define_native_struct(&iin)?
        .add("iin1", Type::Struct(iin1))?
        .add("iin2", Type::Struct(iin2))?
        .build();

    Ok(iin)
}

fn declare_flags_struct(lib: &mut LibraryBuilder) -> Result<NativeStructHandle, BindingError> {
    let flags_struct = lib.declare_native_struct("Flags")?;
    let flags_struct = lib
        .define_native_struct(&flags_struct)?
        .add("value", Type::Uint8)?
        .build();

    let flag_enum = lib
        .define_native_enum("Flag")?
        .push("Online")?
        .push("Restart")?
        .push("CommLost")?
        .push("RemoteForced")?
        .push("LocalForced")?
        .push("ChatterFilter")?
        .push("Rollover")?
        .push("Discontinuity")?
        .push("OverRange")?
        .push("ReferenceErr")?
        .build();

    let flags_is_set_fn = lib
        .declare_native_function("flags_is_set")?
        .param("flags", Type::StructRef(flags_struct.declaration()))?
        .param("flag", Type::Enum(flag_enum))?
        .return_type(ReturnType::Type(Type::Bool))?
        .build()?;

    lib.define_struct(&flags_struct)?
        .method("IsSet", &flags_is_set_fn)?
        .build();

    Ok(flags_struct)
}

fn build_iterator(
    name: &str,
    value_type: Type,
    lib: &mut LibraryBuilder,
    flags_struct: &NativeStructHandle,
    time_struct: &NativeStructHandle,
) -> Result<IteratorHandle, BindingError> {
    let value_struct = lib.declare_native_struct(name)?;
    let value_struct = lib
        .define_native_struct(&value_struct)?
        .add("index", Type::Uint16)?
        .add("value", value_type)?
        .add("flags", Type::Struct(flags_struct.clone()))?
        .add("time", Type::Struct(time_struct.clone()))?
        .build();

    let value_iterator = lib.declare_class(&format!("{}Iterator", name))?;
    let iterator_next_fn = lib
        .declare_native_function(&format!("{}_next", name.to_lowercase()))?
        .param("it", Type::ClassRef(value_iterator.clone()))?
        .return_type(ReturnType::Type(Type::StructRef(
            value_struct.declaration(),
        )))?
        .build()?;

    let value_iterator = lib.define_iterator(&iterator_next_fn, &value_struct)?;

    Ok(value_iterator)
}
