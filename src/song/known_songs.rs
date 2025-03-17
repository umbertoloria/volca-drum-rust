use crate::song::song::{
    BassLine, BassLinePart, BassPattern, DrumPattern, KeyboardPattern, KeyboardPatternChord, Song,
    SongDetails, SongMetronomeDataClickOn, SongMetronomeData, SongSection, SongSectionKind, SongTempo,
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
        metronome_data: SongMetronomeData {
            click_on: SongMetronomeDataClickOn::OnEvery1_4ths,
            note: "C4".to_string(),
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
        bass_patterns: HashMap::new(),
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
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
                bass_pattern_key: None,
                notes: None,
            },
        ]
        .into(),
    }
}
pub fn get_song_o1() -> Song {
    Song {
        id: "o1".into(),
        details: SongDetails {
            author: "O1".into(),
            title: "O1".into(),
        },
        tempo: SongTempo {
            bpm: 68,
            time_signature: (4, 4),
        },
        metronome_data: SongMetronomeData {
            click_on: SongMetronomeDataClickOn::OnEvery1_4ths,
            note: "B4".to_string(),
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
        keyboard_patterns: HashMap::from([
            (
                "A".into(),
                KeyboardPattern {
                    key: "A".into(),
                    chords: [
                        KeyboardPatternChord {
                            chord_name: "Bm".into(),
                            notes: vec!["B3".into(), "D4".into(), "F#4".into()],
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 6,
                        },
                        KeyboardPatternChord {
                            chord_name: "F".into(),
                            notes: vec!["F3".into(), "A3".into(), "C4".into()],
                            from_1_16th_incl: 7,
                            to_1_16th_incl: 16,
                        },
                        KeyboardPatternChord {
                            chord_name: "Bm".into(),
                            notes: vec!["B3".into(), "D4".into(), "F#4".into()],
                            from_1_16th_incl: 16 + 1,
                            to_1_16th_incl: 16 + 6,
                        },
                        KeyboardPatternChord {
                            chord_name: "F".into(),
                            notes: vec!["F3".into(), "A3".into(), "C4".into()],
                            from_1_16th_incl: 16 + 7,
                            to_1_16th_incl: 16 + 16,
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
                            chord_name: "Am7".into(),
                            notes: vec!["A3".into(), "C4".into(), "E4".into(), "G4".into()],
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "Dmaj7".into(),
                            notes: vec!["D3".into(), "F#3".into(), "A4".into(), "Db5".into()],
                            from_1_16th_incl: 8 + 1,
                            to_1_16th_incl: 8 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "Am7".into(),
                            notes: vec!["A3".into(), "C4".into(), "E4".into(), "G4".into()],
                            from_1_16th_incl: 16 + 1,
                            to_1_16th_incl: 16 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "Dmaj7".into(),
                            notes: vec!["D3".into(), "F#3".into(), "A4".into(), "Db5".into()],
                            from_1_16th_incl: 24 + 1,
                            to_1_16th_incl: 24 + 8,
                        },
                    ]
                    .into(),
                },
            ),
            (
                "C".into(),
                KeyboardPattern {
                    key: "C".into(),
                    chords: [
                        KeyboardPatternChord {
                            chord_name: "F#".into(),
                            notes: vec!["F#3".into(), "A#3".into(), "C#4".into()],
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "A".into(),
                            notes: vec!["A3".into(), "C4".into(), "E4".into()],
                            from_1_16th_incl: 8 + 1,
                            to_1_16th_incl: 8 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "F#".into(),
                            notes: vec!["F#3".into(), "A#3".into(), "C#4".into()],
                            from_1_16th_incl: 16 + 1,
                            to_1_16th_incl: 16 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "A".into(),
                            notes: vec!["A3".into(), "C4".into(), "E4".into()],
                            from_1_16th_incl: 24 + 1,
                            to_1_16th_incl: 24 + 8,
                        },
                    ]
                    .into(),
                },
            ),
            (
                "D".into(),
                KeyboardPattern {
                    key: "D".into(),
                    chords: [
                        KeyboardPatternChord {
                            chord_name: "C#m".into(),
                            notes: vec!["C#3".into(), "E3".into(), "G#3".into()],
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "Bb".into(),
                            notes: vec!["Bb3".into(), "D4".into(), "F4".into()],
                            from_1_16th_incl: 8 + 1,
                            to_1_16th_incl: 8 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "C#m".into(),
                            notes: vec!["C#3".into(), "E3".into(), "G#3".into()],
                            from_1_16th_incl: 16 + 1,
                            to_1_16th_incl: 16 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "Bb".into(),
                            notes: vec!["Bb3".into(), "D4".into(), "F4".into()],
                            from_1_16th_incl: 24 + 1,
                            to_1_16th_incl: 24 + 8,
                        },
                    ]
                    .into(),
                },
            ),
        ]),
        bass_patterns: HashMap::new(),
        sections: [
            // Intro 4
            SongSection {
                kind: SongSectionKind::Intro,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: None,
                notes: None,
            },
            // Verse 4
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: None,
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("C".into()),
                bass_pattern_key: None,
                notes: None,
            },
            // Bridge 4
            SongSection {
                kind: SongSectionKind::Bridge,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("D".into()),
                bass_pattern_key: None,
                notes: None,
            },
            // Verse 4
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 4,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("C".into()),
                bass_pattern_key: None,
                notes: None,
            },
        ]
        .into(),
    }
}

pub fn get_song_o2() -> Song {
    Song {
        id: "o2".into(),
        details: SongDetails {
            author: "O2".into(),
            title: "O2".into(),
        },
        tempo: SongTempo {
            bpm: 40,
            time_signature: (4, 4),
        },
        metronome_data: SongMetronomeData {
            click_on: SongMetronomeDataClickOn::OnEvery1_4ths,
            note: "A4".to_string(),
        },
        drum_patterns: HashMap::from([]),
        keyboard_patterns: HashMap::from([
            (
                "A".into(),
                KeyboardPattern {
                    key: "A".into(),
                    chords: [
                        KeyboardPatternChord {
                            chord_name: "Am".into(),
                            notes: vec!["A3".into(), "C4".into(), "E4".into()],
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "Em".into(),
                            notes: vec!["B3".into(), "E4".into(), "G4".into()],
                            from_1_16th_incl: 8 + 1,
                            to_1_16th_incl: 8 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "Dm".into(),
                            notes: vec!["A3".into(), "D4".into(), "F4".into()],
                            from_1_16th_incl: 8 + 8 + 1,
                            to_1_16th_incl: 8 + 8 + 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "F".into(),
                            notes: vec!["C4".into(), "E4".into(), "F4".into(), "A4".into()],
                            from_1_16th_incl: 8 + 8 + 8 + 1,
                            to_1_16th_incl: 8 + 8 + 8 + 8,
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
                            chord_name: "C".into(),
                            notes: vec!["C4".into(), "E4".into(), "G4".into()],
                            from_1_16th_incl: 1,
                            to_1_16th_incl: 8,
                        },
                        KeyboardPatternChord {
                            chord_name: "F".into(),
                            notes: vec!["C4".into(), "F4".into(), "A4".into()],
                            from_1_16th_incl: 8 + 1,
                            to_1_16th_incl: 8 + 8,
                        },
                    ]
                    .into(),
                },
            ),
        ]),
        bass_patterns: HashMap::from([
            (
                "A".into(),
                BassPattern {
                    key: "A".into(),
                    bass_line: BassLine {
                        parts: vec![
                            BassLinePart {
                                tonic: "A2".into(),
                                line: "1_1_1_11".into(),
                            },
                            BassLinePart {
                                tonic: "E2".into(),
                                line: "1_1_11_1".into(),
                            },
                            BassLinePart {
                                tonic: "D2".into(),
                                line: "1_11_1_1".into(),
                            },
                            BassLinePart {
                                tonic: "F2".into(),
                                line: "11_1_11_".into(),
                            },
                        ],
                    },
                },
            ),
            (
                "B".into(),
                BassPattern {
                    key: "B".into(),
                    bass_line: BassLine {
                        parts: vec![
                            BassLinePart {
                                tonic: "C2".into(),
                                line: "1_1_11_1".into(),
                            },
                            BassLinePart {
                                tonic: "F2".into(),
                                line: "1_1_11_1".into(),
                            },
                        ],
                    },
                },
            ),
        ]),
        sections: [
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
            ///////////////
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("A".into()),
                bass_pattern_key: Some("A".into()),
                notes: None,
            },
            // Verse 8
            SongSection {
                kind: SongSectionKind::Verse,
                bars: 8,
                time_signature: (4, 4),
                num_1_16s_in_a_quarter: 4,
                drum_pattern_key: None,
                keyboard_pattern_key: Some("B".into()),
                bass_pattern_key: Some("B".into()),
                notes: None,
            },
        ]
        .into(),
    }
}
