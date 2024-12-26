use crate::yaml_song_reader::{YamlSong, YamlSongSection};
use std::fmt::{Display, Formatter};

pub struct Song {
    pub details: SongDetails,
    pub tempo: SongTempo,
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
pub struct SongSection {
    pub kind: SongSectionKind,
    pub bars: usize,
    pub notes: Option<String>,
}

// Song Section Kind
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
        sections: yaml_song
            .sections
            .iter()
            .map(|section| SongSection {
                kind: convert_section_kind_from_string(section).unwrap(),
                bars: section.bars,
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
        sections: [
            SongSection {
                kind: SongSectionKind::Intro,
                bars: 4,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 8,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 8,
                notes: None,
            },
            SongSection {
                kind: SongSectionKind::Outro,
                bars: 4,
                notes: None,
            },
        ]
        .into(),
    }
}
