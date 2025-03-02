use crate::music_thread::music_thread::main_music_thread;
use crate::music_thread::music_thread_comm::create_music_thread_comm_instances;
use crate::server::main_server::main_server_thread;
use crate::server::main_server_comm::create_channel_for_server_thread;

mod devices;
mod instruments;
mod midi;
mod music_thread;
mod players;
mod server;
mod song;
mod utils;

fn main() {
    // COMMUNICATIONS
    let (main_thread_comm_sender, rx_to_web_server) = create_channel_for_server_thread();
    let (music_thread_comm_sender, music_thread_comm_receiver) =
        create_music_thread_comm_instances();

    // SOCKET SERVER
    // TODO: It is wise to clone this TX?
    let main_thread_comm_sender_clone = main_thread_comm_sender.clone();
    let server_thread = main_server_thread(
        rx_to_web_server,
        main_thread_comm_sender_clone,
        music_thread_comm_sender,
    );

    // MUSIC THREAD
    let music_thread = main_music_thread(music_thread_comm_receiver, main_thread_comm_sender);

    // CLOSE THREADS
    server_thread.join().unwrap();
    println!("Server thread ended!");
    music_thread.join().unwrap();
    println!("Music thread ended!");
}
