use crate::ffi;
use std::time::Duration;

pub struct CallbackSource {
    callback: Option<ffi::CallbackInterface>,
    value: u32,
}

impl CallbackSource {
    fn new() -> Self {
        Self {
            callback: None,
            value: 0,
        }
    }

    fn set(&mut self, cb: ffi::CallbackInterface) {
        self.callback = Some(cb);
    }

    fn set_value(&mut self, value: u32) -> u32 {
        self.value = value;
        self.callback
            .as_ref()
            .map_or(0, |cb| cb.on_value(value).unwrap_or(0))
    }

    fn set_duration(&mut self, value: Duration) -> Duration {
        self.callback
            .as_ref()
            .map_or(Duration::from_millis(0), |cb| {
                cb.on_duration(value.as_millis() as u64)
                    .map_or(Duration::from_millis(0), Duration::from_millis)
            })
    }
}

pub unsafe fn cbsource_new() -> *mut CallbackSource {
    let cb_source = Box::new(CallbackSource::new());
    Box::into_raw(cb_source)
}

pub unsafe fn cbsource_destroy(cb_source: *mut CallbackSource) {
    if !cb_source.is_null() {
        Box::from_raw(cb_source);
    }
}

pub unsafe fn cbsource_set_interface(cb_source: *mut CallbackSource, cb: ffi::CallbackInterface) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set(cb);
}

pub unsafe fn cbsource_call_one_time(
    cb_source: *mut CallbackSource,
    cb: ffi::OneTimeCallbackInterface,
) -> u32 {
    let cb_source = cb_source.as_mut().unwrap();
    cb.on_value(cb_source.value).unwrap_or(0)
}

pub unsafe fn cbsource_set_value(cb_source: *mut CallbackSource, value: u32) -> u32 {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_value(value)
}

pub unsafe fn cbsource_set_duration(cb_source: *mut CallbackSource, value: u64) -> u64 {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source
        .set_duration(Duration::from_millis(value))
        .as_millis() as u64
}
