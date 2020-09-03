use crate::ffi;

pub(crate) fn struct_by_value_echo(value: ffi::Structure) -> ffi::Structure {
    {
        value.interface_value.on_value(&value);
    }
    value
}

pub(crate) unsafe fn struct_by_reference_echo(value: *const ffi::Structure) -> ffi::Structure {
    let value = std::ptr::read(value);
    {
        value.interface_value.on_value(&value);
    }

    value
}
