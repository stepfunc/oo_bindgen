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
    pub boolean_value: bool,
    pub uint8_value: u8,
    pub int8_value: i8,
    pub uint16_value: u16,
    pub int16_value: i16,
    pub uint32_value: u32,
    pub int32_value: i32,
    pub uint64_value: u64,
    pub int64_value: i64,

    // Complex types
    pub structure_value: OtherStructure,
    pub enum_value: StructureEnum,

    // Duration types
    pub duration_millis: u64,
    pub duration_seconds: u64,
    pub duration_seconds_float: f32,
}

#[no_mangle]
pub unsafe extern "C" fn struct_by_value_echo(value: Structure) -> Structure {
    value
}

#[no_mangle]
pub unsafe extern "C" fn struct_by_reference_echo(value: *const Structure) -> Structure {
    value.as_ref().unwrap().clone()
}
