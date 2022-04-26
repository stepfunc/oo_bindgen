pub(crate) fn invoke_universal_interface(
    value: crate::ffi::UniversalOuterStruct,
    callback: crate::ffi::UniversalInterface,
) -> crate::ffi::UniversalOuterStruct {
    callback.on_value(value).unwrap()
}
