use crate::ffi;

static mut CONSTRUCTION_COUNTER: u32 = 0;

pub struct TestClass {
    value: u32,
}

pub unsafe fn test_class_create(value: u32) -> *mut TestClass {
    CONSTRUCTION_COUNTER += 1;
    let testclass = Box::new(TestClass { value });
    Box::into_raw(testclass)
}

pub unsafe fn test_class_destroy(testclass: *mut TestClass) {
    CONSTRUCTION_COUNTER -= 1;
    if !testclass.is_null() {
        drop(Box::from_raw(testclass));
    };
}

pub unsafe fn test_class_get_value(testclass: *const TestClass) -> u32 {
    let testclass = testclass.as_ref().unwrap();
    testclass.value
}

pub unsafe fn test_class_increment_value(testclass: *mut TestClass) {
    let testclass = testclass.as_mut().unwrap();
    testclass.value += 1;
}

pub unsafe fn test_class_add_async(
    testclass: *const TestClass,
    value: u32,
    cb: ffi::GetValueCallback,
) {
    let testclass = testclass.as_ref().unwrap();
    cb.on_complete(testclass.value + value);
}

pub unsafe fn construction_counter() -> u32 {
    CONSTRUCTION_COUNTER
}
