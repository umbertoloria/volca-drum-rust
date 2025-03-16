use crate::music::note::Note;

pub trait AbstractBassBasedInstrument {
    fn play_notes_start(&mut self, note: &Note);
    fn play_notes_stop(&mut self, note: &Note);
}
