pub(crate) fn get_u32_value(cb: crate::ffi::DefaultedInterface) -> u32 {
    cb.get_u32_value().unwrap_or(0)
}

pub(crate) fn get_duration_value(cb: crate::ffi::DefaultedInterface) -> std::time::Duration {
    cb.get_duration_ms()
        .unwrap_or(std::time::Duration::from_secs(0))
}
