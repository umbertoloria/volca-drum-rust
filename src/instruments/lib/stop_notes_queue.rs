use crate::music::note::Note;
use std::collections::{HashMap, HashSet};

pub struct StopNotesQueue {
    stop_notes_queue: HashMap<usize, HashSet<String>>,
}
impl StopNotesQueue {
    pub fn new() -> Self {
        Self {
            stop_notes_queue: HashMap::new(),
        }
    }
    pub fn add_notes_to_stop_notes_queue(&mut self, notes: &Vec<Note>, index_1_16th: usize) {
        // Param "index_1_16th" starts from 1.
        if let Some(queued_notes_to_stop) = self.stop_notes_queue.get_mut(&index_1_16th) {
            for note in notes {
                queued_notes_to_stop.insert(note.to_hash());
            }
        } else {
            let mut queued_notes_to_stop = HashSet::new();
            for note in notes {
                queued_notes_to_stop.insert(note.to_hash());
            }
            self.stop_notes_queue
                .insert(index_1_16th, queued_notes_to_stop);
        }
    }
    pub fn dequeue_notes_at_this_1_16th(&mut self, index_1_16th: usize) -> Option<Vec<Note>> {
        // Param "index_1_16th" starts from 1.
        if let Some(queued_notes_to_stop) = self.stop_notes_queue.remove(&index_1_16th) {
            let mut notes_to_stop = Vec::new();
            for note_hash in queued_notes_to_stop {
                let note = Note::from_hash(note_hash);
                notes_to_stop.push(note);
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
    pub fn add_notes_to_stop_notes_queue(&mut self, note: &Note, index_1_16th: usize) {
        // TODO: Avoid cloning Note string
        let notes = vec![note.clone()];
        self.inner
            .add_notes_to_stop_notes_queue(&notes, index_1_16th);
    }
    pub fn dequeue_notes_at_this_1_16th(&mut self, index_1_16th: usize) -> Option<Vec<Note>> {
        self.inner.dequeue_notes_at_this_1_16th(index_1_16th)
    }
}
