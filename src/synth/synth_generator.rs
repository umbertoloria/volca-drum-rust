use crate::synth::sound::synth_patch::{SynthPatchInjector, SynthPatchWrapper};

pub struct SynthGenerator {
    synth_patch_injector: SynthPatchInjector,
    volume: f32,
}
impl SynthGenerator {
    pub fn new(synth_patch_injector: SynthPatchInjector, volume: f32) -> Self {
        Self {
            synth_patch_injector,
            volume,
        }
    }
    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn generate_synth_patch_wrapper(&self, t0_ms: u128) -> SynthPatchWrapper {
        self.synth_patch_injector.get_synth_patch_wrapper(t0_ms)
    }
}
