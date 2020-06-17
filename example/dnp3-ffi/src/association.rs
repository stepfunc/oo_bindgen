use dnp3::master::error::{CommandError, CommandResponseError, TimeSyncError};
use dnp3::master::handle::AssociationHandle;
use dnp3::master::request::{CommandMode, TimeSyncProcedure};
use crate::ffi;
use crate::command::Command;

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

unsafe impl Send for ffi::CommandTaskCallback {}
unsafe impl Sync for ffi::CommandTaskCallback {}

pub unsafe fn association_operate(association: *mut Association, mode: ffi::CommandMode, command: *const Command, callback: ffi::CommandTaskCallback) {
    if let Some(association) = association.as_mut() {
        if let Some(command) = command.as_ref() {
            if let Some(cb) = callback.on_complete {
                let mode = match mode {
                    ffi::CommandMode::DirectOperate => CommandMode::DirectOperate,
                    ffi::CommandMode::SelectBeforeOperate => CommandMode::SelectBeforeOperate,
                };

                let handle = &mut association.handle;
                let cmd = command.clone();
                association.runtime.spawn(async move {
                    let result = match handle.operate(mode, cmd.build()).await {
                        Ok(_) => ffi::CommandResult::Success,
                        Err(CommandError::Task(_)) => ffi::CommandResult::TaskError,
                        Err(CommandError::Response(err)) => match err {
                            CommandResponseError::Request(_) => ffi::CommandResult::TaskError,
                            CommandResponseError::BadStatus(_) => ffi::CommandResult::BadStatus,
                            CommandResponseError::HeaderCountMismatch => ffi::CommandResult::HeaderCountMismatch,
                            CommandResponseError::HeaderTypeMismatch => ffi::CommandResult::HeaderTypeMismatch,
                            CommandResponseError::ObjectCountMismatch => ffi::CommandResult::ObjectCountMismatch,
                            CommandResponseError::ObjectValueMismatch => ffi::CommandResult::ObjectValueMismatch,
                        },
                    };
                    cb(result, callback.arg);
                });
            }
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
