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

pub(crate) fn get_struct(
    password: &CStr,
) -> std::result::Result<crate::ffi::OtherStructure, crate::ffi::MyError> {
    if password.to_str()? == PASSWORD {
        Ok(crate::ffi::OtherStructureFields {
            test: 41,
            first_enum_value: crate::ffi::StructureEnum::Var2,
            int1: 1,
            bool2: false,
            second_enum_value: crate::ffi::StructureEnum::Var2,
        }
        .into())
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

pub(crate) unsafe fn destroy_class_with_password(instance: *mut crate::ClassWithPassword) {
    if !instance.is_null() {
        Box::from_raw(instance);
    }
}

impl From<Utf8Error> for crate::ffi::MyError {
    fn from(_: Utf8Error) -> Self {
        crate::ffi::MyError::BadPassword
    }
}
