use crate::devices::volca_drum::volca_drum::DRUM_CH_KICK;
use crate::devices::volca_drum::volca_drum_patch::{
    VolcaDrumPatch, VolcaDrumPatchLayout, VolcaDrumPatchLayoutAmpEg, VolcaDrumPatchLayoutModType,
    VolcaDrumPatchLayoutSoundSrcType,
};
use crate::midi::midi_device::MidiDevice;

type BoxMidiDevice = Box<dyn MidiDevice>;
pub struct VolcaDrumPatchManager {
    patch: VolcaDrumPatch,
}
impl VolcaDrumPatchManager {
    pub fn new(device: &mut BoxMidiDevice, volca_drum_patch: VolcaDrumPatch) -> Self {
        let mut result = Self {
            patch: volca_drum_patch.clone(),
        };
        result.apply_sound(device, volca_drum_patch);
        result
    }
    pub fn apply_sound(&mut self, device: &mut BoxMidiDevice, volca_drum_patch: VolcaDrumPatch) {
        // FIXME: Check any changes on *any* layout, not only Kick
        if self.patch.kick.sound_src_type != volca_drum_patch.kick.sound_src_type
            || self.patch.kick.mod_type != volca_drum_patch.kick.mod_type
            || self.patch.kick.amp_eg != volca_drum_patch.kick.amp_eg
        {
            self.patch.kick.sound_src_type = volca_drum_patch.kick.sound_src_type;
            self.patch.kick.mod_type = volca_drum_patch.kick.mod_type;
            self.patch.kick.amp_eg = volca_drum_patch.kick.amp_eg;
            self.set_patch_sound(
                device,
                DRUM_CH_KICK,
                &self.patch.kick.sound_src_type,
                &self.patch.kick.mod_type,
                &self.patch.kick.amp_eg,
            );
        }
        if self.patch.kick.level != volca_drum_patch.kick.level {
            self.patch.kick.level = volca_drum_patch.kick.level;
            self.set_patch_level(device, DRUM_CH_KICK, self.patch.kick.level as u8);
        }
        if self.patch.kick.pitch != volca_drum_patch.kick.pitch {
            self.patch.kick.pitch = volca_drum_patch.kick.pitch;
            self.set_patch_pitch(device, DRUM_CH_KICK, self.patch.kick.pitch as u8);
        }
        if self.patch.kick.eg_attack != volca_drum_patch.kick.eg_attack {
            self.patch.kick.eg_attack = volca_drum_patch.kick.eg_attack;
            self.set_patch_eg_att(device, DRUM_CH_KICK, self.patch.kick.eg_attack as u8);
        }
        if self.patch.kick.eg_release != volca_drum_patch.kick.eg_release {
            self.patch.kick.eg_release = volca_drum_patch.kick.eg_release;
            self.set_patch_eg_rel(device, DRUM_CH_KICK, self.patch.kick.eg_release as u8);
        }
        if self.patch.kick.mod_amount != volca_drum_patch.kick.mod_amount {
            self.patch.kick.mod_amount = volca_drum_patch.kick.mod_amount;
            self.set_patch_mod_amount(device, DRUM_CH_KICK, self.patch.kick.mod_amount as u8);
        }
        if self.patch.kick.mod_rate != volca_drum_patch.kick.mod_rate {
            self.patch.kick.mod_rate = volca_drum_patch.kick.mod_rate;
            self.set_patch_mod_rate(device, DRUM_CH_KICK, self.patch.kick.mod_rate as u8);
        }

        // self.mute_layer_2(device, DRUM_CH_KICK);
        // self.mute_layer_2(device, DRUM_CH_HH);
        // self.mute_layer_2(device, DRUM_CH_SNARE);

        /* // Old:
        self.apply_patch_on_layout_1(device, DRUM_CH_KICK, volca_drum_patch.kick);
        self.mute_layer_2(device, DRUM_CH_KICK);

        self.apply_patch_on_layout_1(device, DRUM_CH_HH, volca_drum_patch.hh);
        self.mute_layer_2(device, DRUM_CH_HH);

        self.apply_patch_on_layout_1(device, DRUM_CH_SNARE, volca_drum_patch.snare);
        self.mute_layer_2(device, DRUM_CH_SNARE);
        */
    }
    fn apply_patch_on_layout_1(
        &self,
        device: &mut BoxMidiDevice,
        channel: u8,
        patch_layout: VolcaDrumPatchLayout,
    ) {
        // Maybe well be removed :_(
        // println!("Applying patch layout: {:?}", patch_layout);
        self.set_patch_sound(
            device,
            channel,
            &patch_layout.sound_src_type,
            &patch_layout.mod_type,
            &patch_layout.amp_eg,
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
        sound_src_type: &VolcaDrumPatchLayoutSoundSrcType,
        mod_type: &VolcaDrumPatchLayoutModType,
        amp_eg: &VolcaDrumPatchLayoutAmpEg,
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
        println!(
            "Volca Drum: updated channel={} sound to {:?}/{:?}/{:?}",
            channel, &sound_src_type, &mod_type, &amp_eg
        );
    }
    pub fn set_patch_level(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_LEVEL_1, value);
        println!("Volca Drum: updated channel={} level to {}", channel, value);
    }
    pub fn set_patch_pitch(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_PITCH_1, value);
        println!("Volca Drum: updated channel={} pitch to {}", channel, value);
    }
    pub fn set_patch_eg_att(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_EG_ATT_1, value);
        println!(
            "Volca Drum: updated channel={} eg_att to {}",
            channel, value
        );
    }
    pub fn set_patch_eg_rel(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_EG_REL_1, value);
        println!(
            "Volca Drum: updated channel={} eg_rel to {}",
            channel, value
        );
    }
    pub fn set_patch_mod_amount(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_MOD_AMOUNT, value);
        println!(
            "Volca Drum: updated channel={} mod_amount to {}",
            channel, value
        );
    }
    pub fn set_patch_mod_rate(&self, device: &mut BoxMidiDevice, channel: u8, value: u8) {
        self.send_cc_message(device, channel, CC_MOD_RATE_1, value);
        println!(
            "Volca Drum: updated channel={} mod_rate to {}",
            channel, value
        );
    }
    pub fn mute_layer_2(&self, device: &mut BoxMidiDevice, channel: u8) {
        self.send_cc_message(device, channel, CC_LEVEL_2, 0);
        println!("Volca Drum: updated channel={} muted layer 2", channel);
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
