use crate::synth::lfo::lfo::LFO;
use crate::utils::timing::get_now_millis;
use core::time::Duration;
use rodio::source::{SamplesConverter, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};

// Oscillator
pub struct WTOscillator {
    wave_table: Vec<f32>,
}
impl WTOscillator {
    pub fn new(
        //
        wave_table: Vec<f32>,
    ) -> Self {
        Self {
            //
            wave_table,
        }
    }
    pub fn get_wave_table_len(&self) -> usize {
        self.wave_table.len()
    }
    pub fn get_from_wave_table(&self, index: usize) -> f32 {
        self.wave_table[index]
    }
    pub fn lerp(&self, index: f32) -> f32 {
        let truncated_index = index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}

// Sample & Source
pub struct DigitalMonoSynthSampleSource {
    sample_rate: u32,
    oscillator: WTOscillator,
    now_millis: u128,
    lfo: LFO,
    index: f32,
    index_increment: f32,
}
impl DigitalMonoSynthSampleSource {
    pub fn new(
        //
        sample_rate: u32,
        oscillator: WTOscillator,
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

// Mono Synth
pub struct DigitalMonoSynth {
    volume: f32,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}
impl DigitalMonoSynth {
    pub fn new(volume: f32) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = create_empty_sink(&stream_handle, volume);
        Self {
            volume,
            stream,
            stream_handle,
            sink,
        }
    }
    pub fn play(&mut self, sample: DigitalMonoSynthSamplesConverter) {
        self.sink = create_empty_sink(&self.stream_handle, self.volume);
        self.sink.append(sample);
    }
    pub fn pause(&mut self) {
        self.sink = create_empty_sink(&self.stream_handle, self.volume);
    }
}
fn create_empty_sink(stream_handle: &OutputStreamHandle, volume: f32) -> Sink {
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(volume);
    sink
}
