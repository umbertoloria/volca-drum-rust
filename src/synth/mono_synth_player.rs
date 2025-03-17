use crate::synth::audio_channel::AudioChannel;
use crate::synth::digital_mono_synth_sample_source::DigitalMonoSynthSamplesConverter;
use rodio::Sink;

pub struct MonoSynthPlayer {
    volume: f32,
    audio_channel: AudioChannel,
    sink: Sink,
}
impl MonoSynthPlayer {
    pub fn new(volume: f32) -> Self {
        // TODO: Avoid creating OutputStream channel every time!
        let audio_channel = AudioChannel::new();
        let sink = audio_channel.create_sink(volume); // Empty Sink.
        Self {
            volume,
            audio_channel,
            sink,
        }
    }
    pub fn play(&mut self, sample: DigitalMonoSynthSamplesConverter) {
        self.sink = self.audio_channel.create_sink(self.volume);
        self.sink.append(sample);
    }
    pub fn pause(&mut self) {
        self.sink = self.audio_channel.create_sink(self.volume);
    }
}
