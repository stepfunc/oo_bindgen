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

pub unsafe fn invoke_callback(values: &CStr, callback: ffi::ValuesReceiver) {
    let mut iter = StringIterator::new(values.to_bytes().to_vec());
    callback.on_characters(&mut iter)
}

pub unsafe fn string_iterator_next<'a>(
    value: *mut StringIterator,
) -> Option<&'a ffi::StringIteratorItem> {
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
