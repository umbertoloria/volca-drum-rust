fn get_octave_offset_from_letter(note_bytes: &[u8]) -> u8 {
    let number_byte = note_bytes[note_bytes.len() - 1];
    match number_byte {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        _ => 0,
    }
    // TODO: Log for unknown octave letter
}
fn get_note_first_letter_and_alteration(note_bytes: &[u8]) -> [u8; 2] {
    // Returns [Byte of First Letter, 0=flat, 1=normal, 2=sharp].
    [
        note_bytes[0],
        if note_bytes.len() > 2 {
            match note_bytes[1] {
                b'b' => 0,
                b'#' => 2,
                _ => 1,
                // TODO: Log for unknown alteration letter
            }
        } else {
            1
        },
    ]
}
fn get_note_offset_from_note(note_bytes: &[u8]) -> u8 {
    // Returns a number from 0 to 11 meaning all "12" Notes in Music.
    let note_first_letter_and_alteration = get_note_first_letter_and_alteration(note_bytes);
    match (
        note_first_letter_and_alteration[0],
        note_first_letter_and_alteration[1],
    ) {
        // C
        (b'C', 1) => 0,

        // C#/Db
        (b'C', 2) => 1,
        (b'D', 0) => 1,

        // D
        (b'D', 1) => 2,

        // D#/Eb
        (b'D', 2) => 3,
        (b'E', 0) => 3,

        // E
        (b'E', 1) => 4,

        // F
        (b'F', 1) => 5,

        // F#/Gb
        (b'F', 2) => 6,
        (b'G', 0) => 6,

        // G
        (b'G', 1) => 7,

        // G#/Ab
        (b'G', 2) => 8,
        (b'A', 0) => 8,

        // A
        (b'A', 1) => 9,

        // A#/Bb
        (b'A', 2) => 10,
        (b'B', 0) => 10,

        // B
        (b'B', 1) => 11,

        _ => 0, // Defaults to "C".
    }
    // TODO: Log for unknown note letter
}

pub struct Note {
    pub octave: u8,
    pub offset: u8, // From 0 to 11 (incl.).
}
impl Note {
    pub fn new(note_str: &String) -> Self {
        let note_bytes = note_str.as_bytes();
        Self {
            octave: get_octave_offset_from_letter(note_bytes),
            offset: get_note_offset_from_note(note_bytes),
        }
    }
}
