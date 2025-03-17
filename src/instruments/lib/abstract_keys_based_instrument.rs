use crate::music::note::Note;

pub trait AbstractKeysBasedInstrument {
    fn play_notes_start(&mut self, notes: &Vec<Note>);
    fn play_notes_stop(&mut self, notes: &Vec<Note>);
}

pub trait AbstractBassBasedInstrument {
    fn play_notes_start(&mut self, note: &Note);
    fn play_notes_stop(&mut self, note: &Note);
}

pub trait AbstractDrumsBasedInstrument {
    fn play_sounds_start(&mut self, sounds: &Vec<String>);
    fn play_sounds_stop(&mut self, sounds: &Vec<String>);
}
