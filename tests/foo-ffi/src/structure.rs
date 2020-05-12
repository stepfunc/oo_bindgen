#[repr(C)]
#[derive(Clone)]
pub struct OtherStructure {
    test: u16,
}

#[repr(C)]
#[derive(Clone)]
pub enum StructureEnum {
    Var1,
    Var2,
    Var3
}

#[repr(C)]
#[derive(Clone)]
pub struct Structure {
    // Native types
    pub booleanValue: bool,
    pub uint8Value: u8,
    pub int8Value: i8,
    pub uint16Value: u16,
    pub int16Value: i16,
    pub uint32Value: u32,
    pub int32Value: i32,
    pub uint64Value: u64,
    pub int64Value: i64,

    // Complex types
    pub structureValue: OtherStructure,
    pub enumValue: StructureEnum,

    // Duration types
    pub durationMillis: u64,
    pub durationSeconds: u64,
    pub durationSecondsFloat: f32,
}

#[no_mangle]
pub unsafe extern "C" fn struct_by_value_echo(value: Structure) -> Structure {
    value
}

#[no_mangle]
pub unsafe extern "C" fn struct_by_reference_echo(value: *const Structure) -> Structure {
    value.as_ref().unwrap().clone()
}
