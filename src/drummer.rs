use crate::player::{PlayerObserver, TempoSnapshot};
use crate::song::{DrumPattern, Song, SongSection};
use crate::volca_drum::VolcaDrum;

pub struct Drummer {
    song: Song,
    // Charts
    pattern: Option<DrumPattern>,
    // Outputs
    volca_drum: VolcaDrum,
    // Internal Player
    curr_section_index: usize,
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
    fn get_current_song_section(&self) -> Option<&SongSection> {
        if self.curr_section_index < self.song.sections.len() {
            Some(&self.song.sections[self.curr_section_index])
        } else {
            None
        }
    }
    fn update_pattern_from_song_section(&mut self) {
        self.pattern = match self.get_current_song_section() {
            Some(current_song_section) => {
                match &current_song_section.drum_pattern_key {
                    Some(drum_pattern_key) => {
                        // TODO: Avoid cloning pattern key
                        let drum_pattern = self
                            .song
                            .get_drum_pattern_clone_from_key(drum_pattern_key.into())
                            .expect("Unable to find right Drum Pattern");
                        Some(drum_pattern)
                    }
                    None => None,
                }
            }
            None => None,
        };
    }
}
impl PlayerObserver for Drummer {
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
    fn teach_song(&mut self, song: Song) {
        self.song = song;
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
        if tempo_snapshot.cur_bar == tempo_snapshot.section_bar_last
            && tempo_snapshot.cur_quarter == self.song.tempo.time_signature.0
            && tempo_snapshot.cur_1_16 == 4
        {
            // Assuming this is the last hit
            self.curr_section_index += 1;
            self.update_pattern_from_song_section();
        }
    }
}
