use crate::instruments::lib::abstract_keys_based_instrument::AbstractInstrumentPoly;
use crate::instruments::lib::stop_notes_queue::StopQueueNotes;
use crate::music::note::Note;

pub struct InstrumentWithQueuedNotes {
    queue: StopQueueNotes,
    instrument: Box<dyn AbstractInstrumentPoly>,
}
impl InstrumentWithQueuedNotes {
    pub fn new(
        //
        instrument: Box<dyn AbstractInstrumentPoly>,
    ) -> Self {
        Self {
            queue: StopQueueNotes::new(),
            instrument,
        }
    }
    pub fn attack_notes(&mut self, notes: &Vec<Note>) {
        self.instrument.play_notes_start(&notes);
    }
    pub fn notify_release_notes_at(&mut self, notes: &Vec<Note>, index_1_16th: usize) {
        // FIXME: Use MOD with "index_1_16th" otherwise you never stop after number 16 or similar
        self.queue.enqueue(&notes, index_1_16th);
    }
    pub fn stop_notes_queued_on_this_1_16th(&mut self, index_1_16th: usize) {
        let notes_to_stop = self.queue.dequeue_at(index_1_16th);
        if let Some(notes_to_stop) = notes_to_stop {
            self.instrument.play_notes_stop(&notes_to_stop);
        }
    }
}
