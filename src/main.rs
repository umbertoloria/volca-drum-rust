use crate::instr_comm::{create_instr_comm, start_listening_to_instr_comm_commands, InstrComm};
use crate::instruments::keyboard::Keyboard;
use crate::known_songs::get_song_coez_la_musica_non_c_e;
use crate::midi_controller::init_midi_controller;
use crate::midi_device::{MidiDeviceConcrete, MidiDeviceGhost};
use crate::player::Player;
use crate::sound_panel::SoundPanel;
use crate::volca_drum::VolcaDrum;
use crate::volca_keys::VolcaKeys;
use crate::yaml_patch_reader::read_patch_from_yaml;
use instruments::drummer::Drummer;
use instruments::metronome::Metronome;
use std::thread;

mod cli;
mod composer;
mod input;
mod instr_comm;
mod instruments;
mod known_songs;
mod midi_controller;
mod midi_device;
mod player;
mod song;
mod sound_panel;
mod timing;
mod volca_drum;
mod volca_keys;
mod yaml_patch_reader;
mod yaml_song_reader;

fn main() {
    // SONG
    /*
    let song1_yaml = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");
    let song1 = convert_yaml_into_song(song1_yaml);
    */
    // let song1 = get_dummy_song();
    let song1 = get_song_coez_la_musica_non_c_e();
    /*
    let composer = Composer {
        bpm: 50,
        num_sections: 10,
        click: true,
        tonality_note: TonalityNote::C,
        tonality_mode: TonalityMode::Major,
    };
    let song1 = composer.compose_new_song();
    */

    // INSTRUMENTS
    // Drummer
    let clone_song_drummer = song1.clone();
    let (tx_drummer, rx_drummer) = create_instr_comm();
    let drummer_thread = thread::spawn(move || {
        // let midi_controller = init_midi_controller(Some(1)).unwrap();
        // let midi_device = MidiDeviceConcrete::new(midi_controller.connect_and_get());
        let midi_device = MidiDeviceGhost::new(false);
        let mut volca_drum = VolcaDrum::new(midi_device);

        // Sounds
        let mut sound_panel = SoundPanel {
            volca_drum: &mut volca_drum,
        };
        let patch1 = read_patch_from_yaml("files/patches/1-patch.yaml");
        // TODO: Make sure it always sounds ok from the first hit
        sound_panel.set_from_patch(patch1);

        // Instrument
        let mut drummer = Drummer::new(clone_song_drummer, volca_drum);
        start_listening_to_instr_comm_commands(rx_drummer, &mut drummer);
    });

    // Keyboard
    let clone_song_keyboard = song1.clone();
    let (tx_keyboard, rx_keyboard) = create_instr_comm();
    let keyboard_thread = thread::spawn(move || {
        let midi_device =
            MidiDeviceConcrete::new(init_midi_controller(Some(0)).unwrap().connect_and_get());
        // let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        // Instrument
        let mut keyboard = Keyboard::new(clone_song_keyboard, volca_keys);
        start_listening_to_instr_comm_commands(rx_keyboard, &mut keyboard);
    });

    // Metronome
    let clone_song_metronome = song1.clone();
    let (tx_metronome, rx_metronome) = create_instr_comm();
    let metronome_thread = thread::spawn(move || {
        // let midi_device =
        //     MidiDeviceConcrete::new(init_midi_controller(Some(0)).unwrap().connect_and_get());
        let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        // Instrument
        let mut metronome = Metronome::new(clone_song_metronome, volca_keys);
        start_listening_to_instr_comm_commands(rx_metronome, &mut metronome);
    });

    // PLAYER
    // TODO: Enable Interactive CLI or not
    let enable_interactive_cli = true;
    // let enable_interactive_cli = false;
    let instr_comm = InstrComm {
        tx_list: vec![
            // List of Instruments Communicators
            tx_drummer,
            tx_keyboard,
            tx_metronome,
        ],
    };
    let mut player = Player::new(enable_interactive_cli, instr_comm);
    player.play_song(song1).unwrap();

    // CLOSE THREADS
    drummer_thread.join().unwrap();
    keyboard_thread.join().unwrap();
    metronome_thread.join().unwrap();
}
