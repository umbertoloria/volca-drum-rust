use crate::music_thread::music_thread::{main_music_thread, CommFromMusicThread};

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
    // main_server();

    // MUSIC THREAD
    let (music_thread, rx_music_thread) = main_music_thread();
    for msg in rx_music_thread {
        match msg {
            // TODO: Communicate via broadcast with all Clients
            CommFromMusicThread::SongStarted => {
                println!("Song started");
            }
            CommFromMusicThread::SongPlayingUpdate => {
                println!("Song playing update");
            }
            CommFromMusicThread::SongEnded => {
                println!("Song ended");
            }
        }
    }
    music_thread.join().unwrap();
    println!("Music thread ended!");
}
