use crate::song::song::{
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
        drum_patterns: HashMap::from([
            (
                "A".into(),
                DrumPattern {
                    key: "A".into(),
                    num_1_4: 4,
                    hh: "x x x x x x x x ".into(),
                    sn: "    x       x   ".into(),
                    kk: "x  x  x  xx   x ".into(),
                },
            ),
            (
                "B".into(),
                DrumPattern {
                    key: "B".into(),
                    num_1_4: 4,
                    // Should be on Ride and with Accents on every 1/4ths.
                    hh: "x x x x x x x x ".into(),
                    sn: "    x       x   ".into(),
                    kk: "x  x  x  xx   x ".into(),
                },
            ),
            (
                // HH on 1/8ths.
                "C".into(),
                DrumPattern {
                    key: "C".into(),
                    num_1_4: 2,
                    hh: "x x x x x x x x ".into(),
                    sn: "                ".into(),
                    kk: "                ".into(),
                },
            ),
        ]),
        keyboard_patterns: HashMap::from([(
            "A".into(),
            KeyboardPattern {
                key: "A".into(),
                chords: [
                    KeyboardPatternChord {
                        chord_name: "Am".into(),
                        from_1_16th_incl: 1,
                        to_1_16th_incl: 6,
                        notes: [
                            //
                            "A3".into(),
                            "C4".into(),
                            "E4".into(),
                        ]
                        .into(),
                    },
                    KeyboardPatternChord {
                        chord_name: "G".into(),
                        from_1_16th_incl: 7,
                        to_1_16th_incl: 16,
                        notes: [
                            //
                            "G3".into(),
                            "B3".into(),
                            "D4".into(),
                        ]
                        .into(),
                    },
                    KeyboardPatternChord {
                        chord_name: "F".into(),
                        from_1_16th_incl: 16 + 1,
                        to_1_16th_incl: 16 + 6,
                        notes: [
                            //
                            "F3".into(),
                            "A3".into(),
                            "C4".into(),
                        ]
                        .into(),
                    },
                    KeyboardPatternChord {
                        chord_name: "C".into(),
                        from_1_16th_incl: 16 + 7,
                        to_1_16th_incl: 16 + 16,
                        notes: [
                            //
                            "G3".into(),
                            "C4".into(),
                            "E4".into(),
                        ]
                        .into(),
                    },
                ]
                .into(),
            },
        )]),
        sections: [
            // Intro 3+1*, no drums + *fill
            SongSection {
                kind: SongSectionKind::Intro,
                bars: 3 + 1,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                // drum_pattern_key: None, //Always have HH or some sort of click.
                drum_pattern_key: Some("C".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 4+2
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4 + 2,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Chorus 4+4*, *strong
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 4 + 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("B".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Post-chorus 2
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 2,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("C".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 4+2
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4 + 2,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("A".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Chorus 4+4*, *strong
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 4 + 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("B".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 4, no drums
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None, // Avoid having no drums at all!
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 4, "Distant Drums"
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None, // Have "Distant Drums"!
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 4
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("C".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 4, no drums
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None, // Avoid having no drums at all!
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Chorus 8
            SongSection {
                kind: SongSectionKind::Chorus,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: Some("B".into()),
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
            // Outro 2, no drums
            SongSection {
                kind: SongSectionKind::Outro,
                bars: 2,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None, // Avoid having no drums at all!
                keyboard_pattern_key: Some("A".into()),
                notes: None,
            },
        ]
        .into(),
    }
}
