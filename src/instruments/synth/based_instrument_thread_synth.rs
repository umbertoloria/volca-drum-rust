use crate::instruments::lib::abstract_bass_based_instrument::AbstractBassBasedInstrument;
use crate::instruments::lib::abstract_keys_based_instrument::AbstractKeysBasedInstrument;
use crate::instruments::synth::synth_thread::{
    create_synth_thread_comm, synth_thread, SynthCommand,
};
use crate::music::note::Note;
use crate::thread_comm::thread_comm::ThreadCommSender;
use std::thread::JoinHandle;

// KEYS-BASED INSTRUMENT: SYNTH
pub struct KeysBasedInstrumentThreadSynth {
    synth_thread_handle: JoinHandle<()>,
    synth_command_sender: ThreadCommSender<SynthCommand>,
}
impl KeysBasedInstrumentThreadSynth {
    pub fn new(thread_name: String, synth_volume: f32, enable_logging: bool) -> Self {
        let (synth_command_sender, synth_command_receiver) = create_synth_thread_comm();
        let synth_thread_handle = synth_thread(
            thread_name,
            synth_volume,
            synth_command_receiver,
            enable_logging,
        );
        Self {
            synth_thread_handle,
            synth_command_sender,
        }
    }
}
impl AbstractKeysBasedInstrument for KeysBasedInstrumentThreadSynth {
    fn play_notes_start(&mut self, notes: &Vec<Note>) {
        // println!("play_notes_start: {:?}", notes);

        // TODO: Avoid cloning Notes
        self.synth_command_sender
            .send(SynthCommand::StartNotes(notes.clone()));
    }
    fn play_notes_stop(&mut self, notes: &Vec<Note>) {
        // println!("play_notes_stop: {:?}", notes_str_list);

        // TODO: Try to use "notes"
        self.synth_command_sender.send(SynthCommand::StopNote);
    }
}

// BASS-BASED INSTRUMENT: SYNTH
pub struct BassBasedInstrumentThreadSynth {
    synth_thread_handle: JoinHandle<()>,
    synth_command_sender: ThreadCommSender<SynthCommand>,
}
impl BassBasedInstrumentThreadSynth {
    pub fn new(thread_name: String, synth_volume: f32, enable_logging: bool) -> Self {
        let (synth_command_sender, synth_command_receiver) = create_synth_thread_comm();
        let synth_thread_handle = synth_thread(
            thread_name,
            synth_volume,
            synth_command_receiver,
            enable_logging,
        );
        Self {
            synth_thread_handle,
            synth_command_sender,
        }
    }
}
impl AbstractBassBasedInstrument for BassBasedInstrumentThreadSynth {
    fn play_notes_start(&mut self, note: &Note) {
        // println!("play_notes_start: {:?}", note);

        // TODO: Avoid cloning Note
        let notes = vec![note.clone()];
        self.synth_command_sender
            .send(SynthCommand::StartNotes(notes));
    }
    fn play_notes_stop(&mut self, note: &Note) {
        // println!("play_notes_stop: {:?}", note);

        // TODO: Try to use "note"
        self.synth_command_sender.send(SynthCommand::StopNote);
    }
}
