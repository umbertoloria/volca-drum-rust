use crate::music::note::Note;
use crate::song::composer::random_chord_progression_composer::RandomChordProgressionComposer;
use crate::song::song::{
    get_standard_click_note, KeyboardPattern, KeyboardPatternChord, Song, SongDetails,
    SongMetronomeData, SongMetronomeDataClickOn, SongSection, SongSectionKind, SongTempo,
};
use std::collections::HashMap;
/*pub enum TonalityNote {
    C,
    Cs, // Or Db, for now are the same...
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}
pub enum TonalityMode {
    Major,
    Minor,
}*/
pub struct Composer {
    pub bpm: usize,
    pub click: bool,
    // pub tonality_note: TonalityNote,
    // pub tonality_mode: TonalityMode,
    pub tonality_note: Note,
    pub composer_mode: ComposerMode,
    pub num_keyboard_patterns: usize,
    pub song: Song,
}
impl Composer {
    pub fn new(
        //
        bpm: usize,
        click: bool,
        tonality_note: Note,
        composer_mode: ComposerMode,
    ) -> Self {
        let song = Song {
            id: "composed-song".into(),
            details: SongDetails {
                author: "Smart composer".into(),
                title: "Smart song".into(),
            },
            tempo: SongTempo {
                bpm,
                time_signature: (4, 4),
            },
            metronome_data: if click {
                Some(SongMetronomeData {
                    click_on: SongMetronomeDataClickOn::OnEvery1_4ths,
                    note: get_standard_click_note(),
                })
            } else {
                None
            },
            drum_patterns: HashMap::new(),
            keyboard_patterns: HashMap::new(),
            bass_patterns: HashMap::new(),
            sections: Vec::new(),
        };
        Self {
            bpm,
            click,
            tonality_note,
            composer_mode,
            num_keyboard_patterns: 0,
            song,
        }
    }
    pub fn compose_new_song(&mut self, num_sections: usize) {
        for _ in 0..num_sections {
            self.add_new_random_section(8);
        }
    }
    fn add_new_random_section(&mut self, bars: usize) {
        // Generate Chords
        let mut chords = Vec::new();
        // First Chord is on the Tonic.
        let first_degree = 1;
        let chord = ComposerChord::new(&self.composer_mode, &self.tonality_note, first_degree);
        chords.push(chord);

        let mut random_composer = RandomChordProgressionComposer::new(first_degree);
        for _ in 1..bars {
            let rand_degree = random_composer.generate_new_degree();
            let chord = ComposerChord::new(&self.composer_mode, &self.tonality_note, rand_degree);
            chords.push(chord);
        }

        // Keyboard Pattern
        let keyboard_pattern_key = self.add_keyboard_pattern_with_chords(chords);

        // Song Section
        self.song.sections.push(
            //
            SongSection {
                kind: SongSectionKind::Verse,
                bars,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some(keyboard_pattern_key),
                bass_pattern_key: None,
                notes: None,
            },
        );
    }
    fn add_keyboard_pattern_with_chords(&mut self, chords: Vec<ComposerChord>) -> String {
        let new_keyboard_pattern_key = format!("K{}", self.num_keyboard_patterns);
        self.num_keyboard_patterns = self.num_keyboard_patterns + 1;

        let mut keyboard_chords = Vec::new();
        let mut last_index_1_16th_offset = 0;
        for chord in chords {
            keyboard_chords.push(
                //
                KeyboardPatternChord {
                    //
                    chord_name: chord.chord_name.into(),
                    notes: chord.notes,
                    from_1_16th_incl: last_index_1_16th_offset + 1,
                    to_1_16th_incl: last_index_1_16th_offset + 16,
                },
            );
            last_index_1_16th_offset += 16;
        }

        self.song.keyboard_patterns.insert(
            new_keyboard_pattern_key.clone(),
            KeyboardPattern {
                key: new_keyboard_pattern_key.clone(),
                chords: keyboard_chords,
            },
        );

        new_keyboard_pattern_key
    }
    pub fn get_song(self) -> Song {
        // println!("{:?}", &self.song);
        self.song
    }
}

#[derive(Clone, Debug)]
pub struct ComposerChord {
    pub chord_name: String,
    pub notes: Vec<Note>,
}
impl ComposerChord {
    pub fn new(
        //
        composer_mode: &ComposerMode,
        base_note: &Note,
        degree: usize, // From 1 to 7.
    ) -> Self {
        let offsets = get_scale_offsets_by_mode(composer_mode);

        let root_note = Note {
            octave: base_note.octave,
            offset: (base_note.offset + offsets[degree - 1]) % NUM_NOTES_12,
        };
        let third_note = Note {
            octave: base_note.octave,
            offset: (base_note.offset + offsets[(degree - 1 + 2) % offsets.len()]) % NUM_NOTES_12,
        };
        let fifth_note = Note {
            octave: base_note.octave,
            offset: (base_note.offset + offsets[(degree - 1 + 2 + 2) % offsets.len()])
                % NUM_NOTES_12,
        };

        // TODO: Support Chords Inversions and Voice Leading
        Self {
            // TODO: Improve Chord Name
            chord_name: root_note.to_string().into(),
            notes: vec![root_note, third_note, fifth_note],
        }
    }
}
const NUM_NOTES_12: u8 = 12;

pub enum ComposerMode {
    // TODO: Support other Modes
    Major, // Ionian
    // Dorian,
    // Phrygian,
    // Lydian,
    // Mixolydian,
    Minor, // Aeolian
           // Locrian,
}
const SCALE_MAJOR: [u8; 7] = [
    0,  // 1 T
    2,  // 2 T
    4,  // 3 S
    5,  // 4 T
    7,  // 5 T
    9,  // 6 T
    11, // 7 S
];
const SCALE_MINOR: [u8; 7] = [
    0,  // 1 T
    2,  // 2 S
    3,  // 3 T
    5,  // 4 T
    7,  // 5 S
    8,  // 6 T
    10, // 7 T
];
fn get_scale_offsets_by_mode(composer_mode: &ComposerMode) -> [u8; 7] {
    match composer_mode {
        ComposerMode::Major => SCALE_MAJOR,
        ComposerMode::Minor => SCALE_MINOR,
    }
}
