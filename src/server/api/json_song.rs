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
            // Chord Changes in Time
            let mut chord_changes_in_time = Vec::new();
            if let Some(keyboard_pattern_key) = section.keyboard_pattern_key {
                let keyboard_pattern = song.keyboard_patterns.get(&keyboard_pattern_key).unwrap();
                let mut num_bars_covered_for_now = 0;
                while num_bars_covered_for_now < section.bars {
                    for keyboard_pattern_chord in &keyboard_pattern.chords {
                        // Assuming 4/4 bars with 1/4ths made of *4* 1/16ths.
                        let num_1_16ths_in_bar = num_bars_covered_for_now * 4 * 4;
                        chord_changes_in_time.push(
                            //
                            JsonSongSectionChordChange {
                                i_1_16th_start: num_1_16ths_in_bar
                                    + keyboard_pattern_chord.from_1_16th_incl,
                                // TODO: Avoid Keyboard Pattern Chord Name clone
                                chord_name: keyboard_pattern_chord.chord_name.clone(),
                            },
                        );
                    }
                    num_bars_covered_for_now += keyboard_pattern.get_ceil_num_bars_coverage();
                }
            }

            // Section
            sections.push(
                //
                JsonSongSection {
                    id: i_section,
                    kind: get_song_section_kind_string(&section.kind),
                    bars: section.bars,
                    first_bar_num: i_bar,
                    time_signature_top: section.time_signature.0,
                    time_signature_down: section.time_signature.1,
                    chord_changes_in_time,
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
    chord_changes_in_time: Vec<JsonSongSectionChordChange>,
}
#[derive(Serialize, Deserialize)]
pub struct JsonSongSectionChordChange {
    i_1_16th_start: usize,
    chord_name: String,
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
