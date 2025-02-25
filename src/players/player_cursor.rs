use crate::players::player::TempoSnapshot;

pub struct PlayerCursor {
    tempo_snapshot: TempoSnapshot,
}
impl PlayerCursor {
    pub fn new() -> Self {
        Self {
            tempo_snapshot: TempoSnapshot {
                cur_bar: 1,
                cur_quarter: 1,
                cur_1_8: 1,
                cur_1_16: 1,
                section_bar_first: 0,
                section_bar_last: 0,
            },
        }
    }
    pub fn starts_new_section_with_many_bars(&mut self, bars_count: usize) {
        self.tempo_snapshot.section_bar_first = self.tempo_snapshot.cur_bar;
        self.tempo_snapshot.section_bar_last =
            self.tempo_snapshot.section_bar_first + bars_count - 1;
    }
    pub fn next_1_16th(&mut self) {
        self.tempo_snapshot.cur_1_16 += 1;
        self.tempo_snapshot.cur_1_8 = if self.tempo_snapshot.cur_1_16 > 2 {
            2
        } else {
            1
        };
        if self.tempo_snapshot.cur_1_16 > 4 {
            self.tempo_snapshot.cur_1_16 = 1;
            self.tempo_snapshot.cur_1_8 = 1;
            self.tempo_snapshot.cur_quarter += 1;
        }
        if self.tempo_snapshot.cur_quarter > 4 {
            self.tempo_snapshot.cur_quarter = 1;
            self.tempo_snapshot.cur_bar += 1;
        }
    }
    pub fn get_tempo_snapshot(&self) -> TempoSnapshot {
        // TODO: Avoid cloning Tempo Snapshot
        self.tempo_snapshot.clone()
    }
}
