use crate::song;
use crate::song::{
    DrumPattern, KeyboardPattern, KeyboardPatternChord, Song, SongDetails, SongSection, SongTempo,
};
use std::collections::HashMap;

pub struct Composer {
    pub bpm: usize,
    pub num_sections: usize,
    pub click: bool,
    pub tonality_note: TonalityNote,
    pub tonality_mode: TonalityMode,
}
pub enum TonalityNote {
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
}
impl Composer {
    pub fn compose_new_song(&self) -> Song {
        Song {
            details: SongDetails {
                author: "Smart composer".into(),
                title: "Smart song".into(),
            },
            tempo: SongTempo {
                bpm: self.bpm,
                time_signature: (4, 4),
            },
            drum_patterns: HashMap::from([(
                "CLICK".into(),
                DrumPattern {
                    key: "CLICK".into(),
                    num_1_4: 4,
                    hh: "                ".into(),
                    sn: "                ".into(),
                    kk: "x x x x x x x x ".into(),
                },
            )]),
            keyboard_patterns: HashMap::from([(
                "A".into(),
                KeyboardPattern {
                    key: "A".into(),
                    chords: [KeyboardPatternChord {
                        chord_name: "C".into(),
                        from_1_16th_incl: 1,
                        to_1_16th_incl: 16 + 16 + 16 + 16,
                        notes: [
                            //
                            "C2".into(),
                            //
                            "C3".into(),
                            //
                            "E3".into(),
                            //
                            "G3".into(),
                        ]
                        .into(),
                    }]
                    .into(),
                },
            )]),
            sections: [SongSection {
                kind: song::SongSectionKind::Verse,
                bars: self.num_sections,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("CLICK".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            }]
            .into(),
        }
    }
}
