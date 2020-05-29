use crate::ffi;
use crate::association::Association;
use crate::handler::ReadHandlerAdapter;

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
