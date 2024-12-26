use crate::yaml_song_reader::{YamlSong, YamlSongSection};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Song {
    pub details: SongDetails,
    pub tempo: SongTempo,
    pub drum_patterns: HashMap<String, DrumPattern>,
    pub sections: Vec<SongSection>,
}
impl Song {
    pub fn get_drum_pattern_clone_from_key(&self, drum_pattern_key: String) -> Option<DrumPattern> {
        if let Some(drum_pattern) = self.drum_patterns.get(&drum_pattern_key) {
            // TODO: Avoid cloning the drum pattern
            Some(drum_pattern.clone())
        } else {
            None
        }
    }
}

pub struct SongDetails {
    pub author: String,
    pub title: String,
}
pub struct SongTempo {
    pub bpm: usize,
    // Assuming bpm ticks to 1/4.
    pub time_signature: (usize, usize), // Es. (4, 4) for 4/4 bars.
}

// Song Section
pub struct SongSection {
    pub kind: SongSectionKind,
    pub bars: usize,
    pub drum_pattern_key: Option<String>,
    pub notes: Option<String>,
}
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

// Songs
pub fn convert_yaml_into_song(yaml_song: YamlSong) -> Song {
    Song {
        details: SongDetails {
            title: yaml_song.title,
            author: yaml_song.author,
        },
        tempo: SongTempo {
            bpm: yaml_song.tempo_1_4,
            time_signature: (4, 4),
        },
        drum_patterns: HashMap::new(),
        sections: yaml_song
            .sections
            .iter()
            .map(|section| SongSection {
                kind: convert_section_kind_from_string(section).unwrap(),
                bars: section.bars,
                drum_pattern_key: None,
                notes: section.notes.clone(),
            })
            .collect(),
    }
}
pub fn get_dummy_song() -> Song {
    Song {
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
        sections: [
            SongSection {
                kind: SongSectionKind::Intro,
                bars: 4,
                drum_pattern_key: None,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                drum_pattern_key: Some("A".into()),
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 8,
                drum_pattern_key: Some("B".into()),
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                drum_pattern_key: Some("A".into()),
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 8,
                drum_pattern_key: Some("B".into()),
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Outro,
                bars: 4,
                drum_pattern_key: None,
                notes: None,
            },
        ]
        .into(),
    }
}
