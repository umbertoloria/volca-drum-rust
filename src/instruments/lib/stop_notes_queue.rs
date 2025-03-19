use crate::instruments::lib::drum_sounds::DrumSound;
use crate::music::note::Note;
use std::collections::{BTreeSet, HashMap};

type SetNotesToStop = BTreeSet<String>;
struct StopI2SetQueue {
    // From Index 1/16th to Set of Notes.
    queue_i2set: HashMap<usize, SetNotesToStop>,
}
impl StopI2SetQueue {
    pub fn new() -> Self {
        Self {
            queue_i2set: HashMap::new(),
        }
    }
    pub fn add_notes_to_stop(&mut self, list: &Vec<String>, index_1_16th: usize) {
        // Param "index_1_16th" starts from 1.
        if let Some(set) = self.queue_i2set.get_mut(&index_1_16th) {
            for item in list {
                // TODO: Avoid cloning string
                let set_item = item.clone();
                set.insert(set_item);
            }
        } else {
            let mut set = SetNotesToStop::new();
            for item in list {
                // TODO: Avoid cloning Note string
                set.insert(item.clone());
            }
            self.queue_i2set.insert(index_1_16th, set);
        }
    }
    pub fn dequeue_notes_at_this_1_16th(&mut self, index_1_16th: usize) -> Option<Vec<String>> {
        // Param "index_1_16th" starts from 1.
        if let Some(set) = self.queue_i2set.remove(&index_1_16th) {
            let mut result = Vec::new();
            for item in set {
                result.push(item);
            }
            Some(result)
        } else {
            None
        }
    }
}

// For multiple Notes.
pub struct StopQueueNotes {
    inner: StopI2SetQueue,
    log: bool,
}
impl StopQueueNotes {
    pub fn new(log: bool) -> Self {
        Self {
            inner: StopI2SetQueue::new(),
            log,
        }
    }
    pub fn enqueue(&mut self, notes: &Vec<Note>, index_1_16th: usize) {
        if self.log {
            println!("StopQueueNotes.enq {}: {:?}", index_1_16th, notes);
        }
        let mut list = Vec::new();
        for note in notes {
            let item = note.to_hash();
            list.push(item);
        }
        self.inner.add_notes_to_stop(&list, index_1_16th);
    }
    pub fn dequeue_at(&mut self, index_1_16th: usize) -> Option<Vec<Note>> {
        if let Some(list) = self.inner.dequeue_notes_at_this_1_16th(index_1_16th) {
            let mut result = Vec::new();
            for item in list {
                let result_item = Note::from_hash(item);
                result.push(result_item);
            }
            if self.log {
                println!("StopQueueNotes.deq {}: {:?}", index_1_16th, result);
            }
            Some(result)
        } else {
            None
        }
    }
}

// For single Note.
pub struct StopQueueNote {
    inner: StopQueueNotes,
    log: bool,
}
impl StopQueueNote {
    pub fn new(log: bool) -> Self {
        Self {
            inner: StopQueueNotes::new(false),
            log,
        }
    }
    pub fn enqueue(&mut self, note: &Note, index_1_16th: usize) {
        if self.log {
            println!("StopQueueNote.enq {}:       {:?}", index_1_16th, note);
        }
        // TODO: Avoid cloning Note
        let list = vec![note.clone()];
        self.inner.enqueue(&list, index_1_16th);
    }
    pub fn dequeue_at(&mut self, index_1_16th: usize) -> Option<Vec<Note>> {
        let result = self.inner.dequeue_at(index_1_16th);
        if self.log {
            println!("StopQueueNote.deq {}: {:?}", index_1_16th, result);
        }
        result
    }
}

// For multiple Drum Sounds.
pub struct StopQueueDrumSounds {
    inner: StopI2SetQueue,
    log: bool,
}
impl StopQueueDrumSounds {
    pub fn new(log: bool) -> Self {
        Self {
            inner: StopI2SetQueue::new(),
            log,
        }
    }
    pub fn enqueue(&mut self, sounds: &Vec<DrumSound>, index_1_16th: usize) {
        if self.log {
            println!("StopQueueDrumSounds.enq {}: {:?}", index_1_16th, sounds);
        }
        let mut list = Vec::new();
        for sound in sounds {
            let item = sound.to_string();
            list.push(item);
        }
        self.inner.add_notes_to_stop(&list, index_1_16th);
    }
    pub fn dequeue_at(&mut self, index_1_16th: usize) -> Option<Vec<DrumSound>> {
        if let Some(list) = self.inner.dequeue_notes_at_this_1_16th(index_1_16th) {
            let mut result = Vec::new();
            for item in list {
                let result_item = DrumSound::from_string(&item);
                if let Some(result_item) = result_item {
                    result.push(result_item);
                } else {
                    // Should never happen.
                    println!("StopQueueDrumSounds: found unknown DrumSound in queue");
                }
            }
            if self.log {
                println!("StopQueueDrumSounds.deq {}: {:?}", index_1_16th, result);
            }
            Some(result)
        } else {
            None
        }
    }
}
