use crate::instruments::lib::abstract_keys_based_instrument::AbstractKeysBasedInstrument;
use crate::instruments::lib::stop_notes_queue::StopNotesQueue;
use crate::music::note::Note;

pub struct KeysWithQueue {
    stop_notes_queue: StopNotesQueue,
    instrument: Box<dyn AbstractKeysBasedInstrument>,
}
impl KeysWithQueue {
    pub fn new(
        //
        stop_notes_queue: StopNotesQueue,
        instrument: Box<dyn AbstractKeysBasedInstrument>,
    ) -> Self {
        Self {
            stop_notes_queue,
            instrument,
        }
    }
    pub fn attack_notes(&mut self, notes: &Vec<Note>) {
        self.instrument.play_notes_start(&notes);
    }
    pub fn notify_release_notes_at(&mut self, notes: &Vec<Note>, index_1_16th: usize) {
        self.stop_notes_queue
            .add_notes_to_stop_notes_queue(&notes, index_1_16th);
    }
    pub fn stop_notes_queued_on_this_1_16th(&mut self, index_1_16th: usize) {
        let notes_to_stop = self
            .stop_notes_queue
            .dequeue_notes_at_this_1_16th(index_1_16th);
        if let Some(notes_to_stop) = notes_to_stop {
            self.instrument.play_notes_stop(&notes_to_stop);
        }
    }
}
