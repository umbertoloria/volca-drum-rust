use crate::song::song::{
    get_standard_click_note, KeyboardPattern, KeyboardPatternChord, Song, SongDetails,
    SongMetronomeData, SongMetronomeDataClickOn, SongSection, SongSectionKind, SongTempo,
};
use std::collections::HashMap;

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
pub struct Composer {
    pub bpm: usize,
    pub num_sections: usize,
    pub click: bool,
    pub tonality_note: TonalityNote,
    pub tonality_mode: TonalityMode,
}
impl Composer {
    pub fn new(
        bpm: usize,
        num_sections: usize,
        click: bool,
        tonality_note: TonalityNote,
        tonality_mode: TonalityMode,
    ) -> Self {
        Self {
            bpm,
            num_sections,
            click,
            tonality_note,
            tonality_mode,
        }
    }
    pub fn compose_new_song(&self) -> Song {
        Song {
            id: "composed-song".into(),
            details: SongDetails {
                author: "Smart composer".into(),
                title: "Smart song".into(),
            },
            tempo: SongTempo {
                bpm: self.bpm,
                time_signature: (4, 4),
            },
            metronome_data: Some(SongMetronomeData {
                click_on: SongMetronomeDataClickOn::OnEvery1_4ths,
                note: get_standard_click_note(),
            }),
            drum_patterns: HashMap::from([
                //
                /*(
                    "CLICK".into(),
                    DrumPattern {
                        key: "CLICK".into(),
                        num_1_4: 4,
                        hh: "                ".into(),
                        sn: "                ".into(),
                        kk: "x x x x x x x x ".into(),
                    },
                ),*/
            ]),
            keyboard_patterns: HashMap::from([(
                "A".into(),
                KeyboardPattern {
                    key: "A".into(),
                    chords: [
                        //
                        KeyboardPatternChord {
                            chord_name: "C".into(),
                            notes: vec!["C2".into(), "C3".into(), "E3".into(), "G3".into()],
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 16 + 16 + 16 + 16,
                        },
                    ]
                    .into(),
                },
            )]),
            bass_patterns: HashMap::new(),
            sections: [SongSection {
                kind: SongSectionKind::Verse,
                bars: self.num_sections,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                // drum_pattern_key: Some("CLICK".into()),
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: None,
                notes: None,
            }]
            .into(),
        }
    }
}
