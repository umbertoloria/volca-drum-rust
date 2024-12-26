use crate::cli::clear_terminal_screen;
use crate::drummer::Drummer;
use crate::song::{Song, SongSection};
use midir::MidiOutputConnection;
use std::thread::sleep;
use std::time::Duration;

// Durations
pub const DUR_1_4: Duration = Duration::from_millis(1000);
pub const DUR_1_8: Duration = Duration::from_millis(500);
pub const DUR_1_16: Duration = Duration::from_millis(250);
pub const DUR_1_32: Duration = Duration::from_millis(125);
pub const BPM_DEFAULT: f64 = 60.0;

pub fn play_song(song: Song, volca_drum: &mut MidiOutputConnection) {
    println!(
        "Play song \"{}\" by \"{}\"",
        song.details.title, song.details.author
    );

    let mut player = Player::new(song.tempo.bpm);

    for section in &song.sections {
        // Print section info
        let mut section_notes = "";
        if let Some(x) = &section.notes {
            section_notes = x;
        }
        if section.bars < 1 {
            continue;
        }
        println!("New section: type {:6} -> {}", section.kind, section_notes);
        player.start_new_section(section.bars);

        // Assuming bars or 4/4. Assuming 1/4 is two 1/8s.
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
    println!("Song end.");
}

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
        }
    }
    pub fn start_new_section(&mut self, bars_count: usize) {
        self.section_bar_first = self.cur_bar;
        self.section_bar_last = self.section_bar_first + bars_count - 1;
    }
    pub fn play_1_16th_now(&self, section: &SongSection, volca_drum: &mut MidiOutputConnection) {
        // Play music
        self.drummer
            .play_1_16th(self.cur_quarter, self.cur_1_16, volca_drum);

        if self.section_bar_first <= self.cur_bar && self.cur_bar <= self.section_bar_last {
            let tot_bars_in_section = self.section_bar_last - self.section_bar_first + 1;
            let tot_1_4s_in_section = tot_bars_in_section * 4;
            let tot_1_16ths_in_section = tot_1_4s_in_section * 4;
            let cur_1_16ths_in_section = (self.cur_bar - self.section_bar_first) * 16
                + (self.cur_quarter - 1) * 4
                + (self.cur_1_16 - 1);

            clear_terminal_screen();
            println!("  .:[ {} ]:.", section.kind);
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
        }

        // Wait time
        let millis_1_16th = DUR_1_16.mul_f64(BPM_DEFAULT).div_f64(self.bpm as f64);
        sleep(millis_1_16th);
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
