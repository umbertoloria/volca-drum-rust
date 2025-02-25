use crate::players::player::{BPM_DEFAULT, DUR_1_16};
use crate::song::song::Song;
use std::slice::Iter;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_now_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (since_the_epoch.as_secs() as u128) * 1000
        + (since_the_epoch.subsec_nanos() as u128) / 1_000_000
}
pub fn get_now_millis_sub_second() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (since_the_epoch.subsec_nanos() as u128) / 1_000_000
}

pub fn wait_until_millis(next_hit_millis: u128) {
    let now_millis = get_now_millis();
    if next_hit_millis > now_millis {
        let wait_until_next_millis = next_hit_millis - now_millis;
        sleep(Duration::from_millis(wait_until_next_millis as u64));
    } else {
        // No sleep. It would have gone to overflow.
    }
}

// BPM Timing Sync Monitor
#[derive(Debug, Clone)]
pub struct SongInstant {
    pub i_section: usize,
    i_section_bar: usize,
    i_quarter: usize,
    i_quarter_1_16th: usize,
    millis: u128,
}
impl SongInstant {
    pub fn new(
        i_section: usize,
        i_section_bar: usize,
        i_quarter: usize,
        i_quarter_1_16th: usize,
        millis: u128,
    ) -> Self {
        Self {
            i_section,
            i_section_bar,
            i_quarter,
            i_quarter_1_16th,
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
pub fn get_song_instant_list_from_song_and_start_from_millis(
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

            for i_quarter in 0..song.tempo.time_signature.0 {
                // Beginning of a quarter.

                for i_quarter_1_16th in 0..4 {
                    // Beginning of a 1/16th.

                    let song_instant = SongInstant::new(
                        i_section,
                        i_section_bar,
                        i_quarter,
                        i_quarter_1_16th,
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
pub fn wait_around_bpm(moments_iter: &mut Iter<SongInstant>) {
    if let Some(next_hit) = moments_iter.next() {
        let next_hit_millis = next_hit.millis;
        wait_until_millis(next_hit_millis);
    } else {
        // This should be the last 1/16th.
    }
}
