use crate::music_thread::music_thread_comm::{MusicThreadCommReceiver, MusicThreadCommand};
use crate::music_thread::volca_drum_thread::{
    create_volca_drum_thread_comm, volca_drum_thread, VolcaDrumCommand,
};
use crate::players::play_queue::play_song_example_with_updates;
use crate::server::main_server_comm::MainThreadCommSender;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
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

fn music_thread_logics(
    music_thread_comm_receiver: MusicThreadCommReceiver,
    main_thread_comm_sender: MainThreadCommSender,
) {
    // Volca Drum Thread
    let (volca_drum_command_sender, volca_drum_command_receiver) = create_volca_drum_thread_comm();
    let volca_drum_thread = volca_drum_thread(volca_drum_command_receiver);

    // Play Queue Thread
    let (play_queue_comm_sender, play_queue_comm_receiver) = create_play_queue_comm();
    let play_queue_thread = play_queue_thread(play_queue_comm_receiver, main_thread_comm_sender);

    for command in music_thread_comm_receiver.get_recv_iter() {
        match command {
            MusicThreadCommand::PlaySong() => {
                play_queue_comm_sender.send(PlayQueueCommand::RequestToPlay);
            }
            MusicThreadCommand::ApplyVolcaDrumPatch(yaml_patch_file) => {
                volca_drum_command_sender.send(VolcaDrumCommand::ApplyPatch(yaml_patch_file));
            }
        }
    }

    play_queue_comm_sender.send(PlayQueueCommand::CloseThread);
    play_queue_thread.join().unwrap();

    volca_drum_command_sender.send(VolcaDrumCommand::CloseThread);
    volca_drum_thread.join().unwrap();
}

// PLAY QUEUE THREAD
enum PlayQueueCommand {
    RequestToPlay,
    CloseThread,
}
type PlayQueueRequestReceiver = ThreadCommReceiver<PlayQueueCommand>;
fn create_play_queue_comm() -> (
    ThreadCommSender<PlayQueueCommand>,
    ThreadCommReceiver<PlayQueueCommand>,
) {
    let (play_queue_comm_sender, play_queue_comm_receiver) =
        create_thread_comm_instances::<PlayQueueCommand>();
    (play_queue_comm_sender, play_queue_comm_receiver)
}
fn play_queue_thread(
    play_queue_comm_receiver: PlayQueueRequestReceiver,
    main_thread_comm_sender: MainThreadCommSender,
) -> JoinHandle<()> {
    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));
    thread::spawn(move || {
        for command in play_queue_comm_receiver.get_recv_iter() {
            match command {
                PlayQueueCommand::RequestToPlay => {
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
                PlayQueueCommand::CloseThread => {
                    break;
                }
            }
        }
    })
}
