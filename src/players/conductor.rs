use crate::instruments::instr_comm::InstrComm;
use crate::players::cli::clear_terminal_screen;
use crate::players::realtime_player::create_realtime_player;
use crate::song::song::Song;
use crate::utils::timing::get_now_millis;
use std::time::Duration;

// Durations
pub const DUR_1_4: Duration = Duration::from_millis(1000);
pub const DUR_1_8: Duration = Duration::from_millis(500);
pub const DUR_1_16: Duration = Duration::from_millis(250);
pub const DUR_1_32: Duration = Duration::from_millis(125);
pub const BPM_DEFAULT: f64 = 60.0;

pub struct Conductor {
    instr_comm: InstrComm,
    enable_interactive_cli: bool,
}
impl Conductor {
    pub fn new(instr_comm: InstrComm, enable_interactive_cli: bool) -> Self {
        Self {
            instr_comm,
            enable_interactive_cli,
        }
    }

    pub fn play_song(&mut self, song: Song) -> Result<(), String> {
        if song.sections.len() == 0 {
            return Err("Song has no sections".into());
        }

        // Play song!
        let global_delay = 1_000; // Wait one second.
        let start_from_millis = get_now_millis() + global_delay;
        // TODO: Avoid cloning Song ID
        self.instr_comm
            .play_song(song.id.clone(), start_from_millis);

        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let tempo_snapshot = realtime_player.want_and_get_next_tempo_snapshot();

            self.play_1_16th_now(tempo_snapshot);

            realtime_player.prepare_next_1_16th();
        }

        // TODO: Restore Interactive CLI feature

        self.instr_comm.shutdown();

        Ok(())
    }

    pub fn play_1_16th_now(&mut self, tempo_snapshot: &TempoSnapshot) {
        // Interactive CLI
        if self.enable_interactive_cli {
            clear_terminal_screen();
            // TODO: Maybe show song author & title here
            // println!("  .:[ {} ]:.", section.kind);
            println!("  .:[ {} ]:.", "Section Kind");

            println!("  Now: {}", tempo_snapshot.string_info());
            // TODO: Print info about what all instruments are playing...
            /*
            for instrument in &mut self.instruments {
                let instrument_name = instrument.get_instrument_name();
                let short_info = instrument.get_short_info();
                println!("  {}: {}", instrument_name, short_info);
            }
            */

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
        }
    }
}

// Tempo Snapshot
#[derive(Debug, Clone)]
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
    pub fn is_this_the_last_1_16th_of_this_section(&self, song: &Song) -> bool {
        // Assuming this is the last hit (what if there was a "6/8"?)
        self.cur_bar == self.section_bar_last
            && self.cur_quarter == song.tempo.time_signature.0
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
