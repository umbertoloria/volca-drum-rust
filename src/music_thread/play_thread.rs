use crate::devices::sound_panel::SoundPanel;
use crate::devices::volca_drum::VolcaDrum;
use crate::devices::volca_keys::VolcaKeys;
use crate::instruments::abs::instr_comm::{
    create_instrument_comm, start_listening_to_instrument_comm_commands, InstrumentBroadcastComm,
    InstrumentCommReceiver,
};
use crate::instruments::bassist::Bassist;
use crate::instruments::drummer::Drummer;
use crate::instruments::keyboardist::Keyboardist;
use crate::instruments::keys_based_instrument_volca_keys::KeysBasedInstrumentVolcaKeys;
use crate::instruments::metronome::Metronome;
use crate::instruments::synth::synth_thread::SynthThreadAudioChannel;
use crate::instruments::synth::thread_for_synth_instrument::ThreadForSynthInstrument;
use crate::midi::midi_controller::init_midi_controller;
use crate::midi::midi_device::MidiDeviceConcrete;
use crate::players::conductor::Conductor;
use crate::server::web_thread_comm::WebThreadCommSender;
use crate::song::song::Song;
use crate::song::yaml_patch_reader::read_patch_from_yaml;
use crate::synth::sound::patches::{
    make_patch_bass_1, make_patch_drums_1, make_patch_keys_1, make_patch_metronome_click,
};
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
    play_queue_instruments_threads_conf: PlayQueueInstrumentsThreadsConf,
    play_queue_comm_receiver: PlayQueueRequestReceiver,
    web_thread_comm_sender: WebThreadCommSender,
) -> JoinHandle<()> {
    // PLAY THREAD
    let is_playing = Arc::new(Mutex::new(AtomicBool::new(false)));
    thread::spawn(move || {
        // INSTRUMENTS THREADS
        let (instr_comm_sender_metronome_synth, instr_comm_receiver_metronome_synth) =
            create_instrument_comm();
        let (instr_comm_sender_drummer, instr_comm_receiver_drummer) = create_instrument_comm();
        let (instr_comm_sender_keyboard, instr_comm_receiver_keyboard) = create_instrument_comm();
        let (instr_comm_sender_keys_synth, instr_comm_receiver_keys_synth) =
            create_instrument_comm();
        let (instr_comm_sender_bass_synth, instr_comm_receiver_bass_synth) =
            create_instrument_comm();
        let (
            //
            metronome_synth_thread,
            drummer_thread,
            keyboard_thread,
            keys_synth_thread,
            bass_synth_thread,
        ) = create_instrument_threads(
            play_queue_instruments_threads_conf,
            instr_comm_receiver_metronome_synth,
            instr_comm_receiver_drummer,
            instr_comm_receiver_keyboard,
            instr_comm_receiver_keys_synth,
            instr_comm_receiver_bass_synth,
        );
        let mut conductor = Conductor::new(
            //
            InstrumentBroadcastComm {
                instrument_comm_senders_list: vec![
                    // List of Instruments Communicators
                    instr_comm_sender_metronome_synth,
                    instr_comm_sender_drummer,
                    instr_comm_sender_keyboard,
                    instr_comm_sender_keys_synth,
                    instr_comm_sender_bass_synth,
                ],
            },
        );

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
        metronome_synth_thread.join().unwrap();
        drummer_thread.join().unwrap();
        keyboard_thread.join().unwrap();
        keys_synth_thread.join().unwrap();
        bass_synth_thread.join().unwrap();
    })
}

fn play_song_example_with_updates(
    song: Song,
    web_thread_comm_sender: WebThreadCommSender,
    conductor: &mut Conductor,
) {
    // TODO: Avoid cloning Song
    web_thread_comm_sender.notify_from_music_thread_song_started(song.clone());

    // TODO: Enable Interactive CLI or not
    // let enable_interactive_cli = true;
    let enable_interactive_cli = false;
    if !enable_interactive_cli {
        println!("Playing song now...");
    }
    conductor
        .play_song(song, &web_thread_comm_sender, enable_interactive_cli)
        .unwrap();

    web_thread_comm_sender.notify_from_music_thread_song_ended();
}

// INSTRUMENTS THREADS
type InstrumentThreadType = JoinHandle<()>;
const METRONOME_SYNTH_ENABLE_LOGGING: bool = false;
const DRUMS_SYNTH_ENABLE_LOGGING: bool = false;
const KEYS_SYNTH_ENABLE_LOGGING: bool = false;
const BASS_SYNTH_ENABLE_LOGGING: bool = false;
fn create_instrument_threads(
    play_queue_instruments_threads_conf: PlayQueueInstrumentsThreadsConf,
    instr_comm_receiver_metronome_synth: InstrumentCommReceiver,
    instr_comm_receiver_drummer: InstrumentCommReceiver,
    instr_comm_receiver_keyboard: InstrumentCommReceiver,
    instr_comm_receiver_keys_synth: InstrumentCommReceiver,
    instr_comm_receiver_bass_synth: InstrumentCommReceiver,
) -> (
    // Instruments Threads
    InstrumentThreadType, // Metronome Synth
    InstrumentThreadType, // Drummer
    InstrumentThreadType, // Keyboard
    InstrumentThreadType, // Keys Synth
    InstrumentThreadType, // Bass Synth
) {
    // Metronome Synth
    let metronome_synth_thread = thread::spawn(move || {
        let mut metronome_synth = Metronome::new(
            Box::new(ThreadForSynthInstrument::new(
                "ThreadMetronomeSynth".into(),
                SynthThreadAudioChannel::MetronomeClick,
                make_patch_metronome_click(
                    play_queue_instruments_threads_conf.metronome_synth_enabled,
                ),
                METRONOME_SYNTH_ENABLE_LOGGING,
            )),
            METRONOME_SYNTH_ENABLE_LOGGING,
        );
        start_listening_to_instrument_comm_commands(
            instr_comm_receiver_metronome_synth,
            &mut metronome_synth,
        );
    });

    // Drummer
    let drummer_thread = thread::spawn(move || {
        if !play_queue_instruments_threads_conf.drummer_enabled {
            return;
        }

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

        let mut drummer = Drummer::new(
            volca_drum,
            Box::new(ThreadForSynthInstrument::new(
                "ThreadDrumsSynth".into(),
                SynthThreadAudioChannel::Main,
                make_patch_drums_1(),
                DRUMS_SYNTH_ENABLE_LOGGING,
            )),
            DRUMS_SYNTH_ENABLE_LOGGING,
        );
        start_listening_to_instrument_comm_commands(instr_comm_receiver_drummer, &mut drummer);
    });

    // Keyboard
    let keyboard_thread = thread::spawn(move || {
        if !play_queue_instruments_threads_conf.keyboard_enabled {
            return;
        }

        let midi_device = MidiDeviceConcrete::new(init_midi_controller("KEYS", Some(1)).unwrap());
        // let midi_device = MidiDeviceGhost::new(false);
        let volca_keys = VolcaKeys::new(midi_device);

        let mut keyboard = Keyboardist::new(
            Box::new(KeysBasedInstrumentVolcaKeys::new(volca_keys)),
            KEYS_SYNTH_ENABLE_LOGGING,
        );
        start_listening_to_instrument_comm_commands(instr_comm_receiver_keyboard, &mut keyboard);
    });

    // Keys Synth
    let keys_synth_thread = thread::spawn(move || {
        let mut keys_synth = Keyboardist::new(
            Box::new(ThreadForSynthInstrument::new(
                "ThreadKeysSynth".into(),
                SynthThreadAudioChannel::Main,
                make_patch_keys_1(play_queue_instruments_threads_conf.keys_synth_enabled),
                KEYS_SYNTH_ENABLE_LOGGING,
            )),
            KEYS_SYNTH_ENABLE_LOGGING,
        );
        start_listening_to_instrument_comm_commands(
            instr_comm_receiver_keys_synth,
            &mut keys_synth,
        );
    });

    // Bass Synth
    let bass_synth_thread = thread::spawn(move || {
        let mut bass_synth = Bassist::new(
            Box::new(ThreadForSynthInstrument::new(
                "ThreadBassSynth".into(),
                SynthThreadAudioChannel::Main,
                make_patch_bass_1(play_queue_instruments_threads_conf.bass_synth_enabled),
                BASS_SYNTH_ENABLE_LOGGING,
            )),
            BASS_SYNTH_ENABLE_LOGGING,
        );
        start_listening_to_instrument_comm_commands(
            instr_comm_receiver_bass_synth,
            &mut bass_synth,
        );
    });

    (
        // Instruments Threads
        metronome_synth_thread,
        drummer_thread,
        keyboard_thread,
        keys_synth_thread,
        bass_synth_thread,
    )
}

pub struct PlayQueueInstrumentsThreadsConf {
    pub metronome_synth_enabled: bool,
    pub drummer_enabled: bool,
    pub keyboard_enabled: bool,
    pub keys_synth_enabled: bool,
    pub bass_synth_enabled: bool,
}
