use crate::yaml_patch_reader::{
    YamlPatchFile, YamlPatchLayout, YamlPatchLayoutAmpEg, YamlPatchLayoutModulationType,
    YamlPatchLayoutSoundSrcType,
};
use midir::MidiOutputConnection;

// DRUM CHANNELS
pub const DRUM_CH_KICK: u8 = 0;
pub const DRUM_CH_HH: u8 = 1;
pub const DRUM_CH_SNARE: u8 = 2;

pub struct SoundPanel<'a> {
    pub conn: &'a mut MidiOutputConnection,
}
impl SoundPanel<'_> {
    // Settings from YAML FILE
    pub fn set_from_patch(&mut self, patch: YamlPatchFile) {
        self.set_patch_layout_to_a_channel(patch.kick, DRUM_CH_KICK);
        self.set_patch_layout_to_a_channel(patch.hh, DRUM_CH_HH);
        self.set_patch_layout_to_a_channel(patch.snare, DRUM_CH_SNARE);
        // Then, add other three channels...

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

    pub fn set_patch_layout_to_a_channel(&mut self, patch_layout: YamlPatchLayout, channel: u8) {
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
        match patch_layout.amp_eg {
            YamlPatchLayoutAmpEg::EnvAd => self.set_amp_eg(channel, AmpEg::EnvAd),
            YamlPatchLayoutAmpEg::EnvExp => self.set_amp_eg(channel, AmpEg::EnvExp),
            YamlPatchLayoutAmpEg::EnvMul => self.set_amp_eg(channel, AmpEg::EnvMul),
        };
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

    // Manual set
    pub fn set_sound_source_type(&mut self, channel: u8, sound_source_type: SoundSourceType) {
        match sound_source_type {
            SoundSourceType::WaveSine => self.send_cc_message(channel, 14, 24),
            SoundSourceType::WaveSaw => self.send_cc_message(channel, 14, 50),
            SoundSourceType::WaveNoiseHPF => self.send_cc_message(channel, 14, 76),
            SoundSourceType::WaveNoiseLPF => self.send_cc_message(channel, 14, 101),
            SoundSourceType::WaveNoiseBPF => self.send_cc_message(channel, 14, 127),
        }
    }
    pub fn set_modulation_type(&mut self, channel: u8, modulation_type: ModulationType) {
        match modulation_type {
            ModulationType::ModExp => self.send_cc_message(channel, 14, 109),
            ModulationType::ModTri => self.send_cc_message(channel, 14, 118),
            ModulationType::ModRand => self.send_cc_message(channel, 14, 127),
        }
    }
    pub fn set_amp_eg(&mut self, channel: u8, amp_eg: AmpEg) {
        match amp_eg {
            AmpEg::EnvAd => self.send_cc_message(channel, 14, 121),
            AmpEg::EnvExp => self.send_cc_message(channel, 14, 124),
            AmpEg::EnvMul => self.send_cc_message(channel, 14, 127),
        }
    }
    pub fn set_param_level(&mut self, channel: u8, param: ParamSoundType, value: u8) {
        match param {
            ParamSoundType::Level => self.send_cc_message(channel, 17, value),
            ParamSoundType::Pitch => self.send_cc_message(channel, 26, value),
            ParamSoundType::EgAttack => self.send_cc_message(channel, 20, value),
            ParamSoundType::EgRelease => self.send_cc_message(channel, 23, value),
            ParamSoundType::ModAmount => self.send_cc_message(channel, 29, value),
            ParamSoundType::ModRate => self.send_cc_message(channel, 46, value),
        }
    }

    fn send_cc_message(&mut self, channel: u8, cc_number: u8, value: u8) {
        let message: [u8; 3] = [
            // 1
            0xb0 | (channel & 0x0f),
            // 2
            cc_number & 0x7f,
            // 3
            value & 0x7f,
        ];
        let _ = self.conn.send(&message);
        // println!("Send msg -> {:?}", message);
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
