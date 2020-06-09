use std::time::Duration;
use crate::ffi::{CallbackInterface, OneTimeCallbackInterface};

struct CallbackAdapter {
    native_cb: CallbackInterface,
}

impl CallbackAdapter {
    fn new(native_cb: CallbackInterface) -> Self {
        Self { native_cb }
    }

    fn on_value(&self, value: u32) {
        if let Some(cb) = self.native_cb.on_value {
            (cb)(value, self.native_cb.data);
        }
    }

    fn on_duration(&self, value: Duration) {
        if let Some(cb) = self.native_cb.on_duration {
            (cb)(value.as_millis() as u64, self.native_cb.data);
        }
    }
}

impl Drop for CallbackAdapter {
    fn drop(&mut self) {
        if let Some(cb) = self.native_cb.on_destroy {
            (cb)(self.native_cb.data);
        }
    }
}

pub struct CallbackSource {
    callbacks: Vec<CallbackAdapter>,
}

impl CallbackSource {
    fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    fn add(&mut self, cb: CallbackAdapter) {
        self.callbacks.push(cb);
    }

    fn set_value(&mut self, value: u32) {
        self.callbacks.iter().for_each(|cb| {
            cb.on_value(value);
        });
    }

    fn set_duration(&mut self, value: Duration) {
        self.callbacks.iter().for_each(|cb| {
            cb.on_duration(value);
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

pub unsafe fn cbsource_add(cb_source: *mut CallbackSource, cb: CallbackInterface) {
    let cb_adapter = CallbackAdapter::new(cb);

    let cb_source = cb_source.as_mut().unwrap();
    cb_source.add(cb_adapter);
}

pub unsafe fn cbsource_add_one_time(cb_source: *mut CallbackSource, cb: OneTimeCallbackInterface) {
    // TODO: implement this and its tests
}

pub unsafe fn cbsource_set_value(cb_source: *mut CallbackSource, value: u32) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_value(value);
}

pub unsafe fn cbsource_set_duration(cb_source: *mut CallbackSource, value: u64) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_duration(Duration::from_millis(value));
}
