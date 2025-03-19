use crate::instruments::lib::abstract_instruments::AbstractInstrumentPolyDrumSounds;
use crate::instruments::lib::drum_sounds::DrumSound;
use crate::instruments::lib::stop_notes_queue::StopQueueDrumSounds;

pub struct InstrumentWithQueuedDrumSounds {
    queue: StopQueueDrumSounds,
    instrument: Box<dyn AbstractInstrumentPolyDrumSounds>,
}
impl InstrumentWithQueuedDrumSounds {
    pub fn new(
        //
        instrument: Box<dyn AbstractInstrumentPolyDrumSounds>,
    ) -> Self {
        Self {
            queue: StopQueueDrumSounds::new(),
            instrument,
        }
    }
    pub fn attack_notes(&mut self, notes: &Vec<DrumSound>) {
        self.instrument.play_sounds_start(&notes);
    }
    pub fn notify_release_notes_at(&mut self, notes: &Vec<DrumSound>, index_1_16th: usize) {
        // FIXME: Use MOD with "index_1_16th" otherwise you never stop after number 16 or similar
        self.queue.enqueue(&notes, index_1_16th);
    }
    pub fn stop_notes_queued_on_this_1_16th(&mut self, index_1_16th: usize) {
        let notes_to_stop = self.queue.dequeue_at(index_1_16th);
        if let Some(notes_to_stop) = notes_to_stop {
            self.instrument.play_sounds_stop(&notes_to_stop);
        }
    }
}
