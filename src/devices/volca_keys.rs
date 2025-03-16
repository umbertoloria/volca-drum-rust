use crate::midi::midi_device::MidiDevice;
use crate::music::note::Note;

const ONE_OCTAVE_OFFSET: u8 = 12;

const PROGRAM_CHANGE: u8 = 0xC0;
const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY: u8 = 0x70;

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
    pub fn note_play_start(&mut self, note: &Note) {
        // println!("VolcaKeys: note_play_start {}", note_str);

        // TODO: Set sounds
        let instr = 1;
        self.send_plain_message(PROGRAM_CHANGE, instr, 0);

        let midi_note = get_midi_note_from_note_str(note);
        self.send_plain_message(NOTE_ON_MSG, midi_note, VELOCITY);

        // sleep(duration.mul_f64(BPM_DEFAULT).div_f64(self.bpm));
        // self.note_play_stop(note_str);
    }

    pub fn note_play_stop(&mut self, note: &Note) {
        // println!("VolcaKeys: note_play_stop {}", note_str);

        // TODO: Set sounds
        let instr = 1;
        self.send_plain_message(PROGRAM_CHANGE, instr, 0);

        let midi_note = get_midi_note_from_note_str(note);
        self.send_plain_message(NOTE_OFF_MSG, midi_note, VELOCITY);
    }

    // LOW LEVEL
    pub fn send_plain_message(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.device.send(a, b, c);
    }
}

fn get_midi_note_from_note_str(note: &Note) -> u8 {
    ONE_OCTAVE_OFFSET + ONE_OCTAVE_OFFSET * note.octave + note.offset
}
