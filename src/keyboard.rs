use crate::player::TempoSnapshot;
use crate::song::KeyboardPattern;
use crate::volca_keys::VolcaKeys;

pub struct Keyboard {
    pattern: Option<KeyboardPattern>,
    chord_index: usize,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pattern: None,
            chord_index: 0,
        }
    }

    pub fn set_pattern(&mut self, pattern: Option<KeyboardPattern>) {
        self.pattern = pattern;
    }

    pub fn get_short_info(&self) -> String {
        if let Some(pattern) = self.pattern.clone() {
            if self.chord_index <= pattern.chords.len() {
                let mut chord = pattern.chords.get(self.chord_index).unwrap();
                // TODO: Avoid cloning pattern
                return format!("{} chord", chord.clone().chord_name);
            }
        }
        "".to_string()
    }

    pub fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot, volca_keys: &mut VolcaKeys) {
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
                let chord = pattern.chords.get(self.chord_index).unwrap().clone();
                let notes = chord.notes;
                self.play_notes(notes, volca_keys);
            }
        }
    }

    pub fn play_notes(&self, notes: Vec<String>, volca_keys: &mut VolcaKeys) {
        // TODO: Using notes and sending them via MIDI
        for note in &notes {
            let note = note.clone();
            volca_keys.note_play_start(note);
        }
    }
}
