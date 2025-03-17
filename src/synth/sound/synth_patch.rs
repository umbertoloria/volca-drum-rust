use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::synth::oscillator::wave_tables::{create_wt_saw, create_wt_square};
use crate::utils::timing::get_now_millis;

pub struct SynthPatchInjector {
    synth_patch: SynthPatch,
}
impl SynthPatchInjector {
    pub fn new(synth_patch: SynthPatch) -> SynthPatchInjector {
        Self { synth_patch }
    }
    pub fn get_synth_patch_wrapper(&self, t0_ms: u128) -> SynthPatchWrapper {
        // FIXME: Avoid cloning Synth Patch
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

#[derive(Clone)]
pub enum SynthPatch {
    SQUARE,
    SAW,
}
impl SynthPatch {
    fn get_sample(&self, t0_ms: u128, index: f32) -> f32 {
        match self {
            SynthPatch::SQUARE => {
                // 1. Oscillator
                let oscillator = WaveTableOscillator::new(create_wt_square());
                let oscillator_value = oscillator.lerp(index);

                // 2. LFO
                let ms = get_now_millis() - t0_ms;
                let lfo = LFO::new(17, 300, 0.3);
                let lfo_value = lfo.get_value(ms);

                oscillator_value * lfo_value
            }
            SynthPatch::SAW => {
                // 1. Oscillator
                // let oscillator = WaveTableOscillator::new(create_wt_sine());
                let oscillator = WaveTableOscillator::new(create_wt_saw());
                let oscillator_value = oscillator.lerp(index);

                // 2. LFO
                let ms = get_now_millis() - t0_ms;
                let lfo = LFO::new(30, 800, 0.7);
                let lfo_value = lfo.get_value(ms);

                oscillator_value * lfo_value
            }
        }
    }
}
