use std::time::Duration;

pub fn duration_ms_echo(value: u64) -> u64 {
    let duration = Duration::from_millis(value);
    duration.as_millis() as u64
}

pub fn duration_s_echo(value: u64) -> u64 {
    let duration = Duration::from_secs(value);
    duration.as_secs()
}

pub fn duration_s_float_echo(value: f32) -> f32 {
    let duration = Duration::from_secs_f32(value);
    duration.as_secs_f32()
}
