use std::fmt::{Display, Formatter};

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

#[derive(Clone, Debug)]
pub struct Note {
    pub octave: u8,
    pub offset: u8, // From 0 to 11 (incl.).
}
impl Note {
    pub fn new(note_str: &str) -> Self {
        let note_bytes = note_str.as_bytes();
        Self {
            octave: get_octave_offset_from_letter(note_bytes),
            offset: get_note_offset_from_note(note_bytes),
        }
    }
    pub fn compare_to(&self, b: &Note) -> CompareEnum {
        if self.octave < b.octave {
            // Note "a" Octave lower.
            CompareEnum::Lower
        } else if self.octave > b.octave {
            // Note "b" Octave higher.
            CompareEnum::Higher
        } else {
            // Notes "a" and "b" in the same Octave.
            if self.offset < b.offset {
                CompareEnum::Lower
            } else if self.offset == b.offset {
                // Same note.
                CompareEnum::Equal
            } else {
                CompareEnum::Higher
            }
        }
    }
    pub fn get_octave_lower(&self) -> Self {
        Self {
            octave: self.octave - 1,
            offset: self.offset,
        }
    }
    pub fn get_octave_higher(&self) -> Self {
        Self {
            octave: self.octave + 1,
            offset: self.offset,
        }
    }
    fn get_new_applying_offset(&self, shift_offset: u8) -> Self {
        let mut offset = self.offset + shift_offset;
        let mut octave = self.octave;
        while offset >= 12 {
            offset -= 12;
            octave += 1;
        }
        Self { octave, offset }
    }
    pub fn get_minor_third(&self) -> Self {
        self.get_new_applying_offset(3)
    }
    pub fn get_major_third(&self) -> Self {
        self.get_new_applying_offset(4)
    }
    pub fn get_fifth(&self) -> Self {
        self.get_new_applying_offset(7)
    }
    pub fn get_minor_seventh(&self) -> Self {
        self.get_new_applying_offset(7 + 3)
    }
    pub fn get_major_seventh(&self) -> Self {
        self.get_new_applying_offset(7 + 4)
    }
    pub fn get_frequency(&self) -> f32 {
        // TODO: This code must be very numerically precise!
        let base_frequency: f32 = match self.offset {
            0 => 16.35,
            1 => 17.32,
            2 => 18.35,
            3 => 19.45,
            4 => 20.60,
            5 => 21.83,
            6 => 22.16,
            7 => 24.50,
            8 => 25.96,
            9 => 27.50,
            10 => 29.14,
            11 => 30.87,
            _ => 0.0, // TODO: Defaults to what?
        };
        let octave_coefficient = 2.0f32.powf(self.octave as f32);
        /*
        let result = base_frequency * octave_coeff;
        println!(" -> from={:?} result={}", note, result);
        result
        */
        base_frequency * octave_coefficient
    }
    // Hash
    pub fn from_hash(note_hash: String) -> Self {
        let octave_part = &note_hash[0..note_hash.len() - 2];
        let note_part = &note_hash[note_hash.len() - 2..];
        let octave = octave_part.parse::<u8>().unwrap();
        let offset = note_part.parse::<u8>().unwrap();
        /*
        // + Debug
        println!(" -> octave_part={octave_part}");
        println!(" -> note_part={note_part}");
        println!(" -> octave={octave}");
        println!(" -> offset={offset}");
        println!();
        // - Debug
        */
        Self {
            //
            octave,
            offset,
        }
    }
    pub fn to_hash(&self) -> String {
        let numeric_hash = (self.octave as usize) * 100 + (self.offset as usize);
        format!("{}", numeric_hash)
    }
    // PRETTY
    pub fn to_pretty_string(&self) -> Result<String, String> {
        // NOTE: This function always prefers "C#" over "Db", for example. So don't use for
        // something like hashing.

        let letter_note = match self.offset {
            // C
            0 => Some("C"),

            // C#/Db
            1 => Some("C#"),

            // D
            2 => Some("D"),

            // D#/Eb
            3 => Some("D#"),

            // E
            4 => Some("E"),

            // F
            5 => Some("F"),

            // F#/Gb
            6 => Some("F#"),

            // G
            7 => Some("G"),

            // G#/Ab
            8 => Some("G#"),

            // A
            9 => Some("A"),

            // A#/Bb
            10 => Some("A#"),

            // B
            11 => Some("B"),

            _ => None,
        };
        /*
        let letter_note = letter_note.expect("Invalid note");
        format!("{}{}", letter_note, self.octave)
        */
        if let Some(letter_note) = letter_note {
            let result = format!("{}{}", letter_note, self.octave);
            Ok(result)
        } else {
            Err("Invalid note".into())
        }
    }
}
impl From<&str> for Note {
    fn from(value: &str) -> Self {
        Note::new(value)
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OCT={}, OFF={}", self.octave, self.offset)
    }
}

// Comparing Notes
pub enum CompareEnum {
    Lower,
    Equal,
    Higher,
}
pub fn get_notes_from_note_str_list(note_str_list: &Vec<String>) -> Vec<Note> {
    let mut notes = Vec::new();
    for note_str in note_str_list {
        let note = Note::new(note_str);
        notes.push(note);
    }
    notes
}
pub fn get_lowest_note(notes: &Vec<Note>) -> &Note {
    let mut result = &notes[0];
    let mut i = 1;
    while i < notes.len() {
        let curr_note_info = &notes[i];
        match curr_note_info.compare_to(result) {
            CompareEnum::Lower => {
                result = curr_note_info;
            }
            CompareEnum::Equal => {
                // Fine.
            }
            CompareEnum::Higher => {
                // Fine.
            }
        }
        i += 1;
    }
    result
}
