use crate::midi_controller::bridge_send_message;
use midir::MidiOutputConnection;

pub struct VolcaDrum<'a> {
    pub conn: &'a mut MidiOutputConnection,
}
impl<'a> VolcaDrum<'a> {
    pub fn send_cc_message(&mut self, channel: u8, cc_number: u8, value: u8) {
        bridge_send_message(
            self.conn,
            0xb0 | (channel & 0x0f),
            cc_number & 0x7f,
            value & 0x7f,
        );
    }
}
