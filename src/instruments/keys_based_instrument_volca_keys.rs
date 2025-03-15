use crate::devices::volca_keys::VolcaKeys;
use crate::instruments::stop_notes_queue::AbstractKeysBasedInstrument;

pub struct KeysBasedInstrumentVolcaKeys {
    volca_keys: VolcaKeys,
}
impl KeysBasedInstrumentVolcaKeys {
    pub fn new(volca_keys: VolcaKeys) -> Self {
        Self { volca_keys }
    }
}
impl AbstractKeysBasedInstrument for KeysBasedInstrumentVolcaKeys {
    fn play_notes_start(&mut self, notes_str_list: &Vec<String>) {
        // println!("play_notes_start: {:?}", notes_str_list);

        for note_str in notes_str_list {
            // TODO: Avoid cloning note
            let note_str_clone = note_str.clone();

            self.volca_keys.note_play_start(note_str_clone);
        }
    }
    fn play_notes_stop(&mut self, notes_str_list: &Vec<String>) {
        // println!("play_notes_stop: {:?}", notes_str_list);

        for note_str in notes_str_list {
            // TODO: Avoid cloning note
            let note_str_clone = note_str.clone();

            self.volca_keys.note_play_stop(note_str_clone);
        }
    }
}
