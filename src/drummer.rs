use crate::player::BPM_DEFAULT;
use midir::MidiOutputConnection;
use std::thread::sleep;
use std::time::Duration;

pub struct Drummer<'a> {
    pub conn: &'a mut MidiOutputConnection,
    pub bpm: f64,
}
impl Drummer<'_> {
    pub fn hit(&mut self, note: u8, instr: u8, duration: Duration) {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const PROGRAM_CHANGE: u8 = 0xC0;
        const VELOCITY: u8 = 0x70;
        // We're ignoring errors in here
        let _ = self.conn.send(&[PROGRAM_CHANGE, instr]);
        let _ = self.conn.send(&[NOTE_ON_MSG, note, VELOCITY]);
        sleep(duration.mul_f64(BPM_DEFAULT).div_f64(self.bpm));
        let _ = self.conn.send(&[NOTE_OFF_MSG, note, VELOCITY]);
    }
}
