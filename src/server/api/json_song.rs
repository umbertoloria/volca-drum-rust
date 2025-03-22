use crate::song::song::{Song, SongSectionKind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonSong {
    id: String,
    author: String,
    title: String,
    tempo: JsonSongTempo,
    sections: Vec<JsonSongSection>,
}
impl JsonSong {
    pub fn new(song: Song) -> Self {
        let mut i_section = 1;
        let mut i_bar = 1;
        let mut sections = Vec::new();
        for section in song.sections {
            sections.push(
                //
                JsonSongSection {
                    id: i_section,
                    kind: get_song_section_kind_string(&section.kind),
                    bars: section.bars,
                    first_bar_num: i_bar,
                    time_signature_top: section.time_signature.0,
                    time_signature_down: section.time_signature.1,
                },
            );
            i_bar += section.bars;
            i_section += 1;
        }
        Self {
            id: song.id,
            author: song.details.author,
            title: song.details.title,
            tempo: JsonSongTempo {
                bpm: song.tempo.bpm,
                time_signature_top: song.tempo.time_signature.0,
                time_signature_down: song.tempo.time_signature.1,
            },
            sections,
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonSongTempo {
    bpm: usize,
    time_signature_top: usize,  // Es. 6 for 6/8 bars.
    time_signature_down: usize, // Es. 8 for 6/8 bars.
}

#[derive(Serialize, Deserialize)]
pub struct JsonSongSection {
    id: usize,
    kind: String,
    bars: usize,
    first_bar_num: usize,
    time_signature_top: usize,
    time_signature_down: usize,
}

fn get_song_section_kind_string(kind: &SongSectionKind) -> String {
    match kind {
        SongSectionKind::Intro => "INTRO".into(),
        SongSectionKind::Verse => "VERSE".into(),
        SongSectionKind::PreChorus => "PRE-CHORUS".into(),
        SongSectionKind::Chorus => "CHORUS".into(),
        SongSectionKind::PostChorus => "POST-CHORUS".into(),
        SongSectionKind::Bridge => "BRIDGE".into(),
        SongSectionKind::Outro => "OUTRO".into(),
    }
}
