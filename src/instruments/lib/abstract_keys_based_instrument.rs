use crate::music::note::Note;

pub trait AbstractKeysBasedInstrument {
    fn play_notes_start(&mut self, notes: &Vec<Note>);
    fn play_notes_stop(&mut self, notes: &Vec<Note>);
}
