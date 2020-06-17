use dnp3::master::error::TimeSyncError;
use dnp3::master::handle::AssociationHandle;
use dnp3::master::request::TimeSyncProcedure;
use crate::ffi;

pub struct Association {
    pub runtime: tokio::runtime::Handle,
    pub handle: AssociationHandle,
}

pub unsafe fn association_destroy(association: *mut Association) {
    if !association.is_null() {
        let association = Box::from_raw(association);
        if tokio::runtime::Handle::try_current().is_err() {
            association.runtime.block_on(association.handle.remove());
        } else {
            log::warn!("Tried calling 'association_destroy' from within a tokio thread");
        }
    }
}

unsafe impl Send for ffi::TimeSyncTaskCallback {}
unsafe impl Sync for ffi::TimeSyncTaskCallback {}

pub unsafe fn association_perform_time_sync(association: *mut Association, mode: ffi::TimeSyncMode, callback: ffi::TimeSyncTaskCallback) {
    if let Some(association) = association.as_mut() {
        if let Some(cb) = callback.on_complete {
            let mode = match mode {
                ffi::TimeSyncMode::LAN => TimeSyncProcedure::LAN,
                ffi::TimeSyncMode::NonLAN => TimeSyncProcedure::NonLAN,
            };

            let handle = &mut association.handle;
            association.runtime.spawn(async move {
                let result = match handle.perform_time_sync(mode).await {
                    Ok(_) => ffi::TimeSyncResult::Success,
                    Err(TimeSyncError::Task(_)) => ffi::TimeSyncResult::TaskError,
                    Err(TimeSyncError::ClockRollback) => ffi::TimeSyncResult::ClockRollback,
                    Err(TimeSyncError::SystemTimeNotUnix) => ffi::TimeSyncResult::SystemTimeNotUnix,
                    Err(TimeSyncError::BadOutstationTimeDelay(_)) => ffi::TimeSyncResult::BadOutstationTimeDelay,
                    Err(TimeSyncError::Overflow) => ffi::TimeSyncResult::Overflow,
                    Err(TimeSyncError::StillNeedsTime) => ffi::TimeSyncResult::StillNeedsTime,
                    Err(TimeSyncError::IINError(_)) => ffi::TimeSyncResult::IINError,
                };
                cb(result, callback.arg);
            });
        }
    }
}
