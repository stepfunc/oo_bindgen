use tokio;
use std::ptr::null_mut;

#[repr(C)]
pub struct RuntimeConfig {
    num_core_threads: u16,
}

fn build_runtime<F>(f: F) -> std::result::Result<tokio::runtime::Runtime, std::io::Error>
where
    F: Fn(&mut tokio::runtime::Builder) -> &mut tokio::runtime::Builder,
{
    f(tokio::runtime::Builder::new().enable_all().threaded_scheduler()).build()
}

#[no_mangle]
pub unsafe extern "C" fn runtime_new(
    config: *const RuntimeConfig,
) -> *mut tokio::runtime::Runtime {
    let result = match config.as_ref() {
        None => build_runtime(|r| r),
        Some(x) => build_runtime(|r| r.core_threads(x.num_core_threads as usize)),
    };

    match result {
        Ok(r) => Box::into_raw(Box::new(r)),
        Err(_) => {
            //log::error!("Unable to build runtime: {}", err);
            null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn runtime_destroy(runtime: *mut tokio::runtime::Runtime) {
    if !runtime.is_null() {
        Box::from_raw(runtime);
    };
}
