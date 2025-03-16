use crate::song::yaml_song_reader::{YamlSong, YamlSongSection};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Song {
    pub id: String,
    pub details: SongDetails,
    pub tempo: SongTempo,
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

#[derive(Clone)]
pub struct SongDetails {
    pub author: String,
    pub title: String,
}
#[derive(Clone)]
pub struct SongTempo {
    pub bpm: usize,
    // Assuming bpm ticks to 1/4.
    pub time_signature: (usize, usize), // Es. (4, 4) for 4/4 bars.
}

// Song Section
#[derive(Clone)]
pub struct SongSection {
    pub kind: SongSectionKind,
    pub bars: usize,
    pub time_signature: (usize, usize), // Es. (4, 4) for 4/4 bars.
    pub num_1_16s_in_a_quarter: usize,
    pub drum_pattern_key: Option<String>,
    pub keyboard_pattern_key: Option<String>,
    pub bass_pattern_key: Option<String>,
    pub notes: Option<String>,
}
impl SongSection {
    pub fn get_num_1_16s(&self) -> usize {
        // Assuming "self.time_signature.1" is 4.
        self.bars * self.time_signature.0 * self.time_signature.1
    }
}
#[derive(Clone)]
pub enum SongSectionKind {
    Intro,
    Verse,
    Chorus,
    Bridge,
    Outro,
}
impl Display for SongSectionKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            SongSectionKind::Intro => write!(f, "Intro"),
            SongSectionKind::Verse => write!(f, "Verse"),
            SongSectionKind::Chorus => write!(f, "Chorus"),
            SongSectionKind::Bridge => write!(f, "Bridge"),
            SongSectionKind::Outro => write!(f, "Outro"),
        }
    }
}
pub fn convert_section_kind_from_string(
    section: &YamlSongSection,
) -> Result<SongSectionKind, &str> {
    match section.kind.as_str() {
        "Intro" => Ok(SongSectionKind::Intro),
        "Verse" => Ok(SongSectionKind::Verse),
        "Chorus" => Ok(SongSectionKind::Chorus),
        "Bridge" => Ok(SongSectionKind::Bridge),
        "Outro" => Ok(SongSectionKind::Outro),
        _ => Err("Unknown Song Kind"),
    }
}

// Drum Pattern
#[derive(Clone)]
pub struct DrumPattern {
    pub key: String, // Es. "A"
    pub num_1_4: usize,
    pub hh: String, // Es. "x x x x x x x x "
    pub sn: String, // Es. "    x  x    x   "
    pub kk: String, // Es. "x       x x    x"
}
#[derive(Clone)]
pub struct KeyboardPattern {
    pub key: String, // Es. "A"
    pub chords: Vec<KeyboardPatternChord>,
}
impl KeyboardPattern {
    // TODO: Duplicated code (*pkf)
    pub fn get_total_to_1_16th_incl(&self) -> usize {
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
#[derive(Clone)]
pub struct KeyboardPatternChord {
    pub chord_name: String, // Es. "Fmaj7"
    pub notes: Vec<String>, // Es. ["F3", "A3", "C4"]
    // Params "from_1_16th_incl" and "to_1_16th_incl" start from 1.
    pub from_1_16th_incl: usize,
    pub to_1_16th_incl: usize,
}
#[derive(Clone)]
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
        println!(" -> original chords");
        for chord in &self.chords {
            println!(" -> {:?}", chord);
        }
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
    pub fn get_total_to_1_16th_incl(&self) -> usize {
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
    pub chord_name: String, // Es. "Fmaj7"
    pub note: String,       // Es. "F3"
    // Params "from_1_16th_incl" and "to_1_16th_incl" start from 1.
    pub from_1_16th_incl: usize,
    pub to_1_16th_incl: usize,
}
#[derive(Clone)]
pub struct BassLine {
    pub parts: Vec<BassLinePart>,
}
#[derive(Clone)]
pub struct BassLinePart {
    // pub num_1_4: usize, // Es. 2
    pub tonic: String, // Es. "F2"
    pub line: String,  // Es. "1_1_1___"
}

// Songs
pub fn convert_yaml_into_song(yaml_song: YamlSong) -> Song {
    Song {
        id: "from-yaml".into(),
        details: SongDetails {
            title: yaml_song.title,
            author: yaml_song.author,
        },
        tempo: SongTempo {
            bpm: yaml_song.tempo_1_4,
            time_signature: (4, 4),
        },
        drum_patterns: HashMap::new(),
        keyboard_patterns: HashMap::new(),
        bass_patterns: HashMap::new(),
        sections: yaml_song
            .sections
            .iter()
            .map(|section| SongSection {
                kind: convert_section_kind_from_string(section).unwrap(),
                bars: section.bars,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: None,
                bass_pattern_key: None,
                notes: section.notes.clone(),
            })
            .collect(),
    }
}
pub fn get_dummy_song() -> Song {
    Song {
        id: "dummy-song".into(),
        details: SongDetails {
            author: "Author 1".into(),
            title: "Title 1".into(),
        },
        tempo: SongTempo {
            bpm: 85,
            time_signature: (4, 4),
        },
        drum_patterns: HashMap::from([
            (
                "A".into(),
                DrumPattern {
                    key: "A".into(),
                    num_1_4: 2,
                    hh: "x x x x x x x x ".into(),
                    sn: "    x  x    x   ".into(),
                    kk: "x       x x    x".into(),
                },
            ),
            (
                "B".into(),
                DrumPattern {
                    key: "B".into(),
                    num_1_4: 2,
                    hh: "x x x xxx x x xx".into(),
                    sn: "    x       x   ".into(),
                    kk: "x       x x     ".into(),
                },
            ),
        ]),
        keyboard_patterns: HashMap::from([
            (
                "A".into(),
                KeyboardPattern {
                    key: "A".into(),
                    chords: [
                        KeyboardPatternChord {
                            chord_name: "F".into(),
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 16,
                            notes: [
                                //
                                "F3".into(),
                                //
                                "A3".into(),
                                //
                                "C4".into(),
                            ]
                            .into(),
                        },
                        KeyboardPatternChord {
                            chord_name: "Dm".into(),
                            from_1_16th_incl: 16 + 1,
                            to_1_16th_incl: 16 + 16,
                            notes: [
                                //
                                "D3".into(),
                                //
                                "F3".into(),
                                //
                                "A3".into(),
                            ]
                            .into(),
                        },
                        KeyboardPatternChord {
                            chord_name: "C".into(),
                            from_1_16th_incl: 16 + 16 + 1,
                            to_1_16th_incl: 16 + 16 + 32,
                            notes: [
                                //
                                "C3".into(),
                                //
                                "E3".into(),
                                //
                                "G3".into(),
                            ]
                            .into(),
                        },
                    ]
                    .into(),
                },
            ),
            (
                "B".into(),
                KeyboardPattern {
                    key: "B".into(),
                    chords: [
                        KeyboardPatternChord {
                            chord_name: "F".into(),
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 16,
                            notes: [
                                //
                                "F3".into(),
                                //
                                "A3".into(),
                                //
                                "C4".into(),
                            ]
                            .into(),
                        },
                        KeyboardPatternChord {
                            chord_name: "Dm".into(),
                            from_1_16th_incl: 16 + 1,
                            to_1_16th_incl: 16 + 16,
                            notes: [
                                //
                                "D3".into(),
                                //
                                "F3".into(),
                                //
                                "A3".into(),
                            ]
                            .into(),
                        },
                        KeyboardPatternChord {
                            chord_name: "C".into(),
                            from_1_16th_incl: 16 + 16 + 1,
                            to_1_16th_incl: 16 + 16 + 16,
                            notes: [
                                //
                                "C3".into(),
                                //
                                "E3".into(),
                                //
                                "G3".into(),
                            ]
                            .into(),
                        },
                        KeyboardPatternChord {
                            chord_name: "G".into(),
                            from_1_16th_incl: 16 + 16 + 16 + 1,
                            to_1_16th_incl: 16 + 16 + 16 + 16,
                            notes: [
                                //
                                "D3".into(),
                                //
                                "G3".into(),
                                //
                                "B3".into(),
                            ]
                            .into(),
                        },
                    ]
                    .into(),
                },
            ),
        ]),
        bass_patterns: HashMap::new(),
        sections: [
            SongSection {
                kind: SongSectionKind::Intro,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: None,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: None,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: None,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: None,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: None,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Outro,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: None,
                notes: None,
            },
        ]
        .into(),
    }
}
