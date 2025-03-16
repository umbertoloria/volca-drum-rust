use crate::utils::timing::get_now_millis;
use core::time::Duration;
use rodio::source::{SamplesConverter, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};

// Oscillator
pub struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    now_millis: u128,
    index: f32,
    index_increment: f32,
}
impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>, now_millis: u128) -> Self {
        Self {
            sample_rate,
            wave_table,
            now_millis,
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

        let mut lfo: f32 = 1.0;

        // LFO
        let ms = get_now_millis() - self.now_millis;

        let lfo_attack__ms = 17;
        let lfo_decay___ms = 200;
        let lfo_sustain_vl = 0.3f32;

        if ms <= lfo_attack__ms {
            // From 0.0 to 1.0.
            lfo = (ms as f32) / lfo_attack__ms as f32;
        } else if ms <= lfo_decay___ms {
            // From 1.0 to "lfo_sustain_vl".
            let delta = (ms - lfo_attack__ms) as f32 / (lfo_decay___ms - lfo_attack__ms) as f32;
            let diff_attach_and_sustain = 1.0 - lfo_sustain_vl;
            lfo = 1.0 - delta * diff_attach_and_sustain;
        } else {
            lfo = lfo_sustain_vl;
        }
        // println!("{:.5}", lfo);

        // TODO: Gently raise Oscillator Phase to avoid Audio Monitors Issues
        sample * lfo
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
impl Iterator for WaveTableOscillator {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}
pub type WaveTableOscillatorSample = SamplesConverter<WaveTableOscillator, f32>;

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
