use crate::music::note::Note;
use std::collections::{HashMap, HashSet};

type SetNotesToStop = HashSet<String>;
pub struct StopNotesQueue {
    // From Index 1/16th to Set of Notes.
    queue_i2set: HashMap<usize, SetNotesToStop>,
}
impl StopNotesQueue {
    pub fn new() -> Self {
        Self {
            queue_i2set: HashMap::new(),
        }
    }
    pub fn add_notes_to_stop(&mut self, notes: &Vec<Note>, index_1_16th: usize) {
        // Param "index_1_16th" starts from 1.
        if let Some(set) = self.queue_i2set.get_mut(&index_1_16th) {
            for note in notes {
                let item = note.to_hash();
                set.insert(item);
            }
        } else {
            let mut set = SetNotesToStop::new();
            for note in notes {
                let item = note.to_hash();
                set.insert(item);
            }
            self.queue_i2set.insert(index_1_16th, set);
        }
    }
    pub fn dequeue_notes_at_this_1_16th(&mut self, index_1_16th: usize) -> Option<Vec<Note>> {
        // Param "index_1_16th" starts from 1.
        if let Some(set) = self.queue_i2set.remove(&index_1_16th) {
            let mut notes_to_stop = Vec::new();
            for item in set {
                let note_to_stop = Note::from_hash(item);
                notes_to_stop.push(note_to_stop);
            }
            Some(notes_to_stop)
        } else {
            None
        }
    }
}

// For bass.
pub struct StopNoteQueueForBass {
    inner: StopNotesQueue,
}
impl StopNoteQueueForBass {
    pub fn new() -> Self {
        Self {
            inner: StopNotesQueue::new(),
        }
    }
    pub fn add_note_to_stop(&mut self, note: &Note, index_1_16th: usize) {
        // TODO: Avoid cloning Note
        let notes = vec![note.clone()];
        self.inner.add_notes_to_stop(&notes, index_1_16th);
    }
    pub fn dequeue_notes_at_this_1_16th(&mut self, index_1_16th: usize) -> Option<Vec<Note>> {
        self.inner.dequeue_notes_at_this_1_16th(index_1_16th)
    }
}
