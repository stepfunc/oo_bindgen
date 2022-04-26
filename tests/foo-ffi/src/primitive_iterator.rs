pub struct RangeIterator {
    current: u32,
    next: u32,
    max: u32,
}

impl RangeIterator {
    pub(crate) fn new(min: u32, max: u32) -> Self {
        Self {
            current: min,
            next: min,
            max,
        }
    }
}

pub(crate) fn range_iterator_next(it: *mut RangeIterator) -> *const u32 {
    let mut it = unsafe { it.as_mut().unwrap() };
    if it.next > it.max {
        return std::ptr::null();
    }
    it.current = it.next;
    it.next += 1;
    &it.current
}

pub(crate) fn invoke_range_callback(min: u32, max: u32, callback: crate::ffi::RangeReceiver) {
    let mut iter = RangeIterator::new(min, max);
    callback.on_range(&mut iter)
}
