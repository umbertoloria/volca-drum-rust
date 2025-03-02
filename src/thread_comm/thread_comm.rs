use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn create_thread_comm_instances<RequestType, Sender: ThreadCommSenderTrait<RequestType>>(
) -> (Sender, ThreadCommReceiver<RequestType>) {
    let (tx, rx) = mpsc::channel::<RequestType>();
    let sender = Sender::new(tx);
    let receiver = ThreadCommReceiver::new(rx);
    (sender, receiver)
}

// SENDER
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
