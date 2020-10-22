use crate::ffi;

pub(crate) fn struct_by_value_echo(value: ffi::Structure) -> ffi::Structure {
    {
        //value.interface_value().on_value(Some(&value));
    }
    value
}

pub(crate) unsafe fn struct_by_reference_echo(value: Option<&ffi::Structure>) -> ffi::Structure {
    let value = std::ptr::read(value.unwrap());
    {
        //value.interface_value().on_value(Some(&value));
    }

    value
}
