use std::ffi::CStr;

pub(crate) fn get_special_number(password: &CStr) -> std::result::Result<u32, crate::ffi::MyError> {
    if password.to_str().unwrap() == "solarwinds123" {
        Ok(42)
    } else {
        Err(crate::ffi::MyError::BadPassword)
    }
}
