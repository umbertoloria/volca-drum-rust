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
