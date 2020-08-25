pub struct IteratorWithLifeTime<'a> {
    phantom: std::marker::PhantomData<&'a usize>
}

pub unsafe fn next_value_with_lifetime(_it: *mut crate::IteratorWithLifeTime) -> *const crate::ffi::IteratorItem {
    return std::ptr::null()
}