use crate::players::realtime_player::TempoSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonTempoSnapshot {
    pub cur_bar: usize,
    pub cur_quarter: usize,
    pub cur_1_8: usize,
    pub cur_1_16: usize,
    pub section_bar_first: usize,
    pub section_bar_last: usize,
}
impl JsonTempoSnapshot {
    pub fn new(tempo_snapshot: &TempoSnapshot) -> Self {
        Self {
            cur_bar: tempo_snapshot.cur_bar,
            cur_quarter: tempo_snapshot.cur_quarter,
            cur_1_8: tempo_snapshot.cur_1_8,
            cur_1_16: tempo_snapshot.cur_1_16,
            section_bar_first: tempo_snapshot.section_bar_first,
            section_bar_last: tempo_snapshot.section_bar_last,
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
