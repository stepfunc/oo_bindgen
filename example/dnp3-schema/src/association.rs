use oo_bindgen::*;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder, association_class: ClassDeclarationHandle) -> Result<(), BindingError> {
    let destroy_fn = lib.declare_native_function("association_destroy")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    // Time sync stuff
    let timesync_mode = lib.define_native_enum("TimeSyncMode")?
        .push("LAN")?
        .push("NonLAN")?
        .build();

    let timesync_result = lib.define_native_enum("TimeSyncResult")?
        .push("Success")?
        .push("TaskError")?
        .push("ClockRollback")?
        .push("SystemTimeNotUnix")?
        .push("BadOutstationTimeDelay")?
        .push("Overflow")?
        .push("StillNeedsTime")?
        .push("IINError")?
        .build();

    let timesync_cb = lib.define_one_time_callback("TimeSyncTaskCallback")?
        .callback("on_complete")?
            .param("result", Type::Enum(timesync_result.clone()))?
            .arg("arg")?
            .return_type(ReturnType::Void)?
            .build()?
        .arg("arg")?
        .build()?;

    let perform_time_sync_fn = lib.declare_native_function("association_perform_time_sync")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .param("mode", Type::Enum(timesync_mode.clone()))?
        .param("callback", Type::OneTimeCallback(timesync_cb.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    lib.define_class(&association_class)?
        .destructor(&destroy_fn)?
        .async_method("PerformTimeSync", &perform_time_sync_fn)?
        .build();

    Ok(())
}
