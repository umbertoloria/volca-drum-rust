use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::utils::timing::get_now_millis;

pub struct MonoSynth {
    oscillator: WaveTableOscillator,
    lfo: LFO,
    sample_rate: u32,
    t0_ms: u128,
    index: f32,
    index_increment: f32,
}
impl MonoSynth {
    pub fn new(
        //
        oscillator: WaveTableOscillator,
        lfo: LFO,
        sample_rate: u32,
        t0_ms: u128,
    ) -> Self {
        Self {
            //
            oscillator,
            lfo,
            sample_rate,
            t0_ms,
            index: 0.0,
            index_increment: 0.0,
        }
    }
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }
    pub fn get_sample_and_prepare_next(&mut self) -> f32 {
        let index = self.get_index_and_prepare_next();

        // Synth Chain:
        // 1. Oscillator
        let oscillator_value = self.oscillator.lerp(index);
        // 2. LFO
        let ms = get_now_millis() - self.t0_ms;
        let lfo_value = self.lfo.get_value(ms);

        oscillator_value * lfo_value
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment =
            frequency * self.oscillator.get_wave_table_len() as f32 / self.sample_rate as f32;
    }
    pub fn get_index_and_prepare_next(&mut self) -> f32 {
        let result = self.index;
        self.index += self.index_increment;
        self.index %= self.oscillator.get_wave_table_len() as f32;
        result
    }
}
