use crate::sound_panel::{DRUM_CH_KICK, DRUM_CH_SNARE};
use midir::MidiOutputConnection;

pub struct Drummer {}
impl Drummer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn play_1_16th(
        &self,
        cur_1_4: usize,
        cur_1_16: usize,
        volca_drum: &mut MidiOutputConnection,
    ) {
        // TODO: Understand what's the "right" default note value.
        const DEFAULT_NOTE_VALUE: u8 = 7;

        // This drum pattern is hard-coded.
        if cur_1_16 == 1 {
            if cur_1_4 % 2 == 0 {
                self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_KICK, volca_drum);
            } else {
                self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_SNARE, volca_drum);
            }
        }
    }

    fn hit(&self, note: u8, instr: u8, volca_drum: &mut MidiOutputConnection) {
        const PROGRAM_CHANGE: u8 = 0xC0;
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x70;

        let _ = volca_drum.send(&[PROGRAM_CHANGE, instr]);

        let _ = volca_drum.send(&[NOTE_ON_MSG, note, VELOCITY]);

        // Are we sure that no wait is fine?
        // sleep(duration.mul_f64(BPM_DEFAULT).div_f64(self.bpm));
        let _ = volca_drum.send(&[NOTE_OFF_MSG, note, VELOCITY]);
    }
}
