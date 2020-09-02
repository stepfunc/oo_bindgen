use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct StringCollection {
    values: Vec<CString>,
}

impl StringCollection {
    fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    fn add(&mut self, value: CString) {
        self.values.push(value);
    }
}

pub unsafe fn collection_create() -> *mut StringCollection {
    let it = Box::new(StringCollection::new());
    Box::into_raw(it)
}

pub unsafe fn collection_destroy(col: *mut StringCollection) {
    if !col.is_null() {
        Box::from_raw(col);
    }
}

pub unsafe fn collection_add(col: *mut StringCollection, value: *const c_char) {
    if let Some(col) = col.as_mut() {
        let value = CStr::from_ptr(value).to_owned();
        col.add(value);
    }
}

pub unsafe fn collection_size(col: *mut StringCollection) -> u32 {
    if let Some(col) = col.as_ref() {
        col.values.len() as u32
    } else {
        0
    }
}

pub unsafe fn collection_get(col: *mut StringCollection, idx: u32) -> *const c_char {
    if let Some(col) = col.as_ref() {
        if let Some(value) = col.values.get(idx as usize) {
            value.as_ptr()
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}
