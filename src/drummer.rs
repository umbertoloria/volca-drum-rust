use crate::player::TempoSnapshot;
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

    pub fn play_1_16th(&self, tempo_snapshot: &TempoSnapshot, volca_drum: &mut VolcaDrum) {
        if let Some(pattern) = &self.pattern {
            let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_bar_from_1() - 1;

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
