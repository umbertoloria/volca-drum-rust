use crate::synth::sound::synth_chain_injector::{GenericSynthChainInjector, SoundSynthPatch};
use crate::synth::synth_generator::SynthGenerator;

pub fn make_patch_1_for_keys() -> SynthGenerator {
    SynthGenerator::new(
        //
        GenericSynthChainInjector::new_box(SoundSynthPatch::SAW),
        0.3,
    )
}

pub fn make_patch_1_for_bass() -> SynthGenerator {
    SynthGenerator::new(
        //
        GenericSynthChainInjector::new_box(SoundSynthPatch::SQUARE),
        0.4,
    )
}
