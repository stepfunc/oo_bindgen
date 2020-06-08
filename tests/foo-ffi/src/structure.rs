use crate::ffi;

pub(crate) fn struct_by_value_echo(value: ffi::Structure) -> ffi::Structure {
    {
        let cb = CallbackAdapter::new(&value.interface_value);
        cb.on_value(&value);
    }
    value
}

pub(crate) unsafe fn struct_by_reference_echo(value: *const ffi::Structure) -> ffi::Structure {
    let value = value.as_ref().unwrap();
    {
        let cb = CallbackAdapter::new(&value.interface_value);
        cb.on_value(&value);
    }
    value.clone()
}

struct CallbackAdapter<'a> {
    native_cb: &'a ffi::StructureInterface,
}

impl<'a> CallbackAdapter<'a> {
    fn new(native_cb: &'a ffi::StructureInterface) -> Self {
        Self { native_cb }
    }

    fn on_value(&self, value: &ffi::Structure) {
        if let Some(cb) = self.native_cb.on_value {
            (cb)(value, self.native_cb.arg);
        }
    }
}

impl<'a> Drop for CallbackAdapter<'a> {
    fn drop(&mut self) {
        if let Some(cb) = self.native_cb.on_destroy {
            (cb)(self.native_cb.arg);
        }
    }
}
