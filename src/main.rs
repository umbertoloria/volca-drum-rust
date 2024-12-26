use crate::midi_controller::init_midi_controller;
use crate::player::play_song;
use crate::song::get_dummy_song;
use crate::sound_panel::SoundPanel;
use crate::yaml_patch_reader::read_patch_from_yaml;
use std::io::Write;

mod cli;
mod drummer;
mod input;
mod midi_controller;
mod player;
mod song;
mod sound_panel;
mod yaml_patch_reader;
mod yaml_song_reader;

fn main() {
    // MIDI
    let midi_controller = init_midi_controller(Some(1)).expect("Unable to create midi controller");
    let mut volca_drum = midi_controller.connect_and_get();

    // SOUNDS
    let mut volca_drum_sound_panel = SoundPanel {
        conn: &mut volca_drum,
    };
    let patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");
    volca_drum_sound_panel.set_from_patch(patch1);

    // SONG
    // let song1_yaml = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");
    // let song1 = convert_yaml_into_song(song1_yaml);
    let song1 = get_dummy_song();
    play_song(song1, &mut volca_drum);

    // MIDI
    volca_drum.close();
}
