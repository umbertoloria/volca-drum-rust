use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn create_thread_comm_instances<RequestType>() -> (
    ThreadCommSender<RequestType>,
    ThreadCommReceiver<RequestType>,
) {
    let (tx, rx) = mpsc::channel::<RequestType>();
    let sender = ThreadCommSender::new(tx);
    let receiver = ThreadCommReceiver::new(rx);
    (sender, receiver)
}

// SENDER
/*
pub trait ThreadCommSenderTrait<T>: Sized {
    fn new(tx: Sender<T>) -> Self;
    fn get_tx(&self) -> &Sender<T>;
    fn clone(&self) -> Self {
        Self::new(self.get_tx().clone())
    }
    fn send(&self, message: T) {
        self.get_tx().send(message).unwrap();
    }
}
*/
pub struct ThreadCommSender<RequestType> {
    tx: Sender<RequestType>,
}
impl<RequestType> ThreadCommSender<RequestType> {
    fn new(tx: Sender<RequestType>) -> Self {
        Self { tx }
    }
    pub fn clone(&self) -> ThreadCommSender<RequestType> {
        Self::new(self.tx.clone())
    }
    pub fn send(&self, payload: RequestType) {
        let _ = &self.tx.send(payload).unwrap();
    }
}

// RECEIVER
/*
pub trait ThreadCommReceiverTrait<T> {
    fn new(rx: Receiver<T>) -> Self;
    fn get_recv_iter(self) -> Receiver<T>;
}
*/
pub struct ThreadCommReceiver<T> {
    rx: Receiver<T>,
}
impl<T> ThreadCommReceiver<T> {
    fn new(rx: Receiver<T>) -> Self {
        Self { rx }
    }
    pub fn get_recv_iter(self) -> Receiver<T> {
        self.rx
    }
}
