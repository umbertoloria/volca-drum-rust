use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::utils::timing::get_now_millis;
use rodio::source::SamplesConverter;
use rodio::Source;
use std::time::Duration;

pub struct DigitalMonoSynthSampleSource {
    sample_rate: u32,
    oscillator: WaveTableOscillator,
    now_millis: u128,
    lfo: LFO,
    index: f32,
    index_increment: f32,
}
impl DigitalMonoSynthSampleSource {
    pub fn new(
        sample_rate: u32,
        oscillator: WaveTableOscillator,
        lfo: LFO,
        now_millis: u128,
    ) -> Self {
        Self {
            sample_rate,
            oscillator,
            now_millis,
            lfo,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment =
            frequency * self.oscillator.get_wave_table_len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.oscillator.get_wave_table_len() as f32;

        // LFO
        let ms = get_now_millis() - self.now_millis;

        let lfo_value = self.lfo.get_value(ms);
        // println!("{:.5}", lfo);

        // TODO: Gently raise Oscillator Phase to avoid Audio Monitors Issues
        sample * lfo_value
    }

    fn lerp(&self) -> f32 {
        self.oscillator.lerp(self.index)
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
        Some(self.get_sample())
    }
}
pub type DigitalMonoSynthSamplesConverter = SamplesConverter<DigitalMonoSynthSampleSource, f32>;
