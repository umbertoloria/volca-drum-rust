use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlPatchFile {
    pub kick: YamlPatchLayout,
    pub hh: YamlPatchLayout,
    pub snare: YamlPatchLayout,
    sound4: YamlPatchLayout,
    sound5: YamlPatchLayout,
    sound6: YamlPatchLayout,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlPatchLayout {
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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum YamlPatchLayoutSoundSrcType {
    WaveSine,
    WaveSaw,
    WaveNoiseHPF,
    WaveNoiseLPF,
    WaveNoiseBPF,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum YamlPatchLayoutModulationType {
    ModExp,
    ModTri,
    ModRand,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum YamlPatchLayoutAmpEg {
    EnvAd,
    EnvExp,
    EnvMul,
}

pub fn read_patch_from_yaml(filepath: &str) -> YamlPatchFile {
    let contents = fs::read_to_string(filepath).expect("Unable to read patch file");

    let data_song: YamlPatchFile =
        serde_yaml::from_str(&contents).expect("Unable to parse patch YAML file");

    data_song
}
