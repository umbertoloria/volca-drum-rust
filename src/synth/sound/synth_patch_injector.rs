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
    pub fn get_synth_patch_wrapper(&self, t0_ms: u128) -> SynthPatchWrapper {
        SynthPatchWrapper::new(t0_ms, self.synth_patch.clone())
    }
}

pub struct SynthPatchWrapper {
    t0_ms: u128,
    synth_patch: SynthPatch,
}
impl SynthPatchWrapper {
    pub fn new(t0_ms: u128, synth_patch: SynthPatch) -> Self {
        Self { t0_ms, synth_patch }
    }
    pub fn get_sample(&self, index: f32) -> f32 {
        self.synth_patch.get_sample(self.t0_ms, index)
    }
}
