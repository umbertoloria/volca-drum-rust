use crate::players::player::play_song_example;
use crate::server::web_thread_comm::WebThreadCommSender;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub enum PlayQueueCommand {
    RequestToPlay,
    CloseThread,
}
type PlayQueueRequestReceiver = ThreadCommReceiver<PlayQueueCommand>;
pub fn create_play_queue_comm() -> (
    ThreadCommSender<PlayQueueCommand>,
    ThreadCommReceiver<PlayQueueCommand>,
) {
    let (play_queue_comm_sender, play_queue_comm_receiver) =
        create_thread_comm_instances::<PlayQueueCommand>();
    (play_queue_comm_sender, play_queue_comm_receiver)
}

pub fn play_queue_thread(
    play_queue_comm_receiver: PlayQueueRequestReceiver,
    web_thread_comm_sender: WebThreadCommSender,
) -> JoinHandle<()> {
    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));
    thread::spawn(move || {
        for command in play_queue_comm_receiver.get_recv_iter() {
            match command {
                PlayQueueCommand::RequestToPlay => {
                    let web_thread_comm_sender_clone = web_thread_comm_sender.clone();

                    // Atomic Read
                    let mut atomic_bool = is_playing.lock().unwrap();
                    let mut value = atomic_bool.get_mut();
                    if *value {
                        println!("*** cannot play, occupied");
                    } else {
                        *value = true;

                        println!("*** can play, now starts");
                        play_song_example_with_updates(web_thread_comm_sender_clone);

                        *value = false;
                    }
                }
                PlayQueueCommand::CloseThread => {
                    break;
                }
            }
        }
    })
}

fn play_song_example_with_updates(web_thread_comm_sender: WebThreadCommSender) {
    web_thread_comm_sender.notify_from_music_thread_song_started();

    play_song_example();

    web_thread_comm_sender.notify_from_music_thread_song_ended();
}
