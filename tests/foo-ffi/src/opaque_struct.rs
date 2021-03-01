pub(crate) unsafe fn opaque_struct_get_id(value: Option<&crate::ffi::OpaqueStruct>) -> u64 {
    value.unwrap().id
}

pub(crate) fn opaque_struct_magic_init() -> crate::ffi::OpaqueStruct {
    crate::ffi::OpaqueStruct { id: 42 }
}
