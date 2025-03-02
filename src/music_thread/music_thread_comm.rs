use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommReceiverTrait,
    ThreadCommSenderTrait,
};
use std::sync::mpsc::{Receiver, Sender};

pub enum MusicThreadRequest {
    // TODO: Add Song ID here
    PlaySong(),
}
pub fn create_music_thread_comm_instances() -> (MusicThreadCommSender, MusicThreadCommReceiver) {
    let (sender, receiver) =
        create_thread_comm_instances::<MusicThreadRequest, MusicThreadCommSender>();
    let music_thread_comm_receiver = MusicThreadCommReceiver::new(receiver);
    (sender, music_thread_comm_receiver)
}

pub struct MusicThreadCommSender {
    tx: Sender<MusicThreadRequest>,
}
impl ThreadCommSenderTrait<MusicThreadRequest> for MusicThreadCommSender {
    fn new(tx: Sender<MusicThreadRequest>) -> Self {
        Self { tx }
    }
    fn get_tx(&self) -> &Sender<MusicThreadRequest> {
        &self.tx
    }
}
impl MusicThreadCommSender {
    pub fn request_play_song(&self) {
        let music_thread_request = MusicThreadRequest::PlaySong();
        self.tx.send(music_thread_request).unwrap();
    }
}

pub struct MusicThreadCommReceiver {
    receiver: ThreadCommReceiver<MusicThreadRequest>,
}
impl MusicThreadCommReceiver {
    pub fn new(receiver: ThreadCommReceiver<MusicThreadRequest>) -> Self {
        Self { receiver }
    }
    pub fn loop_requests(self) -> Receiver<MusicThreadRequest> {
        self.receiver.get_recv_iter()
    }
}
