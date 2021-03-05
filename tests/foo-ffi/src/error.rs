use std::ffi::CStr;
use std::str::Utf8Error;

pub struct ClassWithPassword {
    value: u32,
}

const SPECIAL_VALUE: u32 = 42;
const PASSWORD: &str = "12345";

pub(crate) fn get_special_number(password: &CStr) -> std::result::Result<u32, crate::ffi::MyError> {
    if password.to_str()? == PASSWORD {
        Ok(SPECIAL_VALUE)
    } else {
        Err(crate::ffi::MyError::BadPassword)
    }
}

pub(crate) fn create_class_with_password(
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

pub(crate) unsafe fn get_special_value_from_class(
    instance: *mut crate::ClassWithPassword,
) -> Result<u32, crate::ffi::MyError> {
    match instance.as_ref() {
        Some(x) => Ok(x.value),
        None => Err(crate::ffi::MyError::NullArgument),
    }
}

pub(crate) unsafe fn destroy_class_with_password(
    instance: *mut crate::ClassWithPassword,
) -> crate::ffi::MyError {
    if instance.is_null() {
        return crate::ffi::MyError::NullArgument;
    }

    Box::from_raw(instance);
    crate::ffi::MyError::Ok
}

impl From<Utf8Error> for crate::ffi::MyError {
    fn from(_: Utf8Error) -> Self {
        crate::ffi::MyError::BadPassword
    }
}
