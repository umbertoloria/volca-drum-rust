use crate::synth::sound::synth_chain_injector::{SawSynthChainInjector, SquareSynthChainInjector};
use crate::synth::synth_generator::SynthGenerator;

pub fn make_patch_1_for_keys() -> SynthGenerator {
    SynthGenerator::new(
        //
        SawSynthChainInjector::new_box(),
        0.3,
    )
}

pub fn make_patch_1_for_bass() -> SynthGenerator {
    SynthGenerator::new(
        //
        SquareSynthChainInjector::new_box(),
        0.4,
    )
}
