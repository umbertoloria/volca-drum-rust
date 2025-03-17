use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::synth::oscillator::wave_tables::{create_wt_saw, create_wt_square};
use crate::utils::timing::get_now_millis;

#[derive(Clone)]
pub enum SynthPatch {
    SQUARE,
    SAW,
    MetronomeClick,
}
impl SynthPatch {
    pub fn get_sample(&self, t0_ms: u128, index: f32) -> f32 {
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
            SynthPatch::MetronomeClick => {
                // 1. Oscillator
                let oscillator = WaveTableOscillator::new(create_wt_square());
                let oscillator_value = oscillator.lerp(index);

                // 2. LFO
                let ms = get_now_millis() - t0_ms;
                let lfo = LFO::new(10, 300, 0.0);
                let lfo_value = lfo.get_value(ms);

                oscillator_value * lfo_value
            }
        }
    }
}
