use std::ffi::CStr;
use std::str::Utf8Error;

pub struct ClassWithPassword {
    value: u32,
}

const SPECIAL_VALUE: u32 = 42;
const PASSWORD: &str = "12345";

pub(crate) fn get_special_value(password: &CStr) -> std::result::Result<u32, crate::ffi::MyError> {
    if password.to_str()? == PASSWORD {
        Ok(SPECIAL_VALUE)
    } else {
        Err(crate::ffi::MyError::BadPassword)
    }
}

pub(crate) fn validate_password(password: &CStr) -> std::result::Result<(), crate::ffi::MyError> {
    if password.to_str()? == PASSWORD {
        Ok(())
    } else {
        Err(crate::ffi::MyError::BadPassword)
    }
}

pub(crate) fn echo_password(password: &CStr) -> std::result::Result<&CStr, crate::ffi::MyError> {
    if password.to_str()? == PASSWORD {
        Ok(password)
    } else {
        Err(crate::ffi::MyError::BadPassword)
    }
}

pub(crate) fn class_with_password_create(
    password: &CStr,
) -> std::result::Result<*mut crate::ClassWithPassword, crate::ffi::MyError> {
    if password.to_str()? == PASSWORD {
        Ok(Box::into_raw(Box::new(ClassWithPassword {
            value: SPECIAL_VALUE,
        })))
    } else {
        Err(crate::ffi::MyError::BadPassword)
    }
}

pub(crate) unsafe fn class_with_password_get_special_value(
    instance: *mut crate::ClassWithPassword,
) -> Result<u32, crate::ffi::MyError> {
    match instance.as_ref() {
        Some(x) => Ok(x.value),
        None => Err(crate::ffi::MyError::NullArgument),
    }
}

pub(crate) unsafe fn class_with_password_destroy(instance: *mut crate::ClassWithPassword) {
    if !instance.is_null() {
        Box::from_raw(instance);
    }
}

impl From<Utf8Error> for crate::ffi::MyError {
    fn from(_: Utf8Error) -> Self {
        crate::ffi::MyError::BadPassword
    }
}
