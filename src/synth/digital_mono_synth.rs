use core::time::Duration;
use rodio::source::{SamplesConverter, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}
impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> Self {
        Self {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}
impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
impl Iterator for WaveTableOscillator {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}
pub type WaveTableOscillatorSample = SamplesConverter<WaveTableOscillator, f32>;

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
    pub fn play(&mut self, sample: WaveTableOscillatorSample) {
        self.sink = create_empty_sink(&self.stream_handle, self.volume);
        self.sink.append(sample);
        // sleep(Duration::from_millis(250));
        // self.sink.stop();
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
