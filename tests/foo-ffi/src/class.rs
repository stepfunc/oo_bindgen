use crate::ffi;

static mut CONSTRUCTION_COUNTER: u32 = 0;

pub struct TestClass {
    value: u32,
}

pub unsafe fn testclass_new(value: u32) -> *mut TestClass {
    CONSTRUCTION_COUNTER += 1;
    let testclass = Box::new(TestClass { value });
    Box::into_raw(testclass)
}

pub unsafe fn testclass_destroy(testclass: *mut TestClass) {
    CONSTRUCTION_COUNTER -= 1;
    if !testclass.is_null() {
        Box::from_raw(testclass);
    };
}

pub unsafe fn testclass_get_value(testclass: *const TestClass) -> u32 {
    let testclass = testclass.as_ref().unwrap();
    testclass.value
}

pub unsafe fn testclass_increment_value(testclass: *mut TestClass) {
    let testclass = testclass.as_mut().unwrap();
    testclass.value += 1;
}

pub unsafe fn testclass_get_value_async(testclass: *const TestClass, cb: ffi::GetValueCallback) {
    let testclass = testclass.as_ref().unwrap();
    (cb.on_value.unwrap())(testclass.value, cb.data);
}

pub unsafe fn testclass_construction_counter() -> u32 {
    CONSTRUCTION_COUNTER
}
