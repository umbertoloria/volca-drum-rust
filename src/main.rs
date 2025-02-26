use crate::players::player::play_song_example;
use crate::server::main_server::main_server;

mod devices;
mod instruments;
mod midi;
mod players;
mod server;
mod song;
mod utils;

fn main() {
    // SERVER
    // let server_thread = main_server();
    main_server();

    play_song_example();

    // server_thread.join().unwrap();
}
