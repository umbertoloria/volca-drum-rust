use crate::devices::volca_drum::volca_drum::{
    DRUM_CH_HH, DRUM_CH_KICK, DRUM_CH_SNARE, DRUM_CH_SOUND_4, DRUM_CH_SOUND_5, DRUM_CH_SOUND_6,
};
use crate::devices::volca_drum::volca_drum_patch::{
    VolcaDrumPatch, VolcaDrumPatchLayout, VolcaDrumPatchLayoutAmpEg, VolcaDrumPatchLayoutModType,
    VolcaDrumPatchLayoutSoundSrcType,
};
use crate::midi::midi_device::MidiDevice;

// TODO: Support Waveguide Resonator on Volca Drum
type BoxMidiDevice = Box<dyn MidiDevice>;
pub struct VolcaDrumPatchManager {
    patch: VolcaDrumPatch,
}
impl VolcaDrumPatchManager {
    pub fn new(device: &mut BoxMidiDevice, volca_drum_patch: VolcaDrumPatch) -> Self {
        let mut result = Self {
            patch: volca_drum_patch.clone(),
        };

        // Layer 1
        result.apply_patch_on_diffs_or_force(device, volca_drum_patch, true);

        // Layer 2 (mute)
        mute_layer_2(device, DRUM_CH_KICK);
        mute_layer_2(device, DRUM_CH_HH);
        mute_layer_2(device, DRUM_CH_SNARE);
        mute_layer_2(device, DRUM_CH_SOUND_4);
        mute_layer_2(device, DRUM_CH_SOUND_5);
        mute_layer_2(device, DRUM_CH_SOUND_6);

        result
    }
    pub fn apply(&mut self, device: &mut BoxMidiDevice, patch: VolcaDrumPatch) {
        self.apply_patch_on_diffs_or_force(device, patch, false);
    }
    fn apply_patch_on_diffs_or_force(
        &mut self,
        device: &mut BoxMidiDevice,
        patch: VolcaDrumPatch,
        force: bool,
    ) {
        // Layout 1
        apply_patch_layout(
            DRUM_CH_KICK,
            patch.kick,
            &mut self.patch.kick,
            device,
            force,
        );
        apply_patch_layout(DRUM_CH_HH, patch.hh, &mut self.patch.hh, device, force);
        apply_patch_layout(
            DRUM_CH_SNARE,
            patch.snare,
            &mut self.patch.snare,
            device,
            force,
        );
        apply_patch_layout(
            DRUM_CH_SOUND_4,
            patch.sound4,
            &mut self.patch.sound4,
            device,
            force,
        );
        apply_patch_layout(
            DRUM_CH_SOUND_5,
            patch.sound5,
            &mut self.patch.sound5,
            device,
            force,
        );
        apply_patch_layout(
            DRUM_CH_SOUND_6,
            patch.sound6,
            &mut self.patch.sound6,
            device,
            force,
        );
    }
}

// HIGH LEVEL
fn apply_patch_layout(
    channel: u8,
    new_layout: VolcaDrumPatchLayout,
    mut_curr_layout: &mut VolcaDrumPatchLayout,
    device: &mut BoxMidiDevice,
    force: bool,
) {
    if force
        || mut_curr_layout.sound_src_type != new_layout.sound_src_type
        || mut_curr_layout.mod_type != new_layout.mod_type
        || mut_curr_layout.amp_eg != new_layout.amp_eg
    {
        mut_curr_layout.sound_src_type = new_layout.sound_src_type;
        mut_curr_layout.mod_type = new_layout.mod_type;
        mut_curr_layout.amp_eg = new_layout.amp_eg;
        set_patch_sound(
            device,
            channel,
            &mut_curr_layout.sound_src_type,
            &mut_curr_layout.mod_type,
            &mut_curr_layout.amp_eg,
        );
    }
    if force || mut_curr_layout.level != new_layout.level {
        mut_curr_layout.level = new_layout.level;
        set_patch_level(device, channel, mut_curr_layout.level);
    }
    if force || mut_curr_layout.pitch != new_layout.pitch {
        mut_curr_layout.pitch = new_layout.pitch;
        set_patch_pitch(device, channel, mut_curr_layout.pitch);
    }
    if force || mut_curr_layout.eg_attack != new_layout.eg_attack {
        mut_curr_layout.eg_attack = new_layout.eg_attack;
        set_patch_eg_att(device, channel, mut_curr_layout.eg_attack);
    }
    if force || mut_curr_layout.eg_release != new_layout.eg_release {
        mut_curr_layout.eg_release = new_layout.eg_release;
        set_patch_eg_rel(device, channel, mut_curr_layout.eg_release);
    }
    if force || mut_curr_layout.mod_amount != new_layout.mod_amount {
        mut_curr_layout.mod_amount = new_layout.mod_amount;
        set_patch_mod_amount(device, channel, mut_curr_layout.mod_amount);
    }
    if force || mut_curr_layout.mod_rate != new_layout.mod_rate {
        mut_curr_layout.mod_rate = new_layout.mod_rate;
        set_patch_mod_rate(device, channel, mut_curr_layout.mod_rate);
    }
}

const CC_SOUND_1: u8 = 14;
const CC_LEVEL_1: u8 = 17;
const CC_PITCH_1: u8 = 26;
const CC_EG_ATT_1: u8 = 20;
const CC_EG_REL_1: u8 = 23;
const CC_MOD_AMOUNT_1: u8 = 29;
const CC_MOD_RATE_1: u8 = 46;
// const CC_SOUND_2: u8 = CC_SOUND_1 + 1;
const CC_LEVEL_2: u8 = CC_LEVEL_1 + 1;
// const CC_PITCH_2: u8 = CC_PITCH_1 + 1;
// const CC_EG_ATT_2: u8 = CC_EG_ATT_1 + 1;
// const CC_EG_REL_2: u8 = CC_EG_REL_1 + 1;
// const CC_MOD_AMOUNT_2: u8 = CC_MOD_AMOUNT_1 + 1;
// const CC_MOD_RATE_2: u8 = CC_MOD_RATE_1 + 1;
pub fn set_patch_sound(
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
    send_cc_message(device, channel, CC_SOUND_1, value);
    /*println!(
        "Volca Drum: updated channel={} sound to {:?}/{:?}/{:?}",
        channel, &sound_src_type, &mod_type, &amp_eg
    );*/
}
pub fn set_patch_level(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_LEVEL_1, value);
    // println!("Volca Drum: updated channel={} level to {}", channel, value);
}
pub fn set_patch_pitch(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_PITCH_1, value);
    // println!("Volca Drum: updated channel={} pitch to {}", channel, value);
}
pub fn set_patch_eg_att(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_EG_ATT_1, value);
    /*println!(
        "Volca Drum: updated channel={} eg_att to {}",
        channel, value
    );*/
}
pub fn set_patch_eg_rel(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_EG_REL_1, value);
    /*println!(
        "Volca Drum: updated channel={} eg_rel to {}",
        channel, value
    );*/
}
pub fn set_patch_mod_amount(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_MOD_AMOUNT_1, value);
    /*println!(
        "Volca Drum: updated channel={} mod_amount to {}",
        channel, value
    );*/
}
pub fn set_patch_mod_rate(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_MOD_RATE_1, value);
    /*println!(
        "Volca Drum: updated channel={} mod_rate to {}",
        channel, value
    );*/
}
pub fn mute_layer_2(device: &mut BoxMidiDevice, channel: u8) {
    send_cc_message(device, channel, CC_LEVEL_2, 0);
    // println!("Volca Drum: updated channel={} muted layer 2", channel);
}

// LOW LEVEL
fn send_cc_message(device: &mut BoxMidiDevice, channel: u8, cc_number: u8, value_up_to_255: u8) {
    let safe_value = value_up_to_255 / 2;
    // println!(" -> send {}", safe_value);
    send_plain_message(
        device,
        // 1
        0xb0 | (channel & 0x0f),
        // 2
        cc_number & 0x7f,
        // 3
        safe_value & 0x7f,
    );
}
fn send_plain_message(device: &mut BoxMidiDevice, a: u8, b: u8, c: u8) {
    let _ = device.send(a, b, c);
}
