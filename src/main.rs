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
    let (tx_to_web_server, rx_to_web_server) = create_channel_for_server_thread();
    let (music_thread_comm_sender, music_thread_comm_receiver) =
        create_music_thread_comm_instances();

    // SOCKET SERVER
    // TODO: It is wise to clone this TX?
    let tx_to_web_server_clone = tx_to_web_server.clone();
    let server_thread = main_server_thread(
        rx_to_web_server,
        tx_to_web_server_clone,
        music_thread_comm_sender,
    );

    // MUSIC THREAD
    let music_thread = main_music_thread(music_thread_comm_receiver, tx_to_web_server);

    // CLOSE THREADS
    server_thread.join().unwrap();
    println!("Server thread ended!");
    music_thread.join().unwrap();
    println!("Music thread ended!");
}
