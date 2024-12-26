use crate::midi_controller::init_midi_controller;
use crate::sound_panel::SoundPanel;
use crate::yaml_patch_reader::read_patch_from_yaml;
use std::io::Write;

mod drummer;
mod input;
mod midi_controller;
mod player;
mod sound_panel;
mod yaml_patch_reader;
mod yaml_song_reader;

fn main() {
    // READ SONG FILE
    // let song1 = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");

    // READ PATCH FILE
    let patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");

    // MIDI CONNECTION: START
    let midi_controller = init_midi_controller(Some(1)).expect("Unable to create midi controller");
    let mut conn = midi_controller.connect_and_get();

    // SOUND SETTING
    let mut sound_panel = SoundPanel { conn: &mut conn };
    sound_panel.set_from_patch(patch1);

    // PLAY SONG
    /*
    let player = Player::new(song1.tempo_1_4);
    println!("Play song \"{}\" by \"{}\"", song1.title, song1.author);
    for section in &song1.sections {
        let mut notes = "";
        if let Some(x) = &section.notes {
            notes = x;
        }
        println!("New section: type {:6} -> {}", section.kind, notes);
        player.play_num_bars(section.bars);
    }
    println!("Song end.");
    */

    // MIDI CONNECTION: SHUT-DOWN
    conn.close();
}
