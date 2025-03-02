use crate::devices::sound_panel::SoundPanel;
use crate::devices::volca_drum::VolcaDrum;
use crate::devices::volca_keys::VolcaKeys;
use crate::instruments::drummer::Drummer;
use crate::instruments::instr_comm::{
    create_instrument_comm, start_listening_to_instrument_comm_commands, InstrumentBroadcastComm,
};
use crate::instruments::keyboard::Keyboard;
use crate::midi::midi_controller::init_midi_controller;
use crate::midi::midi_device::MidiDeviceConcrete;
use crate::players::conductor::Conductor;
use crate::song::known_songs::get_song_coez_la_musica_non_c_e;
use crate::song::yaml_patch_reader::read_patch_from_yaml;
use std::thread;

pub fn play_song_example() {
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

    // Metronome
    /*let clone_song_metronome = song1.clone();
    let (tx_metronome, rx_metronome) = create_instr_comm();
    let metronome_thread = thread::spawn(move || {
        // let midi_device = MidiDeviceConcrete::new(init_midi_controller("CLICK", Some(0)).unwrap());
        let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        // Instrument
        let mut metronome = Metronome::new(clone_song_metronome, volca_keys);
        start_listening_to_instr_comm_commands(rx_metronome, &mut metronome);
    });*/

    // Drummer
    let clone_song_drummer = song1.clone();
    let (tx_drummer, rx_drummer) = create_instrument_comm();
    let drummer_thread = thread::spawn(move || {
        let midi_device = MidiDeviceConcrete::new(init_midi_controller("DRUMS", Some(1)).unwrap());
        // let midi_device = MidiDeviceGhost::new(false);
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
        start_listening_to_instrument_comm_commands(rx_drummer, &mut drummer);
    });

    // Keyboard
    let clone_song_keyboard = song1.clone();
    let (tx_keyboard, rx_keyboard) = create_instrument_comm();
    let keyboard_thread = thread::spawn(move || {
        let midi_device = MidiDeviceConcrete::new(init_midi_controller("KEYS", Some(0)).unwrap());
        // let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        // Instrument
        let mut keyboard = Keyboard::new(clone_song_keyboard, volca_keys);
        start_listening_to_instrument_comm_commands(rx_keyboard, &mut keyboard);
    });

    // CONDUCTOR
    // TODO: Enable Interactive CLI or not
    // let enable_interactive_cli = true;
    let enable_interactive_cli = false;
    let instrument_broadcast_comm = InstrumentBroadcastComm {
        tx_list: vec![
            // List of Instruments Communicators
            // tx_metronome,
            tx_drummer,
            tx_keyboard,
        ],
    };
    let mut conductor = Conductor::new(instrument_broadcast_comm, enable_interactive_cli);
    if !enable_interactive_cli {
        println!("Playing song now...");
    }
    conductor.play_song(song1).unwrap();

    // CLOSE THREADS
    // metronome_thread.join().unwrap();
    drummer_thread.join().unwrap();
    keyboard_thread.join().unwrap();
}
