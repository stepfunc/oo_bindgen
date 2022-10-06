pub struct PrimitivePointers {
    bool_value: bool,
    u8_value: u8,
    float_value: f32,
    double_value: f64,
}

impl Default for PrimitivePointers {
    fn default() -> Self {
        Self {
            bool_value: false,
            u8_value: 0,
            float_value: 0.0,
            double_value: 0.0,
        }
    }
}

pub(crate) fn primitive_pointers_create() -> *mut crate::PrimitivePointers {
    Box::leak(Box::new(PrimitivePointers::default()))
}

pub(crate) unsafe fn primitive_pointers_destroy(instance: *mut crate::PrimitivePointers) {
    drop(Box::from_raw(instance));
}

pub(crate) unsafe fn primitive_pointers_get_bool(
    instance: *mut crate::PrimitivePointers,
    value: bool,
) -> *const bool {
    let mut instance = instance.as_mut().unwrap();
    instance.bool_value = value;
    &instance.bool_value
}

pub(crate) unsafe fn primitive_pointers_get_u8(
    instance: *mut crate::PrimitivePointers,
    value: u8,
) -> *const u8 {
    let mut instance = instance.as_mut().unwrap();
    instance.u8_value = value;
    &instance.u8_value
}

pub(crate) unsafe fn primitive_pointers_get_float(
    instance: *mut crate::PrimitivePointers,
    value: f32,
) -> *const f32 {
    let mut instance = instance.as_mut().unwrap();
    instance.float_value = value;
    &instance.float_value
}

pub(crate) unsafe fn primitive_pointers_get_double(
    instance: *mut crate::PrimitivePointers,
    value: f64,
) -> *const f64 {
    let mut instance = instance.as_mut().unwrap();
    instance.double_value = value;
    &instance.double_value
}
