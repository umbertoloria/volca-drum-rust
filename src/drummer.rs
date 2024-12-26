use crate::song::DrumPattern;
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
            format!("part \"{}\"", pattern.key)
        } else {
            "no drums".to_string()
        }
    }

    pub fn play_1_16th(&self, cur_1_4: usize, cur_1_16: usize, volca_drum: &mut VolcaDrum) {
        if let Some(pattern) = &self.pattern {
            // From 0 to 15 (it depends...)
            let index_1_16th = (cur_1_4 - 1) * 4 + (cur_1_16 - 1);

            let hh_symbol = pattern.hh.get(index_1_16th..=index_1_16th).unwrap();
            let sn_symbol = pattern.sn.get(index_1_16th..=index_1_16th).unwrap();
            let kk_symbol = pattern.kk.get(index_1_16th..=index_1_16th).unwrap();

            if hh_symbol != " " {
                volca_drum.hit_hh();
            }
            if kk_symbol != " " {
                volca_drum.hit_kick();
            }
            if sn_symbol != " " {
                volca_drum.hit_snare();
            }
        }
    }
}
