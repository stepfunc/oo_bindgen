pub enum EnumZeroToFive {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

pub enum EnumOneToSix {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub enum EnumDisjoint {
    Five = 5,
    One = 1,
    Twenty = 20,
    Four = 4,
    Seven = 7,
    Two = 2,
}

#[no_mangle]
pub unsafe extern "C" fn enum_zero_to_five_echo(value: EnumZeroToFive) -> EnumZeroToFive {
    value
}

#[no_mangle]
pub unsafe extern "C" fn enum_one_to_six_echo(value: EnumOneToSix) -> EnumOneToSix {
    value
}

#[no_mangle]
pub unsafe extern "C" fn enum_disjoint_echo(value: EnumDisjoint) -> EnumDisjoint {
    value
}
