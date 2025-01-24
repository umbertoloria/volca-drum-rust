use crate::instrument::Instrument;
use crate::player::TempoSnapshot;
use crate::song::{DrumPattern, Song};
use crate::volca_drum::VolcaDrum;
use std::process::exit;

pub struct Drummer {
    // Song
    song: Song,

    // Charts
    curr_section_index: usize,
    pattern: Option<DrumPattern>,

    // Outputs
    volca_drum: VolcaDrum,
}
impl Drummer {
    pub fn new(song: Song, volca_drum: VolcaDrum) -> Self {
        Self {
            song,
            pattern: None,
            volca_drum,
            curr_section_index: 0,
        }
    }
    fn update_pattern_from_song_section(&mut self) {
        if self.curr_section_index < self.song.sections.len() {
            let current_song_section = &self.song.sections[self.curr_section_index];
            self.pattern = match &current_song_section.drum_pattern_key {
                Some(drum_pattern_key) => {
                    let drum_pattern = self
                        .song
                        .get_drum_pattern_from_key(drum_pattern_key.into())
                        .expect("Unable to find right Drum Pattern")
                        // TODO: Avoid cloning pattern
                        .clone();
                    Some(drum_pattern)
                }
                None => None,
            }
        } else {
            self.pattern = None;
        }
    }
}
impl Instrument for Drummer {
    fn get_instrument_name(&self) -> String {
        "Drummer".into()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            format!("part \"{}\"", pattern.key)
        } else {
            "no drums".to_string()
        }
    }
    fn teach_song(&mut self, song_id: String) {
        if self.song.id != song_id {
            println!("Drummer doesn't know the song");
            exit(0x0100);
        }
        // Start from beginning.
        self.curr_section_index = 0;
        self.update_pattern_from_song_section();
    }
    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot) {
        if let Some(pattern) = &self.pattern {
            let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_bar_from_1() - 1;

            let hh_symbol = pattern.hh.get(index_1_16th..=index_1_16th).unwrap();
            let sn_symbol = pattern.sn.get(index_1_16th..=index_1_16th).unwrap();
            let kk_symbol = pattern.kk.get(index_1_16th..=index_1_16th).unwrap();

            if hh_symbol != " " {
                self.volca_drum.hit_hh();
            }
            if kk_symbol != " " {
                self.volca_drum.hit_kick();
            }
            if sn_symbol != " " {
                self.volca_drum.hit_snare();
            }
        }

        // Preparing the next hit!
        if tempo_snapshot.is_this_the_last_1_16th_of_this_section(&self.song) {
            self.curr_section_index += 1;
            self.update_pattern_from_song_section();
        }
    }
}
