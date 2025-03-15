use std::collections::{HashMap, HashSet};

// KEYS WITH QUEUE
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

// STOP NOTES QUEUE
pub struct StopNotesQueue {
    stop_notes_queue: HashMap<usize, HashSet<String>>,
}
impl StopNotesQueue {
    pub fn new() -> Self {
        Self {
            stop_notes_queue: HashMap::new(),
        }
    }
    pub fn add_notes_to_stop_notes_queue(
        &mut self,
        notes_str_list: &Vec<String>,
        index_1_16th: usize,
    ) {
        // Param "index_1_16th" starts from 1.
        if let Some(queued_notes_to_stop) = self.stop_notes_queue.get_mut(&index_1_16th) {
            for note_str in notes_str_list {
                // TODO: Avoid cloning note
                let note_str_clone = note_str.clone();
                queued_notes_to_stop.insert(note_str_clone);
            }
        } else {
            let mut queued_notes_to_stop = HashSet::new();
            for note_str in notes_str_list {
                // TODO: Avoid cloning note
                let note_str_clone = note_str.clone();
                queued_notes_to_stop.insert(note_str_clone);
            }
            self.stop_notes_queue
                .insert(index_1_16th, queued_notes_to_stop);
        }
    }
    pub fn dequeue_notes_at_this_1_16th_from_stop_notes_queue(
        &mut self,
        index_1_16th: usize,
    ) -> Option<Vec<String>> {
        // Param "index_1_16th" starts from 1.
        if let Some(queued_notes_to_stop) = self.stop_notes_queue.remove(&index_1_16th) {
            let mut notes_str_list_to_stop = Vec::new();
            for note_str in queued_notes_to_stop {
                notes_str_list_to_stop.push(note_str);
            }
            Some(notes_str_list_to_stop)
        } else {
            None
        }
    }
}

// ABSTRACT KEYS-BASED INSTRUMENT
pub trait AbstractKeysBasedInstrument {
    fn play_notes_start(&mut self, notes_str_list: &Vec<String>);
    fn play_notes_stop(&mut self, notes_str_list: &Vec<String>);
}
