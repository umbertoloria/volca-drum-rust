use crate::synth::audio_channel::AudioChannel;
use crate::synth::digital_mono_synth_sample_source::DigitalMonoSynthSamplesConverter;
use rodio::Sink;

pub struct MonoSynthPlayer<'a> {
    volume: f32,
    audio_channel: &'a AudioChannel,
    sink: Sink,
}
impl<'a> MonoSynthPlayer<'a> {
    pub fn new(audio_channel: &'a AudioChannel, volume: f32) -> MonoSynthPlayer {
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
