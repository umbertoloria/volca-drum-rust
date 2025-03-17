use crate::synth::sound::patches::SynthPatch;

pub struct SynthPatchInjector {
    synth_patch: SynthPatch,
    volume: f32,
}
impl SynthPatchInjector {
    pub fn new(synth_patch: SynthPatch, volume: f32) -> SynthPatchInjector {
        Self {
            synth_patch,
            volume,
        }
    }
    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn get_synth_patch(&self) -> &SynthPatch {
        &self.synth_patch
    }
}
