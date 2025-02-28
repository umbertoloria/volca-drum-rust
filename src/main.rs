use crate::music_thread::music_thread::main_music_thread;
use crate::server::main_server::main_server_thread;

mod devices;
mod instruments;
mod midi;
mod music_thread;
mod players;
mod server;
mod song;
mod utils;

fn main() {
    // SOCKET SERVER
    let (server_thread, tx_to_web_server) = main_server_thread();

    // MUSIC THREAD
    let (music_thread, rx_music_thread) = main_music_thread();
    for msg in rx_music_thread {
        tx_to_web_server.send(msg.clone()).unwrap();
    }

    // CLOSE THREADS
    server_thread.join().unwrap();
    println!("Server thread ended!");
    music_thread.join().unwrap();
    println!("Music thread ended!");
}
