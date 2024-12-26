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

pub struct SoundPanel<'a> {
    pub conn: &'a mut MidiOutputConnection,
}
impl SoundPanel<'_> {
    pub fn set_from_patch(&mut self, patch: YamlPatchFile) {
        self.config_patch_onto_channel_and_layout1(DRUM_CH_KICK, patch.kick);
        self.disable_layout_2_sounds(DRUM_CH_KICK);

        self.config_patch_onto_channel_and_layout1(DRUM_CH_HH, patch.hh);
        self.disable_layout_2_sounds(DRUM_CH_HH);

        self.config_patch_onto_channel_and_layout1(DRUM_CH_SNARE, patch.snare);
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

    // Settings from YAML FILE
    pub fn config_patch_onto_channel_and_layout1(
        &mut self,
        channel: u8,
        patch_layout: YamlPatchLayout,
    ) {
        self.set_sound_source_type(
            channel,
            CC_NUMBER_LAYOUT_1_SOUND,
            match patch_layout.sound_src_type {
                YamlPatchLayoutSoundSrcType::WaveSine => SoundSourceType::WaveSine,
                YamlPatchLayoutSoundSrcType::WaveSaw => SoundSourceType::WaveSaw,
                YamlPatchLayoutSoundSrcType::WaveNoiseHPF => SoundSourceType::WaveNoiseHPF,
                YamlPatchLayoutSoundSrcType::WaveNoiseLPF => SoundSourceType::WaveNoiseLPF,
                YamlPatchLayoutSoundSrcType::WaveNoiseBPF => SoundSourceType::WaveNoiseBPF,
            },
        );
        self.set_modulation_type(
            channel,
            CC_NUMBER_LAYOUT_1_SOUND,
            match patch_layout.mod_type {
                YamlPatchLayoutModulationType::ModExp => ModulationType::ModExp,
                YamlPatchLayoutModulationType::ModTri => ModulationType::ModTri,
                YamlPatchLayoutModulationType::ModRand => ModulationType::ModRand,
            },
        );
        self.set_amp_eg(
            channel,
            CC_NUMBER_LAYOUT_1_SOUND,
            match patch_layout.amp_eg {
                YamlPatchLayoutAmpEg::EnvAd => AmpEg::EnvAd,
                YamlPatchLayoutAmpEg::EnvExp => AmpEg::EnvExp,
                YamlPatchLayoutAmpEg::EnvMul => AmpEg::EnvMul,
            },
        );
        self.set_param_level(channel, ParamSoundType::Level1, patch_layout.level as u8);
        self.set_param_level(channel, ParamSoundType::Pitch1, patch_layout.pitch as u8);
        self.set_param_level(
            channel,
            ParamSoundType::EgAttack1,
            patch_layout.eg_attack as u8,
        );
        self.set_param_level(
            channel,
            ParamSoundType::EgRelease1,
            patch_layout.eg_release as u8,
        );
        self.set_param_level(
            channel,
            ParamSoundType::ModAmount1,
            patch_layout.mod_amount as u8,
        );
        self.set_param_level(
            channel,
            ParamSoundType::ModRate1,
            patch_layout.mod_rate as u8,
        );
    }
    fn disable_layout_2_sounds(&mut self, channel: u8) {
        self.set_param_level(channel, ParamSoundType::Level2, 0);
    }

    // Manual set
    fn set_sound_source_type(
        &mut self,
        channel: u8,
        cc_number: u8,
        sound_source_type: SoundSourceType,
    ) {
        self.send_cc_message(
            channel,
            cc_number,
            match sound_source_type {
                SoundSourceType::WaveSine => 24,
                SoundSourceType::WaveSaw => 50,
                SoundSourceType::WaveNoiseHPF => 76,
                SoundSourceType::WaveNoiseLPF => 101,
                SoundSourceType::WaveNoiseBPF => 127,
            },
        );
    }
    fn set_modulation_type(&mut self, channel: u8, cc_number: u8, modulation_type: ModulationType) {
        self.send_cc_message(
            channel,
            cc_number,
            match modulation_type {
                ModulationType::ModExp => 109,
                ModulationType::ModTri => 118,
                ModulationType::ModRand => 127,
            },
        );
    }
    fn set_amp_eg(&mut self, channel: u8, cc_number: u8, amp_eg: AmpEg) {
        self.send_cc_message(
            channel,
            cc_number,
            match amp_eg {
                AmpEg::EnvAd => 121,
                AmpEg::EnvExp => 124,
                AmpEg::EnvMul => 127,
            },
        );
    }
    fn set_param_level(&mut self, channel: u8, param: ParamSoundType, value: u8) {
        self.send_cc_message(
            channel,
            match param {
                // Layout 1
                ParamSoundType::Level1 => 17,
                ParamSoundType::Pitch1 => 26,
                ParamSoundType::EgAttack1 => 20,
                ParamSoundType::EgRelease1 => 23,
                ParamSoundType::ModAmount1 => 29,
                ParamSoundType::ModRate1 => 46,
                // Layout 2
                ParamSoundType::Level2 => 18, // Used for disabling layout 2 sounds (for now).
            },
            value,
        );
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
    // Layout 1
    Level1,
    Pitch1,
    EgAttack1,
    EgRelease1,
    ModAmount1,
    ModRate1,
    // Layout 2
    Level2,
}
