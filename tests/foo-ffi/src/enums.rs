use crate::ffi::{EnumDisjoint, EnumOneToSix, EnumSingle, EnumZeroToFive};

pub fn enum_zero_to_five_echo(value: EnumZeroToFive) -> EnumZeroToFive {
    value
}

pub fn enum_one_to_six_echo(value: EnumOneToSix) -> EnumOneToSix {
    value
}

pub fn enum_disjoint_echo(value: EnumDisjoint) -> EnumDisjoint {
    value
}

pub fn enum_single_echo(value: EnumSingle) -> EnumSingle {
    value
}
