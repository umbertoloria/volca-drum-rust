use crate::devices::sound_panel::SoundPanel;
use crate::devices::volca_drum::VolcaDrum;
use crate::instruments::bass_synth::BassSynth;
use crate::instruments::drummer::Drummer;
use crate::instruments::instr_comm::{
    create_instrument_comm, start_listening_to_instrument_comm_commands, InstrumentBroadcastComm,
    InstrumentCommReceiver,
};
use crate::instruments::keyboardist::Keyboardist;
use crate::instruments::keys_based_instrument_synth::KeysBasedInstrumentSynth;
use crate::midi::midi_controller::init_midi_controller;
use crate::midi::midi_device::MidiDeviceConcrete;
use crate::players::conductor::Conductor;
use crate::server::web_thread_comm::WebThreadCommSender;
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
    RequestToPlay(Song),
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
    // PLAY THREAD
    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));
    thread::spawn(move || {
        // INSTRUMENTS THREADS
        let (instrument_comm_sender_drummer, instrument_comm_receiver_drummer) =
            create_instrument_comm();
        /*let (instrument_comm_sender_keyboard, instrument_comm_receiver_keyboard) =
        create_instrument_comm();*/
        let (instrument_comm_sender_synth, instrument_comm_receiver_synth) =
            create_instrument_comm();
        let (instrument_comm_sender_bass_synth, instrument_comm_receiver_bass_synth) =
            create_instrument_comm();
        let (
            drummer_thread,
            // keyboard_thread,
            synth_thread,
            bass_synth_thread,
        ) = create_instrument_threads(
            instrument_comm_receiver_drummer,
            // instrument_comm_receiver_keyboard,
            instrument_comm_receiver_synth,
            instrument_comm_receiver_bass_synth,
        );
        let instrument_broadcast_comm = InstrumentBroadcastComm {
            instrument_comm_senders_list: vec![
                // List of Instruments Communicators
                instrument_comm_sender_drummer,
                // instrument_comm_sender_keyboard,
                instrument_comm_sender_synth,
                instrument_comm_sender_bass_synth,
            ],
        };
        let mut conductor = Conductor::new(instrument_broadcast_comm);

        // PLAY THREAD
        for command in play_queue_comm_receiver.get_recv_iter() {
            match command {
                PlayQueueCommand::RequestToPlay(song) => {
                    let web_thread_comm_sender_clone = web_thread_comm_sender.clone();

                    // Atomic Read
                    let mut atomic_bool = is_playing.lock().unwrap();
                    let mut value = atomic_bool.get_mut();
                    if *value {
                        println!("*** cannot play, occupied");
                    } else {
                        *value = true;

                        println!("*** can play, now starts");
                        play_song_example_with_updates(
                            song,
                            web_thread_comm_sender_clone,
                            &mut conductor,
                        );

                        *value = false;
                    }
                }
                PlayQueueCommand::CloseThread => {
                    break;
                }
            }
        }

        // CLOSE INSTRUMENTS THREADS
        drummer_thread.join().unwrap();
        // keyboard_thread.join().unwrap();
        synth_thread.join().unwrap();
        bass_synth_thread.join().unwrap();
    })
}

fn play_song_example_with_updates(
    song: Song,
    web_thread_comm_sender: WebThreadCommSender,
    conductor: &mut Conductor,
) {
    web_thread_comm_sender.notify_from_music_thread_song_started();

    // TODO: Enable Interactive CLI or not
    // let enable_interactive_cli = true;
    let enable_interactive_cli = false;
    if !enable_interactive_cli {
        println!("Playing song now...");
    }
    conductor.play_song(song, enable_interactive_cli).unwrap();

    web_thread_comm_sender.notify_from_music_thread_song_ended();
}

// INSTRUMENTS THREADS
type InstrumentThreadType = JoinHandle<()>;
fn create_instrument_threads(
    instrument_comm_receiver_drummer: InstrumentCommReceiver,
    // instrument_comm_receiver_keyboard: InstrumentCommReceiver,
    instrument_comm_receiver_synth: InstrumentCommReceiver,
    instrument_comm_receiver_bass_synth: InstrumentCommReceiver,
) -> (
    // Instruments Threads
    InstrumentThreadType,
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
        let mut drummer = Drummer::new(volca_drum);
        start_listening_to_instrument_comm_commands(instrument_comm_receiver_drummer, &mut drummer);
    });

    // Keyboard
    /*let keyboard_thread = thread::spawn(move || {
        let midi_device = MidiDeviceConcrete::new(init_midi_controller("KEYS", Some(0)).unwrap());
        // let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        // Instrument
        let mut keyboard = Keyboardist::new(
            "Keyboard        ".into(),
            Box::new(VolcaKeysBasedInstrument::new(volca_keys)),
        );
        start_listening_to_instrument_comm_commands(
            instrument_comm_receiver_keyboard,
            &mut keyboard,
        );
    });*/

    // Synth
    let synth_thread = thread::spawn(move || {
        // Instrument
        let mut synth = Keyboardist::new(
            "SynthKeys       ".into(),
            Box::new(KeysBasedInstrumentSynth::new()),
        );
        start_listening_to_instrument_comm_commands(instrument_comm_receiver_synth, &mut synth);
    });

    // Bass Synth
    let bass_synth_thread = thread::spawn(move || {
        // Instrument
        let mut bass_synth = Bassist::new(
            "SynthBass       ".into(),
            Box::new(BassBasedInstrumentSynth::new()),
        );
        start_listening_to_instrument_comm_commands(
            instrument_comm_receiver_bass_synth,
            &mut bass_synth,
        );
    });

    (
        // Instruments Threads
        drummer_thread,
        // keyboard_thread,
        synth_thread,
        bass_synth_thread,
    )
}
