use crate::music_thread::music_thread_comm::MusicThreadCommSender;
use crate::players::play_queue::play_song_in_queue;
use crate::server::main_server::BroadcastSenderToServerThread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub enum WSMusicThreadResponse {
    SongStarted,
    SongPlayingUpdate,
    SongEnded,
}

pub enum MusicThreadRequest {
    PlaySong(),
}

pub type MusicThreadRequestsTx = Sender<MusicThreadRequest>;
type MusicThreadRequestsRx = Receiver<MusicThreadRequest>;
pub fn create_channel_for_music_thread() -> (MusicThreadCommSender, MusicThreadRequestsRx) {
    let (music_thread_requests_tx, music_thread_requests_rx) =
        mpsc::channel::<MusicThreadRequest>();
    let music_thread_comm_sender = MusicThreadCommSender::new(music_thread_requests_tx);
    (music_thread_comm_sender, music_thread_requests_rx)
}

pub fn main_music_thread(
    music_thread_requests_rx: MusicThreadRequestsRx,
    tx_to_web_server: BroadcastSenderToServerThread,
) -> JoinHandle<()> {
    thread::spawn(move || {
        music_thread_logics(music_thread_requests_rx, tx_to_web_server);
    })
}

fn music_thread_logics(
    music_thread_requests_rx: MusicThreadRequestsRx,
    tx_to_web_server: BroadcastSenderToServerThread,
) {
    for music_thread_request in music_thread_requests_rx {
        // TODO: Avoid cloning every time...
        let tx_to_web_server_cloned = tx_to_web_server.clone();
        match music_thread_request {
            MusicThreadRequest::PlaySong() => {
                play_song_in_queue(tx_to_web_server_cloned);
            }
        }
    }
}
