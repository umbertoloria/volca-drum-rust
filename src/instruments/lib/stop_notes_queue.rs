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
    pub fn dequeue_notes_at_this_1_16th(&mut self, index_1_16th: usize) -> Option<Vec<String>> {
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
    pub fn add_notes_to_stop_notes_queue(&mut self, note_str: &String, index_1_16th: usize) {
        // TODO: Avoid cloning Note string
        let notes_str_list = vec![note_str.clone()];
        self.inner
            .add_notes_to_stop_notes_queue(&notes_str_list, index_1_16th);
    }
    pub fn dequeue_notes_at_this_1_16th(&mut self, index_1_16th: usize) -> Option<Vec<String>> {
        self.inner.dequeue_notes_at_this_1_16th(index_1_16th)
    }
}
