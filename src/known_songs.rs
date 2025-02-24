use crate::song::{
    DrumPattern, KeyboardPattern, KeyboardPatternChord, Song, SongDetails, SongSection,
    SongSectionKind, SongTempo,
};
use std::collections::HashMap;

pub fn get_song_coez_la_musica_non_c_e() -> Song {
    Song {
        id: "coez-la-musica-non-c-e".into(),
        details: SongDetails {
            author: "Coez".into(),
            title: "La musica non c'è".into(),
        },
        tempo: SongTempo {
            bpm: 68,
            time_signature: (4, 4),
        },
        drum_patterns: HashMap::from([(
            "A".into(),
            DrumPattern {
                key: "A".into(),
                num_1_4: 4,
                hh: "x x x x x x x x ".into(),
                sn: "    x       x   ".into(),
                kk: "x  x  x  xx   x ".into(),
            },
        )]),
        keyboard_patterns: HashMap::from([(
            "A".into(),
            KeyboardPattern {
                key: "A".into(),
                chords: [
                    KeyboardPatternChord {
                        chord_name: "Am".into(),
                        from_1_16th_incl: 1,
                        to_1_16th_incl: 16,
                        notes: [
                            //
                            "A3".into(),
                        ]
                        .into(),
                    },
                    KeyboardPatternChord {
                        chord_name: "G".into(),
                        from_1_16th_incl: 16 + 1,
                        to_1_16th_incl: 16 + 16,
                        notes: [
                            //
                            "G3".into(),
                        ]
                        .into(),
                    },
                    KeyboardPatternChord {
                        chord_name: "F".into(),
                        from_1_16th_incl: 16 + 16 + 1,
                        to_1_16th_incl: 16 + 16 + 16,
                        notes: [
                            //
                            "F3".into(),
                        ]
                        .into(),
                    },
                    KeyboardPatternChord {
                        chord_name: "C".into(),
                        from_1_16th_incl: 16 + 16 + 16 + 1,
                        to_1_16th_incl: 16 + 16 + 16 + 16,
                        notes: [
                            //
                            "C3".into(),
                        ]
                        .into(),
                    },
                ]
                .into(),
            },
        )]),
        sections: [SongSection {
            kind: SongSectionKind::Intro,
            bars: 4,
            time_signature: (4, 4),
            num_1_16s_in_a_quarter: 4,
            drum_pattern_key: None,
            keyboard_pattern_key: Some("A".into()),
            notes: None,
        }]
        .into(),
    }
}
