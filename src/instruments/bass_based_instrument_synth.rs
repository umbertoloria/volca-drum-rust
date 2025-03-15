use crate::instruments::lib::abstract_bass_based_instrument::AbstractBassBasedInstrument;
use crate::instruments::synth_thread::{create_synth_thread_comm, synth_thread, SynthCommand};
use crate::music::note::Note;
use crate::thread_comm::thread_comm::ThreadCommSender;
use std::thread::JoinHandle;

// FIXME: Too much similar to KeysBasedInstrumentSynth
pub struct BassBasedInstrumentSynth {
    synth_thread_handle: JoinHandle<()>,
    synth_command_sender: ThreadCommSender<SynthCommand>,
}
impl BassBasedInstrumentSynth {
    pub fn new() -> Self {
        let (synth_command_sender, synth_command_receiver) = create_synth_thread_comm();
        let synth_thread_handle = synth_thread("BassSynth".into(), 0.3, synth_command_receiver);
        Self {
            synth_thread_handle,
            synth_command_sender,
        }
    }
}
impl AbstractBassBasedInstrument for BassBasedInstrumentSynth {
    fn play_notes_start(&mut self, note_str: &String) {
        // println!("play_notes_start: {:?}", note_str);

        let notes = vec![
            //
            Note::new(note_str),
        ];
        self.synth_command_sender
            .send(SynthCommand::StartNotes(notes));
    }
    fn play_notes_stop(&mut self, note_str: &String) {
        // println!("play_notes_stop: {:?}", note_str);

        // TODO: Try to use "note_str"
        self.synth_command_sender.send(SynthCommand::StopNote);
    }
}
