use crate::devices::volca_drum::volca_drum::{DRUM_CH_HH, DRUM_CH_KICK, DRUM_CH_SNARE};
use crate::devices::volca_drum::volca_drum_patch::{
    VolcaDrumPatch, VolcaDrumPatchLayout, VolcaDrumPatchLayoutAmpEg, VolcaDrumPatchLayoutModType,
    VolcaDrumPatchLayoutSoundSrcType,
};
use crate::midi::midi_device::MidiDevice;

type BoxMidiDevice = Box<dyn MidiDevice>;
pub struct VolcaDrumPatchManager {}
impl VolcaDrumPatchManager {
    pub fn new() -> Self {
        Self {}
    }
    pub fn apply_sound(&self, device: &mut BoxMidiDevice, volca_drum_patch: VolcaDrumPatch) {
        self.apply_patch_on_layout_1(device, DRUM_CH_KICK, volca_drum_patch.kick);
        self.mute_layer_2(device, DRUM_CH_KICK);

        self.apply_patch_on_layout_1(device, DRUM_CH_HH, volca_drum_patch.hh);
        self.mute_layer_2(device, DRUM_CH_HH);

        self.apply_patch_on_layout_1(device, DRUM_CH_SNARE, volca_drum_patch.snare);
        self.mute_layer_2(device, DRUM_CH_SNARE);
    }
    fn apply_patch_on_layout_1(
        &self,
        device: &mut BoxMidiDevice,
        channel: u8,
        patch_layout: VolcaDrumPatchLayout,
    ) {
        // println!("Applying patch layout: {:?}", patch_layout);
        self.set_patch_sound(
            device,
            channel,
            patch_layout.sound_src_type,
            patch_layout.mod_type,
            patch_layout.amp_eg,
        );
        self.set_patch_level(device, channel, patch_layout.level as u8);
        self.set_patch_pitch(device, channel, patch_layout.pitch as u8);
        self.set_patch_eg_att(device, channel, patch_layout.eg_attack as u8);
        self.set_patch_eg_rel(device, channel, patch_layout.eg_release as u8);
        self.set_patch_mod_amount(device, channel, patch_layout.mod_amount as u8);
        self.set_patch_mod_rate(device, channel, patch_layout.mod_rate as u8);
    }
    // SOUND FOR CHANNEL
    pub fn set_patch_sound(
        &self,
        device: &mut BoxMidiDevice,
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
        self.send_cc_message(device, channel, CC_SOUND_1, value);
    }
    pub fn set_patch_level(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_LEVEL_1, value);
    }
    pub fn set_patch_pitch(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_PITCH_1, value);
    }
    pub fn set_patch_eg_att(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_EG_ATT_1, value);
    }
    pub fn set_patch_eg_rel(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_EG_REL_1, value);
    }
    pub fn set_patch_mod_amount(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_MOD_AMOUNT, value);
    }
    pub fn set_patch_mod_rate(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_MOD_RATE_1, value);
    }
    pub fn mute_layer_2(&self, device: &mut BoxMidiDevice, channel: u8) {
        self.send_cc_message(device, channel, CC_LEVEL_2, 0);
    }

    // LOW LEVEL
    fn send_cc_message(&self, device: &mut BoxMidiDevice, channel: u8, cc_number: u8, value: u8) {
        self.send_plain_message(
            device,
            // 1
            0xb0 | (channel & 0x0f),
            // 2
            cc_number & 0x7f,
            // 3
            value & 0x7f,
        );
    }
    fn send_plain_message(&self, device: &mut BoxMidiDevice, a: u8, b: u8, c: u8) {
        let _ = device.send(a, b, c);
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
