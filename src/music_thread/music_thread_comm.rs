use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSenderTrait,
};
use std::sync::mpsc::Sender;

pub enum MusicThreadRequest {
    // TODO: Add Song ID here
    PlaySong(),
}
pub fn create_music_thread_comm_instances() -> (MusicThreadCommSender, MusicThreadCommReceiver) {
    let (sender, receiver) =
        create_thread_comm_instances::<MusicThreadRequest, MusicThreadCommSender>();
    (sender, receiver)
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

pub type MusicThreadCommReceiver = ThreadCommReceiver<MusicThreadRequest>;
