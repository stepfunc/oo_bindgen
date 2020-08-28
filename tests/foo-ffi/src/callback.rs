use crate::ffi;
use std::time::Duration;

pub struct CallbackSource {
    callbacks: Vec<ffi::CallbackInterface>,
    value: u32,
}

impl CallbackSource {
    fn new() -> Self {
        Self {
            callbacks: Vec::new(),
            value: 0,
        }
    }

    fn add(&mut self, cb: ffi::CallbackInterface) {
        self.callbacks.push(cb);
    }

    fn set_value(&mut self, value: u32) {
        self.value = value;
        self.callbacks.iter().for_each(|cb| {
            cb.on_value(value);
        });
    }

    fn set_duration(&mut self, value: Duration) {
        self.callbacks.iter().for_each(|cb| {
            cb.on_duration(value.as_millis() as u64);
        });
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

pub unsafe fn cbsource_add(cb_source: *mut CallbackSource, cb: ffi::CallbackInterface) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.add(cb);
}

pub unsafe fn cbsource_add_one_time(
    cb_source: *mut CallbackSource,
    cb: ffi::OneTimeCallbackInterface,
) {
    let cb_source = cb_source.as_mut().unwrap();
    cb.on_value(cb_source.value);
}

pub unsafe fn cbsource_set_value(cb_source: *mut CallbackSource, value: u32) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_value(value);
}

pub unsafe fn cbsource_set_duration(cb_source: *mut CallbackSource, value: u64) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_duration(Duration::from_millis(value));
}
