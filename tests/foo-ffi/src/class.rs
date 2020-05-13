static mut CONSTRUCTION_COUNTER: u32 = 0;

pub struct TestClass {
    value: u32,
}

#[no_mangle]
pub unsafe extern "C" fn testclass_new(value: u32) -> *mut TestClass {
    CONSTRUCTION_COUNTER += 1;
    let testclass = Box::new(TestClass{ value });
    Box::into_raw(testclass)
}

#[no_mangle]
pub unsafe extern "C" fn testclass_destroy(testclass: *mut TestClass) {
    CONSTRUCTION_COUNTER -= 1;
    if !testclass.is_null() {
        Box::from_raw(testclass);
    };
}

#[no_mangle]
pub unsafe extern "C" fn testclass_get_value(testclass: *const TestClass) -> u32 {
    let testclass = testclass.as_ref().unwrap();
    testclass.value
}

#[no_mangle]
pub unsafe extern "C" fn testclass_increment_value(testclass: *mut TestClass) {
    let testclass = testclass.as_mut().unwrap();
    testclass.value += 1;
}

#[no_mangle]
pub unsafe extern "C" fn testclass_construction_counter() -> u32 {
    CONSTRUCTION_COUNTER
}
