use crate::midi_device::MidiDevice;

const ONE_OCTAVE_OFFSET: u8 = 12;

pub struct VolcaKeys {
    pub device: Box<dyn MidiDevice>,
}
impl VolcaKeys {
    pub fn new(device: impl MidiDevice + 'static) -> Self {
        Self {
            device: Box::new(device),
        }
    }

    // HIGH LEVEL
    pub fn note_play_start(&mut self, note_str: String) {
        // TODO: Start playing note "note"

        // TODO: Improve if possible
        const PROGRAM_CHANGE: u8 = 0xC0;
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x70;

        let note_bytes = note_str.as_bytes();
        let note_bytes_param: [u8; 2] = [
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
        ];
        let note = ONE_OCTAVE_OFFSET
            + (get_octave_offset_from_letter(note_bytes[note_bytes.len() - 1])
                + get_note_offset_from_letter(note_bytes_param));

        // println!("note_bytes_param {:?}, NOTE={}", note_bytes_param, note);

        // TODO: Set sounds
        let instr = 1;
        self.send_plain_message(PROGRAM_CHANGE, instr, 0);
        // let _ = volca_drum.send(&[PROGRAM_CHANGE, instr]);

        self.send_plain_message(NOTE_ON_MSG, note, VELOCITY);

        // Are we sure that no wait is fine?
        // sleep(duration.mul_f64(BPM_DEFAULT).div_f64(self.bpm));
        self.send_plain_message(NOTE_OFF_MSG, note, VELOCITY);
    }

    // LOW LEVEL
    pub fn send_plain_message(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.device.send(a, b, c);
    }
}

fn get_note_offset_from_letter(letter_byte: [u8; 2]) -> u8 {
    match (letter_byte[0], letter_byte[1]) {
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

        _ => 0,
    }
    // TODO: Log for unknown note letter
}

fn get_note_alteration_number(alteration: u8) -> i8 {
    match alteration {
        b'b' => -1,
        b'#' => 1,
        _ => 0,
    }
    // TODO: Log for unknown note letter
}

fn get_octave_offset_from_letter(number_byte: u8) -> u8 {
    ONE_OCTAVE_OFFSET
        * match number_byte {
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
