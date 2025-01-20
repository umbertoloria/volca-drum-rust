use crate::drummer::Drummer;
use crate::keyboard::Keyboard;
use crate::midi_controller::init_midi_controller;
use crate::midi_device::MidiDeviceConcrete;
use crate::player::{Player, PlayerObserver};
use crate::song::get_dummy_song;
use crate::sound_panel::SoundPanel;
use crate::volca_drum::VolcaDrum;
use crate::volca_keys::VolcaKeys;
use crate::yaml_patch_reader::read_patch_from_yaml;

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
    let midi_controller_1 = init_midi_controller(Some(1)).unwrap();
    let midi_controller_2 = init_midi_controller(Some(0)).unwrap();

    // SONG
    // let song1_yaml = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");
    // let song1 = convert_yaml_into_song(song1_yaml);
    let song1 = get_dummy_song();

    // INSTRUMENTS
    let mut instruments: Vec<Box<dyn PlayerObserver>> = Vec::new();

    // Drums
    let midi_device_1 = MidiDeviceConcrete::new(midi_controller_1.connect_and_get());
    // let midi_device_1 = MidiDeviceGhost::new();
    let mut volca_drum = VolcaDrum::new(midi_device_1);
    // Drums: Sounds
    let mut sound_panel = SoundPanel {
        volca_drum: &mut volca_drum,
    };
    let patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");
    // TODO: Make sure it always sounds ok from the first hit
    sound_panel.set_from_patch(patch1);
    let drummer = Drummer::new(song1.clone(), volca_drum);
    instruments.push(Box::new(drummer));

    // Keyboard
    let midi_device_2 = MidiDeviceConcrete::new(midi_controller_2.connect_and_get());
    let volca_keys = VolcaKeys::new(midi_device_2);
    let keyboard = Keyboard::new(song1.clone(), volca_keys);
    instruments.push(Box::new(keyboard));

    // PLAYER
    let enable_interactive_cli = true;
    let mut player = Player::new(enable_interactive_cli, instruments);
    player.play_song(song1).unwrap();
}
