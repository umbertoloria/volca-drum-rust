use crate::music_thread::music_thread_comm::MusicThreadCommSender;
use crate::players::play_queue::play_song_example_with_updates;
use crate::server::main_server::BroadcastSenderToServerThread;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
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

enum PlayQueueRequest {
    RequestToPlay,
}
fn music_thread_logics(
    music_thread_requests_rx: MusicThreadRequestsRx,
    tx_to_web_server: BroadcastSenderToServerThread,
) {
    let (tx, rx) = mpsc::channel::<PlayQueueRequest>();

    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));

    let tx_to_web_server_clone = tx_to_web_server.clone();
    let play_queue_thread = thread::spawn(move || {
        for request in rx {
            match request {
                PlayQueueRequest::RequestToPlay => {
                    let tx_to_web_server_clone = tx_to_web_server_clone.clone();
                    let mut atomic_bool = is_playing.lock().unwrap();
                    let mut value = atomic_bool.get_mut();
                    if *value {
                        println!("*** cannot play, occupied");
                    } else {
                        *value = true;

                        println!("*** can play, now starts");
                        play_song_example_with_updates(tx_to_web_server_clone);

                        *value = false;
                    }
                }
            }
        }
    });

    for music_thread_request in music_thread_requests_rx {
        match music_thread_request {
            MusicThreadRequest::PlaySong() => {
                tx.send(PlayQueueRequest::RequestToPlay).unwrap();
            }
        }
    }

    // play_queue_thread.abort();
}
