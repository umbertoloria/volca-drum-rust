use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};

pub enum MusicThreadRequest {
    // TODO: Add Song ID here
    PlaySong(),
}

pub fn create_music_thread_comm_instances() -> (MusicThreadCommSender, MusicThreadCommReceiver) {
    let (sender, receiver) = create_thread_comm_instances::<MusicThreadRequest>();
    (sender, receiver)
}

pub type MusicThreadCommSender = ThreadCommSender<MusicThreadRequest>;
pub type MusicThreadCommReceiver = ThreadCommReceiver<MusicThreadRequest>;
