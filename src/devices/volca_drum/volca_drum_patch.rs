#[derive(Clone, Debug)]
pub struct VolcaDrumPatch {
    pub kick: VolcaDrumPatchLayout,
    pub hh: VolcaDrumPatchLayout,
    pub snare: VolcaDrumPatchLayout,
    pub sound4: VolcaDrumPatchLayout,
    pub sound5: VolcaDrumPatchLayout,
    pub sound6: VolcaDrumPatchLayout,
}
#[derive(Clone, Debug)]
pub struct VolcaDrumPatchLayout {
    pub sound_src_type: VolcaDrumPatchLayoutSoundSrcType,
    pub mod_type: VolcaDrumPatchLayoutModType,
    pub amp_eg: VolcaDrumPatchLayoutAmpEg,
    pub level: u8,      // From 0 to 255.
    pub pitch: u8,      // From 0 to 255.
    pub eg_attack: u8,  // From 0 to 255.
    pub eg_release: u8, // From 0 to 255.
    pub mod_amount: u8, // TODO: Should go from -100 to +100. For now 128=0.
    pub mod_rate: u8,   // From 0 to 255.
}
#[derive(Clone, Debug, PartialEq)]
pub enum VolcaDrumPatchLayoutSoundSrcType {
    WaveSine,
    WaveSaw,
    WaveNoiseHPF,
    WaveNoiseLPF,
    WaveNoiseBPF,
}
#[derive(Clone, Debug, PartialEq)]
pub enum VolcaDrumPatchLayoutModType {
    ModExp,
    ModTri,
    ModRand,
}
#[derive(Clone, Debug, PartialEq)]
pub enum VolcaDrumPatchLayoutAmpEg {
    EnvAd,
    EnvExp,
    EnvMul,
}
