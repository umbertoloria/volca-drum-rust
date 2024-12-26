use crate::input::get_console_int_input;
use crate::midi_controller::init_midi_controller;
use crate::player::play_song;
use crate::song::get_dummy_song;
use crate::sound_panel::SoundPanel;
use crate::volca_drum::VolcaDrum;
use crate::yaml_patch_reader::read_patch_from_yaml;
use std::io::Write;

mod cli;
mod drummer;
mod input;
mod midi_controller;
mod player;
mod song;
mod sound_panel;
mod volca_drum;
mod yaml_patch_reader;
mod yaml_song_reader;

fn main() {
    // MIDI
    let midi_controller = init_midi_controller(Some(1)).expect("Unable to create midi controller");
    let mut volca_drum = midi_controller.connect_and_get();
    // TODO: Make sure it always connect

    // SOUNDS
    let mut volca_drum_sound_panel = SoundPanel {
        volca_drum: VolcaDrum {
            conn: &mut volca_drum,
        },
    };
    let patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");
    // TODO: Make sure it always sounds ok from the first hit
    volca_drum_sound_panel.set_from_patch(patch1);
    while {
        let num = get_console_int_input("1 per sound refresh, 0 per uscire", 0, 1);

        let patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");
        volca_drum_sound_panel.set_from_patch(patch1);

        num > 0
    } {}

    // SONG
    // let song1_yaml = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");
    // let song1 = convert_yaml_into_song(song1_yaml);
    let song1 = get_dummy_song();
    play_song(song1, &mut volca_drum);

    // MIDI
    volca_drum.close();
}
