use crate::players::conductor::{BPM_DEFAULT, DUR_1_16};
use crate::song::song::{Song, SongSection};
use crate::utils::timing::wait_until_millis;

// REALTIME PLAYER
pub struct RealtimePlayer {
    tempo_snapshot: TempoSnapshot,
    start_from_millis: u128,
    song_instants: Vec<SongInstant>,
    i_next_song_instant: usize,
}
impl RealtimePlayer {
    fn new(song: &Song, start_from_millis: u128) -> Self {
        let song_instants =
            create_song_instant_list_from_song_and_start_from_millis(song, start_from_millis);

        // println!("{}: starts at {}", instrument_name, start_from_millis);

        Self {
            tempo_snapshot: TempoSnapshot {
                cur_bar: 1,
                cur_quarter: 1,
                cur_1_8: 1,
                cur_1_16: 1,
                section_bar_first: 0,
                section_bar_last: 0,
            },
            start_from_millis,
            song_instants,
            i_next_song_instant: 0,
        }
    }
    pub fn has_next_song_instant(&self) -> bool {
        self.i_next_song_instant < self.song_instants.len()
    }
    pub fn want_and_get_next_tempo_snapshot(&mut self) -> &TempoSnapshot {
        // Assuming "self.has_next_song_instant()" is *TRUE*.
        let song_instant = &self.song_instants[self.i_next_song_instant];
        self.i_next_song_instant += 1;

        // Waiting for BPM sync
        song_instant.wait_for_this_moment_to_arrive();

        // Tempo Signature update
        if song_instant.is_first_of_section() {
            let bars_count = song_instant.num_bars_in_current_section;
            self.tempo_snapshot.section_bar_first = self.tempo_snapshot.cur_bar;
            self.tempo_snapshot.section_bar_last =
                self.tempo_snapshot.section_bar_first + bars_count - 1;
        }

        let tempo_snapshot = &self.tempo_snapshot;

        /*
        let now = get_now_millis_sub_second();
        println!(
            "{}{}: hit {} at {}",
            " ".repeat(tempo_snapshot.cur_1_16),
            instrument_name,
            tempo_snapshot.cur_1_16,
            now
        );
        */

        tempo_snapshot
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
}

#[derive(Clone, Debug)]
pub struct SongInstant {
    pub i_section: usize,
    i_section_bar: usize,
    i_quarter: usize,
    i_quarter_1_16th: usize,
    num_bars_in_current_section: usize,
    millis: u128,
}
impl SongInstant {
    pub fn new(
        i_section: usize,
        i_section_bar: usize,
        i_quarter: usize,
        i_quarter_1_16th: usize,
        num_bars_in_current_section: usize,
        millis: u128,
    ) -> Self {
        Self {
            i_section,
            i_section_bar,
            i_quarter,
            i_quarter_1_16th,
            num_bars_in_current_section,
            millis,
        }
    }
    pub fn is_first_of_section(&self) -> bool {
        self.i_section_bar == 0 && self.i_quarter == 0 && self.i_quarter_1_16th == 0
    }
    pub fn wait_for_this_moment_to_arrive(&self) {
        wait_until_millis(self.millis);
    }
}
fn create_song_instant_list_from_song_and_start_from_millis(
    song: &Song,
    start_from_millis: u128,
) -> Vec<SongInstant> {
    // Assuming no BPM change during the song.
    let millis_1_16th = DUR_1_16
        .mul_f64(BPM_DEFAULT)
        .div_f64((&song.tempo).bpm as f64)
        .as_millis();

    let mut result = Vec::new();

    let mut next_instant_millis = start_from_millis;
    for i_section in 0..song.sections.len() {
        // Beginning of a new section.
        let section = &song.sections[i_section];

        for i_section_bar in 0..section.bars {
            // Beginning of a new bar.

            for i_quarter in 0..section.time_signature.top {
                // Beginning of a quarter.

                for i_quarter_1_16th in 0..4 {
                    // Beginning of a 1/16th.

                    let song_instant = SongInstant::new(
                        i_section,
                        i_section_bar,
                        i_quarter,
                        i_quarter_1_16th,
                        section.bars,
                        next_instant_millis,
                    );
                    result.push(song_instant);

                    next_instant_millis += millis_1_16th;
                }
            }
        }
    }
    /*
    for result_item in &result {
        println!("{:?}", result_item);
    }
    */

    result
}

pub fn create_realtime_player(song: &Song, start_from_millis: u128) -> RealtimePlayer {
    RealtimePlayer::new(song, start_from_millis)
}

// TEMPO SNAPSHOT
#[derive(Clone, Debug)]
pub struct TempoSnapshot {
    pub cur_bar: usize,
    pub cur_quarter: usize,
    pub cur_1_8: usize,
    pub cur_1_16: usize,
    pub section_bar_first: usize,
    pub section_bar_last: usize,
}
impl TempoSnapshot {
    pub fn get_tot_1_16ths_in_section(&self) -> usize {
        // Assuming 4/4 and four 1/16ths in 1/4th.
        self.get_tot_bars_in_section() * 16
    }
    pub fn get_cur_1_16ths_in_section_from_1(&self) -> usize {
        // From 1 to...
        (self.get_cur_bar_in_section() - 1) * 16 + self.get_cur_1_16ths_in_bar_from_1()
    }
    pub fn get_cur_1_16ths_in_bar_from_1(&self) -> usize {
        // From 1 to...
        (self.cur_quarter - 1) * 4 + self.cur_1_16
    }
    fn get_cur_bar_in_section(&self) -> usize {
        self.cur_bar - self.section_bar_first + 1
    }
    pub fn get_tot_bars_in_section(&self) -> usize {
        self.section_bar_last - self.section_bar_first + 1
    }
    pub fn is_this_the_last_1_16th_of_this_section(&self, curr_song_section: &SongSection) -> bool {
        // Assuming this is the last hit (what if there was a "6/8"?)
        self.cur_bar == self.section_bar_last
            && self.cur_quarter == curr_song_section.time_signature.top
            && self.cur_1_16 == 4
    }
    pub fn string_info(&self) -> String {
        format!(
            "{}th of {} bars in section / {}.{} / {}th global bar",
            self.get_cur_bar_in_section(),
            self.get_tot_bars_in_section(),
            self.cur_quarter,
            self.cur_1_16,
            self.cur_bar
        )
        .into()
    }
}
