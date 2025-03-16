use crate::instruments::lib::abstract_bass_based_instrument::AbstractBassBasedInstrument;
use crate::instruments::lib::stop_notes_queue::StopNoteQueueForBass;
use crate::music::note::Note;

pub struct BassWithQueue {
    stop_note_queue: StopNoteQueueForBass,
    instrument: Box<dyn AbstractBassBasedInstrument>,
}
impl BassWithQueue {
    pub fn new(
        //
        instrument: Box<dyn AbstractBassBasedInstrument>,
    ) -> Self {
        Self {
            stop_note_queue: StopNoteQueueForBass::new(),
            instrument,
        }
    }
    pub fn attack_note(&mut self, note: &Note) {
        self.instrument.play_notes_start(note);
    }
    pub fn notify_release_note_at(&mut self, note: &Note, index_1_16th: usize) {
        self.stop_note_queue
            .add_notes_to_stop_notes_queue(note, index_1_16th);
    }
    pub fn stop_notes_queued_on_this_1_16th(&mut self, index_1_16th: usize) {
        let notes_to_stop = self
            .stop_note_queue
            .dequeue_notes_at_this_1_16th(index_1_16th);
        if let Some(notes_to_stop) = notes_to_stop {
            // Still a list of Notes, you never know :)
            for note_to_stop in notes_to_stop {
                self.instrument.play_notes_stop(&note_to_stop);
            }
        }
    }
}
