use crate::instruments::instrument::Instrument;
use crate::player::TempoSnapshot;
use crate::song::song::Song;
use crate::VolcaKeys;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

pub struct Metronome {
    // Song
    song: Song,

    // Outputs
    volca_keys: VolcaKeys,
}
impl Metronome {
    pub fn new(song: Song, volca_keys: VolcaKeys) -> Self {
        Self { song, volca_keys }
    }
    fn play_note(&mut self, note: String) {
        // println!("play_note {}", note.clone());
        self.volca_keys.note_play_start(note.clone());

        let duration = Duration::new(0, 500_000_000);
        sleep(duration);

        self.volca_keys.note_play_stop(note);
    }
}
impl Instrument for Metronome {
    fn get_instrument_name(&self) -> String {
        "Metronome".into()
    }
    fn get_short_info(&self) -> String {
        "Metronome".to_string()
    }
    fn teach_song(&mut self, song_id: String) {
        if self.song.id != song_id {
            println!("Metronome doesn't know the song");
            exit(0x0100);
        }
    }
    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot) {
        let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_bar_from_1();
        // Assuming every 1/4th has 4 1/16ths.
        let offset_1_16th = (index_1_16th - 1) % 4;
        if offset_1_16th == 0 {
            // New 1/4th started. Hit the metronome!
            let metronome_note = "A4".to_string();
            self.play_note(metronome_note);
        }
    }
}
