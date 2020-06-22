use oo_bindgen::class::*;
use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(
    lib: &mut LibraryBuilder,
    association_class: ClassDeclarationHandle,
    request_class: ClassHandle,
) -> Result<(), BindingError> {
    let destroy_fn = lib
        .declare_native_function("association_destroy")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    // Poll stuff
    let poll = lib.declare_class("Poll")?;

    let poll_demand_fn = lib.declare_native_function("poll_demand")?
        .param("poll", Type::ClassRef(poll.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let poll_destroy_fn = lib.declare_native_function("poll_destroy")?
        .param("poll", Type::ClassRef(poll.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let poll = lib.define_class(&poll)?
        .destructor(&poll_destroy_fn)?
        .method("Demand", &poll_demand_fn)?
        .build();

    let add_poll_fn = lib.declare_native_function("association_add_poll")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .param("request", Type::ClassRef(request_class.declaration()))?
        .param("period", Type::Duration(DurationMapping::Milliseconds))?
        .return_type(ReturnType::Type(Type::ClassRef(poll.declaration())))?
        .build()?;

    // Read stuff
    let read_result = lib
        .define_native_enum("ReadResult")?
        .push("Success")?
        .push("TaskError")?
        .build();

    let read_cb = lib
        .define_one_time_callback("ReadTaskCallback")?
        .callback("on_complete")?
        .param("result", Type::Enum(read_result.clone()))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .arg("arg")?
        .build()?;

    let read_fn = lib
        .declare_native_function("association_read")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .param("request", Type::ClassRef(request_class.declaration()))?
        .param("callback", Type::OneTimeCallback(read_cb.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    // Command stuff
    let command_mode = lib
        .define_native_enum("CommandMode")?
        .push("DirectOperate")?
        .push("SelectBeforeOperate")?
        .build();

    let trip_close_code = lib
        .define_native_enum("TripCloseCode")?
        .variant("Nul", 0)?
        .variant("Close", 1)?
        .variant("Trip", 2)?
        .variant("Reserved", 3)?
        .build();

    let op_type = lib
        .define_native_enum("OpType")?
        .variant("Nul", 0)?
        .variant("PulseOn", 1)?
        .variant("PulseOff", 2)?
        .variant("LatchOn", 3)?
        .variant("LatchOff", 4)?
        .build();

    let control_code = lib.declare_native_struct("ControlCode")?;
    let control_code = lib
        .define_native_struct(&control_code)?
        .add("tcc", Type::Enum(trip_close_code.clone()))?
        .add("clear", Type::Bool)?
        .add("queue", Type::Bool)?
        .add("op_type", Type::Enum(op_type.clone()))?
        .build();

    let g12v1_struct = lib.declare_native_struct("G12V1")?;
    let g12v1_struct = lib
        .define_native_struct(&g12v1_struct)?
        .add("code", Type::Struct(control_code.clone()))?
        .add("count", Type::Uint8)?
        .add("on_time", Type::Uint32)?
        .add("off_time", Type::Uint32)?
        .build();

    let command = lib.declare_class("Command")?;

    let command_new_fn = lib
        .declare_native_function("command_new")?
        .return_type(ReturnType::Type(Type::ClassRef(command.clone())))?
        .build()?;

    let command_destroy_fn = lib
        .declare_native_function("command_destroy")?
        .param("command", Type::ClassRef(command.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u8_g12v1_fn = lib
        .declare_native_function("command_add_u8_g12v1")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint8)?
        .param("header", Type::Struct(g12v1_struct.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u16_g12v1_fn = lib
        .declare_native_function("command_add_u16_g12v1")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint16)?
        .param("header", Type::Struct(g12v1_struct.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u8_g41v1_fn = lib
        .declare_native_function("command_add_u8_g41v1")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint8)?
        .param("value", Type::Sint32)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u16_g41v1_fn = lib
        .declare_native_function("command_add_u16_g41v1")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint16)?
        .param("value", Type::Sint32)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u8_g41v2_fn = lib
        .declare_native_function("command_add_u8_g41v2")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint8)?
        .param("value", Type::Sint16)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u16_g41v2_fn = lib
        .declare_native_function("command_add_u16_g41v2")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint16)?
        .param("value", Type::Sint16)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u8_g41v3_fn = lib
        .declare_native_function("command_add_u8_g41v3")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint8)?
        .param("value", Type::Float)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u16_g41v3_fn = lib
        .declare_native_function("command_add_u16_g41v3")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint16)?
        .param("value", Type::Float)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u8_g41v4_fn = lib
        .declare_native_function("command_add_u8_g41v4")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint8)?
        .param("value", Type::Double)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command_add_u16_g41v4_fn = lib
        .declare_native_function("command_add_u16_g41v4")?
        .param("command", Type::ClassRef(command.clone()))?
        .param("idx", Type::Uint16)?
        .param("value", Type::Double)?
        .return_type(ReturnType::Void)?
        .build()?;

    let command = lib
        .define_class(&command)?
        .constructor(&command_new_fn)?
        .destructor(&command_destroy_fn)?
        .method("AddU8G12V1", &command_add_u8_g12v1_fn)?
        .method("AddU16G12V1", &command_add_u16_g12v1_fn)?
        .method("AddU8G41V1", &command_add_u8_g41v1_fn)?
        .method("AddU16G41V1", &command_add_u16_g41v1_fn)?
        .method("AddU8G41V2", &command_add_u8_g41v2_fn)?
        .method("AddU16G41V2", &command_add_u16_g41v2_fn)?
        .method("AddU8G41V3", &command_add_u8_g41v3_fn)?
        .method("AddU16G41V3", &command_add_u16_g41v3_fn)?
        .method("AddU8G41V4", &command_add_u8_g41v4_fn)?
        .method("AddU16G41V4", &command_add_u16_g41v4_fn)?
        .build();

    let command_result = lib
        .define_native_enum("CommandResult")?
        .push("Success")?
        .push("TaskError")?
        .push("BadStatus")?
        .push("HeaderCountMismatch")?
        .push("HeaderTypeMismatch")?
        .push("ObjectCountMismatch")?
        .push("ObjectValueMismatch")?
        .build();

    let command_cb = lib
        .define_one_time_callback("CommandTaskCallback")?
        .callback("on_complete")?
        .param("result", Type::Enum(command_result.clone()))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .arg("arg")?
        .build()?;

    let operate_fn = lib
        .declare_native_function("association_operate")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .param("mode", Type::Enum(command_mode.clone()))?
        .param("command", Type::ClassRef(command.declaration()))?
        .param("callback", Type::OneTimeCallback(command_cb.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    // Time sync stuff
    let timesync_mode = lib
        .define_native_enum("TimeSyncMode")?
        .push("LAN")?
        .push("NonLAN")?
        .build();

    let timesync_result = lib
        .define_native_enum("TimeSyncResult")?
        .push("Success")?
        .push("TaskError")?
        .push("ClockRollback")?
        .push("SystemTimeNotUnix")?
        .push("BadOutstationTimeDelay")?
        .push("Overflow")?
        .push("StillNeedsTime")?
        .push("IINError")?
        .build();

    let timesync_cb = lib
        .define_one_time_callback("TimeSyncTaskCallback")?
        .callback("on_complete")?
        .param("result", Type::Enum(timesync_result.clone()))?
        .arg("arg")?
        .return_type(ReturnType::Void)?
        .build()?
        .arg("arg")?
        .build()?;

    let perform_time_sync_fn = lib
        .declare_native_function("association_perform_time_sync")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .param("mode", Type::Enum(timesync_mode.clone()))?
        .param("callback", Type::OneTimeCallback(timesync_cb.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    lib.define_class(&association_class)?
        .destructor(&destroy_fn)?
        .method("AddPoll", &add_poll_fn)?
        .async_method("Read", &read_fn)?
        .async_method("Operate", &operate_fn)?
        .async_method("PerformTimeSync", &perform_time_sync_fn)?
        .build();

    Ok(())
}
