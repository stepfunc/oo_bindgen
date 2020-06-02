use crate::ffi::Structure;

pub(crate) fn struct_by_value_echo(value: Structure) -> Structure {
    value
}

pub(crate) unsafe fn struct_by_reference_echo(value: *const Structure) -> Structure {
    value.as_ref().unwrap().clone()
}
