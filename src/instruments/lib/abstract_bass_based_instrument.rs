pub trait AbstractBassBasedInstrument {
    fn play_notes_start(&mut self, note_str: &String);
    fn play_notes_stop(&mut self, note_str: &String);
}
