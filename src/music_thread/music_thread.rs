use crate::music::note::Note;
use crate::music_thread::music_thread_comm::{MusicThreadCommReceiver, MusicThreadCommand};
use crate::music_thread::play_thread::{
    create_play_queue_comm, play_queue_thread, PlayQueueCommand,
};
use crate::music_thread::volca_drum_thread::{create_volca_drum_thread_comm, VolcaDrumCommand};
use crate::server::web_thread_comm::WebThreadCommSender;
use crate::song::composer::{Composer, ComposerMode};
use crate::song::known_songs::{get_song_coez_la_musica_non_c_e, get_song_o1, get_song_o2};
use crate::song::song::Song;
use std::thread;
use std::thread::JoinHandle;

pub fn main_music_thread(
    music_thread_comm_receiver: MusicThreadCommReceiver,
    web_thread_comm_sender: WebThreadCommSender,
) -> JoinHandle<()> {
    thread::spawn(move || {
        music_thread_logics(music_thread_comm_receiver, web_thread_comm_sender);
    })
}

fn music_thread_logics(
    music_thread_comm_receiver: MusicThreadCommReceiver,
    web_thread_comm_sender: WebThreadCommSender,
) {
    // Volca Drum Thread
    let (volca_drum_command_sender, volca_drum_command_receiver) = create_volca_drum_thread_comm();
    // TODO: For now it's disabled
    // let volca_drum_thread = volca_drum_thread(volca_drum_command_receiver);

    // Play Queue Thread
    let (play_queue_comm_sender, play_queue_comm_receiver) = create_play_queue_comm();
    let play_queue_thread = play_queue_thread(play_queue_comm_receiver, web_thread_comm_sender);

    for command in music_thread_comm_receiver.get_recv_iter() {
        match command {
            MusicThreadCommand::PlaySong() => {
                // TODO: Pick song using Song ID from Web
                let song = get_song_to_play();
                play_queue_comm_sender.send(PlayQueueCommand::RequestToPlay(song));
            }
            MusicThreadCommand::ApplyVolcaDrumPatch(yaml_patch_file) => {
                volca_drum_command_sender.send(VolcaDrumCommand::ApplyPatch(yaml_patch_file));
            }
        }
    }

    play_queue_comm_sender.send(PlayQueueCommand::CloseThread);
    play_queue_thread.join().unwrap();

    volca_drum_command_sender.send(VolcaDrumCommand::CloseThread);
    // volca_drum_thread.join().unwrap();
}

pub fn get_song_to_play() -> Song {
    // SONG
    /*
    let song_yaml = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");
    let song = convert_yaml_into_song(song_yaml);
    */
    // let song = get_dummy_song();
    let mut composer = Composer::new(
        //
        210,
        // true,
        false,
        // TonalityNote::C,
        // TonalityMode::Major,
        Note::new("C4"),
        ComposerMode::Major,
        // ComposerMode::Minor,
    );
    composer.compose_new_song(10);
    let song = composer.get_song();
    let song = get_song_coez_la_musica_non_c_e();
    let song = get_song_o1();
    let song = get_song_o2(80);
    let song = get_song_o2(100);
    song
}
