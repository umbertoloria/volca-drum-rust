use crate::midi_controller::bridge_send_message;
use crate::yaml_patch_reader::{
    YamlPatchFile, YamlPatchLayout, YamlPatchLayoutAmpEg, YamlPatchLayoutModulationType,
    YamlPatchLayoutSoundSrcType,
};
use midir::MidiOutputConnection;

// DRUM CHANNELS
pub const DRUM_CH_KICK: u8 = 0;
pub const DRUM_CH_HH: u8 = 1;
pub const DRUM_CH_SNARE: u8 = 2;

// CC NUMBERS
const CC_NUMBER_LAYOUT_1_SOUND: u8 = 14;
const CC_NUMBER_LAYOUT_2_LEVEL: u8 = 18; // Used for disabling layout 2 sounds (for now).

pub struct SoundPanel<'a> {
    pub conn: &'a mut MidiOutputConnection,
}
impl SoundPanel<'_> {
    // Settings from YAML FILE
    pub fn set_from_patch(&mut self, patch: YamlPatchFile) {
        self.set_to_channel_just_patch_layout_1(DRUM_CH_KICK, patch.kick);
        self.disable_layout_2_sounds(DRUM_CH_KICK);

        self.set_to_channel_just_patch_layout_1(DRUM_CH_HH, patch.hh);
        self.disable_layout_2_sounds(DRUM_CH_HH);

        self.set_to_channel_just_patch_layout_1(DRUM_CH_SNARE, patch.snare);
        self.disable_layout_2_sounds(DRUM_CH_SNARE);

        // TODO: Use the other 3 sounds

        /*
        // Before, here there was a manual params set via console input...
        self.set_param_level(0, ParamSoundType::Level, 127);
        loop {
            let value = get_console_int_input("Give me pitch value: ", 0, 127);
            if value == 0 {
                break;
            }

            // Before, it was: "drummer.send_cc_message(0, 26, sine_val_u8);"
            self.set_param_level(0, ParamSoundType::Pitch, value);
        }
        */
    }

    pub fn set_to_channel_just_patch_layout_1(
        &mut self,
        channel: u8,
        patch_layout: YamlPatchLayout,
    ) {
        // Sound Source Type
        match patch_layout.sound_src_type {
            YamlPatchLayoutSoundSrcType::WaveSine => {
                self.set_sound_source_type(channel, SoundSourceType::WaveSine)
            }
            YamlPatchLayoutSoundSrcType::WaveSaw => {
                self.set_sound_source_type(channel, SoundSourceType::WaveSaw)
            }
            YamlPatchLayoutSoundSrcType::WaveNoiseHPF => {
                self.set_sound_source_type(channel, SoundSourceType::WaveNoiseHPF)
            }
            YamlPatchLayoutSoundSrcType::WaveNoiseLPF => {
                self.set_sound_source_type(channel, SoundSourceType::WaveNoiseLPF)
            }
            YamlPatchLayoutSoundSrcType::WaveNoiseBPF => {
                self.set_sound_source_type(channel, SoundSourceType::WaveNoiseBPF)
            }
        };
        // Modulation Type
        match patch_layout.mod_type {
            YamlPatchLayoutModulationType::ModExp => {
                self.set_modulation_type(channel, ModulationType::ModExp)
            }
            YamlPatchLayoutModulationType::ModTri => {
                self.set_modulation_type(channel, ModulationType::ModTri)
            }
            YamlPatchLayoutModulationType::ModRand => {
                self.set_modulation_type(channel, ModulationType::ModRand)
            }
        };
        // Amp Eg
        match patch_layout.amp_eg {
            YamlPatchLayoutAmpEg::EnvAd => self.set_amp_eg(channel, AmpEg::EnvAd),
            YamlPatchLayoutAmpEg::EnvExp => self.set_amp_eg(channel, AmpEg::EnvExp),
            YamlPatchLayoutAmpEg::EnvMul => self.set_amp_eg(channel, AmpEg::EnvMul),
        };
        // Params
        self.set_param_level(channel, ParamSoundType::Level, patch_layout.level as u8);
        self.set_param_level(channel, ParamSoundType::Pitch, patch_layout.pitch as u8);
        self.set_param_level(
            channel,
            ParamSoundType::EgAttack,
            patch_layout.eg_attack as u8,
        );
        self.set_param_level(
            channel,
            ParamSoundType::EgRelease,
            patch_layout.eg_release as u8,
        );
        self.set_param_level(
            channel,
            ParamSoundType::ModAmount,
            patch_layout.mod_amount as u8,
        );
        self.set_param_level(
            channel,
            ParamSoundType::ModRate,
            patch_layout.mod_rate as u8,
        );
    }
    fn disable_layout_2_sounds(&mut self, channel: u8) {
        self.send_cc_message(channel, CC_NUMBER_LAYOUT_2_LEVEL, 0)
    }

    // Manual set
    fn set_sound_source_type(&mut self, channel: u8, sound_source_type: SoundSourceType) {
        self.send_cc_message(
            channel,
            CC_NUMBER_LAYOUT_1_SOUND,
            from_sound_source_type_to_value(sound_source_type),
        );
    }
    fn set_modulation_type(&mut self, channel: u8, modulation_type: ModulationType) {
        self.send_cc_message(
            channel,
            CC_NUMBER_LAYOUT_1_SOUND,
            from_modulation_type_to_value(modulation_type),
        );
    }
    fn set_amp_eg(&mut self, channel: u8, amp_eg: AmpEg) {
        self.send_cc_message(
            channel,
            CC_NUMBER_LAYOUT_1_SOUND,
            from_amp_eg_to_value(amp_eg),
        );
    }
    fn set_param_level(&mut self, channel: u8, param: ParamSoundType, value: u8) {
        self.send_cc_message(channel, from_param_to_cc_number(param), value);
    }

    // MIDI communication
    fn send_cc_message(&mut self, channel: u8, cc_number: u8, value: u8) {
        bridge_send_message(
            self.conn,
            0xb0 | (channel & 0x0f),
            cc_number & 0x7f,
            value & 0x7f,
        );
    }
}

fn from_sound_source_type_to_value(sound_source_type: SoundSourceType) -> u8 {
    match sound_source_type {
        SoundSourceType::WaveSine => 24,
        SoundSourceType::WaveSaw => 50,
        SoundSourceType::WaveNoiseHPF => 76,
        SoundSourceType::WaveNoiseLPF => 101,
        SoundSourceType::WaveNoiseBPF => 127,
    }
}
fn from_modulation_type_to_value(modulation_type: ModulationType::ModExp) -> u8 {
    match modulation_type {
        ModulationType::ModExp => 109,
        ModulationType::ModTri => 118,
        ModulationType::ModRand => 127,
    }
}
fn from_amp_eg_to_value(amp_eg: AmpEg) -> u8 {
    match amp_eg {
        AmpEg::EnvAd => 121,
        AmpEg::EnvExp => 124,
        AmpEg::EnvMul => 127,
    }
}
fn from_param_to_cc_number(param: ParamSoundType) -> u8 {
    match param {
        ParamSoundType::Level => 17,
        ParamSoundType::Pitch => 26,
        ParamSoundType::EgAttack => 20,
        ParamSoundType::EgRelease => 23,
        ParamSoundType::ModAmount => 29,
        ParamSoundType::ModRate => 46,
    }
}

enum SoundSourceType {
    WaveSine,
    WaveSaw,
    WaveNoiseHPF,
    WaveNoiseLPF,
    WaveNoiseBPF,
}
enum ModulationType {
    ModExp,
    ModTri,
    ModRand,
}
enum AmpEg {
    EnvAd,
    EnvExp,
    EnvMul,
}
pub enum ParamSoundType {
    Level,
    Pitch,
    EgAttack,
    EgRelease,
    ModAmount,
    ModRate,
}
