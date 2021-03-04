use std::ffi::CStr;
use std::str::Utf8Error;

pub(crate) fn get_special_number(password: &CStr) -> std::result::Result<u32, crate::ffi::MyError> {
    if password.to_str()? == "solarwinds123" {
        Ok(42)
    } else {
        Err(crate::ffi::MyError::BadPassword)
    }
}

impl From<Utf8Error> for crate::ffi::MyError {
    fn from(_: Utf8Error) -> Self {
        crate::ffi::MyError::BadPassword
    }
}
