use crate::instruments::lib::abstract_instruments::AbstractInstrumentMono;
use crate::instruments::lib::abstract_instruments::AbstractInstrumentPoly;
use crate::instruments::synth::synth_thread::{
    create_synth_thread_comm, synth_thread, SynthCommand, SynthThreadAudioChannel,
};
use crate::music::note::Note;
use crate::synth::sound::synth_patch_injector::SynthPatchInjector;
use crate::thread_comm::thread_comm::ThreadCommSender;
use std::thread::JoinHandle;

pub struct ThreadForSynthInstrument {
    synth_thread_handle: JoinHandle<()>,
    synth_command_sender: ThreadCommSender<SynthCommand>,
}
impl ThreadForSynthInstrument {
    pub fn new(
        thread_name: String,
        synth_thread_audio_channel: SynthThreadAudioChannel,
        synth_patch_injector: SynthPatchInjector,
        enable_logging: bool,
    ) -> Self {
        let (synth_command_sender, synth_command_receiver) = create_synth_thread_comm();
        let synth_thread_handle = synth_thread(
            thread_name,
            synth_thread_audio_channel,
            synth_patch_injector,
            synth_command_receiver,
            enable_logging,
        );
        Self {
            synth_thread_handle,
            synth_command_sender,
        }
    }
}
impl AbstractInstrumentPoly for ThreadForSynthInstrument {
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
impl AbstractInstrumentMono for ThreadForSynthInstrument {
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
