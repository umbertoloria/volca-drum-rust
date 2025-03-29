use crate::music_thread::music_thread::main_music_thread;
use crate::music_thread::music_thread_comm::create_music_thread_comm_instances;
use crate::music_thread::play_thread::PlayQueueInstrumentsThreadsConf;
use crate::server::web_thread::web_thread;
use crate::server::web_thread_comm::create_channel_for_server_thread;

mod devices;
mod instruments;
mod midi;
mod music;
mod music_thread;
mod players;
mod server;
mod song;
mod synth;
mod thread_comm;
mod utils;

fn main() {
    let enable_instruments = true;
    let enable_keyboards = true;
    // let enable_instruments = false;
    // let enable_keyboards = false;
    let play_queue_instruments_threads_conf = PlayQueueInstrumentsThreadsConf {
        metronome_synth_enabled: enable_instruments,
        drummer_enabled: enable_instruments,
        keyboard_enabled: enable_keyboards,
        keys_synth_enabled: enable_keyboards,
        bass_synth_enabled: enable_instruments,
    };

    // COMMUNICATIONS
    let (web_thread_comm_sender, web_thread_comm_receiver) = create_channel_for_server_thread();
    let (music_thread_comm_sender, music_thread_comm_receiver) =
        create_music_thread_comm_instances();

    // SOCKET SERVER
    // TODO: It is wise to clone this TX?
    let web_thread_comm_sender_clone = web_thread_comm_sender.clone();
    let server_thread = web_thread(
        web_thread_comm_receiver,
        web_thread_comm_sender_clone,
        music_thread_comm_sender,
    );

    // MUSIC THREAD
    let music_thread = main_music_thread(
        play_queue_instruments_threads_conf,
        music_thread_comm_receiver,
        web_thread_comm_sender,
    );

    // CLOSE THREADS
    server_thread.join().unwrap();
    println!("Server thread ended!");
    music_thread.join().unwrap();
    println!("Music thread ended!");
}
