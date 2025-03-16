use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::utils::timing::get_now_millis;
use rodio::source::SamplesConverter;
use rodio::Source;
use std::time::Duration;

pub struct DigitalMonoSynthSampleSource {
    sample_rate: u32,
    oscillator: WaveTableOscillator,
    t0_ms: u128,
    lfo: LFO,
    index: f32,
    index_increment: f32,
}
impl DigitalMonoSynthSampleSource {
    pub fn new(
        //
        sample_rate: u32,
        oscillator: WaveTableOscillator,
        lfo: LFO,
        t0_ms: u128,
    ) -> Self {
        Self {
            sample_rate,
            oscillator,
            t0_ms,
            lfo,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment =
            frequency * self.oscillator.get_wave_table_len() as f32 / self.sample_rate as f32;
    }

    fn get_sample_and_prepare_next(&mut self) -> f32 {
        let index = self.get_index_and_prepare_next();

        // Synth Chain:
        // 1. Oscillator
        let oscillator_value = self.oscillator.lerp(index);
        // 2. LFO
        let ms = get_now_millis() - self.t0_ms;
        let lfo_value = self.lfo.get_value(ms);

        oscillator_value * lfo_value
    }

    fn get_index_and_prepare_next(&mut self) -> f32 {
        let result = self.index;
        self.index += self.index_increment;
        self.index %= self.oscillator.get_wave_table_len() as f32;
        result
    }
}
impl Source for DigitalMonoSynthSampleSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
impl Iterator for DigitalMonoSynthSampleSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Gently raise Oscillator Phase to avoid Audio Monitors Issues
        Some(self.get_sample_and_prepare_next())
    }
}
pub type DigitalMonoSynthSamplesConverter = SamplesConverter<DigitalMonoSynthSampleSource, f32>;
