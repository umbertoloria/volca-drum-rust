use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_tables::{create_wt_saw, create_wt_square};
use crate::synth::synth_generator::SynthGenerator;

pub fn make_patch_1_for_keys() -> SynthGenerator {
    SynthGenerator::new(
        //
        // create_wt_sine(),
        create_wt_saw(),
        LFO::new(30, 800, 0.7),
        0.3,
    )
}

pub fn make_patch_1_for_bass() -> SynthGenerator {
    SynthGenerator::new(
        //
        create_wt_square(),
        LFO::new(17, 300, 0.3),
        0.4,
    )
}
