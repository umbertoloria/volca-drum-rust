use crate::devices::sound_panel::SoundPanel;
use crate::devices::volca_drum::VolcaDrum;
use crate::devices::volca_keys::VolcaKeys;
use crate::instruments::drummer::Drummer;
use crate::instruments::instr_comm::{
    create_instrument_comm, start_listening_to_instrument_comm_commands, InstrumentBroadcastComm,
    InstrumentCommReceiver,
};
use crate::instruments::keyboard::Keyboard;
use crate::midi::midi_controller::init_midi_controller;
use crate::midi::midi_device::MidiDeviceConcrete;
use crate::players::conductor::Conductor;
use crate::server::web_thread_comm::WebThreadCommSender;
use crate::song::known_songs::get_song_coez_la_musica_non_c_e;
use crate::song::song::Song;
use crate::song::yaml_patch_reader::read_patch_from_yaml;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub enum PlayQueueCommand {
    RequestToPlay,
    CloseThread,
}
type PlayQueueRequestReceiver = ThreadCommReceiver<PlayQueueCommand>;
pub fn create_play_queue_comm() -> (
    ThreadCommSender<PlayQueueCommand>,
    ThreadCommReceiver<PlayQueueCommand>,
) {
    let (play_queue_comm_sender, play_queue_comm_receiver) =
        create_thread_comm_instances::<PlayQueueCommand>();
    (play_queue_comm_sender, play_queue_comm_receiver)
}

pub fn play_queue_thread(
    play_queue_comm_receiver: PlayQueueRequestReceiver,
    web_thread_comm_sender: WebThreadCommSender,
) -> JoinHandle<()> {
    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));
    thread::spawn(move || {
        for command in play_queue_comm_receiver.get_recv_iter() {
            match command {
                PlayQueueCommand::RequestToPlay => {
                    let web_thread_comm_sender_clone = web_thread_comm_sender.clone();

                    // Atomic Read
                    let mut atomic_bool = is_playing.lock().unwrap();
                    let mut value = atomic_bool.get_mut();
                    if *value {
                        println!("*** cannot play, occupied");
                    } else {
                        *value = true;

                        println!("*** can play, now starts");
                        play_song_example_with_updates(web_thread_comm_sender_clone);

                        *value = false;
                    }
                }
                PlayQueueCommand::CloseThread => {
                    break;
                }
            }
        }
    })
}

fn play_song_example_with_updates(web_thread_comm_sender: WebThreadCommSender) {
    web_thread_comm_sender.notify_from_music_thread_song_started();

    play_song_example();

    web_thread_comm_sender.notify_from_music_thread_song_ended();
}

pub fn play_song_example() {
    let song = get_song_to_play();

    // INSTRUMENTS THREADS COMMS
    let (instrument_comm_sender_drummer, instrument_comm_receiver_drummer) =
        create_instrument_comm();
    let (instrument_comm_sender_keyboard, instrument_comm_receiver_keyboard) =
        create_instrument_comm();

    // INSTRUMENTS THREADS
    let (drummer_thread, keyboard_thread) = create_instrument_threads(
        // TODO: Avoid cloning Song
        song.clone(),
        instrument_comm_receiver_drummer,
        instrument_comm_receiver_keyboard,
    );

    // CONDUCTOR
    // TODO: Enable Interactive CLI or not
    // let enable_interactive_cli = true;
    let enable_interactive_cli = false;
    let instrument_broadcast_comm = InstrumentBroadcastComm {
        instrument_comm_senders_list: vec![
            // List of Instruments Communicators
            // instrument_comm_sender_metronome,
            instrument_comm_sender_drummer,
            instrument_comm_sender_keyboard,
        ],
    };
    let mut conductor = Conductor::new(instrument_broadcast_comm, enable_interactive_cli);
    if !enable_interactive_cli {
        println!("Playing song now...");
    }
    conductor.play_song(song).unwrap();

    // CLOSE THREADS
    // metronome_thread.join().unwrap();
    drummer_thread.join().unwrap();
    keyboard_thread.join().unwrap();
}
fn get_song_to_play() -> Song {
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

    song1
}
type InstrumentThreadType = JoinHandle<()>;
fn create_instrument_threads(
    song: Song,
    instrument_comm_receiver_drummer: InstrumentCommReceiver,
    instrument_comm_receiver_keyboard: InstrumentCommReceiver,
) -> (
    // Instruments Threads
    InstrumentThreadType,
    InstrumentThreadType,
) {
    // Metronome
    /*let clone_song_metronome = song1.clone();
    let metronome_thread = thread::spawn(move || {
        // let midi_device = MidiDeviceConcrete::new(init_midi_controller("CLICK", Some(0)).unwrap());
        let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        // Instrument
        let mut metronome = Metronome::new(clone_song_metronome, volca_keys);
        start_listening_to_instr_comm_commands(rx_metronome, &mut metronome);
    });*/

    // Drummer
    let clone_song_drummer = song.clone();
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
        start_listening_to_instrument_comm_commands(instrument_comm_receiver_drummer, &mut drummer);
    });

    // Keyboard
    let clone_song_keyboard = song.clone();
    let keyboard_thread = thread::spawn(move || {
        let midi_device = MidiDeviceConcrete::new(init_midi_controller("KEYS", Some(0)).unwrap());
        // let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        // Instrument
        let mut keyboard = Keyboard::new(clone_song_keyboard, volca_keys);
        start_listening_to_instrument_comm_commands(
            instrument_comm_receiver_keyboard,
            &mut keyboard,
        );
    });

    (
        // Instruments Threads
        drummer_thread,
        keyboard_thread,
    )
}
