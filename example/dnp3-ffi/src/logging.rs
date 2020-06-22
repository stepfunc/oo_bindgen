use crate::ffi;
use dnp3::prelude::master::*;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::ffi::CString;

pub fn logging_set_callback(handler: ffi::Logger) {
    log::set_boxed_logger(Box::new(LoggerAdapter { handler })).unwrap();
}

pub fn logging_set_log_level(level: ffi::LogLevel) {
    let level = match level {
        ffi::LogLevel::Error => LevelFilter::Error,
        ffi::LogLevel::Warn => LevelFilter::Warn,
        ffi::LogLevel::Info => LevelFilter::Info,
        ffi::LogLevel::Debug => LevelFilter::Debug,
        ffi::LogLevel::Trace => LevelFilter::Trace,
    };

    log::set_max_level(level);
}

unsafe impl Send for ffi::Logger {}
unsafe impl Sync for ffi::Logger {}

struct LoggerAdapter {
    handler: ffi::Logger,
}

impl Log for LoggerAdapter {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if let Some(cb) = self.handler.on_message {
            if let Ok(message) = CString::new(format!("{}", record.args())) {
                let level = match record.level() {
                    Level::Error => ffi::LogLevel::Error,
                    Level::Warn => ffi::LogLevel::Warn,
                    Level::Info => ffi::LogLevel::Info,
                    Level::Debug => ffi::LogLevel::Debug,
                    Level::Trace => ffi::LogLevel::Trace,
                };

                (cb)(level, message.as_ptr(), self.handler.arg);
            }
        }
    }

    fn flush(&self) {}
}

impl Drop for LoggerAdapter {
    fn drop(&mut self) {
        if let Some(cb) = self.handler.on_destroy {
            (cb)(self.handler.arg)
        }
    }
}

impl From<ffi::DecodeLogLevel> for DecodeLogLevel {
    fn from(from: ffi::DecodeLogLevel) -> Self {
        match from {
            ffi::DecodeLogLevel::Nothing => DecodeLogLevel::Nothing,
            ffi::DecodeLogLevel::Header => DecodeLogLevel::Header,
            ffi::DecodeLogLevel::ObjectHeaders => DecodeLogLevel::ObjectHeaders,
            ffi::DecodeLogLevel::ObjectValues => DecodeLogLevel::ObjectValues,
        }
    }
}
