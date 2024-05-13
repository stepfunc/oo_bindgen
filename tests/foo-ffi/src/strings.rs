use std::ffi::{CStr, CString};

pub struct StringClass {
    value: CString,
}

impl StringClass {
    fn new() -> Self {
        Self {
            value: CString::new("").unwrap(),
        }
    }
}

pub unsafe fn string_class_create() -> *mut StringClass {
    let string_class = Box::new(StringClass::new());
    Box::into_raw(string_class)
}

pub unsafe fn string_class_destroy(string_class: *mut StringClass) {
    if !string_class.is_null() {
        drop(Box::from_raw(string_class));
    }
}

pub unsafe fn string_class_echo(string_class: *mut StringClass, value: &CStr) -> &CStr {
    let string_class = string_class.as_mut().unwrap();
    value.clone_into(&mut string_class.value);
    &string_class.value
}

pub unsafe fn string_length(value: &CStr) -> u32 {
    value.to_string_lossy().len() as u32
}
