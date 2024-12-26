use midir::MidiOutputConnection;

pub struct VolcaDrum<'a> {
    pub conn: &'a mut MidiOutputConnection,
}
impl<'a> VolcaDrum<'a> {
    pub fn send_plain_message(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.conn.send(&[a, b, c]);
        println!(
            "Send msg -> [{:10} {:10} {:10}]\n            [{:#10x} {:#10x} {:#10x}]\n            [{:#10b} {:#10b} {:#10b}]",
            a, b, c,
            a, b, c,
            a, b, c,
        );
    }
    pub fn send_cc_message(&mut self, channel: u8, cc_number: u8, value: u8) {
        self.send_plain_message(
            // 1
            0xb0 | (channel & 0x0f),
            // 2
            cc_number & 0x7f,
            // 3
            value & 0x7f,
        );
    }
}
