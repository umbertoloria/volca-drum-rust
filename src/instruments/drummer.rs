use crate::devices::volca_drum::VolcaDrum;
use crate::instruments::abs::instrument::Instrument;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{DrumPattern, Song};

pub struct Drummer {
    inner_instrument_name_16_chars: String,

    // Charts
    curr_section_index: usize,
    pattern: Option<DrumPattern>,

    // Outputs
    volca_drum: VolcaDrum,
}
impl Drummer {
    pub fn new(inner_instrument_name_16_chars: String, volca_drum: VolcaDrum) -> Self {
        Self {
            inner_instrument_name_16_chars,
            curr_section_index: 0,
            pattern: None,
            volca_drum,
        }
    }
    fn update_pattern_from_song_section(&mut self, song: &Song) {
        if self.curr_section_index < song.sections.len() {
            let current_song_section = &song.sections[self.curr_section_index];
            self.pattern = match &current_song_section.drum_pattern_key {
                Some(drum_pattern_key) => {
                    let drum_pattern = song
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
    fn play_1_16th(&mut self, song: &Song, tempo_snapshot: &TempoSnapshot) {
        if let Some(pattern) = &self.pattern {
            let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_bar_from_1() - 1;

            let hh_symbol = pattern.hh.get(index_1_16th..=index_1_16th).unwrap();
            let sn_symbol = pattern.sn.get(index_1_16th..=index_1_16th).unwrap();
            let kk_symbol = pattern.kk.get(index_1_16th..=index_1_16th).unwrap();

            // TODO: Let Volca Drum commands happen via something abstract
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
        if tempo_snapshot.is_this_the_last_1_16th_of_this_section(&song) {
            self.curr_section_index += 1;
            self.update_pattern_from_song_section(&song);
        }
    }
}
impl Instrument for Drummer {
    fn get_instrument_name_16_chars(&self) -> String {
        // TODO: Avoid cloning Instrument Name
        self.inner_instrument_name_16_chars.clone()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            format!("part \"{}\"", pattern.key)
        } else {
            "no drums".to_string()
        }
    }
    fn play_song(&mut self, song: Song, start_from_millis: u128) {
        // TODO: Duplicated code (*hjk)
        // Start from beginning.
        self.curr_section_index = 0;
        self.update_pattern_from_song_section(&song);

        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let tempo_snapshot = realtime_player.want_and_get_next_tempo_snapshot();

            self.play_1_16th(&song, tempo_snapshot);

            realtime_player.prepare_next_1_16th();
        }
    }
}
