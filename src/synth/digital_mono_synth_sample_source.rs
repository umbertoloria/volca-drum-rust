use crate::synth::mono_synth::MonoSynth;
use rodio::source::SamplesConverter;
use rodio::Source;
use std::time::Duration;

pub struct DigitalMonoSynthSampleSource {
    mono_synth: MonoSynth,
}
impl DigitalMonoSynthSampleSource {
    pub fn new(mono_synth: MonoSynth) -> Self {
        Self { mono_synth }
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
        self.mono_synth.get_sample_rate()
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
impl Iterator for DigitalMonoSynthSampleSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Gently raise Oscillator Phase to avoid Audio Monitors Issues
        Some(self.mono_synth.get_sample_and_prepare_next())
    }
}
pub type DigitalMonoSynthSamplesConverter = SamplesConverter<DigitalMonoSynthSampleSource, f32>;
