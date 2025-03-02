use crate::music_thread::music_thread_comm::{MusicThreadCommReceiver, MusicThreadRequest};
use crate::players::play_queue::play_song_example_with_updates;
use crate::server::main_server_comm::MainThreadCommSender;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

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
    CloseThread,
}
fn music_thread_logics(
    music_thread_comm_receiver: MusicThreadCommReceiver,
    main_thread_comm_sender: MainThreadCommSender,
) {
    // Play Queue Thread
    let (tx, rx) = mpsc::channel::<PlayQueueRequest>();
    let play_queue_thread = play_queue_thread(rx, main_thread_comm_sender);

    for music_thread_request in music_thread_comm_receiver.get_recv_iter() {
        match music_thread_request {
            MusicThreadRequest::PlaySong() => {
                tx.send(PlayQueueRequest::RequestToPlay).unwrap();
            }
        }
    }

    tx.send(PlayQueueRequest::CloseThread).unwrap();
    play_queue_thread.join().unwrap();
}

fn play_queue_thread(
    rx: Receiver<PlayQueueRequest>,
    main_thread_comm_sender: MainThreadCommSender,
) -> JoinHandle<()> {
    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));
    thread::spawn(move || {
        for request in rx {
            match request {
                PlayQueueRequest::RequestToPlay => {
                    let main_thread_comm_sender_clone = main_thread_comm_sender.clone();

                    // Atomic Read
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
                PlayQueueRequest::CloseThread => {
                    break;
                }
            }
        }
    })
}
