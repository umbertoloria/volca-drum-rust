use crate::instruments::lib::abstract_keys_based_instrument::AbstractKeysBasedInstrument;
use crate::instruments::lib::stop_notes_queue::StopNotesQueue;

pub struct KeysWithQueue {
    stop_notes_queue: StopNotesQueue,
    keys_based_instrument: Box<dyn AbstractKeysBasedInstrument>,
}
impl KeysWithQueue {
    pub fn new(
        stop_notes_queue: StopNotesQueue,
        keys_based_instrument: Box<dyn AbstractKeysBasedInstrument>,
    ) -> Self {
        Self {
            stop_notes_queue,
            keys_based_instrument,
        }
    }
    pub fn playing_hit_chord_start(&mut self, notes_str_list: &Vec<String>) {
        self.keys_based_instrument.play_notes_start(notes_str_list);
    }
    pub fn playing_hit_last_before_chord_stop(
        &mut self,
        notes_str_list: &Vec<String>,
        index_1_16th: usize,
    ) {
        self.stop_notes_queue
            .add_notes_to_stop_notes_queue(notes_str_list, index_1_16th);
    }
    pub fn playing_hit_dequeue_and_stop_notes_at_this_1_16th(&mut self, index_1_16th: usize) {
        let note_str_list_to_stop = self
            .stop_notes_queue
            .dequeue_notes_at_this_1_16th_from_stop_notes_queue(index_1_16th);
        if let Some(note_str_list_to_stop) = note_str_list_to_stop {
            self.keys_based_instrument
                .play_notes_stop(&note_str_list_to_stop);
        }
    }
}
