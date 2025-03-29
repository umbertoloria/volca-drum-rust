use crate::devices::volca_drum::volca_drum::{DRUM_CH_HH, DRUM_CH_KICK, DRUM_CH_SNARE};
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

        mute_layer_2(device, DRUM_CH_KICK);
        mute_layer_2(device, DRUM_CH_HH);
        mute_layer_2(device, DRUM_CH_SNARE);

        result.apply_patch_on_diffs(device, volca_drum_patch);

        result
    }
    pub fn apply_patch_on_diffs(&mut self, device: &mut BoxMidiDevice, patch: VolcaDrumPatch) {
        // Layout 1
        apply_sound_layout(device, DRUM_CH_KICK, &mut self.patch.kick, patch.kick);
        apply_sound_layout(device, DRUM_CH_HH, &mut self.patch.hh, patch.hh);
        apply_sound_layout(device, DRUM_CH_SNARE, &mut self.patch.snare, patch.snare);
        // TODO: Manage all Sounds and all Layers
    }
}

// HIGH LEVEL
fn apply_sound_layout(
    device: &mut BoxMidiDevice,
    channel: u8,
    curr_layout_to_edit: &mut VolcaDrumPatchLayout,
    new_layout: VolcaDrumPatchLayout,
) {
    if curr_layout_to_edit.sound_src_type != new_layout.sound_src_type
        || curr_layout_to_edit.mod_type != new_layout.mod_type
        || curr_layout_to_edit.amp_eg != new_layout.amp_eg
    {
        curr_layout_to_edit.sound_src_type = new_layout.sound_src_type;
        curr_layout_to_edit.mod_type = new_layout.mod_type;
        curr_layout_to_edit.amp_eg = new_layout.amp_eg;
        set_patch_sound(
            device,
            channel,
            &curr_layout_to_edit.sound_src_type,
            &curr_layout_to_edit.mod_type,
            &curr_layout_to_edit.amp_eg,
        );
    }
    if curr_layout_to_edit.level != new_layout.level {
        curr_layout_to_edit.level = new_layout.level;
        set_patch_level(device, channel, curr_layout_to_edit.level);
    }
    if curr_layout_to_edit.pitch != new_layout.pitch {
        curr_layout_to_edit.pitch = new_layout.pitch;
        set_patch_pitch(device, channel, curr_layout_to_edit.pitch);
    }
    if curr_layout_to_edit.eg_attack != new_layout.eg_attack {
        curr_layout_to_edit.eg_attack = new_layout.eg_attack;
        set_patch_eg_att(device, channel, curr_layout_to_edit.eg_attack);
    }
    if curr_layout_to_edit.eg_release != new_layout.eg_release {
        curr_layout_to_edit.eg_release = new_layout.eg_release;
        set_patch_eg_rel(device, channel, curr_layout_to_edit.eg_release);
    }
    if curr_layout_to_edit.mod_amount != new_layout.mod_amount {
        curr_layout_to_edit.mod_amount = new_layout.mod_amount;
        set_patch_mod_amount(device, channel, curr_layout_to_edit.mod_amount);
    }
    if curr_layout_to_edit.mod_rate != new_layout.mod_rate {
        curr_layout_to_edit.mod_rate = new_layout.mod_rate;
        set_patch_mod_rate(device, channel, curr_layout_to_edit.mod_rate);
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
    println!(
        "Volca Drum: updated channel={} sound to {:?}/{:?}/{:?}",
        channel, &sound_src_type, &mod_type, &amp_eg
    );
}
pub fn set_patch_level(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_LEVEL_1, value);
    println!("Volca Drum: updated channel={} level to {}", channel, value);
}
pub fn set_patch_pitch(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_PITCH_1, value);
    println!("Volca Drum: updated channel={} pitch to {}", channel, value);
}
pub fn set_patch_eg_att(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_EG_ATT_1, value);
    println!(
        "Volca Drum: updated channel={} eg_att to {}",
        channel, value
    );
}
pub fn set_patch_eg_rel(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_EG_REL_1, value);
    println!(
        "Volca Drum: updated channel={} eg_rel to {}",
        channel, value
    );
}
pub fn set_patch_mod_amount(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_MOD_AMOUNT, value);
    println!(
        "Volca Drum: updated channel={} mod_amount to {}",
        channel, value
    );
}
pub fn set_patch_mod_rate(device: &mut BoxMidiDevice, channel: u8, value: u8) {
    send_cc_message(device, channel, CC_MOD_RATE_1, value);
    println!(
        "Volca Drum: updated channel={} mod_rate to {}",
        channel, value
    );
}
pub fn mute_layer_2(device: &mut BoxMidiDevice, channel: u8) {
    send_cc_message(device, channel, CC_LEVEL_2, 0);
    println!("Volca Drum: updated channel={} muted layer 2", channel);
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
