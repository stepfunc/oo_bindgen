pub struct Int32Iterator {
    current: i32,
    next: i32,
    max: i32,
}

impl Int32Iterator {
    pub(crate) fn new(start: i32, max: i32) -> Self {
        Self {
            current: start,
            next: start,
            max,
        }
    }
}

pub(crate) fn int32_iterator_next(it: *mut Int32Iterator) -> *const i32 {
    let mut it = unsafe { it.as_mut().unwrap() };
    if it.next > it.max {
        return std::ptr::null();
    }
    it.current = it.next;
    it.next += 1;
    &it.current
}

pub(crate) fn invoke_int32_callback(callback: crate::ffi::IntValueReceiver) {
    let mut iter = Int32Iterator::new(1, 3);
    callback.on_int32(&mut iter)
}
