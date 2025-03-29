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
    pub level: usize,
    pub pitch: usize,
    pub eg_attack: usize,
    pub eg_release: usize,
    pub mod_amount: usize,
    pub mod_rate: usize,
}
#[derive(Clone, Debug)]
pub enum VolcaDrumPatchLayoutSoundSrcType {
    WaveSine,
    WaveSaw,
    WaveNoiseHPF,
    WaveNoiseLPF,
    WaveNoiseBPF,
}
#[derive(Clone, Debug)]
pub enum VolcaDrumPatchLayoutModType {
    ModExp,
    ModTri,
    ModRand,
}
#[derive(Clone, Debug)]
pub enum VolcaDrumPatchLayoutAmpEg {
    EnvAd,
    EnvExp,
    EnvMul,
}
