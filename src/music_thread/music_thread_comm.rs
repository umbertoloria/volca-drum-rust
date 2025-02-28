use crate::music_thread::music_thread::{MusicThreadRequest, MusicThreadRequestsTx};

pub struct MusicThreadCommSender {
    tx: MusicThreadRequestsTx,
}
impl MusicThreadCommSender {
    pub fn new(tx: MusicThreadRequestsTx) -> Self {
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
