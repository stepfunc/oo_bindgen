use std::ffi::{CStr, CString};

pub struct StringCollection {
    values: Vec<CString>,
}

pub struct StringCollectionWithReserve {
    values: Vec<CString>,
}

impl StringCollection {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn add(&mut self, value: CString) {
        self.values.push(value);
    }
}

impl StringCollectionWithReserve {
    fn new(reserve: usize) -> Self {
        Self {
            values: Vec::with_capacity(reserve),
        }
    }

    fn add(&mut self, value: CString) {
        self.values.push(value);
    }
}

pub unsafe fn string_collection_create() -> *mut StringCollection {
    let it = Box::new(StringCollection::new());
    Box::into_raw(it)
}

pub unsafe fn string_collection_destroy(col: *mut StringCollection) {
    if !col.is_null() {
        drop(Box::from_raw(col));
    }
}

pub unsafe fn string_collection_add(col: *mut StringCollection, value: &CStr) {
    if let Some(col) = col.as_mut() {
        let value = value.to_owned();
        col.add(value);
    }
}

pub unsafe fn string_collection_with_reserve_create(
    reserve: u32,
) -> *mut StringCollectionWithReserve {
    let it = Box::new(StringCollectionWithReserve::new(reserve as usize));
    Box::into_raw(it)
}

pub unsafe fn string_collection_with_reserve_destroy(col: *mut StringCollectionWithReserve) {
    if !col.is_null() {
        drop(Box::from_raw(col));
    }
}

pub unsafe fn string_collection_with_reserve_add(
    col: *mut StringCollectionWithReserve,
    value: &CStr,
) {
    if let Some(col) = col.as_mut() {
        let value = value.to_owned();
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

pub unsafe fn collection_with_reserve_size(col: *mut StringCollectionWithReserve) -> u32 {
    if let Some(col) = col.as_ref() {
        col.values.len() as u32
    } else {
        0
    }
}

pub unsafe fn collection_get<'a>(col: *mut StringCollection, idx: u32) -> &'a CStr {
    if let Some(col) = col.as_ref() {
        if let Some(value) = col.values.get(idx as usize) {
            value
        } else {
            CStr::from_ptr(std::ptr::null())
        }
    } else {
        CStr::from_ptr(std::ptr::null())
    }
}

pub unsafe fn collection_with_reserve_get<'a>(
    col: *mut StringCollectionWithReserve,
    idx: u32,
) -> &'a CStr {
    if let Some(col) = col.as_ref() {
        if let Some(value) = col.values.get(idx as usize) {
            value
        } else {
            CStr::from_ptr(std::ptr::null())
        }
    } else {
        CStr::from_ptr(std::ptr::null())
    }
}
