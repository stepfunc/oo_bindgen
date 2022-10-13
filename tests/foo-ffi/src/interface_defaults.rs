pub(crate) fn invoke_do_nothing(cb: crate::ffi::DefaultedInterface) {
    cb.do_nothing()
}

pub(crate) fn get_bool_value(cb: crate::ffi::DefaultedInterface) -> bool {
    cb.get_bool_value().unwrap()
}

pub(crate) fn get_i32_value(cb: crate::ffi::DefaultedInterface) -> i32 {
    cb.get_i32_value().unwrap()
}

pub(crate) fn get_u32_value(cb: crate::ffi::DefaultedInterface) -> u32 {
    cb.get_u32_value().unwrap()
}

pub(crate) fn get_duration_value(cb: crate::ffi::DefaultedInterface) -> std::time::Duration {
    cb.get_duration_ms().unwrap()
}

pub(crate) fn get_switch_pos(cb: crate::ffi::DefaultedInterface) -> crate::ffi::SwitchPosition {
    cb.get_switch_position().unwrap()
}

pub(crate) fn get_wrapped_number(cb: crate::ffi::DefaultedInterface) -> crate::ffi::WrappedNumber {
    cb.get_wrapped_number().unwrap()
}
