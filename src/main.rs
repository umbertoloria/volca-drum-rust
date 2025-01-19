use crate::drummer::Drummer;
use crate::keyboard::Keyboard;
use crate::midi_controller::init_midi_controller;
use crate::midi_device::{MidiDeviceConcrete, MidiDeviceGhost};
use crate::player::{Player, PlayerObserver};
use crate::song::get_dummy_song;
use crate::sound_panel::SoundPanel;
use crate::volca_drum::VolcaDrum;
use crate::volca_keys::VolcaKeys;
use crate::yaml_patch_reader::read_patch_from_yaml;
use std::io::Write;

mod cli;
mod drummer;
mod input;
mod keyboard;
mod midi_controller;
mod midi_device;
mod player;
mod song;
mod sound_panel;
mod volca_drum;
mod volca_keys;
mod yaml_patch_reader;
mod yaml_song_reader;

fn main() {
    // MIDI CONTROLLERS
    // let midi_controller_1 = init_midi_controller(Some(1)).unwrap();
    let midi_controller_2 = init_midi_controller(Some(1)).unwrap();

    // INSTRUMENTS
    // let midi_device_1 = MidiDeviceConcrete::new(midi_controller_1.connect_and_get());
    let midi_device_1 = MidiDeviceGhost::new();
    let mut volca_drum = VolcaDrum::new(midi_device_1);
    let midi_device_2 = MidiDeviceConcrete::new(midi_controller_2.connect_and_get());
    let mut volca_keys = VolcaKeys::new(midi_device_2);

    // SOUNDS
    let mut sound_panel = SoundPanel {
        volca_drum: &mut volca_drum,
    };
    let patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");
    // TODO: Make sure it always sounds ok from the first hit
    sound_panel.set_from_patch(patch1);
    /*while {
        let num = get_console_int_input("1 per sound refresh, 0 per uscire", 0, 1);

        patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");
        sound_panel.set_from_patch(patch1);

        num > 0
    } {}*/

    // SONG
    let mut instruments: Vec<Box<dyn PlayerObserver>> = Vec::new();
    let drummer = Drummer::new(volca_drum);
    instruments.push(Box::new(drummer));
    let keyboard = Keyboard::new(volca_keys);
    instruments.push(Box::new(keyboard));
    let mut player = Player::new(true, instruments);
    // let song1_yaml = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");
    // let song1 = convert_yaml_into_song(song1_yaml);
    let song1 = get_dummy_song();
    player.play_song(song1);
}
