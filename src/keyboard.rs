use crate::player::{PlayerObserver, TempoSnapshot};
use crate::song::{KeyboardPattern, Song, SongSection};
use crate::volca_keys::VolcaKeys;

pub struct Keyboard {
    song: Song,
    // Charts
    pattern: Option<KeyboardPattern>,
    chord_index: usize,
    // Outputs
    volca_keys: VolcaKeys,
    // Internal Player
    curr_section_index: usize,
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
    fn get_current_song_section(&self) -> Option<&SongSection> {
        if self.curr_section_index < self.song.sections.len() {
            Some(&self.song.sections[self.curr_section_index])
        } else {
            None
        }
    }
    fn update_pattern_from_song_section(&mut self) {
        self.pattern = match &self.get_current_song_section() {
            Some(current_song_section) => {
                match &current_song_section.keyboard_pattern_key {
                    Some(keyboard_pattern_key) => {
                        // TODO: Avoid cloning pattern key
                        let keyboard_pattern = self
                            .song
                            .get_keyboard_pattern_clone_from_key(keyboard_pattern_key.into())
                            .expect("Unable to find right Keyboard Pattern");
                        Some(keyboard_pattern)
                    }
                    None => None,
                }
            }
            None => None,
        };
    }
    pub fn play_notes(&mut self, notes: &Vec<String>) {
        // TODO: Using notes and sending them via MIDI
        for note in notes {
            let note = note.clone();
            self.volca_keys.note_play_start(note);
        }
    }
}
impl PlayerObserver for Keyboard {
    fn get_instrument_name(&self) -> String {
        "Keyboard".into()
    }
    fn get_short_info(&self) -> String {
        // TODO: Avoid cloning pattern
        if let Some(pattern) = self.pattern.clone() {
            if self.chord_index < pattern.chords.len() {
                let mut chord = &pattern.chords[self.chord_index];
                return format!(
                    "part \"{}\" / {} chord",
                    pattern.key,
                    chord.clone().chord_name
                );
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
