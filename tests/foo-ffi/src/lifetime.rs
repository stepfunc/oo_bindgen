
pub struct IteratorWithLifeTime<'a> {
    phantom: std::marker::PhantomData<&'a usize>,
}

pub unsafe fn next_value_with_lifetime(
    _it: *mut crate::IteratorWithLifeTime,
) -> Option<&crate::ffi::IteratorItem> {
    None
}

