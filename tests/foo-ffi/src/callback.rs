use std::ffi::c_void;
use std::time::Duration;

#[repr(C)]
pub struct NativeCallbackInterface {
    on_value: extern "C" fn(value: u32, data: *mut c_void),
    on_duration: extern "C" fn(value: u64, data: *mut c_void),
    on_destroy: extern "C" fn(data: *mut c_void),
    data: *mut c_void,
}

struct CallbackAdapter {
    native_cb: NativeCallbackInterface,
}

impl CallbackAdapter {
    fn new(native_cb: NativeCallbackInterface) -> Self {
        Self { native_cb }
    }
}

impl CallbackInterface for CallbackAdapter {
    fn on_value(&self, value: u32) {
        (self.native_cb.on_value)(value, self.native_cb.data);
    }

    fn on_duration(&self, value: Duration) {
        (self.native_cb.on_duration)(value.as_millis() as u64, self.native_cb.data);
    }
}

impl Drop for CallbackAdapter {
    fn drop(&mut self) {
        (self.native_cb.on_destroy)(self.native_cb.data);
    }
}

trait CallbackInterface {
    fn on_value(&self, value: u32);
    fn on_duration(&self, value: Duration);
}

pub struct CallbackSource {
    callbacks: Vec<Box<dyn CallbackInterface>>,
}

impl CallbackSource {
    fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    fn add(&mut self, cb: Box<dyn CallbackInterface>) {
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

#[no_mangle]
pub unsafe extern "C" fn cbsource_new() -> *mut CallbackSource {
    let cb_source = Box::new(CallbackSource::new());
    Box::into_raw(cb_source)
}

#[no_mangle]
pub unsafe extern "C" fn cbsource_destroy(cb_source: *mut CallbackSource) {
    if !cb_source.is_null() {
        Box::from_raw(cb_source);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cbsource_add(cb_source: *mut CallbackSource, cb: NativeCallbackInterface) {
    let cb_adapter = Box::new(CallbackAdapter::new(cb));

    let cb_source = cb_source.as_mut().unwrap();
    cb_source.add(cb_adapter);
}

#[no_mangle]
pub unsafe extern "C" fn cbsource_set_value(cb_source: *mut CallbackSource, value: u32) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_value(value);
}

#[no_mangle]
pub unsafe extern "C" fn cbsource_set_duration(cb_source: *mut CallbackSource, value: u64) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_duration(Duration::from_millis(value));
}
