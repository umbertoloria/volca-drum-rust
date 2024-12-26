use crate::cli::clear_terminal_screen;
use crate::drummer::Drummer;
use crate::keyboard::Keyboard;
use crate::song::{Song, SongSection};
use crate::volca_drum::VolcaDrum;
use std::thread::sleep;
use std::time::Duration;

// Durations
pub const DUR_1_4: Duration = Duration::from_millis(1000);
pub const DUR_1_8: Duration = Duration::from_millis(500);
pub const DUR_1_16: Duration = Duration::from_millis(250);
pub const DUR_1_32: Duration = Duration::from_millis(125);
pub const BPM_DEFAULT: f64 = 60.0;

pub fn play_song(song: Song, volca_drum: &mut VolcaDrum) {
    let mut player = Player::new(song.tempo.bpm);

    for section in &song.sections {
        // Beginning of a new section.

        if section.bars < 1 {
            continue;
        }
        player.set_new_window_section(section.bars);

        // Drum Pattern
        if section.drum_pattern_key.is_none() {
            player.drummer.set_pattern(None);
        } else {
            // TODO: Avoid cloning the drum pattern key
            let drum_pattern_key = section.drum_pattern_key.clone().unwrap();
            player
                .drummer
                .set_pattern(song.get_drum_pattern_clone_from_key(drum_pattern_key));
        }
        // Keyboard Pattern
        if section.keyboard_pattern_key.is_none() {
            player.keyboard.set_pattern(None);
        } else {
            // TODO: Avoid cloning the pattern key
            let pattern_key = section.keyboard_pattern_key.clone().unwrap();
            player
                .keyboard
                .set_pattern(song.get_keyboard_pattern_clone_from_key(pattern_key));
        }

        // Play section
        for _ in 0..section.bars {
            // Beginning of a new bar.
            for _ in 0..song.tempo.time_signature.0 {
                // Beginning of a quarter.
                for _ in 0..4 {
                    // Beginning of a 1/16th.
                    player.play_1_16th_now(section, volca_drum);
                    player.next_1_16th();
                }
            }
        }
    }
}

// Player
pub struct Player {
    // Compose-time.
    bpm: usize,
    // Assuming all 4/4 bars.
    // Assuming bpm ticks to 1/4.

    // Play-time.
    cur_bar: usize,
    cur_quarter: usize,
    cur_1_8: usize,
    cur_1_16: usize,
    section_bar_first: usize,
    section_bar_last: usize,

    // Musicians
    drummer: Drummer,
    keyboard: Keyboard,
}
impl Player {
    pub fn new(bpm: usize) -> Self {
        Self {
            bpm,
            cur_bar: 1,
            cur_quarter: 1,
            cur_1_8: 1,
            cur_1_16: 1,
            section_bar_first: 0,
            section_bar_last: 0,
            drummer: Drummer::new(),
            keyboard: Keyboard::new(),
        }
    }
    pub fn set_new_window_section(&mut self, bars_count: usize) {
        self.section_bar_first = self.cur_bar;
        self.section_bar_last = self.section_bar_first + bars_count - 1;
    }
    fn get_tempo_snapshot(&self) -> TempoSnapshot {
        let cur_bar_in_section = self.cur_bar - self.section_bar_first + 1;
        let tot_bars_in_section = self.section_bar_last - self.section_bar_first + 1;
        TempoSnapshot {
            cur_bar_in_section,
            tot_bars_in_section,
            cur_bar: self.cur_bar,
            cur_quarter: self.cur_quarter,
            cur_1_8: self.cur_1_8,
            cur_1_16: self.cur_1_16,
        }
    }

    pub fn play_1_16th_now(&mut self, section: &SongSection, volca_drum: &mut VolcaDrum) {
        let tempo_snapshot = self.get_tempo_snapshot();

        // Play music
        self.drummer.play_1_16th(&tempo_snapshot, volca_drum);
        self.keyboard.play_1_16th(&tempo_snapshot);

        // + Interactive screen
        clear_terminal_screen();
        // TODO: Maybe show song author & title here
        println!("  .:[ {} ]:.", section.kind);

        println!("  Now: {}", tempo_snapshot.string_info());
        println!("  Drummer: {}", self.drummer.get_short_info());
        println!("  Keyboard: {}", self.keyboard.get_short_info());

        let tot_bars_in_section = tempo_snapshot.tot_bars_in_section;
        let tot_1_16ths_in_section = tempo_snapshot.tot_1_16ths_in_section();
        let cur_1_16ths_in_section =
            tempo_snapshot.get_cur_1_16ths_in_section(self.section_bar_first);
        println!(
            "  {}",
            (1..=tot_bars_in_section) // Or: (self.section_bar_first..=self.section_bar_last)
                .map(|n| format!("{:16}", format!("{}th bar", n)))
                .collect::<String>()
        );
        println!("  {}", "1 . 2 . 3 . 4 . ".repeat(tot_bars_in_section));
        println!("  {}", "V   .   v   .   ".repeat(tot_bars_in_section));
        println!(
            "  {}*{}",
            "-".repeat(cur_1_16ths_in_section),
            " ".repeat(tot_1_16ths_in_section - cur_1_16ths_in_section - 1)
        );
        // - Interactive screen

        // Wait time
        let millis_1_16th = DUR_1_16.mul_f64(BPM_DEFAULT).div_f64(self.bpm as f64);
        sleep(millis_1_16th);
        // TODO: Metronome is not really precise due to processing slow-down
    }

    pub fn next_1_16th(&mut self) {
        self.cur_1_16 += 1;
        self.cur_1_8 = if self.cur_1_16 > 2 { 2 } else { 1 };
        if self.cur_1_16 > 4 {
            self.cur_1_16 = 1;
            self.cur_1_8 = 1;
            self.cur_quarter += 1;
        }
        if self.cur_quarter > 4 {
            self.cur_quarter = 1;
            self.cur_bar += 1;
        }
    }
}

// Tempo Snapshot
pub struct TempoSnapshot {
    pub cur_bar_in_section: usize,
    pub tot_bars_in_section: usize,
    pub cur_bar: usize,
    pub cur_quarter: usize,
    pub cur_1_8: usize,
    pub cur_1_16: usize,
}
impl TempoSnapshot {
    pub fn get_tot_1_4s_in_section(&self) -> usize {
        self.tot_bars_in_section * 4
    }
    pub fn tot_1_16ths_in_section(&self) -> usize {
        self.get_tot_1_4s_in_section() * 4
    }
    pub fn get_cur_1_16ths_in_section(&self, section_bar_first: usize) -> usize {
        (self.cur_bar - section_bar_first) * 16 + (self.cur_quarter - 1) * 4 + (self.cur_1_16 - 1)
    }
    pub fn get_cur_1_16th_in_the_whole_bar_from_1(&self) -> usize {
        // From 1 to...
        (self.cur_quarter - 1) * 4 + (self.cur_1_16 - 1) + 1
    }
    pub fn get_cur_1_16th_in_the_whole_section_from_1(&self) -> usize {
        (self.cur_bar_in_section - 1) * 16 + self.get_cur_1_16th_in_the_whole_bar_from_1()
    }
    pub fn string_info(&self) -> String {
        format!(
            "{}th of {} bars in section / {}.{} / {}th global bar",
            self.cur_bar_in_section,
            self.tot_bars_in_section,
            self.cur_quarter,
            self.cur_1_16,
            self.cur_bar
        )
        .into()
    }
}
