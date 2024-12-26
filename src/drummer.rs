use crate::song::DrumPattern;
use crate::sound_panel::{DRUM_CH_HH, DRUM_CH_KICK, DRUM_CH_SNARE};
use crate::volca_drum::VolcaDrum;

pub struct Drummer {
    pattern: Option<DrumPattern>,
}

impl Drummer {
    pub fn new() -> Self {
        Self { pattern: None }
    }

    pub fn set_pattern(&mut self, pattern: Option<DrumPattern>) {
        self.pattern = pattern;
    }

    pub fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            format!("{}", pattern.key)
        } else {
            // Like when I write "no drums" or "φ".
            "".to_string()
        }
    }

    pub fn play_1_16th(&self, cur_1_4: usize, cur_1_16: usize, volca_drum: &mut VolcaDrum) {
        if let Some(pattern) = &self.pattern {
            let index = (cur_1_4 - 1) * 4 + (cur_1_16 - 1);

            let hh_symbol = pattern.hh.get(index..=index).unwrap();
            let sn_symbol = pattern.sn.get(index..=index).unwrap();
            let kk_symbol = pattern.kk.get(index..=index).unwrap();

            // TODO: Understand what's the "right" default note value.
            const DEFAULT_NOTE_VALUE: u8 = 7;
            if hh_symbol != " " {
                self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_HH, volca_drum);
            }
            if kk_symbol != " " {
                self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_KICK, volca_drum);
            }
            if sn_symbol != " " {
                self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_SNARE, volca_drum);
            }
        }
    }

    fn hit(&self, note: u8, instr: u8, volca_drum: &mut VolcaDrum) {
        const PROGRAM_CHANGE: u8 = 0xC0;
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x70;

        volca_drum.send_plain_message(PROGRAM_CHANGE, instr, 0);
        // let _ = volca_drum.send(&[PROGRAM_CHANGE, instr]);

        volca_drum.send_plain_message(NOTE_ON_MSG, note, VELOCITY);

        // Are we sure that no wait is fine?
        // sleep(duration.mul_f64(BPM_DEFAULT).div_f64(self.bpm));
        volca_drum.send_plain_message(NOTE_OFF_MSG, note, VELOCITY);
    }
}
