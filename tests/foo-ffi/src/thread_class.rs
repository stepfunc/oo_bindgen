use std::thread::JoinHandle;

enum Message {
    Update(u32),
    Add(u32, crate::ffi::AddHandler),
    Stop,
}

struct ThreadData {
    value: u32,
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
            Message::Add(x, cb) => {
                data.value += x;
                data.receiver.on_value_change(data.value);
                cb.on_complete(data.value);
            }
            Message::Stop => return,
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
        receiver,
        rx,
    };
    let join_handle = Some(std::thread::spawn(|| run(thread_data)));
    let class = Box::new(ThreadClass { tx, join_handle });
    Box::into_raw(class)
}

pub(crate) unsafe fn thread_class_destroy(instance: *mut ThreadClass) {
    if !instance.is_null() {
        Box::from_raw(instance);
    }
}

pub(crate) unsafe fn thread_class_update(instance: *mut ThreadClass, value: u32) {
    if let Some(x) = instance.as_ref() {
        x.tx.send(Message::Update(value)).unwrap()
    }
}

pub(crate) unsafe fn thread_class_add(
    instance: *mut ThreadClass,
    value: u32,
    callback: crate::ffi::AddHandler,
) {
    if let Some(x) = instance.as_ref() {
        x.tx.send(Message::Add(value, callback)).unwrap()
    }
}
