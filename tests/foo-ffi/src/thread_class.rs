use crate::ffi::promise::{DropError, Promise};
use crate::ffi::MathIsBroken;
use std::thread::JoinHandle;

enum Message {
    Update(u32),
    Add(
        u32,
        Box<dyn Promise<Result<u32, crate::ffi::MathIsBroken>> + Send>,
    ),
    QueueAddError(crate::ffi::MathIsBroken),
    DropAdd,
    Operation(crate::ffi::Operation),
    Stop,
}

struct ThreadData {
    value: u32,
    error_queue: Vec<crate::ffi::MathIsBroken>,
    drop_add: bool,
    receiver: crate::ffi::ValueChangeListener,
    rx: std::sync::mpsc::Receiver<Message>,
}

pub struct ThreadClass {
    tx: std::sync::mpsc::Sender<Message>,
    join_handle: Option<JoinHandle<()>>,
}

impl Drop for ThreadClass {
    fn drop(&mut self) {
        let _ = self.tx.send(Message::Stop);
        if let Some(x) = self.join_handle.take() {
            let _ = x.join();
        }
    }
}

fn run(mut data: ThreadData) {
    loop {
        let x = match data.rx.recv() {
            Ok(x) => x,
            Err(_) => return,
        };

        match x {
            Message::Update(x) => {
                data.value = x;
                data.receiver.on_value_change(x);
            }
            Message::Add(x, mut reply) => {
                if data.drop_add {
                    // we don't reply at all, we just let the promise drop
                    data.drop_add = false;
                } else if let Some(err) = data.error_queue.pop() {
                    reply.complete(Err(err));
                } else {
                    data.value += x;
                    data.receiver.on_value_change(data.value);
                    reply.complete(Ok(data.value));
                }
            }
            Message::Operation(op) => {
                if let Some(x) = op.execute(data.value) {
                    data.value = x;
                    data.receiver.on_value_change(data.value);
                }
            }
            Message::Stop => return,
            Message::QueueAddError(err) => data.error_queue.push(err),
            Message::DropAdd => {
                data.drop_add = true;
            }
        }
    }
}

pub(crate) fn thread_class_create(
    value: u32,
    receiver: crate::ffi::ValueChangeListener,
) -> *mut ThreadClass {
    let (tx, rx) = std::sync::mpsc::channel();
    let thread_data = ThreadData {
        value,
        error_queue: Default::default(),
        drop_add: false,
        receiver,
        rx,
    };
    let join_handle = Some(std::thread::spawn(|| run(thread_data)));
    let class = Box::new(ThreadClass { tx, join_handle });
    Box::into_raw(class)
}

pub(crate) unsafe fn thread_class_destroy(instance: *mut ThreadClass) {
    if !instance.is_null() {
        drop(Box::from_raw(instance));
    }
}

pub(crate) unsafe fn thread_class_update(instance: *mut ThreadClass, value: u32) {
    if let Some(x) = instance.as_ref() {
        x.tx.send(Message::Update(value)).unwrap()
    }
}

impl DropError for MathIsBroken {
    const ERROR_ON_DROP: Self = Self::Dropped;
}

pub(crate) unsafe fn thread_class_add(
    instance: *mut ThreadClass,
    value: u32,
    callback: impl crate::ffi::promise::Promise<Result<u32, crate::ffi::MathIsBroken>>,
) {
    if let Some(x) = instance.as_ref() {
        x.tx.send(Message::Add(value, Box::new(callback))).unwrap()
    }
}

pub(crate) unsafe fn thread_class_execute(
    instance: *mut ThreadClass,
    operation: crate::ffi::Operation,
) {
    if let Some(x) = instance.as_ref() {
        x.tx.send(Message::Operation(operation)).unwrap()
    }
}

pub(crate) unsafe fn thread_class_queue_error(
    instance: *mut ThreadClass,
    err: crate::ffi::MathIsBroken,
) {
    if let Some(x) = instance.as_ref() {
        x.tx.send(Message::QueueAddError(err)).unwrap()
    }
}

pub(crate) unsafe fn thread_class_drop_next_add(instance: *mut crate::ThreadClass) {
    if let Some(x) = instance.as_ref() {
        x.tx.send(Message::DropAdd).unwrap()
    }
}
