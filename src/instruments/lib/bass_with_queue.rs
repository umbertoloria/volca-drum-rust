use crate::instruments::lib::abstract_bass_based_instrument::AbstractBassBasedInstrument;
use crate::instruments::lib::stop_notes_queue::StopNotesQueue;

pub struct BassWithQueue {
    stop_notes_queue: StopNotesQueue,
    bass_based_instrument: Box<dyn AbstractBassBasedInstrument>,
}
impl BassWithQueue {
    pub fn new(
        stop_notes_queue: StopNotesQueue,
        keys_based_instrument: Box<dyn AbstractBassBasedInstrument>,
    ) -> Self {
        Self {
            stop_notes_queue,
            bass_based_instrument: keys_based_instrument,
        }
    }
    pub fn playing_hit_chord_start(&mut self, note_str: &String) {
        self.bass_based_instrument.play_notes_start(note_str);
    }
    pub fn playing_hit_last_before_chord_stop(&mut self, note_str: &String, index_1_16th: usize) {
        // FIXME: Why a list?
        // FIXME: Avoid cloning Note string
        let notes_str_list = vec![note_str.clone()];

        self.stop_notes_queue
            .add_notes_to_stop_notes_queue(&notes_str_list, index_1_16th);
    }
    pub fn playing_hit_dequeue_and_stop_notes_at_this_1_16th(&mut self, index_1_16th: usize) {
        let note_str_list_to_stop = self
            .stop_notes_queue
            .dequeue_notes_at_this_1_16th_from_stop_notes_queue(index_1_16th);
        if let Some(note_str_list_to_stop) = note_str_list_to_stop {
            for note_str_to_stop in note_str_list_to_stop {
                // FIXME: Why a list?
                self.bass_based_instrument
                    .play_notes_stop(&note_str_to_stop);
            }
        }
    }
}
