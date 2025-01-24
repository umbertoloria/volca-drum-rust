use crate::player::{BPM_DEFAULT, DUR_1_16};
use crate::song::Song;
use std::cmp::max;
use std::slice::Iter;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_moments_vec_from_song_start(song: &Song, first_moment_millis: u128) -> Vec<u128> {
    // Assuming no BPM change during the song.
    let millis_1_16th = DUR_1_16
        .mul_f64(BPM_DEFAULT)
        .div_f64((&song.tempo).bpm as f64)
        .as_millis();

    let mut curr_millis = first_moment_millis;
    let mut result = vec![];
    for section in &song.sections {
        for _ in 0..section.bars {
            for _ in 0..song.tempo.time_signature.0 {
                for _ in 0..4 {
                    // Beginning of a 1/16th.
                    result.push(curr_millis);
                    curr_millis += millis_1_16th;
                }
            }
        }
    }
    // println!("{:?}", result);

    result
}
pub fn get_now_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (since_the_epoch.as_secs() as u128) * 1000
        + (since_the_epoch.subsec_nanos() as u128) / 1_000_000
}

// BPM Timing Sync Monitor
pub fn wait_around_bpm(moments_iter: &mut Iter<u128>) {
    if let Some(next_hit_millis) = moments_iter.next() {
        let now_millis = get_now_millis();

        let wait_until_next_millis = max(0, next_hit_millis - now_millis);
        sleep(Duration::from_millis(wait_until_next_millis as u64));
    } else {
        // This should be the last 1/16th.
    }
}
