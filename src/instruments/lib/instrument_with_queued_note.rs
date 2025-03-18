use crate::instruments::lib::abstract_keys_based_instrument::AbstractInstrumentMono;
use crate::instruments::lib::stop_notes_queue::StopQueueNote;
use crate::music::note::Note;

pub struct InstrumentWithQueuedNote {
    queue: StopQueueNote,
    instrument: Box<dyn AbstractInstrumentMono>,
}
impl InstrumentWithQueuedNote {
    pub fn new(
        //
        instrument: Box<dyn AbstractInstrumentMono>,
    ) -> Self {
        Self {
            queue: StopQueueNote::new(),
            instrument,
        }
    }
    pub fn attack_note(&mut self, note: &Note) {
        self.instrument.play_notes_start(note);
    }
    pub fn notify_release_note_at(&mut self, note: &Note, index_1_16th: usize) {
        self.queue.enqueue(note, index_1_16th);
    }
    pub fn stop_notes_queued_on_this_1_16th(&mut self, index_1_16th: usize) {
        let notes_to_stop = self.queue.dequeue_at(index_1_16th);
        if let Some(notes_to_stop) = notes_to_stop {
            // Still a list of Notes, you never know :)
            for note_to_stop in notes_to_stop {
                self.instrument.play_notes_stop(&note_to_stop);
            }
        }
    }
}
