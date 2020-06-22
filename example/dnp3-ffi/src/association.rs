use crate::command::Command;
use crate::request::Request;
use crate::ffi;
use dnp3::master::association::PollHandle;
use dnp3::master::error::{CommandError, CommandResponseError, TimeSyncError};
use dnp3::master::handle::AssociationHandle;
use dnp3::master::request::{CommandMode, TimeSyncProcedure};
use std::time::Duration;

pub struct Association {
    pub(crate) runtime: tokio::runtime::Handle,
    pub(crate) handle: AssociationHandle,
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

pub struct Poll {
    runtime: tokio::runtime::Handle,
    handle: PollHandle,
}

impl Poll {
    fn new(runtime: tokio::runtime::Handle, handle: PollHandle) -> Self {
        Self {
            runtime,
            handle,
        }
    }
}

pub unsafe fn poll_demand(poll: *mut Poll) {
    if let Some(poll) = poll.as_mut() {
        if tokio::runtime::Handle::try_current().is_err() {
            poll.runtime.block_on(poll.handle.demand());
        } else {
            log::warn!("Tried calling 'poll_demand' from within a tokio thread");
        }
    }
}

pub unsafe fn poll_destroy(poll: *mut Poll) {
    if !poll.is_null() {
        let poll = Box::from_raw(poll);
        if tokio::runtime::Handle::try_current().is_err() {
            poll.runtime.block_on(poll.handle.remove());
        } else {
            log::warn!("Tried calling 'poll_destroy' from within a tokio thread");
        }
    }
}

pub unsafe fn association_add_poll(association: *mut Association, request: *const Request, period: u64) -> *mut Poll {
    let association = match association.as_mut() {
        Some(association) => association,
        None => return std::ptr::null_mut(),
    };

    let request = match request.as_ref() {
        Some(request) => request,
        None => return std::ptr::null_mut(),
    };

    let period = Duration::from_millis(period);

    if tokio::runtime::Handle::try_current().is_err() {
        if let Ok(handle) = association.runtime.block_on(association.handle.add_poll(request.build(), period)) {
            let poll = Box::new(Poll::new(association.runtime.clone(), handle));
            Box::into_raw(poll)
        } else {
            log::warn!("Poll creation failure");
            std::ptr::null_mut()
        }
    } else {
        log::warn!("Tried calling 'association_add_poll' from within a tokio thread");
        std::ptr::null_mut()
    }
}

unsafe impl Send for ffi::ReadTaskCallback {}
unsafe impl Sync for ffi::ReadTaskCallback {}

pub unsafe fn association_read(
    association: *mut Association,
    request: *const Request,
    callback: ffi::ReadTaskCallback,
) {
    let association = match association.as_mut() {
        Some(association) => association,
        None => {
            if let Some(cb) = callback.on_complete {
                cb(ffi::ReadResult::TaskError, callback.arg);
            }
            return;
        }
    };

    let request = match request.as_ref() {
        Some(request) => request,
        None => {
            if let Some(cb) = callback.on_complete {
                cb(ffi::ReadResult::TaskError, callback.arg);
            }
            return;
        }
    };

    let handle = &mut association.handle;
    let req = request.build();
    association.runtime.spawn(async move {
        let result = match handle.read(req).await {
            Ok(_) => ffi::ReadResult::Success,
            Err(_) => ffi::ReadResult::TaskError,
        };

        if let Some(cb) = callback.on_complete {
            cb(result, callback.arg);
        }
    });
}

unsafe impl Send for ffi::CommandTaskCallback {}
unsafe impl Sync for ffi::CommandTaskCallback {}

pub unsafe fn association_operate(
    association: *mut Association,
    mode: ffi::CommandMode,
    command: *const Command,
    callback: ffi::CommandTaskCallback,
) {
    let association = match association.as_mut() {
        Some(association) => association,
        None => {
            if let Some(cb) = callback.on_complete {
                cb(ffi::CommandResult::TaskError, callback.arg);
            }
            return;
        }
    };

    let command = match command.as_ref() {
        Some(command) => command,
        None => {
            if let Some(cb) = callback.on_complete {
                cb(ffi::CommandResult::TaskError, callback.arg);
            }
            return;
        }
    };

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
                CommandResponseError::HeaderCountMismatch => {
                    ffi::CommandResult::HeaderCountMismatch
                }
                CommandResponseError::HeaderTypeMismatch => ffi::CommandResult::HeaderTypeMismatch,
                CommandResponseError::ObjectCountMismatch => {
                    ffi::CommandResult::ObjectCountMismatch
                }
                CommandResponseError::ObjectValueMismatch => {
                    ffi::CommandResult::ObjectValueMismatch
                }
            },
        };

        if let Some(cb) = callback.on_complete {
            cb(result, callback.arg);
        }
    });
}

unsafe impl Send for ffi::TimeSyncTaskCallback {}
unsafe impl Sync for ffi::TimeSyncTaskCallback {}

pub unsafe fn association_perform_time_sync(
    association: *mut Association,
    mode: ffi::TimeSyncMode,
    callback: ffi::TimeSyncTaskCallback,
) {
    let association = match association.as_mut() {
        Some(association) => association,
        None => {
            if let Some(cb) = callback.on_complete {
                cb(ffi::TimeSyncResult::TaskError, callback.arg);
            }
            return;
        }
    };

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
            Err(TimeSyncError::BadOutstationTimeDelay(_)) => {
                ffi::TimeSyncResult::BadOutstationTimeDelay
            }
            Err(TimeSyncError::Overflow) => ffi::TimeSyncResult::Overflow,
            Err(TimeSyncError::StillNeedsTime) => ffi::TimeSyncResult::StillNeedsTime,
            Err(TimeSyncError::IINError(_)) => ffi::TimeSyncResult::IINError,
        };

        if let Some(cb) = callback.on_complete {
            cb(result, callback.arg);
        }
    });
}
