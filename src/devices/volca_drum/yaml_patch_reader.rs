use crate::devices::volca_drum::volca_drum_patch::{
    VolcaDrumPatch, VolcaDrumPatchLayout, VolcaDrumPatchLayoutAmpEg, VolcaDrumPatchLayoutModType,
    VolcaDrumPatchLayoutSoundSrcType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlPatchFile {
    pub kick: YamlPatchLayout,
    pub hh: YamlPatchLayout,
    pub snare: YamlPatchLayout,
    pub sound4: YamlPatchLayout,
    pub sound5: YamlPatchLayout,
    pub sound6: YamlPatchLayout,
}
impl YamlPatchFile {
    pub fn get_volca_drum_patch(&self) -> VolcaDrumPatch {
        VolcaDrumPatch {
            kick: self.kick.get_volca_drum_patch_layout(),
            hh: self.hh.get_volca_drum_patch_layout(),
            snare: self.snare.get_volca_drum_patch_layout(),
            sound4: self.sound4.get_volca_drum_patch_layout(),
            sound5: self.sound5.get_volca_drum_patch_layout(),
            sound6: self.sound6.get_volca_drum_patch_layout(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct YamlPatchLayout {
    pub sound_src_type: YamlPatchLayoutSoundSrcType,
    pub mod_type: YamlPatchLayoutModulationType,
    pub amp_eg: YamlPatchLayoutAmpEg,
    pub level: usize,
    pub pitch: usize,
    pub eg_attack: usize,
    pub eg_release: usize,
    pub mod_amount: usize,
    pub mod_rate: usize,
}
impl YamlPatchLayout {
    pub fn get_volca_drum_patch_layout(&self) -> VolcaDrumPatchLayout {
        VolcaDrumPatchLayout {
            sound_src_type: match &self.sound_src_type {
                YamlPatchLayoutSoundSrcType::WaveSine => VolcaDrumPatchLayoutSoundSrcType::WaveSine,
                YamlPatchLayoutSoundSrcType::WaveSaw => VolcaDrumPatchLayoutSoundSrcType::WaveSaw,
                YamlPatchLayoutSoundSrcType::WaveNoiseHPF => {
                    VolcaDrumPatchLayoutSoundSrcType::WaveNoiseHPF
                }
                YamlPatchLayoutSoundSrcType::WaveNoiseLPF => {
                    VolcaDrumPatchLayoutSoundSrcType::WaveNoiseLPF
                }
                YamlPatchLayoutSoundSrcType::WaveNoiseBPF => {
                    VolcaDrumPatchLayoutSoundSrcType::WaveNoiseBPF
                }
            },
            mod_type: match &self.mod_type {
                YamlPatchLayoutModulationType::ModExp => VolcaDrumPatchLayoutModType::ModExp,
                YamlPatchLayoutModulationType::ModTri => VolcaDrumPatchLayoutModType::ModTri,
                YamlPatchLayoutModulationType::ModRand => VolcaDrumPatchLayoutModType::ModRand,
            },
            amp_eg: match &self.amp_eg {
                YamlPatchLayoutAmpEg::EnvAd => VolcaDrumPatchLayoutAmpEg::EnvAd,
                YamlPatchLayoutAmpEg::EnvExp => VolcaDrumPatchLayoutAmpEg::EnvExp,
                YamlPatchLayoutAmpEg::EnvMul => VolcaDrumPatchLayoutAmpEg::EnvMul,
            },
            level: self.level,
            pitch: self.pitch,
            eg_attack: self.eg_attack,
            eg_release: self.eg_release,
            mod_amount: self.mod_amount,
            mod_rate: self.mod_rate,
        }
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum YamlPatchLayoutSoundSrcType {
    WaveSine,
    WaveSaw,
    WaveNoiseHPF,
    WaveNoiseLPF,
    WaveNoiseBPF,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum YamlPatchLayoutModulationType {
    ModExp,
    ModTri,
    ModRand,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum YamlPatchLayoutAmpEg {
    EnvAd,
    EnvExp,
    EnvMul,
}

pub fn parse_patch_from_yaml(to_parse: &str) -> Option<YamlPatchFile> {
    match serde_yaml::from_str::<YamlPatchFile>(&to_parse) {
        Ok(yaml_patch_file) => Some(yaml_patch_file),
        Err(_) => None,
    }
}
