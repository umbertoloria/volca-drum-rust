use crate::synth::sound::synth_patch::SynthPatch;
use crate::synth::sound::synth_patch_injector::SynthPatchInjector;

pub fn make_patch_1_for_keys() -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::SAW, 0.3)
}
pub fn make_patch_1_for_bass() -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::SQUARE, 0.4)
}
pub fn make_patch_1_for_metronome() -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::MetronomeClick, 0.8)
}
