use crate::instruments::stop_notes_queue::AbstractKeysBasedInstrument;
use crate::instruments::synth_thread::{create_synth_thread_comm, synth_thread, SynthCommand};
use crate::music::note::get_notes_from_note_str_list;
use crate::thread_comm::thread_comm::ThreadCommSender;
use std::thread::JoinHandle;

pub struct KeysBasedInstrumentSynth {
    synth_thread_handle: JoinHandle<()>,
    synth_command_sender: ThreadCommSender<SynthCommand>,
}
impl KeysBasedInstrumentSynth {
    pub fn new() -> Self {
        let (synth_command_sender, synth_command_receiver) = create_synth_thread_comm();
        let synth_thread_handle = synth_thread("Synth".into(), 0.3, synth_command_receiver);
        Self {
            synth_thread_handle,
            synth_command_sender,
        }
    }
}
impl AbstractKeysBasedInstrument for KeysBasedInstrumentSynth {
    fn play_notes_start(&mut self, notes_str_list: &Vec<String>) {
        // println!("play_notes_start: {:?}", notes_str_list);

        let notes = get_notes_from_note_str_list(notes_str_list);
        self.synth_command_sender
            .send(SynthCommand::StartNotes(notes));
    }
    fn play_notes_stop(&mut self, notes_str_list: &Vec<String>) {
        // println!("play_notes_stop: {:?}", notes_str_list);

        // TODO: Try to use "notes_str_list"
        self.synth_command_sender.send(SynthCommand::StopNote);
    }
}
