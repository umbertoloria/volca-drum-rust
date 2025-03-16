use crate::instruments::lib::abstract_bass_based_instrument::AbstractBassBasedInstrument;
use crate::instruments::lib::stop_notes_queue::StopNoteQueueForBass;
use crate::music::note::Note;

pub struct BassWithQueue {
    stop_note_queue: StopNoteQueueForBass,
    bass_based_instrument: Box<dyn AbstractBassBasedInstrument>,
}
impl BassWithQueue {
    pub fn new(
        //
        bass_based_instrument: Box<dyn AbstractBassBasedInstrument>,
    ) -> Self {
        Self {
            stop_note_queue: StopNoteQueueForBass::new(),
            bass_based_instrument,
        }
    }
    pub fn playing_hit_chord_start(&mut self, note_str: &String) {
        let note = Note::new(note_str);
        self.bass_based_instrument.play_notes_start(&note);
    }
    pub fn playing_hit_last_before_chord_stop(&mut self, note_str: &String, index_1_16th: usize) {
        self.stop_note_queue
            .add_notes_to_stop_notes_queue(note_str, index_1_16th);
    }
    pub fn playing_hit_dequeue_and_stop_notes_at_this_1_16th(&mut self, index_1_16th: usize) {
        let note_str_list_to_stop = self
            .stop_note_queue
            .dequeue_notes_at_this_1_16th(index_1_16th);
        if let Some(note_str_list_to_stop) = note_str_list_to_stop {
            // A list since you can never know...
            for note_str_to_stop in note_str_list_to_stop {
                let note_to_stop = Note::new(&note_str_to_stop);
                self.bass_based_instrument.play_notes_stop(&note_to_stop);
            }
        }
    }
}
