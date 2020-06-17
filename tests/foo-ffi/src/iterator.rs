use std::ffi::CStr;
use std::os::raw::c_char;
use crate::ffi;

pub struct StringIterator {
    iter: std::vec::IntoIter<u8>,
    current: Option<ffi::StringIteratorItem>,
}

impl StringIterator {
    fn new(vec: Vec<u8>) -> Self {
        Self {
            iter: vec.into_iter(),
            current: None,
        }
    }

    fn next(&mut self) {
        match self.iter.next() {
            Some(val) => self.current = Some(ffi::StringIteratorItem{value: val}),
            None => self.current = None,
        }
    }
}

pub unsafe fn iterator_create(value: *const c_char) -> *mut StringIterator {
    let bytes = CStr::from_ptr(value).to_bytes().to_vec();
    let it = Box::new(StringIterator::new(bytes));
    Box::into_raw(it)
}

pub unsafe fn iterator_destroy(it: *mut StringIterator) {
    if !it.is_null() {
        Box::from_raw(it);
    }
}

pub unsafe fn iterator_next(value: *mut StringIterator) -> *const ffi::StringIteratorItem {
    if let Some(it) = value.as_mut() {
        it.next();
        match &it.current {
            Some(val) => val as *const _,
            None => std::ptr::null(),
        }
    } else {
        std::ptr::null()
    }
}
