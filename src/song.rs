use crate::yaml_song_reader::{YamlSong, YamlSongSection};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Song {
    pub details: SongDetails,
    pub tempo: SongTempo,
    pub drum_patterns: HashMap<String, DrumPattern>,
    pub sections: Vec<SongSection>,
}
pub struct SongDetails {
    pub author: String,
    pub title: String,
}
pub struct SongTempo {
    pub bpm: usize,
    // Assuming all 4/4 bars.
    // Assuming bpm ticks to 1/4.
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
pub struct DrumPattern {
    num_1_4: usize,
    hh: String, // Es. "x x x x x x x x "
    sn: String, // Es. "    x  x    x   "
    kk: String, // Es. "x       x x    x"
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
        tempo: SongTempo { bpm: 85 },
        drum_patterns: HashMap::from([
            (
                "A".into(),
                DrumPattern {
                    num_1_4: 2,
                    hh: "x x x x x x x x ".into(),
                    sn: "    x  x    x   ".into(),
                    kk: "x       x x    x".into(),
                },
            ),
            (
                "B".into(),
                DrumPattern {
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
