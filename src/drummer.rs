use crate::player::{PlayerObserver, TempoSnapshot};
use crate::song::{DrumPattern, Song, SongSection};
use crate::volca_drum::VolcaDrum;

pub struct Drummer {
    song: Option<Song>,
    pattern: Option<DrumPattern>,
    volca_drum: VolcaDrum,
}
impl Drummer {
    pub fn new(volca_drum: VolcaDrum) -> Self {
        Self {
            song: None,
            pattern: None,
            volca_drum,
        }
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
        let song = song.clone();
        self.song = Some(song);
    }
    fn set_pattern_from_song_section(&mut self, song: &Song, section: &SongSection) {
        self.pattern = match &section.drum_pattern_key {
            Some(drum_pattern_key) => {
                // TODO: Avoid cloning pattern key
                let drum_pattern = song
                    .get_drum_pattern_clone_from_key(drum_pattern_key.into())
                    .expect("Unable to find right Drum Pattern");
                Some(drum_pattern)
            }
            None => None,
        };
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
    }
}
