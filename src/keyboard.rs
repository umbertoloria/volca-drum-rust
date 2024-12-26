use crate::song::KeyboardPattern;

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
                return chord.clone().chord_name;
            }
        }
        "".to_string()
    }

    pub fn play_1_16th(&mut self, cur_bar_in_section: usize, cur_1_4: usize, cur_1_16: usize) {
        if let Some(pattern) = &self.pattern {
            // From 1 to ...
            let index_1_16th =
                (cur_bar_in_section - 1) * 16 + (cur_1_4 - 1) * 4 + (cur_1_16 - 1) + 1;
            // TODO: Avoid cloning pattern
            // TODO: This is slow
            let mut i = 0;
            for chord in &pattern.chords {
                if chord.from_1_16th_incl <= index_1_16th && index_1_16th <= chord.to_1_16th_incl {
                    self.chord_index = i;
                    break;
                }
                i += 1;
            }
        }
    }
}
