use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn create_thread_comm_instances<T>() -> (ThreadCommSender<T>, ThreadCommReceiver<T>) {
    let (tx, rx) = mpsc::channel::<T>();
    let thread_comm_sender = ThreadCommSenderTrait::new(tx);
    let thread_comm_receiver = ThreadCommReceiverTrait::new(rx);
    (thread_comm_sender, thread_comm_receiver)
}

// SENDER
pub trait ThreadCommSenderWrapperTrait<T>: Sized {
    fn new(sender: ThreadCommSender<T>) -> Self;
    fn get_sender(&self) -> &ThreadCommSender<T>;
    fn clone(&self) -> Self {
        Self::new(self.get_sender().clone())
    }
}
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
pub struct ThreadCommSender<T> {
    tx: Sender<T>,
}
impl<T> ThreadCommSenderTrait<T> for ThreadCommSender<T> {
    fn new(tx: Sender<T>) -> Self {
        Self { tx }
    }
    fn get_tx(&self) -> &Sender<T> {
        &self.tx
    }
}

// RECEIVER
pub trait ThreadCommReceiverTrait<T> {
    fn new(rx: Receiver<T>) -> Self;
    fn get_recv_iter(self) -> Receiver<T>;
}
pub struct ThreadCommReceiver<T> {
    rx: Receiver<T>,
}
impl<T> ThreadCommReceiverTrait<T> for ThreadCommReceiver<T> {
    fn new(rx: Receiver<T>) -> Self {
        Self { rx }
    }
    fn get_recv_iter(self) -> Receiver<T> {
        self.rx
    }
}
