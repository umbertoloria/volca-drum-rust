use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlPatchFile {
    pub kick: YamlPatchLayout,
    pub hh: YamlPatchLayout,
    pub snare: YamlPatchLayout,
    pub sound4: YamlPatchLayout,
    pub sound5: YamlPatchLayout,
    pub sound6: YamlPatchLayout,
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
    parse_patch_from_yaml(&contents).expect("Unable to parse patch YAML file")
}

pub fn parse_patch_from_yaml(to_parse: &str) -> Option<YamlPatchFile> {
    match serde_yaml::from_str::<YamlPatchFile>(&to_parse) {
        Ok(yaml_patch_file) => Some(yaml_patch_file),
        Err(_) => None,
    }
}
