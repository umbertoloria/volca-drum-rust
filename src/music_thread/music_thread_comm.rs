use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommReceiverTrait, ThreadCommSender,
    ThreadCommSenderTrait, ThreadCommSenderWrapperTrait,
};
use std::sync::mpsc::Receiver;

pub enum MusicThreadRequest {
    // TODO: Add Song ID here
    PlaySong(),
}
pub fn create_music_thread_comm_instances() -> (MusicThreadCommSender, MusicThreadCommReceiver) {
    let (sender, receiver) = create_thread_comm_instances::<MusicThreadRequest>();
    let music_thread_comm_sender = MusicThreadCommSender::new(sender);
    let music_thread_comm_receiver = MusicThreadCommReceiver::new(receiver);
    (music_thread_comm_sender, music_thread_comm_receiver)
}

pub struct MusicThreadCommSender {
    sender: ThreadCommSender<MusicThreadRequest>,
}
impl ThreadCommSenderWrapperTrait<MusicThreadRequest> for MusicThreadCommSender {
    fn new(sender: ThreadCommSender<MusicThreadRequest>) -> Self {
        Self { sender }
    }
    fn get_sender(&self) -> &ThreadCommSender<MusicThreadRequest> {
        &self.sender
    }
}
impl MusicThreadCommSender {
    pub fn request_play_song(&self) {
        let music_thread_request = MusicThreadRequest::PlaySong();
        self.sender.send(music_thread_request);
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
