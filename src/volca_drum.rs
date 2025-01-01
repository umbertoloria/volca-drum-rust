use crate::midi_device::MidiDevice;

pub const DRUM_CH_KICK: u8 = 0;
pub const DRUM_CH_HH: u8 = 1;
pub const DRUM_CH_SNARE: u8 = 2;
// TODO: Understand what's the "right" default note value.
const DEFAULT_NOTE_VALUE: u8 = 7;

pub struct VolcaDrum {
    pub device: Box<dyn MidiDevice>,
}
impl VolcaDrum {
    pub fn new(device: impl MidiDevice + 'static) -> Self {
        Self {
            device: Box::new(device),
        }
    }

    // HIGH LEVEL
    pub fn hit_hh(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_HH);
    }
    pub fn hit_kick(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_KICK);
    }
    pub fn hit_snare(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_SNARE);
    }
    pub fn hit(&mut self, note: u8, instr: u8) {
        // TODO: Improve if possible
        const PROGRAM_CHANGE: u8 = 0xC0;
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x70;

        self.send_plain_message(PROGRAM_CHANGE, instr, 0);
        // let _ = volca_drum.send(&[PROGRAM_CHANGE, instr]);

        self.send_plain_message(NOTE_ON_MSG, note, VELOCITY);

        // Are we sure that no wait is fine?
        // sleep(duration.mul_f64(BPM_DEFAULT).div_f64(self.bpm));
        self.send_plain_message(NOTE_OFF_MSG, note, VELOCITY);
    }

    // LOW LEVEL
    pub fn send_plain_message(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.device.send(a, b, c);
    }

    pub fn send_cc_message(&mut self, channel: u8, cc_number: u8, value: u8) {
        self.send_plain_message(
            // 1
            0xb0 | (channel & 0x0f),
            // 2
            cc_number & 0x7f,
            // 3
            value & 0x7f,
        );
    }
}
