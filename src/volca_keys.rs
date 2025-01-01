use crate::midi_device::MidiDevice;

pub struct VolcaKeys {
    pub device: Box<dyn MidiDevice>,
}
impl VolcaKeys {
    pub fn new(device: impl MidiDevice + 'static) -> Self {
        Self {
            device: Box::new(device),
        }
    }

    // HIGH LEVEL
    pub fn note_play_start(&mut self, note_str: String) {
        // TODO: Start playing note "note"
    }

    // LOW LEVEL
    pub fn send_plain_message(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.device.send(a, b, c);
    }
}
