use crate::music::note::Note;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn get_standard_click_note() -> Note {
    "A4".into()
}

#[derive(Clone, Debug)]
pub struct Song {
    pub id: String,
    pub details: SongDetails,
    pub tempo: SongTempo,
    pub metronome_data: Option<SongMetronomeData>,
    pub drum_patterns: HashMap<String, DrumPattern>,
    pub keyboard_patterns: HashMap<String, KeyboardPattern>,
    pub bass_patterns: HashMap<String, BassPattern>,
    pub sections: Vec<SongSection>,
}
impl Song {
    pub fn get_drum_pattern_from_key(&self, pattern_key: String) -> Option<&DrumPattern> {
        self.drum_patterns.get(&pattern_key)
    }
    pub fn get_keyboard_pattern_from_key(&self, pattern_key: String) -> Option<&KeyboardPattern> {
        self.keyboard_patterns.get(&pattern_key)
    }
    pub fn get_bass_pattern_from_key(&self, pattern_key: String) -> Option<&BassPattern> {
        self.bass_patterns.get(&pattern_key)
    }
}

#[derive(Clone, Debug)]
pub struct SongDetails {
    pub author: String,
    pub title: String,
}
#[derive(Clone, Debug)]
pub struct SongTempo {
    pub bpm: usize,
}
#[derive(Clone, Debug)]
pub struct SongTimeSignature {
    pub top: usize,
    pub down: usize,
    pub i_1_8ths_triplets: bool,
}
pub const TIME_SIGNATURE_4_4: SongTimeSignature = SongTimeSignature {
    top: 4,
    down: 4,
    i_1_8ths_triplets: false,
};
#[derive(Clone, Debug)]
pub struct SongMetronomeData {
    pub click_on: SongMetronomeDataClickOn,
    pub note: Note,
}
#[derive(Clone, Debug)]
pub enum SongMetronomeDataClickOn {
    OnEvery1_4ths,
    OnEvery1_8ths,
}

// Song Section
#[derive(Clone, Debug)]
pub struct SongSection {
    pub kind: SongSectionKind,
    pub bars: usize,
    pub time_signature: SongTimeSignature,
    pub drum_pattern_key: Option<String>,
    pub keyboard_pattern_key: Option<String>,
    pub bass_pattern_key: Option<String>,
    pub notes: Option<String>,
}
impl SongSection {
    pub fn get_num_1_16s(&self) -> usize {
        // Assuming "self.time_signature.down" is 4, so 1/4ths.
        let num_1_16ths_in_one_quarter = 4;
        self.bars * self.time_signature.top * num_1_16ths_in_one_quarter
    }
}
#[derive(Clone, Debug)]
pub enum SongSectionKind {
    Intro,
    Verse,
    PreChorus,
    Chorus,
    PostChorus,
    Bridge,
    Outro,
}
impl Display for SongSectionKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            SongSectionKind::Intro => write!(f, "Intro"),
            SongSectionKind::Verse => write!(f, "Verse"),
            SongSectionKind::PreChorus => write!(f, "PreChorus"),
            SongSectionKind::Chorus => write!(f, "Chorus"),
            SongSectionKind::PostChorus => write!(f, "PostChorus"),
            SongSectionKind::Bridge => write!(f, "Bridge"),
            SongSectionKind::Outro => write!(f, "Outro"),
        }
    }
}

// Drum Pattern
#[derive(Clone, Debug)]
pub struct DrumPattern {
    pub key: String, // Es. "A"
    pub num_1_4: usize,
    pub hh: String, // Es. "x x x x x x x x "
    pub sn: String, // Es. "    x  x    x   "
    pub kk: String, // Es. "x       x x    x"
}
#[derive(Clone, Debug)]
pub struct KeyboardPattern {
    pub key: String, // Es. "A"
    pub chords: Vec<KeyboardPatternChord>,
}
impl KeyboardPattern {
    // TODO: Duplicated code (*pkf)
    fn get_total_to_1_16th_incl(&self) -> usize {
        let last_chord = &self.chords[self.chords.len() - 1];
        last_chord.to_1_16th_incl
    }
    pub fn get_ceil_num_bars_coverage(&self) -> usize {
        let total_1_16ths_incl = self.get_total_to_1_16th_incl();
        // Assuming 1/4ths is a group of "4" 1/16ths.
        let ceil_1_4ths = (total_1_16ths_incl as f64 / 4.0).ceil() as usize;
        // Assuming 4/4 bars.
        ceil_1_4ths / 4
    }
}
#[derive(Clone, Debug)]
pub struct KeyboardPatternChord {
    pub chord_name: String, // Es. "Fmaj7"
    pub notes: Vec<Note>,   // Es. ["F3", "A3", "C4"]
    // Params "from_1_16th_incl" and "to_1_16th_incl" start from 1.
    pub from_1_16th_incl: usize,
    pub to_1_16th_incl: usize,
}
#[derive(Clone, Debug)]
pub struct BassPattern {
    pub key: String, // Es. "A"
    pub bass_line: BassLine,
}
impl BassPattern {
    pub fn get_chords(&self) -> Vec<BassPatternChord> {
        let mut chords = Vec::new();
        let mut next_1_16th = 1;
        for bass_line_part in &self.bass_line.parts {
            let line = &bass_line_part.line;
            let mut i = 0;
            while i < line.len() {
                let char = line.get(i..=i).unwrap();
                if char == "1" {
                    chords.push(
                        //
                        BassPatternChord {
                            chord_name: format!("Root of {}", bass_line_part.tonic),
                            // TODO: Avoid cloning Note string
                            note: bass_line_part.tonic.clone(),
                            from_1_16th_incl: next_1_16th,
                            to_1_16th_incl: next_1_16th, // Temporary
                        },
                    );
                } else if char == "_" {
                    chords.last_mut().unwrap().to_1_16th_incl = next_1_16th;
                } else if char == " " {
                    // Nothing.
                }
                i += 1;
                next_1_16th += 1;
            }
        }

        /*
        // + Debug
        println!(" -> result");
        for chord in &chords {
            println!(" -> {:?}", chord);
        }
        println!();
        // - Debug
        */

        chords
    }
    // TODO: Duplicated code (*pkf)
    fn get_total_to_1_16th_incl(&self) -> usize {
        let bass_chords = self.get_chords();
        let last_chord = &bass_chords[bass_chords.len() - 1];
        last_chord.to_1_16th_incl
    }
    pub fn get_ceil_num_bars_coverage(&self) -> usize {
        let total_1_16ths_incl = self.get_total_to_1_16th_incl();
        // Assuming 1/4ths is a group of "4" 1/16ths.
        let ceil_1_4ths = (total_1_16ths_incl as f64 / 4.0).ceil() as usize;
        // Assuming 4/4 bars.
        ceil_1_4ths / 4
    }
}
#[derive(Clone, Debug)]
pub struct BassPatternChord {
    // TODO: Try to remove this struct and use BassLine directly
    pub chord_name: String, // Es. "Fmaj7"
    pub note: Note,         // Es. "F3"
    // Params "from_1_16th_incl" and "to_1_16th_incl" start from 1.
    pub from_1_16th_incl: usize,
    pub to_1_16th_incl: usize,
}
#[derive(Clone, Debug)]
pub struct BassLine {
    pub parts: Vec<BassLinePart>,
}
#[derive(Clone, Debug)]
pub struct BassLinePart {
    pub tonic: Note,        // Es. "F2"
    pub line: &'static str, // Es. "1_1_1___"
}
