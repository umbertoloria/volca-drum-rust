use crate::synth::sound::synth_chain_injector::SquareSynthChainInjector;
use crate::synth::synth_generator::SynthGenerator;

pub fn make_patch_1_for_keys() -> SynthGenerator {
    SynthGenerator::new(
        //
        // create_wt_sine(),
        // create_wt_saw(),
        // LFO::new(30, 800, 0.7),
        SquareSynthChainInjector::new_box(),
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
