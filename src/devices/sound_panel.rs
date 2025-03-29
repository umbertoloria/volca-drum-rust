use crate::devices::volca_drum::{VolcaDrum, DRUM_CH_HH, DRUM_CH_KICK, DRUM_CH_SNARE};
use crate::song::yaml_patch_reader::{
    YamlPatchFile, YamlPatchLayout, YamlPatchLayoutAmpEg, YamlPatchLayoutModulationType,
    YamlPatchLayoutSoundSrcType,
};

// CC NUMBERS
const CC_NUMBER_LAYOUT_1_SOUND: u8 = 14;

pub struct SoundPanel<'a> {
    pub volca_drum: &'a mut VolcaDrum,
}
impl<'a> SoundPanel<'a> {
    pub fn new(volca_drum: &'a mut VolcaDrum) -> SoundPanel<'a> {
        Self { volca_drum }
    }
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
        // println!("Applying patch: {:?}", patch_layout);
        let source_sound_type = match patch_layout.sound_src_type {
            YamlPatchLayoutSoundSrcType::WaveSine => SoundSourceType::WaveSine,
            YamlPatchLayoutSoundSrcType::WaveSaw => SoundSourceType::WaveSaw,
            YamlPatchLayoutSoundSrcType::WaveNoiseHPF => SoundSourceType::WaveNoiseHPF,
            YamlPatchLayoutSoundSrcType::WaveNoiseLPF => SoundSourceType::WaveNoiseLPF,
            YamlPatchLayoutSoundSrcType::WaveNoiseBPF => SoundSourceType::WaveNoiseBPF,
        };
        let modulation_type = match patch_layout.mod_type {
            YamlPatchLayoutModulationType::ModExp => ModulationType::ModExp,
            YamlPatchLayoutModulationType::ModTri => ModulationType::ModTri,
            YamlPatchLayoutModulationType::ModRand => ModulationType::ModRand,
        };
        let amp_eg = match patch_layout.amp_eg {
            YamlPatchLayoutAmpEg::EnvAd => AmpEg::EnvAd,
            YamlPatchLayoutAmpEg::EnvExp => AmpEg::EnvExp,
            YamlPatchLayoutAmpEg::EnvMul => AmpEg::EnvMul,
        };
        self.set_sound_patch(
            channel,
            CC_NUMBER_LAYOUT_1_SOUND,
            source_sound_type,
            modulation_type,
            amp_eg,
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
    fn set_sound_patch(
        &mut self,
        channel: u8,
        cc_number: u8,
        sound_source_type: SoundSourceType,
        modulation_type: ModulationType,
        amp_eg: AmpEg,
    ) {
        let source_type_value = match sound_source_type {
            SoundSourceType::WaveSine => 0,
            SoundSourceType::WaveSaw => 26,
            SoundSourceType::WaveNoiseHPF => 52,
            SoundSourceType::WaveNoiseLPF => 77,
            SoundSourceType::WaveNoiseBPF => 103,
        };
        let modulation_type_value = match modulation_type {
            ModulationType::ModExp => 0,
            ModulationType::ModTri => 9,
            ModulationType::ModRand => 18,
        };
        let amp_eg_value = match amp_eg {
            AmpEg::EnvAd => 0,
            AmpEg::EnvExp => 3,
            AmpEg::EnvMul => 6,
        };
        let value = source_type_value + modulation_type_value + amp_eg_value;
        self.volca_drum.send_cc_message(channel, cc_number, value);
    }
    fn set_param_level(&mut self, channel: u8, param: ParamSoundType, value: u8) {
        let cc_number = match param {
            // Layout 1
            ParamSoundType::Level1 => 17,
            ParamSoundType::Pitch1 => 26,
            ParamSoundType::EgAttack1 => 20,
            ParamSoundType::EgRelease1 => 23,
            ParamSoundType::ModAmount1 => 29,
            ParamSoundType::ModRate1 => 46,
            // Layout 2
            ParamSoundType::Level2 => 18, // Layout 2 Level is only used to disable Layout 2.
        };
        self.volca_drum.send_cc_message(channel, cc_number, value);
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
