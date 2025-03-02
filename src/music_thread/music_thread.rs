use crate::music_thread::music_thread_comm::{MusicThreadCommReceiver, MusicThreadRequest};
use crate::players::play_queue::play_song_example_with_updates;
use crate::server::main_server_comm::MainThreadCommSender;
use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub enum WSMusicThreadResponse {
    SongStarted,
    SongPlayingUpdate,
    SongEnded,
}

pub fn main_music_thread(
    music_thread_comm_receiver: MusicThreadCommReceiver,
    main_thread_comm_sender: MainThreadCommSender,
) -> JoinHandle<()> {
    thread::spawn(move || {
        music_thread_logics(music_thread_comm_receiver, main_thread_comm_sender);
    })
}

enum PlayQueueRequest {
    RequestToPlay,
}
fn music_thread_logics(
    music_thread_comm_receiver: MusicThreadCommReceiver,
    main_thread_comm_sender: MainThreadCommSender,
) {
    let (tx, rx) = mpsc::channel::<PlayQueueRequest>();

    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));

    let play_queue_thread = thread::spawn(move || {
        for request in rx {
            match request {
                PlayQueueRequest::RequestToPlay => {
                    let main_thread_comm_sender_clone = main_thread_comm_sender.clone();
                    let mut atomic_bool = is_playing.lock().unwrap();
                    let mut value = atomic_bool.get_mut();
                    if *value {
                        println!("*** cannot play, occupied");
                    } else {
                        *value = true;

                        println!("*** can play, now starts");
                        play_song_example_with_updates(main_thread_comm_sender_clone);

                        *value = false;
                    }
                }
            }
        }
    });

    for music_thread_request in music_thread_comm_receiver.loop_requests() {
        match music_thread_request {
            MusicThreadRequest::PlaySong() => {
                tx.send(PlayQueueRequest::RequestToPlay).unwrap();
            }
        }
    }

    // play_queue_thread.abort();
}
