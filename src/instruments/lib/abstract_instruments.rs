use crate::instruments::lib::drum_sounds::DrumSound;
use crate::music::note::Note;

pub trait AbstractInstrumentPoly {
    fn play_notes_start(&mut self, notes: &Vec<Note>);
    fn play_notes_stop(&mut self, notes: &Vec<Note>);
}

pub trait AbstractInstrumentMono {
    fn play_notes_start(&mut self, note: &Note);
    fn play_notes_stop(&mut self, note: &Note);
}

pub trait AbstractInstrumentPolyDrumSounds {
    fn play_sounds_start(&mut self, sounds: &Vec<DrumSound>);
    fn play_sounds_stop(&mut self, sounds: &Vec<DrumSound>);
}
