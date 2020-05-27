use crate::association::Association;
use crate::ffi;
use dnp3::app::header::{ResponseFunction, ResponseHeader};
use dnp3::master::association::Configuration;
use dnp3::master::handle::{AssociationHandler, MasterHandle, ReadHandler};
use dnp3::master::request::{EventClasses, TimeSyncProcedure};

pub struct Master {
    pub runtime: *mut tokio::runtime::Runtime,
    pub handle: MasterHandle,
}

pub unsafe fn master_destroy(master: *mut Master) {
    if !master.is_null() {
        Box::from_raw(master);
    }
}

pub unsafe fn master_add_association(master: *mut Master, address: u16, config: ffi::AssociationConfiguration, handlers: ffi::AssociationHandlers) -> *mut Association {
    let master = master.as_mut().unwrap();

    let config = Configuration::new(
        convert_event_classes(&config.disable_unsol_classes),
        convert_event_classes(&config.enable_unsol_classes),
        convert_auto_time_sync(&config.auto_time_sync),
    );

    let handler = AssociationHandlerAdapter {
        integrity_handler: ReadHandlerAdapter::new(handlers.integrity_handler),
        unsolicited_handler: ReadHandlerAdapter::new(handlers.unsolicited_handler),
        default_poll_handler: ReadHandlerAdapter::new(handlers.default_poll_handler),
    };

    let runtime = master.runtime.as_mut().unwrap();
    let handle = runtime.block_on(master.handle.add_association(address, config, Box::new(handler))).unwrap();
    let association = Association {
        runtime: master.runtime,
        handle
    };
    Box::into_raw(Box::new(association))
}

fn convert_event_classes(config: &ffi::EventClasses) -> EventClasses {
    EventClasses::new(
        config.class1,
        config.class2,
        config.class3,
    )
}

fn convert_auto_time_sync(config: &ffi::AutoTimeSync) -> Option<TimeSyncProcedure> {
    match config {
        ffi::AutoTimeSync::None => None,
        ffi::AutoTimeSync::LAN => Some(TimeSyncProcedure::LAN),
        ffi::AutoTimeSync::NonLAN => Some(TimeSyncProcedure::NonLAN),
    }
}

unsafe impl Send for ffi::ReadHandler {}
unsafe impl Sync for ffi::ReadHandler {}

struct ReadHandlerAdapter {
    native_cb: ffi::ReadHandler,
}

impl ReadHandlerAdapter {
    fn new(native_cb: ffi::ReadHandler) -> Self {
        Self { native_cb }
    }
}

impl ReadHandler for ReadHandlerAdapter {
    fn begin_fragment(&mut self, header: ResponseHeader) {
        if let Some(cb) = self.native_cb.begin_fragment {
            let header = header.into();
            (cb)(header, self.native_cb.arg);
        }
    }

    fn end_fragment(&mut self, header: ResponseHeader) {
        if let Some(cb) = self.native_cb.end_fragment {
            let header = header.into();
            (cb)(header, self.native_cb.arg);
        }
    }

    fn handle_binary(&mut self, _info: dnp3::master::handle::HeaderInfo, _iter: &mut dyn Iterator<Item = (dnp3::app::measurement::Binary, u16)>) {
        // TODO: implement this
    }

    fn handle_double_bit_binary(
        &mut self,
        _info: dnp3::master::handle::HeaderInfo,
        _iter: &mut dyn Iterator<Item = (dnp3::app::measurement::DoubleBitBinary, u16)>,
    ) {
        // TODO: implement this
    }

    fn handle_binary_output_status(
        &mut self,
        _info: dnp3::master::handle::HeaderInfo,
        _iter: &mut dyn Iterator<Item = (dnp3::app::measurement::BinaryOutputStatus, u16)>,
    ) {
        // TODO: implement this
    }

    fn handle_counter(&mut self, _info: dnp3::master::handle::HeaderInfo, _iter: &mut dyn Iterator<Item = (dnp3::app::measurement::Counter, u16)>) {
        // TODO: implement this
    }

    fn handle_frozen_counter(
        &mut self,
        _info: dnp3::master::handle::HeaderInfo,
        _iter: &mut dyn Iterator<Item = (dnp3::app::measurement::FrozenCounter, u16)>,
    ) {
        // TODO: implement this
    }

    fn handle_analog(&mut self, _info: dnp3::master::handle::HeaderInfo, _iter: &mut dyn Iterator<Item = (dnp3::app::measurement::Analog, u16)>) {
        // TODO: implement this
    }

    fn handle_analog_output_status(
        &mut self,
        _info: dnp3::master::handle::HeaderInfo,
        _iter: &mut dyn Iterator<Item = (dnp3::app::measurement::AnalogOutputStatus, u16)>,
    ) {
        // TODO: implement this
    }

    fn handle_octet_string<'a>(
        &mut self,
        _info: dnp3::master::handle::HeaderInfo,
        _iter: &mut dyn Iterator<Item = (dnp3::app::parse::bytes::Bytes<'a>, u16)>,
    ) {
        // TODO: implement this
    }

}

struct AssociationHandlerAdapter {
    integrity_handler: ReadHandlerAdapter,
    unsolicited_handler: ReadHandlerAdapter,
    default_poll_handler: ReadHandlerAdapter,
}

impl AssociationHandler for AssociationHandlerAdapter {
    fn get_integrity_handler(&mut self) -> &mut dyn ReadHandler {
        &mut self.integrity_handler
    }

    fn get_unsolicited_handler(&mut self) -> &mut dyn ReadHandler {
        &mut self.unsolicited_handler
    }

    fn get_default_poll_handler(&mut self) -> &mut dyn ReadHandler {
        &mut self.default_poll_handler
    }
}

impl From<ResponseHeader> for ffi::ResponseHeader {
    fn from(header: ResponseHeader) -> ffi::ResponseHeader {
        ffi::ResponseHeader {
            control: ffi::Control {
                fir: header.control.fir,
                fin: header.control.fin,
                con: header.control.con,
                uns: header.control.uns,
                seq: header.control.seq.value(),
            },
            func: match header.function {
                ResponseFunction::Response => ffi::ResponseFunction::Response,
                ResponseFunction::UnsolicitedResponse => ffi::ResponseFunction::UnsolicitedResponse,
            },
            iin: ffi::IIN {
                iin1: ffi::IIN1 { value: header.iin.iin1.value },
                iin2: ffi::IIN2 { value: header.iin.iin2.value },
            },
        }
    }
}
