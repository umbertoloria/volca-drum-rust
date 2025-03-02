use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub enum MusicThreadRequest {
    // TODO: Add Song ID here
    PlaySong(),
}
pub fn create_music_thread_comm_instances() -> (MusicThreadCommSender, MusicThreadCommReceiver) {
    let (tx, rx) = mpsc::channel::<MusicThreadRequest>();
    let music_thread_comm_sender = MusicThreadCommSender::new(tx);
    let music_thread_comm_receiver = MusicThreadCommReceiver::new(rx);
    (music_thread_comm_sender, music_thread_comm_receiver)
}

// SENDER
pub struct MusicThreadCommSender {
    tx: Sender<MusicThreadRequest>,
}
impl MusicThreadCommSender {
    pub fn new(tx: Sender<MusicThreadRequest>) -> Self {
        Self { tx }
    }
    pub fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
    pub fn request_play_song(&self) {
        let music_thread_request = MusicThreadRequest::PlaySong();
        self.tx.send(music_thread_request).unwrap();
    }
}

// RECEIVER
pub struct MusicThreadCommReceiver {
    rx: Receiver<MusicThreadRequest>,
}
impl MusicThreadCommReceiver {
    pub fn new(rx: Receiver<MusicThreadRequest>) -> Self {
        Self { rx }
    }
    pub fn loop_requests(self) -> Receiver<MusicThreadRequest> {
        self.rx
    }
}
