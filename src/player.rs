use crate::drummer::DUR_1_4;
use std::thread::sleep;

pub const BPM_DEFAULT: f64 = 60.0;

pub struct Player {
    bpm: usize,
    // Assuming all 4/4 bars.
    // Assuming bpm ticks to 1/4.
}

impl Player {
    pub fn new(bpm: usize) -> Self {
        Self { bpm }
    }

    // Playing bars
    pub fn play_one_bar(&self) {
        let sleep_for_millis = DUR_1_4
            .mul_f64(BPM_DEFAULT)
            .div_f64(self.bpm as f64) // One quarter.
            .mul_f64(4.0);
        sleep(sleep_for_millis);
    }
    pub fn play_num_bars(&self, num_bars: usize) {
        for _ in 0..num_bars {
            self.play_one_bar();
        }
    }
}
