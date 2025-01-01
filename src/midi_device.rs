use midir::MidiOutputConnection;

pub struct MidiDevice {
    pub conn: MidiOutputConnection,
}
impl MidiDevice {
    pub fn send(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.conn.send(&[a, b, c]);
        /*println!(
            "Send msg -> [{:10} {:10} {:10}]\n            [{:#10x} {:#10x} {:#10x}]\n            [{:#10b} {:#10b} {:#10b}]",
            a, b, c,
            a, b, c,
            a, b, c,
        );*/
    }
    pub fn close(self) {
        let _ = self.conn.close();
    }
}
