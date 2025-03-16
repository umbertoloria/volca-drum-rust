use crate::synth::digital_mono_synth_sample_source::DigitalMonoSynthSamplesConverter;
use rodio::{OutputStream, OutputStreamHandle, Sink};

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
