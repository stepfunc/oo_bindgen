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
        .push("Response", "Solicited response")?
        .push("UnsolicitedResponse", "Unsolicited response")?
        .doc("Type of response")?
        .build()?;

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
        .push("Range8", "8-bit start stop (0x00)")?
        .push("Range16", "16-bit start stop (0x01)")?
        .push("AllObjects", "All objects (0x06)")?
        .push("Count8", "8-bit count (0x07)")?
        .push("Count16", "16-bit count (0x08)")?
        .push("CountAndPrefix8", "8-bit count and prefix (0x17)")?
        .push("CountAndPrefix16", "16-bit count and prefix (0x28)")?
        .push("FreeFormat16", "16-bit free format (0x5B)")?
        .doc("Qualifier code used in the response")?
        .build()?;

    let header_info = lib.declare_native_struct("HeaderInfo")?;
    let header_info = lib
        .define_native_struct(&header_info)?
        .add("variation", Type::Enum(variation_enum))?
        .add("qualifier", Type::Enum(qualifier_code_enum))?
        .build();

    let flags_struct = declare_flags_struct(lib)?;

    let time_quality_enum = lib
        .define_native_enum("TimeQuality")?
        .push("Synchronized", "The timestamp is UTC synchronized at the remote device")?
        .push("NotSynchronized", "The device indicates the timestamp may be not be synchronized")?
        .push("Invalid", "Timestamp is not valid, ignore the value and use a local timestamp")?
        .doc("Timestamp quality")?
        .build()?;

    let timestamp_struct = lib.declare_native_struct("Timestamp")?;
    let timestamp_struct = lib
        .define_native_struct(&timestamp_struct)?
        .add("value", Type::Uint64)?
        .add("quality", Type::Enum(time_quality_enum))?
        .build();

    let double_bit_enum = lib
        .define_native_enum("DoubleBit")?
        .push("Intermediate", "Transition between conditions")?
        .push("DeterminedOff", "Determined to be OFF")?
        .push("DeterminedOn", "Determined to be ON")?
        .push("Indeterminate", "Abnormal or custom condition")?
        .doc("Double-bit binary input value")?
        .build()?;

    let binary_it = build_iterator("Binary", Type::Bool, lib, &flags_struct, &timestamp_struct)?;
    let double_bit_binary_it = build_iterator(
        "DoubleBitBinary",
        Type::Enum(double_bit_enum),
        lib,
        &flags_struct,
        &timestamp_struct,
    )?;
    let bos_it = build_iterator(
        "BinaryOutputStatus",
        Type::Bool,
        lib,
        &flags_struct,
        &timestamp_struct,
    )?;
    let counter_it = build_iterator("Counter", Type::Uint32, lib, &flags_struct, &timestamp_struct)?;
    let frozen_counter_it = build_iterator(
        "FrozenCounter",
        Type::Uint32,
        lib,
        &flags_struct,
        &timestamp_struct,
    )?;
    let analog_it = build_iterator("Analog", Type::Double, lib, &flags_struct, &timestamp_struct)?;
    let aos_it = build_iterator(
        "AnalogOutputStatus",
        Type::Double,
        lib,
        &flags_struct,
        &timestamp_struct,
    )?;

    let read_handler_interface = lib
        .define_interface("ReadHandler")?
        .callback("begin_fragment")?
        .param("header", Type::Struct(response_header.clone()))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("end_fragment")?
        .param("header", Type::Struct(response_header))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("handle_binary")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(binary_it))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("handle_double_bit_binary")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(double_bit_binary_it))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("handle_binary_output_status")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(bos_it))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("handle_counter")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(counter_it))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("handle_frozen_counter")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(frozen_counter_it))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("handle_analog")?
        .param("info", Type::Struct(header_info.clone()))?
        .param("it", Type::Iterator(analog_it))?
        .arg("arg")?
        .return_type(ReturnType::void())?
        .build()?
        .callback("handle_analog_output_status")?
        .param("info", Type::Struct(header_info))?
        .param("it", Type::Iterator(aos_it))?
        .arg("arg")?
        .return_type(ReturnType::void())?
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
        .push("Broadcast", "Indicate that the message was broadcasted")?
        .push("Class1Events", "Outstation has Class 1 events not reported yet")?
        .push("Class2Events", "Outstation has Class 2 events not reported yet")?
        .push("Class3Events", "Outstation has Class 3 events not reported yet")?
        .push("NeedTime", "Outstation indicates it requires time synchronization from the master")?
        .push("LocalControl", "At least one point of the outstation is in the local operation mode")?
        .push("DeviceTrouble", "Outstation reports abnormal condition")?
        .push("DeviceRestart", "Outstation has restarted")?
        .doc("First IIN bit flags")?
        .build()?;

    let iin1_is_set_fn = lib
        .declare_native_function("iin1_is_set")?
        .param("iin1", Type::StructRef(iin1.declaration()), "IIN1 to check")?
        .param("flag", Type::Enum(iin1_flag), "Flag to check")?
        .return_type(ReturnType::new(Type::Bool, "true if the flag is set, false otherwise"))?
        .doc("Check if a particular flag is set in the IIN1 byte")?
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
        .push("NoFuncCodeSupport", "Function code is not supported by the outstation")?
        .push("ObjectUnknown", "Request contains an unknown point")?
        .push("ParameterError", "Unable to parse request or invalid qualifier code")?
        .push("EventBufferOverflow", "Event buffer overflow, at least one event was lost")?
        .push("AlreadyExecuting", "Cannot perform operation because an execution is already in progress")?
        .push("ConfigCorrupt", "Outstation reports a configuration corruption")?
        .doc("Second IIN bit flags")?
        .build()?;

    let iin2_is_set_fn = lib
        .declare_native_function("iin2_is_set")?
        .param("iin2", Type::StructRef(iin2.declaration()), "IIN2 to check")?
        .param("flag", Type::Enum(iin2_flag), "Flag to check")?
        .return_type(ReturnType::new(Type::Bool, "true if the flag is set, false otherwise"))?
        .doc("Check if a particular flag is set in the IIN2 byte")?
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
        .push("Online", "Point is online")?
        .push("Restart", "Point has not been updated from the field since device reset")?
        .push("CommLost", "Communication failure between the device where the data originates and the reporting device")?
        .push("RemoteForced", "The data value is overridden in a downstream reporting device")?
        .push("LocalForced", "The data value is overridden bu the device that reports this flag as set")?
        .push("ChatterFilter", "The binary data value is presently changing between states at a sufficiently high enough rate to activate a chatter filter (only for single and double-bit binary input objects)")?
        .push("Rollover", "Counter has rollover (only for counter objects). This flag is obsolete.")?
        .push("Discontinuity", "The reported counter value cannot be compared against a prior value to obtain the correct count difference (only for counter objects)")?
        .push("OverRange", "The data object's true value exceeds the valid measurement range of the object (only for analog input and output objects)")?
        .push("ReferenceErr", "The measurement process determined that the object's data value might not have the expected level of accuracy (only for analog input and output objects)")?
        .doc("Single bit in point flag")?
        .build()?;

    let flags_is_set_fn = lib
        .declare_native_function("flags_is_set")?
        .param("flags", Type::StructRef(flags_struct.declaration()), "Flags byte to check")?
        .param("flag", Type::Enum(flag_enum), "Flag to check")?
        .return_type(ReturnType::new(Type::Bool, "true if flag is set, false otherwise"))?
        .doc("Check if a particular flag is set in the flags byte")?
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
    timestamp_struct: &NativeStructHandle,
) -> Result<IteratorHandle, BindingError> {
    let value_struct = lib.declare_native_struct(name)?;
    let value_struct = lib
        .define_native_struct(&value_struct)?
        .add("index", Type::Uint16)?
        .add("value", value_type)?
        .add("flags", Type::Struct(flags_struct.clone()))?
        .add("time", Type::Struct(timestamp_struct.clone()))?
        .build();

    let value_iterator = lib.declare_class(&format!("{}Iterator", name))?;
    let iterator_next_fn = lib
        .declare_native_function(&format!("{}_next", name.to_lowercase()))?
        .param("it", Type::ClassRef(value_iterator.clone()), "Iterator")?
        .return_type(ReturnType::new(Type::StructRef(
            value_struct.declaration(),
        ), "Next value of the iterator or NULL if the iterator reached the end"))?
        .doc("Get the next value of the iterator")?
        .build()?;

    let value_iterator = lib.define_iterator(&iterator_next_fn, &value_struct)?;

    Ok(value_iterator)
}
