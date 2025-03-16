use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::utils::timing::get_now_millis;

pub struct SynthChain {
    oscillator: WaveTableOscillator,
    lfo: LFO,
    t0_ms: u128,
}
impl SynthChain {
    pub fn new(
        //
        oscillator: WaveTableOscillator,
        lfo: LFO,
        t0_ms: u128,
    ) -> Self {
        Self {
            //
            oscillator,
            lfo,
            t0_ms,
        }
    }
    pub fn get_sample(&self, index: f32) -> f32 {
        // 1. Oscillator
        let oscillator_value = self.oscillator.lerp(index);

        // 2. LFO
        let ms = get_now_millis() - self.t0_ms;
        let lfo_value = self.lfo.get_value(ms);

        oscillator_value * lfo_value
    }
}
