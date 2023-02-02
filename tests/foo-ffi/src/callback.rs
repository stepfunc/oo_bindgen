use std::time::Duration;

use crate::ffi;

pub struct CallbackSource {
    callback: Option<ffi::CallbackInterface>,
    _value: u32,
}

impl CallbackSource {
    fn new() -> Self {
        Self {
            callback: None,
            _value: 0,
        }
    }

    fn set(&mut self, cb: ffi::CallbackInterface) {
        self.callback = Some(cb);
    }

    fn set_value(&mut self, value: u32) -> u32 {
        self._value = value;
        self.callback
            .as_ref()
            .map_or(0, |cb| cb.on_value(value).unwrap_or(0))
    }

    fn set_duration(&mut self, value: Duration) -> Duration {
        self.callback
            .as_ref()
            .map_or(Duration::from_millis(0), |cb| {
                cb.on_duration(value)
                    .map_or(Duration::from_millis(0), |value| value)
            })
    }
}

pub unsafe fn callback_source_create() -> *mut CallbackSource {
    let cb_source = Box::new(CallbackSource::new());
    Box::into_raw(cb_source)
}

pub unsafe fn callback_source_destroy(cb_source: *mut CallbackSource) {
    if !cb_source.is_null() {
        drop(Box::from_raw(cb_source));
    }
}

pub unsafe fn callback_source_set_interface(
    cb_source: *mut CallbackSource,
    cb: ffi::CallbackInterface,
) {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set(cb);
}

pub unsafe fn callback_source_set_value(cb_source: *mut CallbackSource, value: u32) -> u32 {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_value(value)
}

pub unsafe fn callback_source_set_duration(
    cb_source: *mut CallbackSource,
    value: Duration,
) -> Duration {
    let cb_source = cb_source.as_mut().unwrap();
    cb_source.set_duration(value)
}

pub unsafe fn callback_source_invoke_on_names(
    cb_source: *mut crate::CallbackSource,
    names: ffi::Names,
) {
    let cb_source = cb_source.as_mut().unwrap();

    let names = ffi::NamesFields {
        first_name: names.first_name(),
        last_name: names.last_name(),
    };

    if let Some(cb) = &cb_source.callback {
        cb.on_names(names.into());
    }
}
