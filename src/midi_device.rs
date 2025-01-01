use midir::MidiOutputConnection;

pub trait MidiDevice {
    fn send(&mut self, a: u8, b: u8, c: u8);
    // fn close(self);
}

pub struct MidiDeviceConcrete {
    pub conn: MidiOutputConnection,
}
impl MidiDeviceConcrete {
    pub fn new(conn: MidiOutputConnection) -> Self {
        Self { conn }
    }
}
impl MidiDevice for MidiDeviceConcrete {
    fn send(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.conn.send(&[a, b, c]);
        /*println!(
            "Send msg -> [{:10} {:10} {:10}]\n            [{:#10x} {:#10x} {:#10x}]\n            [{:#10b} {:#10b} {:#10b}]",
            a, b, c,
            a, b, c,
            a, b, c,
        );*/
    }
}

// MIDI Device Ghost
pub struct MidiDeviceGhost {}

impl MidiDeviceGhost {
    pub fn new() -> Self {
        Self {}
    }
}

impl MidiDevice for MidiDeviceGhost {
    fn send(&mut self, a: u8, b: u8, c: u8) {
        println!("send to MIDI device ghost: {} {} {}", a, b, c)
    }
}
