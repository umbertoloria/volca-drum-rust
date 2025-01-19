use crate::player::{PlayerObserver, TempoSnapshot};
use crate::song::{KeyboardPattern, Song};
use crate::volca_keys::VolcaKeys;

pub struct Keyboard {
    // Song
    song: Song,

    // Charts
    curr_section_index: usize,
    pattern: Option<KeyboardPattern>,
    chord_index: usize,

    // Outputs
    volca_keys: VolcaKeys,
}
impl Keyboard {
    pub fn new(song: Song, volca_keys: VolcaKeys) -> Self {
        Self {
            song,
            pattern: None,
            chord_index: 0,
            volca_keys,
            curr_section_index: 0,
        }
    }
    fn update_pattern_from_song_section(&mut self) {
        if self.curr_section_index < self.song.sections.len() {
            let current_song_section = &self.song.sections[self.curr_section_index];
            self.pattern = match &current_song_section.keyboard_pattern_key {
                Some(keyboard_pattern_key) => {
                    let keyboard_pattern = self
                        .song
                        .get_keyboard_pattern_from_key(keyboard_pattern_key.into())
                        .expect("Unable to find right Keyboard Pattern")
                        // TODO: Avoid cloning pattern
                        .clone();
                    Some(keyboard_pattern)
                }
                None => None,
            }
        } else {
            self.pattern = None;
        }
    }
    pub fn play_notes(&mut self, notes: &Vec<String>) {
        for note in notes {
            self.volca_keys.note_play_start(note);
        }
    }
}
impl PlayerObserver for Keyboard {
    fn get_instrument_name(&self) -> String {
        "Keyboard".into()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            if self.chord_index < pattern.chords.len() {
                let mut chord = &pattern.chords[self.chord_index];
                return format!("part \"{}\" / {} chord", pattern.key, chord.chord_name);
            }
        }
        "".to_string()
    }
    fn teach_song(&mut self, song: Song) {
        self.song = song;
        // Start from beginning.
        self.curr_section_index = 0;
        self.update_pattern_from_song_section();
    }
    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot) {
        if let Some(pattern) = &self.pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern = index_1_16th % (bars_covered_by_pattern * 16);

            // TODO: Avoid cloning pattern
            // TODO: This is slow
            let mut i = 0;
            for chord in &pattern.chords {
                if chord.from_1_16th_incl <= index_1_16th_for_pattern
                    && index_1_16th_for_pattern <= chord.to_1_16th_incl
                {
                    self.chord_index = i;
                    break;
                }
                i += 1;
            }

            if 0 <= self.chord_index && self.chord_index < pattern.chords.len() {
                let pattern = self.pattern.clone().unwrap();
                let chord = &pattern.chords[self.chord_index];
                self.play_notes(&chord.notes);
            }
        }

        // Preparing the next hit!
        if tempo_snapshot.is_this_the_last_1_16th_of_this_section(&self.song) {
            self.curr_section_index += 1;
            self.update_pattern_from_song_section();
        }
    }
}
