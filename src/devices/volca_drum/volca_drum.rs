use crate::devices::volca_drum::volca_drum_patch::{
    VolcaDrumPatchLayoutAmpEg, VolcaDrumPatchLayoutModType, VolcaDrumPatchLayoutSoundSrcType,
};
use crate::midi::midi_device::MidiDevice;

pub const DRUM_CH_KICK: u8 = 0;
pub const DRUM_CH_HH: u8 = 1;
pub const DRUM_CH_SNARE: u8 = 2;
// TODO: Understand what's the "right" default note value.
const DEFAULT_NOTE_VALUE: u8 = 7;

pub struct VolcaDrum {
    pub device: Box<dyn MidiDevice>,
}
impl VolcaDrum {
    pub fn new(device: impl MidiDevice + 'static) -> Self {
        Self {
            device: Box::new(device),
        }
    }

    // HIGH LEVEL
    pub fn hit_hh(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_HH);
    }
    pub fn hit_kick(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_KICK);
    }
    pub fn hit_snare(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_SNARE);
    }
    fn hit(&mut self, note: u8, instr: u8) {
        // TODO: Improve if possible
        const PROGRAM_CHANGE: u8 = 0xC0;
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x70;

        self.send_plain_message(PROGRAM_CHANGE, instr, 0);
        // let _ = volca_drum.send(&[PROGRAM_CHANGE, instr]);

        self.send_plain_message(NOTE_ON_MSG, note, VELOCITY);

        // Are we sure that no wait is fine?
        // sleep(Duration::from_millis(50));
        self.send_plain_message(NOTE_OFF_MSG, note, VELOCITY);
    }

    // SOUNDS (Layout 1 only)
    pub fn set_patch_sound(
        &mut self,
        channel: u8,
        sound_src_type: VolcaDrumPatchLayoutSoundSrcType,
        mod_type: VolcaDrumPatchLayoutModType,
        amp_eg: VolcaDrumPatchLayoutAmpEg,
    ) {
        let source_type_value = match sound_src_type {
            VolcaDrumPatchLayoutSoundSrcType::WaveSine => 0,
            VolcaDrumPatchLayoutSoundSrcType::WaveSaw => 26,
            VolcaDrumPatchLayoutSoundSrcType::WaveNoiseHPF => 52,
            VolcaDrumPatchLayoutSoundSrcType::WaveNoiseLPF => 77,
            VolcaDrumPatchLayoutSoundSrcType::WaveNoiseBPF => 103,
        };
        let mod_type_value = match mod_type {
            VolcaDrumPatchLayoutModType::ModExp => 0,
            VolcaDrumPatchLayoutModType::ModTri => 9,
            VolcaDrumPatchLayoutModType::ModRand => 18,
        };
        let amp_eg_value = match amp_eg {
            VolcaDrumPatchLayoutAmpEg::EnvAd => 0,
            VolcaDrumPatchLayoutAmpEg::EnvExp => 3,
            VolcaDrumPatchLayoutAmpEg::EnvMul => 6,
        };
        let value = source_type_value + mod_type_value + amp_eg_value;
        self.send_cc_message(channel, CC_SOUND_1, value);
    }
    pub fn set_patch_level(&mut self, channel: u8, value: u8) {
        self.send_cc_message(channel, CC_LEVEL_1, value);
    }
    pub fn set_patch_pitch(&mut self, channel: u8, value: u8) {
        self.send_cc_message(channel, CC_PITCH_1, value);
    }
    pub fn set_patch_eg_att(&mut self, channel: u8, value: u8) {
        self.send_cc_message(channel, CC_EG_ATT_1, value);
    }
    pub fn set_patch_eg_rel(&mut self, channel: u8, value: u8) {
        self.send_cc_message(channel, CC_EG_REL_1, value);
    }
    pub fn set_patch_mod_amount(&mut self, channel: u8, value: u8) {
        self.send_cc_message(channel, CC_MOD_AMOUNT, value);
    }
    pub fn set_patch_mod_rate(&mut self, channel: u8, value: u8) {
        self.send_cc_message(channel, CC_MOD_RATE_1, value);
    }
    pub fn mute_layer_2(&mut self, channel: u8) {
        self.send_cc_message(channel, CC_LEVEL_2, 0);
    }

    // LOW LEVEL
    fn send_cc_message(&mut self, channel: u8, cc_number: u8, value: u8) {
        self.send_plain_message(
            // 1
            0xb0 | (channel & 0x0f),
            // 2
            cc_number & 0x7f,
            // 3
            value & 0x7f,
        );
    }
    fn send_plain_message(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.device.send(a, b, c);
    }
}

const CC_SOUND_1: u8 = 14;
const CC_LEVEL_1: u8 = 17;
const CC_PITCH_1: u8 = 26;
const CC_EG_ATT_1: u8 = 20;
const CC_EG_REL_1: u8 = 23;
const CC_MOD_AMOUNT: u8 = 29;
const CC_MOD_RATE_1: u8 = 46;
const CC_LEVEL_2: u8 = 18;
