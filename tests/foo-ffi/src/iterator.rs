use crate::ffi;
use std::ffi::CStr;

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
            Some(val) => self.current = Some(ffi::StringIteratorItemFields { value: val }),
            None => self.current = None,
        }
    }
}

pub unsafe fn iterator_create<'a>(value: &'a CStr) -> *mut StringIterator {
    let bytes = value.to_bytes().to_vec();
    let it = Box::new(StringIterator::new(bytes));
    Box::into_raw(it)
}

pub unsafe fn iterator_destroy(it: *mut StringIterator) {
    if !it.is_null() {
        Box::from_raw(it);
    }
}

pub unsafe fn iterator_next<'a>(value: *mut StringIterator) -> Option<&'a ffi::StringIteratorItem> {
    if let Some(it) = value.as_mut() {
        it.next();
        match &it.current {
            Some(val) => Some(val),
            None => None,
        }
    } else {
        None
    }
}
