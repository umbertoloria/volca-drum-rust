use crate::players::player::TempoSnapshot;
use crate::song::song::Song;
use crate::utils::timing::{get_song_instant_list_from_song_and_start_from_millis, SongInstant};

pub struct PlayerCursor {
    tempo_snapshot: TempoSnapshot,
    song: Song,
    start_from_millis: u128,
    song_instants: Vec<SongInstant>,
    i_next_song_instant: usize,
}
impl PlayerCursor {
    pub fn new(song: Song, start_from_millis: u128) -> Self {
        let song_instants =
            get_song_instant_list_from_song_and_start_from_millis(&song, start_from_millis);
        Self {
            tempo_snapshot: TempoSnapshot {
                cur_bar: 1,
                cur_quarter: 1,
                cur_1_8: 1,
                cur_1_16: 1,
                section_bar_first: 0,
                section_bar_last: 0,
            },
            song,
            start_from_millis,
            song_instants,
            i_next_song_instant: 0,
        }
    }
    pub fn has_next_song_instant(&self) -> bool {
        self.i_next_song_instant < self.song_instants.len()
    }
    pub fn want_and_get_next_tempo_snapshot(&mut self) -> TempoSnapshot {
        // Assuming "self.has_next_song_instant()" is *TRUE*.
        let song_instant = &self.song_instants[self.i_next_song_instant];
        self.i_next_song_instant += 1;

        // Waiting for BPM sync
        song_instant.wait_for_this_moment_to_arrive();

        // Tempo Signature update
        // TODO: Avoid cloning Song Section
        let section = self.song.sections[song_instant.i_section].clone();
        if song_instant.is_first_of_section() {
            self.starts_new_section_with_many_bars(section.bars);
        }

        let tempo_snapshot = self.get_tempo_snapshot();

        tempo_snapshot
    }
    pub fn starts_new_section_with_many_bars(&mut self, bars_count: usize) {
        self.tempo_snapshot.section_bar_first = self.tempo_snapshot.cur_bar;
        self.tempo_snapshot.section_bar_last =
            self.tempo_snapshot.section_bar_first + bars_count - 1;
    }
    pub fn prepare_next_1_16th(&mut self) {
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
