use crate::instruments::instr_comm::InstrumentBroadcastComm;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
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
    instrument_broadcast_comm: InstrumentBroadcastComm,
    enable_interactive_cli: bool,
}
impl Conductor {
    pub fn new(
        instrument_broadcast_comm: InstrumentBroadcastComm,
        enable_interactive_cli: bool,
    ) -> Self {
        Self {
            instrument_broadcast_comm,
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
        self.instrument_broadcast_comm
            .play_song(song.id.clone(), start_from_millis);

        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let tempo_snapshot = realtime_player.want_and_get_next_tempo_snapshot();

            self.play_1_16th_now(tempo_snapshot);

            realtime_player.prepare_next_1_16th();
        }

        // TODO: Restore Interactive CLI feature

        self.instrument_broadcast_comm.shutdown();

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

pub fn clear_terminal_screen() {
    print!("\x1Bc");
}
