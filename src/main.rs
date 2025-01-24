use crate::composer::{Composer, TonalityMode, TonalityNote};
use crate::drummer::Drummer;
use crate::keyboard::Keyboard;
use crate::midi_controller::init_midi_controller;
use crate::midi_device::MidiDeviceConcrete;
use crate::player::{
    create_communication_channel_for_instrument, Player, PlayerCommunicator,
    PlayerCommunicatorEnumCommand, PlayerObserver,
};
use crate::sound_panel::SoundPanel;
use crate::volca_drum::VolcaDrum;
use crate::volca_keys::VolcaKeys;
use crate::yaml_patch_reader::read_patch_from_yaml;
use std::thread;

mod cli;
mod composer;
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
    // SONG
    // let song1_yaml = read_song_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");
    // let song1 = convert_yaml_into_song(song1_yaml);
    // let song1 = get_dummy_song();
    let composer = Composer {
        bpm: 50,
        num_sections: 10,
        click: true,
        tonality_note: TonalityNote::C,
        tonality_mode: TonalityMode::Major,
    };
    let song1 = composer.compose_new_song();

    // INSTRUMENTS
    // Drummer
    let clone_song_drummer = song1.clone();
    let (tx_drummer, rx_drummer) = create_communication_channel_for_instrument();
    let drummer_thread = thread::spawn(move || {
        // MIDI CONTROLLERS
        let midi_controller_1 = init_midi_controller(Some(1)).unwrap();

        // INSTRUMENT
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
        let mut drummer = Drummer::new(clone_song_drummer, volca_drum);
        for received in rx_drummer {
            match received {
                PlayerCommunicatorEnumCommand::TeachSong(song_id) => {
                    drummer.teach_song(song_id);
                }
                PlayerCommunicatorEnumCommand::PlayHit(tempo_signature) => {
                    drummer.play_1_16th(&tempo_signature);
                }
                PlayerCommunicatorEnumCommand::Shutdown => {
                    break;
                }
            }
        }
    });

    // Keyboard
    let clone_song_keyboard = song1.clone();
    let (tx_keyboard, rx_keyboard) = create_communication_channel_for_instrument();
    let keyboard_thread = thread::spawn(move || {
        // MIDI CONTROLLERS
        let midi_controller_2 = init_midi_controller(Some(0)).unwrap();

        // INSTRUMENT
        let midi_device_2 = MidiDeviceConcrete::new(midi_controller_2.connect_and_get());
        let volca_keys = VolcaKeys::new(midi_device_2);
        let mut keyboard = Keyboard::new(clone_song_keyboard, volca_keys);
        for received in rx_keyboard {
            match received {
                PlayerCommunicatorEnumCommand::TeachSong(song_id) => {
                    keyboard.teach_song(song_id);
                }
                PlayerCommunicatorEnumCommand::PlayHit(tempo_signature) => {
                    keyboard.play_1_16th(&tempo_signature);
                }
                PlayerCommunicatorEnumCommand::Shutdown => {
                    break;
                }
            }
        }
    });

    // PLAYER
    let mut tx_list = Vec::new();
    tx_list.push(tx_keyboard);
    tx_list.push(tx_drummer);
    let enable_interactive_cli = true;
    let player_communicator = PlayerCommunicator { tx_list };
    let mut player = Player::new(enable_interactive_cli, player_communicator);
    player.play_song(song1).unwrap();

    // CLOSE THREADS
    drummer_thread.join().unwrap();
    keyboard_thread.join().unwrap();
}
