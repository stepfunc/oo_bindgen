pub(crate) fn increment_universal_struct(
    mut value: crate::ffi::UniversalOuterStruct,
) -> crate::ffi::UniversalOuterStruct {
    value.inner.value += 1;
    value.delay += 1;
    value
}
