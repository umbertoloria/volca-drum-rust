use crate::devices::volca_keys::VolcaKeys;
use crate::instruments::lib::abstract_keys_based_instrument::AbstractInstrumentPoly;
use crate::music::note::Note;

pub struct KeysBasedInstrumentVolcaKeys {
    volca_keys: VolcaKeys,
}
impl KeysBasedInstrumentVolcaKeys {
    pub fn new(volca_keys: VolcaKeys) -> Self {
        Self { volca_keys }
    }
}
impl AbstractInstrumentPoly for KeysBasedInstrumentVolcaKeys {
    fn play_notes_start(&mut self, notes: &Vec<Note>) {
        // println!("play_notes_start: {:?}", notes);

        for note in notes {
            self.volca_keys.note_play_start(note);
        }
    }
    fn play_notes_stop(&mut self, notes: &Vec<Note>) {
        // println!("play_notes_stop: {:?}", notes);

        for note in notes {
            self.volca_keys.note_play_stop(note);
        }
    }
}
