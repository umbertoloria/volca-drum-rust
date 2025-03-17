use crate::synth::sound::synth_patch::{SynthPatch, SynthPatchInjector};
use crate::synth::synth_generator::SynthGenerator;

pub fn make_patch_1_for_keys() -> SynthGenerator {
    SynthGenerator::new(
        //
        SynthPatchInjector::new(SynthPatch::SAW),
        0.3,
    )
}

pub fn make_patch_1_for_bass() -> SynthGenerator {
    SynthGenerator::new(
        //
        SynthPatchInjector::new(SynthPatch::SQUARE),
        0.4,
    )
}
