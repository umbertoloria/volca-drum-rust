use crate::cli::clear_terminal_screen;
use crate::drummer::Drummer;
use crate::keyboard::Keyboard;
use crate::song::{Song, SongSection, SongTempo};
use std::thread::sleep;
use std::time::Duration;

// Durations
pub const DUR_1_4: Duration = Duration::from_millis(1000);
pub const DUR_1_8: Duration = Duration::from_millis(500);
pub const DUR_1_16: Duration = Duration::from_millis(250);
pub const DUR_1_32: Duration = Duration::from_millis(125);
pub const BPM_DEFAULT: f64 = 60.0;

pub trait PlayerObserver {
    fn get_short_info(&self) -> String;
    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot);
}

pub struct Player {
    enable_interactive_cli: bool,
    tempo_snapshot: TempoSnapshot,

    // Musicians
    drummer: Drummer,
    keyboard: Keyboard,
}
impl Player {
    pub fn new(enable_interactive_cli: bool, drummer: Drummer, keyboard: Keyboard) -> Self {
        Self {
            enable_interactive_cli,
            tempo_snapshot: TempoSnapshot {
                cur_bar: 1,
                cur_quarter: 1,
                cur_1_8: 1,
                cur_1_16: 1,
                section_bar_first: 0,
                section_bar_last: 0,
            },
            drummer,
            keyboard,
        }
    }

    pub fn play_song(&mut self, song: Song) {
        for section in &song.sections {
            // Beginning of a new section.

            if section.bars < 1 {
                continue;
            }
            self.starts_new_section_with_many_bars(section.bars);

            // Drum Pattern
            let drum_pattern_for_section = match &section.drum_pattern_key {
                Some(drum_pattern_key) => {
                    // TODO: Avoid cloning pattern key
                    let drum_pattern = song
                        .get_drum_pattern_clone_from_key(drum_pattern_key.into())
                        .expect("Unable to find right Drum Pattern");
                    Some(drum_pattern)
                }
                None => None,
            };
            self.drummer.set_pattern(drum_pattern_for_section);

            // Keyboard Pattern
            let keyboard_pattern_for_section = match &section.keyboard_pattern_key {
                Some(keyboard_pattern_key) => {
                    // TODO: Avoid cloning pattern key
                    let keyboard_pattern = song
                        .get_keyboard_pattern_clone_from_key(keyboard_pattern_key.into())
                        .expect("Unable to find right Keyboard Pattern");
                    Some(keyboard_pattern)
                }
                None => None,
            };
            self.keyboard.set_pattern(keyboard_pattern_for_section);

            // Play section
            for _ in 0..section.bars {
                // Beginning of a new bar.
                for _ in 0..song.tempo.time_signature.0 {
                    // Beginning of a quarter.
                    for _ in 0..4 {
                        // Beginning of a 1/16th.
                        self.play_1_16th_now(section, &song.tempo);
                        self.next_1_16th();
                    }
                }
            }
        }
    }

    pub fn starts_new_section_with_many_bars(&mut self, bars_count: usize) {
        self.tempo_snapshot.section_bar_first = self.tempo_snapshot.cur_bar;
        self.tempo_snapshot.section_bar_last =
            self.tempo_snapshot.section_bar_first + bars_count - 1;
    }

    pub fn play_1_16th_now(&mut self, section: &SongSection, song_tempo: &SongTempo) {
        let tempo_snapshot = &self.tempo_snapshot;

        // Play music
        self.drummer.play_1_16th(tempo_snapshot);
        self.keyboard.play_1_16th(tempo_snapshot);

        if self.enable_interactive_cli {
            // + Interactive screen
            clear_terminal_screen();
            // TODO: Maybe show song author & title here
            println!("  .:[ {} ]:.", section.kind);

            println!("  Now: {}", tempo_snapshot.string_info());
            println!("  Drummer: {}", self.drummer.get_short_info());
            println!("  Keyboard: {}", self.keyboard.get_short_info());

            let tot_bars_in_section = tempo_snapshot.get_tot_bars_in_section();
            let tot_1_16ths_in_section = tempo_snapshot.get_tot_1_16ths_in_section();
            let cur_1_16ths_in_section = tempo_snapshot.get_cur_1_16ths_in_section_from_1() - 1;
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
        }

        // Wait time
        let millis_1_16th = DUR_1_16.mul_f64(BPM_DEFAULT).div_f64(song_tempo.bpm as f64);
        sleep(millis_1_16th);
        // TODO: Metronome is not really precise due to processing slow-down
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
}

// Tempo Snapshot
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
