use crate::player::{PlayerObserver, TempoSnapshot};
use crate::song::DrumPattern;
use crate::volca_drum::VolcaDrum;

pub struct Drummer {
    pattern: Option<DrumPattern>,
    volca_drum: VolcaDrum,
}
impl Drummer {
    pub fn new(volca_drum: VolcaDrum) -> Self {
        Self {
            pattern: None,
            volca_drum,
        }
    }

    pub fn set_pattern(&mut self, pattern: Option<DrumPattern>) {
        self.pattern = pattern;
    }
}
impl PlayerObserver for Drummer {
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            format!("part \"{}\"", pattern.key)
        } else {
            "no drums".to_string()
        }
    }

    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot) {
        if let Some(pattern) = &self.pattern {
            let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_bar_from_1() - 1;

            let hh_symbol = pattern.hh.get(index_1_16th..=index_1_16th).unwrap();
            let sn_symbol = pattern.sn.get(index_1_16th..=index_1_16th).unwrap();
            let kk_symbol = pattern.kk.get(index_1_16th..=index_1_16th).unwrap();

            if hh_symbol != " " {
                self.volca_drum.hit_hh();
            }
            if kk_symbol != " " {
                self.volca_drum.hit_kick();
            }
            if sn_symbol != " " {
                self.volca_drum.hit_snare();
            }
        }
    }
}
